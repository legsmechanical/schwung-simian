/*
 * SIMIAN — Schwung plugin_api_v2 wrapper around OneTrick SIMIAN 2's Faust DSP.
 *
 * Upstream: OneTrick SIMIAN 2.0.2, Punk Labs LLC, GPL-3.0-or-later
 * (https://punklabs.com/ot-simian). The synth IS the Faust code: drum.dsp is
 * ONE drum voice — a triangle/cymbal-sample oscillator and a resonant-lowpass
 * noise source, crossfaded, under a shared decay envelope with a click
 * transient — and output.dsp is the global drive / saturation / limiter and
 * the Zita reverb the voices send into. Both are compiled to C++ by
 * scripts/gen_dsp.sh and committed under generated/ — never hand-edited.
 *
 * What this file replaces is upstream's nih-plug/egui host: the ten voices,
 * the note map, the choke arbitration, and the parameter surface Schwung
 * reads.
 *
 * VOICE MODEL — TEN fixed voices, and they are ten copies of one DSP.
 * Kick, Rimshot, Snare, Clap, Low/Mid/High Tom, HH Closed, HH Open, Cymbal.
 * Nothing distinguishes a kick from a hihat except its parameter values; a
 * "kit" is those ten parameter sets, which is exactly what upstream's .preset
 * files carry. They are declared in NOTE ORDER (36,37,38,39,41,42,45,46,48,49)
 * so the Move's drum grid ascends the way every other drum rack does.
 *
 * TRIGGER is edge-detected inside Faust (triggerAndVelocity fires only while
 * Trigger > Trigger'), so a hit is a 0 -> velocity transition the DSP has to
 * SEE. A pad press and its release can both land between two render calls, and
 * upstream's "write velocity, write 0 on note-off" would then be one value and
 * no hit at all. Every note-on here therefore forces the edge: the zone goes
 * to 0 now and render_block raises it after exactly one frame. That costs one
 * sample of delay on the attack and makes a repeated note ALWAYS sound, which
 * for a drum machine is the only acceptable answer.
 *
 * BUSES — the module publishes its ten voices (split_voices) and renders them
 * apart on demand. ⚠ A voice routed to a bus leaves PRE-MASTER: it has its own
 * Gain/Pan/Saturation but not the global Drive, Saturation, limiter or Gain,
 * because those act on a sum that no longer exists once a voice is pulled out
 * of it. The reverb return and every unrouted voice still go through the
 * master stage into main_out. See README.md, "Routing".
 */

#include <cstdint>
#include <cstdio>
#include <cstring>
#include <cstdlib>
#include <cmath>
#include <new>

#include "faust_shim.h"
#include "generated/dsp_drum.hpp"
#include "generated/dsp_output.hpp"
#include "factory_bank.h"     /* SIMIAN_FACTORY[] — generated from Punk Labs' .preset files */

/* ---- host / plugin ABI (mirrors src/host/plugin_api_v1.h in the host) ---- */
#ifndef MOVE_SAMPLE_RATE
#define MOVE_SAMPLE_RATE 44100
#endif
#ifndef MOVE_FRAMES_PER_BLOCK
#define MOVE_FRAMES_PER_BLOCK 128
#endif

typedef struct host_api_v1 {
    uint32_t api_version;
    int sample_rate;
    int frames_per_block;
    uint8_t *mapped_memory;
    int audio_out_offset;
    int audio_in_offset;
    void (*log)(const char *msg);
    int (*midi_send_internal)(const uint8_t *msg, int len);
    int (*midi_send_external)(const uint8_t *msg, int len);
    int (*get_clock_status)(void);
    void *mod_emit_value;
    void *mod_clear_source;
    void *mod_host_ctx;
    float (*get_bpm)(void);
    int (*midi_inject_to_move)(const uint8_t *msg, int len);
    int (*slot_recv_channel)(void *instance);
    /* Appended by the host after slot_recv_channel; unused here, but the tail
     * is carried so this copy stays layout-identical to the real header. */
    double (*get_beat_position)(void);
    int (*midi_send_internal_slot)(int slot, const uint8_t *msg, int len);
    int (*clock_output_enabled)(void);
} host_api_v1_t;

typedef struct plugin_api_v2 {
    uint32_t api_version;
    void *(*create_instance)(const char *module_dir, const char *json_defaults);
    void (*destroy_instance)(void *instance);
    void (*on_midi)(void *instance, const uint8_t *msg, int len, int source);
    void (*set_param)(void *instance, const char *key, const char *val);
    int (*get_param)(void *instance, const char *key, char *buf, int buf_len);
    int (*get_error)(void *instance, char *buf, int buf_len);
    void (*render_block)(void *instance, int16_t *out_interleaved_lr, int frames);
} plugin_api_v2_t;

#define MOVE_PLUGIN_API_VERSION_2 2

static const host_api_v1_t *g_host = NULL;

/* ======================================================================== *
 *  Pads and voices — TEN DSP voices seated on SIXTEEN pads
 *
 *  A drum rack on Move is sixteen pads at notes 36..51, and there are ten
 *  drums, so six pads are ALIASES: a second pad addressing a voice another
 *  pad already owns, at its own pitch offset. Upstream does the same thing
 *  through General MIDI — it routes 40 to the snare and 47 to the mid tom —
 *  but GM's pairs are +2 semitones apart, and in a 4-wide grid a +2 neighbour
 *  is never an adjacent PAD. So the seating is chosen here instead, and every
 *  alias sits next to its parent:
 *
 *      48 HI TOM*   49 CYMBAL    50 CYMBAL*   51 MID TOM*
 *      44 HI TOM    45 LOW TOM*  46 HH OPEN   47 MID TOM
 *      40 CLAP      41 LOW TOM   42 HH CL     43 HH CL*
 *      36 KICK      37 RIM       38 SNARE     39 SNARE*
 *
 *  Snare and the three toms each keep the one alias upstream gives them. The
 *  two extra fall on HH Closed and Cymbal, which is what the two notes
 *  upstream leaves SILENT already mean (44 pedal hat, 51 ride) — every pad
 *  here sounds, as it does in Move's own kits.
 *
 *  ⚠ THE COST, SAID OUT LOUD: these are NOT General MIDI notes. An imported
 *  GM drum loop plays the wrong drums. Upstream's map is one table away.
 *
 *  ⚠ A PAD IS NOT A VOICE, and the two are addressed differently:
 *    - Everything that makes a sound belongs to the VOICE, so both pads of a
 *      pair share it. Turning Snare 2's Decay turns Snare's, because there is
 *      one snare.
 *    - `tune` belongs to the PAD. It is the only thing an alias owns, and it
 *      is what stops a second pad being a duplicate.
 * ======================================================================== */
#define SIMIAN_VOICES 10
#define SIMIAN_PADS   16

typedef struct {
    const char *label;
    const char *role;    /* free-form hint for a consumer seating a grid */
} voice_def_t;

static const voice_def_t VOICES[SIMIAN_VOICES] = {
    {"Kick",      "kick"},
    {"Rimshot",   "rim"},
    {"Snare",     "snare"},
    {"Clap",      "clap"},
    {"Low Tom",   "tom"},
    {"HH Closed", "hihat_closed"},
    {"Mid Tom",   "tom"},
    {"HH Open",   "hihat_open"},
    {"High Tom",  "tom"},
    {"Cymbal",    "cymbal"},
};

enum { V_KICK, V_RIM, V_SNARE, V_CLAP, V_LOWTOM, V_HHCLOSED,
       V_MIDTOM, V_HHOPEN, V_HITOM, V_CYMBAL };

/* `id` is the prefix every one of this pad's parameters carries (child_prefix
 * "pad" + a 1-BASED index, matching child_index_base) and, for the ten pads
 * that own a voice outright, what split_voices publishes. The host
 * substitutes an id into "{id}_send_a" VERBATIM, so those two numberings must
 * agree — a mismatch puts every send on the pad next door and nothing errors.
 *
 * ⚠ NEVER REORDER: the order is the note order, the grid order and the state
 * order at once.
 *
 * `choke` groups pads the way a Move kit does. A hit silences every OTHER
 * VOICE in its group; two pads of one voice need no help, because a second
 * hit on one DSP retriggers it. So group 2 (the cymbals) is declared and
 * inert today, and earns its keep the moment the cymbal is ever split. */
typedef struct {
    const char *id;
    const char *label;   /* what the device shows — keep it under 9 chars */
    int         voice;
    int         note;
    float       tune;    /* default semitone offset; nonzero IS the alias */
    int         choke;   /* 0 = none */
} pad_def_t;

static const pad_def_t PADS[SIMIAN_PADS] = {
    {"pad1",  "Kick",     V_KICK,      36,  0.0f, 0},
    {"pad2",  "Rim",      V_RIM,       37,  0.0f, 0},
    {"pad3",  "Snare",    V_SNARE,     38,  0.0f, 0},
    {"pad4",  "Snare 2",  V_SNARE,     39,  2.0f, 0},   /* upstream's +2 */
    {"pad5",  "Clap",     V_CLAP,      40,  0.0f, 0},
    {"pad6",  "Lo Tom",   V_LOWTOM,    41,  0.0f, 0},
    {"pad7",  "HH Cl",    V_HHCLOSED,  42,  0.0f, 1},
    {"pad8",  "HH Cl 2",  V_HHCLOSED,  43,  3.0f, 1},   /* ours: a tighter hat */
    {"pad9",  "Hi Tom",   V_HITOM,     44,  0.0f, 0},
    {"pad10", "Lo Tom 2", V_LOWTOM,    45,  2.0f, 0},   /* upstream's +2 */
    {"pad11", "HH Open",  V_HHOPEN,    46,  0.0f, 1},
    {"pad12", "Mid Tom",  V_MIDTOM,    47,  0.0f, 0},
    {"pad13", "Hi Tom 2", V_HITOM,     48,  2.0f, 0},   /* upstream's +2 */
    {"pad14", "Cymbal",   V_CYMBAL,    49,  0.0f, 2},
    {"pad15", "Cymbal 2", V_CYMBAL,    50,  7.0f, 2},   /* ours: a ride-ish ping */
    {"pad16", "Mid Tom2", V_MIDTOM,    51,  2.0f, 0},   /* upstream's +2 */
};

/* The pad that OWNS each voice — the first one declaring it. Its id is the
 * canonical key for every shared parameter (state, chain_params) and the id
 * split_voices publishes, because an alias has no audio path of its own. */
static int voice_owner_pad(int v) {
    for (int p = 0; p < SIMIAN_PADS; p++) if (PADS[p].voice == v) return p;
    return 0;
}

/* The OTHER pad on this pad's voice, or -1 when it has none. This is what
 * makes the sharing visible: the widget on the Voice knob draws a chain link
 * and the partner's number, so "editing this also edits that" is on screen
 * instead of being something you have to know. */
static int pad_partner(int pad) {
    for (int q = 0; q < SIMIAN_PADS; q++)
        if (q != pad && PADS[q].voice == PADS[pad].voice) return q;
    return -1;
}

static int note_to_pad(int note) {
    for (int p = 0; p < SIMIAN_PADS; p++) if (PADS[p].note == note) return p;
    return -1;
}

/* ======================================================================== *
 *  Parameter surface
 *
 *  Values are stored and exchanged in DISPLAY units — the same numbers the
 *  .preset files carry — and written straight into the Faust zone, because
 *  params.lib already declares its sliders in those units (hz, %, dB, st, ms).
 *  There is no normalized layer to get wrong.
 * ======================================================================== */

/* One voice's parameters. The wire key is "<voice id>_<key>"; the hierarchy
 * addresses them by the bare key, which child_key_template expands.
 *
 * ⚠ ORDER IS THE STATE AND BANK ORDER. tools/gen_factory_bank.mjs indexes the
 * generated table by position, so inserting a row without regenerating loads
 * every later value one slot out, silently. Append, never insert. */
typedef struct {
    const char *key;      /* Schwung param key, without the voice prefix */
    const char *name;     /* label shown on the device                   */
    const char *zone;     /* Faust label in drum.dsp, or NULL for host-side */
    float       min, max, def;
    const char *unit;     /* "hz", "%", "dB", "st", "ms", or NULL        */
} vparam_def_t;

static const vparam_def_t VOICE_PARAMS[] = {
    /* --- Tone: the oscillator. Waveform morphs triangle -> cymbal sample. --- */
    {"pitch",    "Pitch",    "OscPitch",               30,  4400, 220, "hz"},
    {"wave",     "Wave",     "OscWaveform",             0,   100,   0, "%"},
    {"bend",     "Bend",     "OscEnvBend",            -36,    36,   0, "st"},
    {"bend_dyn", "Bend Dyn", "OscEnvBendDynamics",      0,   100, 100, "%"},

    /* --- Noise: a resonant lowpass on white noise, with its own env bend.
     * `cutoff` is also what the mod wheel sweeps (upstream's DJ filter). --- */
    {"cutoff",   "Cutoff",   "LowpassFreq",           440, 15000, 10000, "hz"},
    {"res",      "Res",      "LowpassQ",                0,   100,   0, "%"},
    {"lp_bend",  "LP Bend",  "LowpassEnvBend",        -36,    36,   0, "st"},
    {"lp_dyn",   "LP Dyn",   "LowpassEnvBendDynamics",  0,   100, 100, "%"},

    /* --- Envelope. One decay drives amplitude, pitch and filter together;
     * Punch bends its slope. Decay is scaled by velocity (70% dynamics, hard
     * coded upstream) and by the pitch wheel's staccato. --- */
    {"decay",    "Decay",    "EnvDecay",               10,  2000, 200, "ms"},
    {"punch",    "Punch",    "EnvPunch",                0,   100,   0, "%"},

    /* --- The two crossfades that make one voice a kick or a hat. --- */
    {"noise",    "Tone/Nse", "FadeToneNoise",           0,   100,  25, "%"},
    {"click",    "Click",    "FadeClick",               0,   100,  50, "%"},

    /* --- Mix. Gain's floor is -60 rather than upstream's -100: below that the
     * knob is scrolling through inaudible, and a preset cannot carry a value
     * the player cannot get back from. --- */
    {"volume",   "Volume",   "Gain",                  -60,     6,   0, "dB"},
    {"pan",      "Pan",      "Pan",                  -100,   100,   0, "%"},
    {"vel_vol",  "Vel Vol",  "GainDynamics",            0,   100, 100, "%"},
    {"sat",      "Sat",      "Saturation",              0,   100,   0, "%"},
    {"reverb",   "Reverb",   "SendReverb",              0,   100,   0, "%"},

    /* --- The host's two return buses. No Faust zone: the host READS these
     * levels off us (voice_send_params) and does the routing itself. --- */
    {"send_a",   "Send A",   NULL,                      0,   100,   0, "%"},
    {"send_b",   "Send B",   NULL,                      0,   100,   0, "%"},
};
#define VP_COUNT ((int)(sizeof(VOICE_PARAMS) / sizeof(VOICE_PARAMS[0])))

/* The global stage. `zone` names a control in output.dsp; NULL is wrapper-side.
 *
 * ⚠ Sensitivity and Transpose have NO ZONE and never did. output.dsp declares
 * Sensitivity_Global and onetrick.lib declares Transpose, but nothing reads
 * either, so Faust prunes them both and upstream's writes land nowhere —
 * Global Transpose is dead in OneTrick SIMIAN 2.0.2. Both are applied here
 * instead, at note-on, which is where they were always meant to act. */
typedef struct {
    const char *key;
    const char *name;
    const char *zone;
    float       min, max, def;
    const char *unit;
} gparam_def_t;

static const gparam_def_t GLOBAL_PARAMS[] = {
    {"gain",      "Gain",      "Gain_Global",       -60,   6,   0, "dB"},
    {"drive",     "Drive",     "Drive_Global",      -60,   6,  -6, "dB"},
    {"master_sat","Sat",       "Saturation_Global",   0, 100,   0, "%"},
    {"rev_size",  "Rev Size",  "ReverbSize_Global",   0, 100,  50, "%"},
    {"rev_gate",  "Rev Gate",  "ReverbGate_Global", -100,  0, -100, "dB"},
    {"sens",      "Vel Sens",  NULL,                  0, 100, 100, "%"},
    {"transpose", "Transpose", NULL,                -12,  12,   0, "st"},
};
#define GP_COUNT ((int)(sizeof(GLOBAL_PARAMS) / sizeof(GLOBAL_PARAMS[0])))

/* The one parameter a PAD owns rather than shares. An alias pad is nothing
 * but its parent voice plus this number, so it is the control that makes the
 * second pad worth having — and it is per pad by definition: two pads with
 * one tune would be the duplicate we started with. */
static const vparam_def_t PAD_PARAMS[] = {
    {"tune", "Tune", NULL, -24, 24, 0, "st"},
};
#define PP_COUNT ((int)(sizeof(PAD_PARAMS) / sizeof(PAD_PARAMS[0])))

/* The focus knob. It is not a sound parameter, but it IS a knob on a page, so
 * it needs a chain_params entry: without one the host guesses its type and
 * range, and a mis-guessed range is a PAD knob that cannot reach pad 16. */
static const gparam_def_t UI_PARAMS[] = {
    /* ⚠ It indexes PADS[], 1..16 — but it is called "voice" because that is
     * what the player is choosing: which DRUM to edit. Two pads can be the
     * same drum, and calling this "Pad" made them read as separate
     * instruments when every control but Tune is shared. The name is the
     * player's word; the index is a pad ordinal. (Josh, 2026-09-21.)
     *
     * 16, not SIMIAN_PADS: the generators parse this table with a regex that
     * only reads numeric literals, and a named constant makes the row
     * invisible to them. */
    {"ui_current_voice", "Voice", NULL, 1, 16, 1, NULL},
};
#define UI_COUNT ((int)(sizeof(UI_PARAMS) / sizeof(UI_PARAMS[0])))

static int find_voice_param(const char *key) {
    for (int i = 0; i < VP_COUNT; i++)
        if (strcmp(VOICE_PARAMS[i].key, key) == 0) return i;
    return -1;
}

static int find_global_param(const char *key) {
    for (int i = 0; i < GP_COUNT; i++)
        if (strcmp(GLOBAL_PARAMS[i].key, key) == 0) return i;
    return -1;
}

/* "pad7_cutoff" -> pad index 6, and the bare key "cutoff". The ids are "pad"
 * plus a 1-based number, so this parses rather than table-scans; a two-digit
 * index must not be read as one. Returns the bare key, or NULL. */
static const char *split_pad_key(const char *key, int *out_pad) {
    if (strncmp(key, "pad", 3) != 0) return NULL;
    const char *p = key + 3;
    if (*p < '1' || *p > '9') return NULL;
    int n = 0;
    while (*p >= '0' && *p <= '9') { n = n * 10 + (*p - '0'); p++; }
    if (*p != '_' || n < 1 || n > SIMIAN_PADS) return NULL;
    *out_pad = n - 1;
    return p + 1;
}

static int find_pad_param(const char *key) {
    for (int i = 0; i < PP_COUNT; i++)
        if (strcmp(PAD_PARAMS[i].key, key) == 0) return i;
    return -1;
}

/* ======================================================================== *
 *  Instance
 * ======================================================================== */
#define SIMIAN_BUF (MOVE_FRAMES_PER_BLOCK * 4)

/* Upstream's SilenceTracker: a voice whose output has been below 0.001 for
 * this long stops being computed at all. Ten Faust instances is the whole CPU
 * budget of this module, and a drum machine is silent most of the time. */
#define SIMIAN_SILENCE_MS 100

typedef struct {
    DSP_Drum    dsp;
    ZoneMap     zones;

    FAUSTFLOAT *zone[VP_COUNT];      /* resolved once; NULL for host-side rows */
    float       value[VP_COUNT];     /* display units */

    FAUSTFLOAT *z_trigger, *z_choke, *z_key, *z_wake, *z_modwheel, *z_pitchwheel;

    float       pending_trigger;     /* velocity waiting for a forced edge, or -1 */
    int         silence_count;       /* consecutive quiet samples */
    int         active;

    float       out3[3][SIMIAN_BUF]; /* L, R, reverb send */
} simian_voice_t;

typedef struct {
    simian_voice_t voice[SIMIAN_VOICES];

    DSP_Output  out;
    ZoneMap     out_zones;
    FAUSTFLOAT *gzone[GP_COUNT];
    float       gvalue[GP_COUNT];
    FAUSTFLOAT *z_out_wake;

    float       pad_tune[SIMIAN_PADS];   /* per-pad semitone offset */

    int         silence_samples;     /* SIMIAN_SILENCE_MS in samples */
    int         cur_preset;          /* index into SIMIAN_FACTORY, -1 = defaults */
    int         octave_transpose;
    char        module_dir[512];

    /* Focus following — see the live-press block in v2_set_param. */
    int         ui_current_voice;    /* a PAD index, 0-based; 1-based on the wire */
    int         ui_auto_select;
    int         live_armed;
    long        live_arm_block;
    int         last_hit_pad;
    long        last_hit_block;
    int         have_vouched;    /* any host has ever vouched */
    long        last_vouch_block;
    long        block;

    float       accL[SIMIAN_BUF], accR[SIMIAN_BUF], accFX[SIMIAN_BUF];
    float       mixL[SIMIAN_BUF], mixR[SIMIAN_BUF];
} simian_t;

/* A vouch and the note it belongs to arrive on different paths a few
 * milliseconds apart. Measured on DR32's device runs: 20 blocks x 2.902 ms =
 * 58 ms, with ~12.5% of presses still missing it. */
#define SIMIAN_LIVE_MATCH_BLOCKS 20

/* ⚠ A vouching host must EXPIRE, or pad-follow dies silently.
 *
 * DR32 latches `host_vouches` for the life of the instance: once anything has
 * vouched once, a bare note-on never moves focus again. The symptom is a
 * browser that follows the pads perfectly and then quietly stops, because
 * every trip out to the knob grid vouches. Here the latch decays instead: a
 * host that vouched a minute ago and has gone quiet is treated as a host that
 * does not vouch, and bare-note following comes back.
 * (~5 s at 128-frame blocks; the exact figure is not load-bearing.) */
#define SIMIAN_VOUCH_TTL_BLOCKS 1700

static float clampf(float v, float lo, float hi) { return v < lo ? lo : (v > hi ? hi : v); }

static void set_voice_value(simian_t *inst, int v, int i, float display) {
    const vparam_def_t *p = &VOICE_PARAMS[i];
    float x = clampf(display, p->min, p->max);
    inst->voice[v].value[i] = x;
    if (inst->voice[v].zone[i]) *inst->voice[v].zone[i] = (FAUSTFLOAT)x;
}

static void set_pad_value(simian_t *inst, int pad, int i, float display) {
    const vparam_def_t *p = &PAD_PARAMS[i];
    inst->pad_tune[pad] = clampf(display, p->min, p->max);   /* one row today */
}

static void set_global_value(simian_t *inst, int i, float display) {
    const gparam_def_t *p = &GLOBAL_PARAMS[i];
    float x = clampf(display, p->min, p->max);
    inst->gvalue[i] = x;
    if (inst->gzone[i]) *inst->gzone[i] = (FAUSTFLOAT)x;
}

/* ======================================================================== *
 *  Voice control — transcribed from onetrick/src/faust/wrapper.rs
 * ======================================================================== */

/* Upstream burns a sample with the gate down and WakeUp toggled so the
 * smoothed parameters restart from their real values. We keep WakeUp pinned
 * on: the Schwung host renders continuously, so there is no idle state to come
 * back from, and the burn would cost a sample of audio per hit. Waking a voice
 * here means only "start computing it again". */
static void voice_wake(simian_voice_t *v) {
    if (v->z_wake) *v->z_wake = 1.0f;
    v->silence_count = 0;
    v->active = 1;
}

static void voice_note_on(simian_t *inst, int vi, float note_offset, float velocity) {
    simian_voice_t *v = &inst->voice[vi];
    voice_wake(v);
    if (v->z_choke) *v->z_choke = 0.0f;          /* unchoke: a hit cancels a choke */
    if (v->z_key)   *v->z_key   = (FAUSTFLOAT)note_offset;
    /* Force the rising edge rather than trusting the note-off to have re-armed
     * it — see the TRIGGER note at the top of the file. */
    if (v->z_trigger) *v->z_trigger = 0.0f;
    v->pending_trigger = velocity;
}

static void voice_choke(simian_voice_t *v) {
    if (v->z_choke) *v->z_choke = 1.0f;
    if (v->z_trigger) *v->z_trigger = 0.0f;
    v->pending_trigger = -1.0f;
}

/* A hit silences every OTHER voice in the pad's choke group, the way a Move
 * kit's chokeGroup does. Its own voice is skipped: the two pads of a pair
 * share one DSP, and a second hit on it already restarts the envelope — so
 * choking it here would cut the hit that just happened. */
static void choke_group_of(simian_t *inst, int pad) {
    int g = PADS[pad].choke;
    if (!g) return;
    for (int q = 0; q < SIMIAN_PADS; q++)
        if (PADS[q].choke == g && PADS[q].voice != PADS[pad].voice)
            voice_choke(&inst->voice[PADS[q].voice]);
}

static void all_notes_off(simian_t *inst) {
    for (int i = 0; i < SIMIAN_VOICES; i++) {
        simian_voice_t *v = &inst->voice[i];
        if (v->z_trigger) *v->z_trigger = 0.0f;
        if (v->z_choke)   *v->z_choke   = 0.0f;
        v->pending_trigger = -1.0f;
    }
}

/* ======================================================================== *
 *  Presets
 * ======================================================================== */
static int preset_count(void) { return SIMIAN_FACTORY_COUNT; }

static void load_preset(simian_t *inst, int idx) {
    if (idx < 0 || idx >= SIMIAN_FACTORY_COUNT) return;
    const simian_preset_t *pr = &SIMIAN_FACTORY[idx];
    for (int g = 0; g < GP_COUNT; g++) set_global_value(inst, g, pr->global[g]);
    for (int v = 0; v < SIMIAN_VOICES; v++)
        for (int i = 0; i < VP_COUNT; i++) set_voice_value(inst, v, i, pr->voice[v][i]);
    inst->cur_preset = idx;
}

static void load_defaults(simian_t *inst) {
    /* A pad's tune default is the pad's, not the parameter's: a nonzero one
     * IS what makes that pad an alias rather than a duplicate. */
    for (int d = 0; d < SIMIAN_PADS; d++) inst->pad_tune[d] = PADS[d].tune;
    for (int g = 0; g < GP_COUNT; g++) set_global_value(inst, g, GLOBAL_PARAMS[g].def);
    for (int v = 0; v < SIMIAN_VOICES; v++)
        for (int i = 0; i < VP_COUNT; i++) set_voice_value(inst, v, i, VOICE_PARAMS[i].def);
    inst->cur_preset = -1;
}

/* ======================================================================== *
 *  UI surface
 *
 *  Modelled on DR32: one child level per page, all three sharing ONE
 *  child_index_param, so they are three views of the same voice.
 *
 *  🔴 child_note_base / child_notes goes on ONE LEVEL ONLY. voicesOf emits a
 *  voice per child of every level that declares a note map, so declaring it on
 *  all three would publish 30 voices and seat three copies of the kit. The
 *  pages would render perfectly; only pages_check's voice list catches it.
 *
 *  ⚠ No key appears on two levels. A repeated hierarchy key makes the C loader
 *  drop ALL metadata for it, which silently kills the per-voice sends.
 * ======================================================================== */
#define CHILD_COMMON \
    "\"child_prefix\":\"pad\",\"child_count\":16,\"child_index_base\":1," \
    "\"child_label\":\"Voice\",\"child_index_param\":\"ui_current_voice\"," \
    "\"child_names\":[\"Kick\",\"Rim\",\"Snare\",\"Snare 2\",\"Clap\",\"Lo Tom\"," \
                     "\"HH Cl\",\"HH Cl 2\",\"Hi Tom\",\"Lo Tom 2\",\"HH Open\"," \
                     "\"Mid Tom\",\"Hi Tom 2\",\"Cymbal\",\"Cymbal 2\",\"Mid Tom2\"]," \
    "\"child_roles\":[\"kick\",\"rim\",\"snare\",\"snare\",\"clap\",\"tom\"," \
                     "\"hihat_closed\",\"hihat_closed\",\"tom\",\"tom\",\"hihat_open\"," \
                     "\"tom\",\"tom\",\"cymbal\",\"cymbal\",\"tom\"]," \
    "\"child_copy_keys\":[\"tune\",\"pitch\",\"wave\",\"bend\",\"bend_dyn\",\"cutoff\",\"res\"," \
        "\"lp_bend\",\"lp_dyn\",\"decay\",\"punch\",\"noise\",\"click\",\"volume\"," \
        "\"pan\",\"vel_vol\",\"sat\",\"reverb\",\"send_a\",\"send_b\"]," \
    "\"child_key_overrides\":{\"ui_current_voice\":\"ui_current_voice\"},"

static const char *kUiHierarchy =
"{\"pad_layout\":\"drums\",\"levels\":{"
 "\"root\":{\"name\":\"SIMIAN\",\"knobs\":[],\"params\":["
   /* No knobs on root: any knob there silently becomes page 1 and buries the
    * kit browser. */
   "{\"level\":\"kits\",\"label\":\"Kit\"},"
   "{\"level\":\"pads\",\"label\":\"Tone\"},"
   "{\"level\":\"pad_noise\",\"label\":\"Noise\"},"
   "{\"level\":\"pad_mix\",\"label\":\"Mix\"},"
   "{\"level\":\"output\",\"label\":\"Master\"}"
 "]},"
 "\"kits\":{\"name\":\"Kit\",\"list_param\":\"preset\",\"count_param\":\"preset_count\","
   "\"name_param\":\"preset_name\"},"
 /* Tune sits beside the Pad selector deliberately: on an alias pad it is the
  * only control not shared with its parent, so it is the first thing you
  * reach for after choosing the pad. */
 "\"pads\":{\"name\":\"Tone\"," CHILD_COMMON
   /* The note map lives here and nowhere else. */
   "\"child_notes\":[36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51],"
   "\"child_press_param\":\"ui_live_press\",\"child_press_note_param\":\"ui_live_note\","
   "\"knobs\":[\"ui_current_voice\",\"tune\",\"pitch\",\"wave\",\"bend\",\"bend_dyn\",\"decay\",\"punch\"],"
   "\"params\":[\"ui_current_voice\",\"tune\",\"pitch\",\"wave\",\"bend\",\"bend_dyn\",\"decay\",\"punch\"]},"
 "\"pad_noise\":{\"name\":\"Noise\"," CHILD_COMMON
   "\"knobs\":[\"cutoff\",\"res\",\"lp_bend\",\"lp_dyn\",\"noise\",\"click\"],"
   "\"params\":[\"cutoff\",\"res\",\"lp_bend\",\"lp_dyn\",\"noise\",\"click\"]},"
 "\"pad_mix\":{\"name\":\"Mix\"," CHILD_COMMON
   "\"knobs\":[\"volume\",\"pan\",\"vel_vol\",\"sat\",\"reverb\",\"send_a\",\"send_b\"],"
   "\"params\":[\"volume\",\"pan\",\"vel_vol\",\"sat\",\"reverb\",\"send_a\",\"send_b\"]},"
 "\"output\":{\"name\":\"Master\","
   "\"knobs\":[\"gain\",\"drive\",\"master_sat\",\"rev_size\",\"rev_gate\",\"sens\",\"transpose\"],"
   "\"params\":[\"gain\",\"drive\",\"master_sat\",\"rev_size\",\"rev_gate\",\"sens\",\"transpose\"]}"
"}}";

/* chain_params is the MODULATION table, not the UI.
 *
 * ⚠ Two readers, one file. A sound_generator's UI comes from the plugin's
 * ui_hierarchy and nowhere else — but parse_chain_params() also reads
 * module.json OFF DISK at load to build the slot's parameter-METADATA table,
 * which drives modulation targets, patch application and float smoothing. It
 * tries the inline ui_hierarchy first; that yields nothing here because the
 * levels reference their params by name, so it falls through to this list. A
 * key missing from it cannot be modulated or patched, and the miss is SILENT.
 *
 * So every key is listed, globals and all ten voices: 7 + 10x19 = 197, inside
 * the host's MAX_CHAIN_PARAMS of 256 and its 64 KB module.json limit. Adding a
 * voice parameter costs ten entries — count before adding the 20th. */
static int append_param_json(char *buf, int n, int len, int first,
                             const char *key, const char *name,
                             float mn, float mx, float def, const char *unit,
                             const char *viz) {
    /* Integer steps: every range here is coarse enough to sweep with an
     * encoder, and params.lib's 0.01 steps exist for a mouse. `max` is
     * declared on percent params deliberately — without it the shared
     * formatter assumes 0..1 and renders 50 as 5000%. */
    n += snprintf(buf + n, len - n,
        "%s{\"key\":\"%s\",\"name\":\"%s\",\"type\":\"int\",\"min\":%d,\"max\":%d,\"default\":%d",
        first ? "" : ",", key, name, (int)mn, (int)mx, (int)def);
    if (unit) n += snprintf(buf + n, len - n, ",\"unit\":\"%s\"", unit);
    if (viz)  n += snprintf(buf + n, len - n, ",\"viz\":%s", viz);
    n += snprintf(buf + n, len - n, "}");
    return n;
}

/* The Voice knob draws itself: canvas.js paints the drum's name and, when two
 * pads share one voice, a chain link and the partner's number. Declaring a
 * "custom:" kind is ALSO what makes the host load our canvas.js at all. An
 * unregistered kind does not claim the cell, so a host that has never heard
 * of this draws the plain number — which is what it does today. */
#define VOICE_VIZ "{\"kind\":\"custom:voicelink\",\"extra_keys\":[\"ui_voice_link\"]}"

static int build_chain_params(char *buf, int len) {
    int n = snprintf(buf, len, "[");
    int first = 1;
    for (int i = 0; i < GP_COUNT && n < len - 256; i++, first = 0) {
        const gparam_def_t *p = &GLOBAL_PARAMS[i];
        n = append_param_json(buf, n, len, first, p->key, p->name, p->min, p->max,
                              p->def, p->unit, NULL);
    }
    for (int i = 0; i < UI_COUNT && n < len - 256; i++, first = 0) {
        const gparam_def_t *p = &UI_PARAMS[i];
        n = append_param_json(buf, n, len, first, p->key, p->name, p->min, p->max,
                              p->def, p->unit,
                              strcmp(p->key, "ui_current_voice") == 0 ? VOICE_VIZ : NULL);
    }
    char key[80], name[80];
    /* Tune is per PAD, so all sixteen are listed, each with its own default. */
    for (int d = 0; d < SIMIAN_PADS; d++)
        for (int i = 0; i < PP_COUNT && n < len - 256; i++, first = 0) {
            const vparam_def_t *p = &PAD_PARAMS[i];
            snprintf(key, sizeof(key), "%s_%s", PADS[d].id, p->key);
            snprintf(name, sizeof(name), "%s %s", PADS[d].label, p->name);
            n = append_param_json(buf, n, len, first, key, name, p->min, p->max,
                                  PADS[d].tune, p->unit, NULL);
        }
    /* Everything else is per VOICE, and an alias addresses the same parameter
     * as its parent — so it is listed ONCE, under the pad that owns the voice.
     * Listing both pads would put two modulation targets on one value, and
     * 16 x 19 would overrun the host's 256 anyway. */
    for (int v = 0; v < SIMIAN_VOICES; v++) {
        int owner = voice_owner_pad(v);
        for (int i = 0; i < VP_COUNT && n < len - 256; i++, first = 0) {
            const vparam_def_t *p = &VOICE_PARAMS[i];
            snprintf(key, sizeof(key), "%s_%s", PADS[owner].id, p->key);
            snprintf(name, sizeof(name), "%s %s", VOICES[v].label, p->name);
            n = append_param_json(buf, n, len, first, key, name, p->min, p->max,
                                  p->def, p->unit, NULL);
        }
    }
    n += snprintf(buf + n, len - n, "]");
    return n;
}

/* TEN entries, not sixteen: an alias pad has no audio path of its own, so the
 * id published is the pad that OWNS the voice — which is also the pad its
 * send levels are keyed under. */
static int build_split_voices(char *buf, int len) {
    int n = snprintf(buf, len, "[");
    for (int v = 0; v < SIMIAN_VOICES && n < len - 64; v++)
        n += snprintf(buf + n, len - n, "%s{\"id\":\"%s\",\"label\":\"%s\",\"role\":\"%s\"}",
                      v ? "," : "", PADS[voice_owner_pad(v)].id, VOICES[v].label, VOICES[v].role);
    n += snprintf(buf + n, len - n, "]");
    return n;
}

/* ---- slot state ---- */
static int json_get_number(const char *json, const char *key, float *out) {
    char search[80];   /* the longest key is "voice10_bend_dyn" plus quotes */
    /* Bounded rather than truncated: a key long enough to overflow this would
     * otherwise be silently cut and then match the WRONG field. */
    size_t klen = strlen(key);
    if (klen + 4 > sizeof(search)) return -1;
    memcpy(search, "\"", 1);
    memcpy(search + 1, key, klen);
    memcpy(search + 1 + klen, "\":", 3);
    const char *pos = strstr(json, search);
    if (!pos) return -1;
    pos += strlen(search);
    while (*pos == ' ') pos++;
    *out = (float)atof(pos);
    return 0;
}

/* Keyed the same way chain_params is: every pad's tune, and each voice's
 * parameters ONCE under the pad that owns it. Writing an alias's copy too
 * would double the blob and make "which one wins on restore" a question. */
static int build_state(simian_t *inst, char *buf, int buf_len) {
    int n = snprintf(buf, buf_len, "{\"preset\":%d,\"octave_transpose\":%d,\"ui_current_voice\":%d",
                     inst->cur_preset, inst->octave_transpose, inst->ui_current_voice + 1);
    for (int g = 0; g < GP_COUNT && n < buf_len - 64; g++)
        n += snprintf(buf + n, buf_len - n, ",\"%s\":%.4f", GLOBAL_PARAMS[g].key, inst->gvalue[g]);
    for (int d = 0; d < SIMIAN_PADS && n < buf_len - 64; d++)
        n += snprintf(buf + n, buf_len - n, ",\"%s_tune\":%.4f", PADS[d].id, inst->pad_tune[d]);
    for (int v = 0; v < SIMIAN_VOICES; v++) {
        int owner = voice_owner_pad(v);
        for (int i = 0; i < VP_COUNT && n < buf_len - 64; i++)
            n += snprintf(buf + n, buf_len - n, ",\"%s_%s\":%.4f",
                          PADS[owner].id, VOICE_PARAMS[i].key, inst->voice[v].value[i]);
    }
    n += snprintf(buf + n, buf_len - n, "}");
    return n;
}

static void restore_state(simian_t *inst, const char *json) {
    float f;
    if (json_get_number(json, "preset", &f) == 0) {
        int idx = (int)f;
        if (idx >= 0 && idx < preset_count()) load_preset(inst, idx);
        else inst->cur_preset = -1;
    }
    if (json_get_number(json, "octave_transpose", &f) == 0) inst->octave_transpose = (int)f;
    if (json_get_number(json, "ui_current_voice", &f) == 0) {
        int d = (int)f - 1;
        if (d >= 0 && d < SIMIAN_PADS) inst->ui_current_voice = d;
    }
    /* After the preset, so a value edited since the kit was picked wins — the
     * slot keeps the sound it was saved with. */
    for (int g = 0; g < GP_COUNT; g++)
        if (json_get_number(json, GLOBAL_PARAMS[g].key, &f) == 0) set_global_value(inst, g, f);
    char key[80];
    for (int d = 0; d < SIMIAN_PADS; d++) {
        snprintf(key, sizeof(key), "%s_tune", PADS[d].id);
        if (json_get_number(json, key, &f) == 0) set_pad_value(inst, d, 0, f);
    }
    for (int v = 0; v < SIMIAN_VOICES; v++) {
        int owner = voice_owner_pad(v);
        for (int i = 0; i < VP_COUNT; i++) {
            snprintf(key, sizeof(key), "%s_%s", PADS[owner].id, VOICE_PARAMS[i].key);
            if (json_get_number(json, key, &f) == 0) set_voice_value(inst, v, i, f);
        }
    }
}

/* ======================================================================== *
 *  v2 entry points
 * ======================================================================== */
static void resolve_zones(simian_t *inst) {
    for (int v = 0; v < SIMIAN_VOICES; v++) {
        simian_voice_t *vv = &inst->voice[v];
        for (int i = 0; i < VP_COUNT; i++)
            vv->zone[i] = VOICE_PARAMS[i].zone ? vv->zones.find(VOICE_PARAMS[i].zone) : nullptr;
        vv->z_trigger    = vv->zones.find("Trigger");
        vv->z_choke      = vv->zones.find("Choke");
        vv->z_key        = vv->zones.find("key");
        vv->z_wake       = vv->zones.find("WakeUp");
        vv->z_modwheel   = vv->zones.find("ModWheel");
        vv->z_pitchwheel = vv->zones.find("PitchWheel");
    }
    for (int g = 0; g < GP_COUNT; g++)
        inst->gzone[g] = GLOBAL_PARAMS[g].zone ? inst->out_zones.find(GLOBAL_PARAMS[g].zone) : nullptr;
    inst->z_out_wake = inst->out_zones.find("WakeUp");
}

static void *v2_create_instance(const char *module_dir, const char *json_defaults) {
    simian_t *inst = new (std::nothrow) simian_t();
    if (!inst) return NULL;
    inst->cur_preset = -1;
    inst->octave_transpose = 0;
    inst->ui_current_voice = 0;
    inst->ui_auto_select = 1;
    inst->live_armed = 0;
    inst->live_arm_block = 0;
    inst->last_hit_pad = -1;
    inst->last_hit_block = 0;
    inst->have_vouched = 0;
    inst->last_vouch_block = 0;
    inst->block = 0;
    if (module_dir) strncpy(inst->module_dir, module_dir, sizeof(inst->module_dir) - 1);

    int sr = (g_host && g_host->sample_rate > 0) ? g_host->sample_rate : MOVE_SAMPLE_RATE;
    inst->silence_samples = sr * SIMIAN_SILENCE_MS / 1000;

    DSP_Drum::classInit(sr);
    DSP_Output::classInit(sr);
    inst->out.init(sr);
    inst->out.buildUserInterface(&inst->out_zones);
    for (int v = 0; v < SIMIAN_VOICES; v++) {
        inst->voice[v].dsp.init(sr);
        inst->voice[v].dsp.buildUserInterface(&inst->voice[v].zones);
        inst->voice[v].pending_trigger = -1.0f;
        inst->voice[v].silence_count = inst->silence_samples;   /* start inactive */
        inst->voice[v].active = 0;
    }
    resolve_zones(inst);

    load_defaults(inst);
    if (preset_count() > 0) load_preset(inst, 0);
    if (json_defaults) restore_state(inst, json_defaults);

    for (int v = 0; v < SIMIAN_VOICES; v++)
        if (inst->voice[v].z_wake) *inst->voice[v].z_wake = 1.0f;
    if (inst->z_out_wake) *inst->z_out_wake = 1.0f;
    return inst;
}

static void v2_destroy_instance(void *instance) {
    simian_t *inst = (simian_t *)instance;
    if (inst) delete inst;
}

static int transport_running(void) {
    /* get_clock_status is the host's "is something sequencing" bit. Absent on
     * an older host, in which case a bare note is treated as a hand. */
    return (g_host && g_host->get_clock_status) ? (g_host->get_clock_status() != 0) : 0;
}

static int host_vouches(const simian_t *inst) {
    return inst->have_vouched &&
           (inst->block - inst->last_vouch_block) <= SIMIAN_VOUCH_TTL_BLOCKS;
}

/* Focus follows a hit if nothing is sequencing, or if a host vouched that a
 * finger caused it. A note alone with a transport running cannot: a live hit
 * and a sequenced one are identical here, so following every note would let
 * playback drag the editor around. */
static void note_hit_focus(simian_t *inst, int pad) {
    inst->last_hit_pad = pad;
    inst->last_hit_block = inst->block;
    if (!inst->ui_auto_select) return;
    if (!transport_running() && !host_vouches(inst)) {
        inst->ui_current_voice = pad;
        inst->live_armed = 0;
        inst->last_hit_pad = -1;
    } else if (inst->live_armed &&
               (inst->block - inst->live_arm_block) <= SIMIAN_LIVE_MATCH_BLOCKS) {
        inst->ui_current_voice = pad;
        inst->live_armed = 0;
        inst->last_hit_pad = -1;   /* consumed: one note vouches for one press */
    }
}

static void v2_on_midi(void *instance, const uint8_t *msg, int len, int source) {
    simian_t *inst = (simian_t *)instance;
    if (!inst || len < 2) return;
    (void)source;
    uint8_t status = msg[0] & 0xF0;
    uint8_t d1 = msg[1];
    uint8_t d2 = (len > 2) ? msg[2] : 0;

    switch (status) {
        case 0x90:
        case 0x80: {
            int on = (status == 0x90 && d2 > 0);
            if (!on) break;   /* percussive: a release ends nothing */
            int note = (int)d1 + inst->octave_transpose * 12;
            if (note < 0 || note > 127) return;

            int pad = note_to_pad(note);
            if (pad < 0) break;          /* not one of our sixteen */
            int vi = PADS[pad].voice;

            /* Sensitivity compresses the velocity range toward 80/127, which
             * is upstream's law, applied here because its Faust zone was
             * pruned. */
            float vel = d2 / 127.0f;
            float sens = inst->gvalue[find_global_param("sens")] * 0.01f;
            vel = vel * sens + (1.0f - sens) * (80.0f / 127.0f);

            /* What the pad is worth on its own: its Tune, plus the module's
             * Transpose, on drum.dsp's own relative `key`. An alias pad is
             * exactly this number away from its parent — with tune at 0 the
             * two pads ARE the same drum, which is the state this design
             * exists to make visible rather than accidental. */
            float offset = inst->pad_tune[pad]
                         + inst->gvalue[find_global_param("transpose")];

            choke_group_of(inst, pad);
            voice_note_on(inst, vi, offset, vel);
            note_hit_focus(inst, pad);
            break;
        }
        case 0xE0: {
            if (len < 3) break;
            int bend = ((int)d2 << 7 | d1) - 8192;
            /* Upstream's `staccato`: the pitch wheel scales every voice's
             * decay from 20% to 180%. It is a performance control, not a
             * pitch bend, which is why it is not saved anywhere. */
            for (int v = 0; v < SIMIAN_VOICES; v++)
                if (inst->voice[v].z_pitchwheel)
                    *inst->voice[v].z_pitchwheel = (FAUSTFLOAT)(bend / 8192.0f);
            break;
        }
        case 0xB0: {
            switch (d1) {
                case 1:   /* mod wheel — upstream's DJ filter, three octaves down */
                    for (int v = 0; v < SIMIAN_VOICES; v++)
                        if (inst->voice[v].z_modwheel)
                            *inst->voice[v].z_modwheel = (FAUSTFLOAT)(d2 / 127.0f);
                    break;
                case 120: case 123:
                    all_notes_off(inst);
                    break;
                default: break;
            }
            break;
        }
        default: break;
    }
}

static void v2_set_param(void *instance, const char *key, const char *val) {
    simian_t *inst = (simian_t *)instance;
    if (!inst || !key || !val) return;

    if (strcmp(key, "all_notes_off") == 0) { all_notes_off(inst); return; }
    if (strcmp(key, "octave_transpose") == 0) { inst->octave_transpose = atoi(val); return; }
    if (strcmp(key, "state") == 0) { restore_state(inst, val); return; }
    if (strcmp(key, "preset") == 0) {
        int idx = atoi(val);
        if (idx >= 0 && idx < preset_count() && idx != inst->cur_preset) load_preset(inst, idx);
        return;
    }
    if (strcmp(key, "ui_current_voice") == 0) {
        int d = atoi(val) - 1;                       /* 1-based on the wire */
        inst->ui_current_voice = (d < 0) ? 0 : (d >= SIMIAN_PADS ? SIMIAN_PADS - 1 : d);
        return;
    }
    if (strcmp(key, "ui_auto_select") == 0) { inst->ui_auto_select = atoi(val) != 0; return; }

    /* A host that EMITS the note can name the voice outright — no correlation,
     * no window. Sequenced notes never reach here; a host calls it only from
     * its live pad paths. Still the NOTE, never an index: note -> voice stays
     * ours, because the pad-to-note map is Move's. */
    if (strcmp(key, "ui_live_note") == 0) {
        inst->have_vouched = 1;
        inst->last_vouch_block = inst->block;
        if (!inst->ui_auto_select) return;
        int n = atoi(val);
        if (n < 0 || n > 127) return;
        int pad = note_to_pad(n);
        if (pad < 0) return;                         /* unmapped: not our note */
        inst->ui_current_voice = pad;
        /* Consume any vouch in flight, so it cannot later credit a SEQUENCED
         * note over the authoritative answer we just took. */
        inst->live_armed = 0;
        inst->last_hit_pad = -1;
        return;
    }

    /* The vouch: "a finger did that", with no way to say WHICH pad — a grid
     * position is not a pad. The note decides. It usually arrives AFTER the
     * note (that comes straight off the MIDI stream, this crosses a process
     * boundary), so look back first and only arm forward if nothing recent
     * matches; handling one order only would drop about half the presses. */
    if (strcmp(key, "ui_live_press") == 0) {
        inst->have_vouched = 1;
        inst->last_vouch_block = inst->block;
        if (!inst->ui_auto_select) return;
        if (inst->last_hit_pad >= 0 &&
            (inst->block - inst->last_hit_block) <= SIMIAN_LIVE_MATCH_BLOCKS) {
            inst->ui_current_voice = inst->last_hit_pad;
            inst->live_armed = 0;
            inst->last_hit_pad = -1;
        } else {
            inst->live_armed = 1;
            inst->live_arm_block = inst->block;
        }
        return;
    }

    int g = find_global_param(key);
    if (g >= 0) { set_global_value(inst, g, (float)atof(val)); return; }

    /* A prefixed key names a PAD. `tune` is that pad's own; everything else
     * belongs to the voice behind it, which an alias shares with its parent —
     * so writing pad4_decay writes the snare's decay, seen from both pads. */
    int pad = -1;
    const char *bare = split_pad_key(key, &pad);
    if (!bare) {
        /* A bare key addresses the FOCUSED pad, which is how a child level's
         * knobs arrive when a consumer has not expanded the template. */
        pad = inst->ui_current_voice;
        bare = key;
    }
    int pi = find_pad_param(bare);
    if (pi >= 0) { set_pad_value(inst, pad, pi, (float)atof(val)); return; }
    pi = find_voice_param(bare);
    if (pi >= 0) set_voice_value(inst, PADS[pad].voice, pi, (float)atof(val));
}

static int v2_get_param(void *instance, const char *key, char *buf, int buf_len) {
    simian_t *inst = (simian_t *)instance;
    if (!inst || !key || !buf || buf_len <= 0) return -1;

    if (strcmp(key, "ui_hierarchy") == 0) return snprintf(buf, buf_len, "%s", kUiHierarchy);
    if (strcmp(key, "chain_params") == 0) return build_chain_params(buf, buf_len);
    if (strcmp(key, "split_voices") == 0) return build_split_voices(buf, buf_len);
    /* [0] is Send A, [1] is Send B; {id} is substituted verbatim. */
    if (strcmp(key, "voice_send_params") == 0)
        return snprintf(buf, buf_len, "[\"{id}_send_a\",\"{id}_send_b\"]");
    if (strcmp(key, "name") == 0) return snprintf(buf, buf_len, "SIMIAN");
    if (strcmp(key, "state") == 0) return build_state(inst, buf, buf_len);
    if (strcmp(key, "preset_count") == 0) return snprintf(buf, buf_len, "%d", preset_count());
    if (strcmp(key, "preset") == 0)
        return snprintf(buf, buf_len, "%d", inst->cur_preset < 0 ? 0 : inst->cur_preset);
    if (strcmp(key, "preset_name") == 0) {
        if (inst->cur_preset < 0 || inst->cur_preset >= preset_count())
            return snprintf(buf, buf_len, "Init");
        return snprintf(buf, buf_len, "%s", SIMIAN_FACTORY[inst->cur_preset].name);
    }
    if (strcmp(key, "octave_transpose") == 0)
        return snprintf(buf, buf_len, "%d", inst->octave_transpose);
    if (strcmp(key, "ui_current_voice") == 0)
        return snprintf(buf, buf_len, "%d", inst->ui_current_voice + 1);
    /* 1-based partner pad, or 0 for "this voice has one pad". Read by the
     * custom widget through its viz extra_keys — one extra value stop. */
    if (strcmp(key, "ui_voice_link") == 0) {
        int q = pad_partner(inst->ui_current_voice);
        return snprintf(buf, buf_len, "%d", q < 0 ? 0 : q + 1);
    }
    /* TEN, not sixteen: the pads are a seating plan, the voices are what can
     * sound at once. */
    if (strcmp(key, "polyphony") == 0) return snprintf(buf, buf_len, "%d", SIMIAN_VOICES);

    int g = find_global_param(key);
    if (g >= 0) return snprintf(buf, buf_len, "%.3f", inst->gvalue[g]);

    int pad = -1;
    const char *bare = split_pad_key(key, &pad);
    if (!bare) { pad = inst->ui_current_voice; bare = key; }
    int pi = find_pad_param(bare);
    if (pi >= 0) return snprintf(buf, buf_len, "%.3f", inst->pad_tune[pad]);
    pi = find_voice_param(bare);
    if (pi >= 0) return snprintf(buf, buf_len, "%.3f", inst->voice[PADS[pad].voice].value[pi]);

    return -1;   /* NEGATIVE for an unknown key: 0 would claim the value is "" */
}

static int v2_get_error(void *instance, char *buf, int buf_len) {
    (void)instance;
    if (buf && buf_len > 0) buf[0] = '\0';
    return 0;
}

/* ======================================================================== *
 *  Render
 *
 *  ONE core for both entry points, because the host flips between them per
 *  frame as voices are assigned to buses and any difference in state would be
 *  audible at the flip. `dest[v]` is where voice v's audio goes: the master
 *  path when it equals `main`, its own bus buffer otherwise.
 * ======================================================================== */
static inline void accum_i16(int16_t *dst, int idx, float v) {
    int s = (int)dst[idx] + (int)lrintf(clampf(v, -1.0f, 1.0f) * 32767.0f);
    dst[idx] = (int16_t)(s > 32767 ? 32767 : (s < -32768 ? -32768 : s));
}

static void render_core(simian_t *inst, int16_t *const *dest, int16_t *main_out, int frames) {
    const int kMax = SIMIAN_BUF;
    int done = 0;

    while (done < frames) {
        int n = frames - done;
        if (n > kMax) n = kMax;
        /* A hit waiting for its rising edge gets exactly one frame with
         * Trigger still at 0, then the edge. */
        for (int v = 0; v < SIMIAN_VOICES; v++)
            if (inst->voice[v].pending_trigger >= 0.0f) { n = 1; break; }

        for (int i = 0; i < n; i++) { inst->accL[i] = 0.0f; inst->accR[i] = 0.0f; inst->accFX[i] = 0.0f; }

        for (int v = 0; v < SIMIAN_VOICES; v++) {
            simian_voice_t *vv = &inst->voice[v];
            if (!vv->active) continue;

            FAUSTFLOAT *outs[3] = {vv->out3[0], vv->out3[1], vv->out3[2]};
            vv->dsp.compute(n, nullptr, outs);

            /* Silence tracking, upstream's: quiet for SIMIAN_SILENCE_MS and the
             * voice stops being computed until something wakes it. */
            for (int i = 0; i < n; i++) {
                if (fabsf(vv->out3[0][i]) > 0.001f || fabsf(vv->out3[1][i]) > 0.001f)
                    vv->silence_count = 0;
                else
                    vv->silence_count++;
            }
            if (vv->silence_count >= inst->silence_samples) vv->active = 0;

            /* The reverb send is global whatever the voice is routed to: it is
             * the send, not the voice. */
            for (int i = 0; i < n; i++) inst->accFX[i] += vv->out3[2][i];

            if (dest[v] == main_out) {
                for (int i = 0; i < n; i++) {
                    inst->accL[i] += vv->out3[0][i];
                    inst->accR[i] += vv->out3[1][i];
                }
            } else {
                for (int i = 0; i < n; i++) {
                    accum_i16(dest[v], (done + i) * 2,     vv->out3[0][i]);
                    accum_i16(dest[v], (done + i) * 2 + 1, vv->out3[1][i]);
                }
            }
        }

        /* The master stage always runs: it carries the reverb return, which
         * belongs to no voice, and keeping it computing keeps its limiter and
         * reverb tail in the same state whichever entry point is live. */
        FAUSTFLOAT *in3[3]  = {inst->accL, inst->accR, inst->accFX};
        FAUSTFLOAT *out2[2] = {inst->mixL, inst->mixR};
        inst->out.compute(n, in3, out2);

        for (int i = 0; i < n; i++) {
            accum_i16(main_out, (done + i) * 2,     inst->mixL[i]);
            accum_i16(main_out, (done + i) * 2 + 1, inst->mixR[i]);
        }
        done += n;

        for (int v = 0; v < SIMIAN_VOICES; v++) {
            simian_voice_t *vv = &inst->voice[v];
            if (vv->pending_trigger >= 0.0f) {
                if (vv->z_trigger) *vv->z_trigger = (FAUSTFLOAT)vv->pending_trigger;
                vv->pending_trigger = -1.0f;
            }
        }
    }
    inst->block++;
}

static void v2_render_block(void *instance, int16_t *out_lr, int frames) {
    simian_t *inst = (simian_t *)instance;
    if (!inst || !out_lr || frames <= 0) return;
    /* render_block OVERWRITES, unlike render_split. Clear once, then share the
     * accumulating core. */
    memset(out_lr, 0, (size_t)frames * 2 * sizeof(int16_t));
    int16_t *dest[SIMIAN_VOICES];
    for (int v = 0; v < SIMIAN_VOICES; v++) dest[v] = out_lr;
    render_core(inst, dest, out_lr, frames);
}

/* A SEPARATE EXPORTED SYMBOL on purpose — a module cannot extend the host's
 * ABI from its side. The host CLEARS every destination before the call,
 * main_out included, and the entries ALIAS: two voices on one bus are handed
 * the same pointer, and a voice on no bus is handed main_out. So: accumulate,
 * never memset, never write past `frames`. */
extern "C" void move_plugin_render_split(void *instance, int16_t *const *voice_out,
                                         int n_voices, int16_t *main_out, int frames) {
    simian_t *inst = (simian_t *)instance;
    if (!inst || !main_out || frames <= 0) return;
    int16_t *dest[SIMIAN_VOICES];
    for (int v = 0; v < SIMIAN_VOICES; v++)
        dest[v] = (voice_out && v < n_voices && voice_out[v]) ? voice_out[v] : main_out;
    render_core(inst, dest, main_out, frames);
}

static plugin_api_v2_t g_plugin_api_v2 = {
    MOVE_PLUGIN_API_VERSION_2,
    v2_create_instance,
    v2_destroy_instance,
    v2_on_midi,
    v2_set_param,
    v2_get_param,
    v2_get_error,
    v2_render_block,
};

extern "C" plugin_api_v2_t *move_plugin_init_v2(const host_api_v1_t *host) {
    g_host = host;
    return &g_plugin_api_v2;
}
