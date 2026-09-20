/*
 * Host-side tests for the SIMIAN wrapper: build native, run on the workstation.
 *
 *   ./scripts/test.sh
 *
 * The wrapper is #included rather than linked so the tests can reach the
 * static tables (VOICE_PARAMS, VOICES, SIMIAN_FACTORY) the module's behaviour
 * is defined by. Everything here is reachable without a device: the DSP is
 * deterministic and needs no audio hardware, so "does this note make a sound
 * on THAT voice" is a real assertion and not a stand-in for playing it.
 */
#include "../src/dsp/simian_plugin.cpp"

#include <cstdio>
#include <cmath>
#include <string>
#include <vector>
#include <cstdarg>

static int g_fail = 0;
static int g_checks = 0;

static void ok(bool cond, const char *what) {
    g_checks++;
    if (!cond) { printf("  FAIL  %s\n", what); g_fail++; }
    else       { printf("  ok    %s\n", what); }
}

static void okf(bool cond, const char *fmt, ...) {
    char buf[512];
    va_list ap;
    va_start(ap, fmt);
    vsnprintf(buf, sizeof(buf), fmt, ap);
    va_end(ap);
    ok(cond, buf);
}

static void note_on(plugin_api_v2_t *api, void *inst, int note, int vel) {
    uint8_t msg[3] = {0x90, (uint8_t)note, (uint8_t)vel};
    api->on_midi(inst, msg, 3, 0);
}

static std::string get(plugin_api_v2_t *api, void *inst, const char *key) {
    static char buf[65536];
    int n = api->get_param(inst, key, buf, sizeof(buf));
    if (n < 0) return std::string("<err>");
    return std::string(buf);
}

/* Render `ms` of audio and return peak amplitude, 0..1. */
static float render_peak(plugin_api_v2_t *api, void *inst, int ms) {
    const int frames = 128;
    int blocks = (MOVE_SAMPLE_RATE * ms / 1000) / frames;
    int16_t buf[frames * 2];
    float peak = 0.0f;
    for (int b = 0; b < blocks; b++) {
        api->render_block(inst, buf, frames);
        for (int i = 0; i < frames * 2; i++) {
            float v = fabsf(buf[i] / 32768.0f);
            if (v > peak) peak = v;
        }
    }
    return peak;
}

/* ---- a minimal JSON reader, enough to walk what the wrapper serves ---- */
static std::string json_field(const std::string &s, size_t from, const char *key) {
    std::string pat = std::string("\"") + key + "\":";
    size_t p = s.find(pat, from);
    if (p == std::string::npos) return "";
    p += pat.size();
    if (s[p] == '"') { size_t e = s.find('"', p + 1); return s.substr(p + 1, e - p - 1); }
    size_t e = s.find_first_of(",}]", p);
    return s.substr(p, e - p);
}

static std::string read_file(const char *path) {
    FILE *f = fopen(path, "rb");
    if (!f) return "";
    std::string out;
    char buf[8192];
    size_t n;
    while ((n = fread(buf, 1, sizeof(buf), f)) > 0) out.append(buf, n);
    fclose(f);
    return out;
}

int main(void) {
    plugin_api_v2_t *api = move_plugin_init_v2(NULL);
    printf("simian_test\n");

    /* ================= tables ================= */
    printf("\ntables:\n");
    ok(VP_COUNT == 19, "19 parameters per voice");
    ok(SIMIAN_VOICES == 10, "ten voices");
    ok(SIMIAN_FACTORY_COUNT == 35, "35 factory kits");
    ok(SIMIAN_FACTORY_VPARAMS == VP_COUNT,
       "factory bank width matches VOICE_PARAMS[] — a mismatch loads every value one slot out");
    ok(SIMIAN_FACTORY_GPARAMS == GP_COUNT, "factory bank global width matches GLOBAL_PARAMS[]");
    ok(SIMIAN_FACTORY_VOICES == SIMIAN_VOICES, "factory bank has a row per voice");

    /* The id is what the host substitutes into "{id}_send_a"; if it stops
     * being "voice" + the 1-based index, every send lands on its neighbour. */
    {
        bool idsok = true;
        for (int v = 0; v < SIMIAN_VOICES; v++) {
            char want[16];
            snprintf(want, sizeof(want), "voice%d", v + 1);
            if (strcmp(VOICES[v].id, want) != 0) idsok = false;
        }
        ok(idsok, "voice ids are voice1..voice10, 1-based, matching child_index_base");
    }
    {
        bool ascending = true;
        for (int v = 1; v < SIMIAN_VOICES; v++)
            if (VOICES[v].note <= VOICES[v - 1].note) ascending = false;
        ok(ascending, "voices are declared in ascending note order");
    }
    {
        /* Every note upstream routes must reach a voice, and the two
         * choke-only notes must NOT be in the map. */
        bool allmapped = true;
        for (int i = 0; i < NOTE_MAP_COUNT; i++)
            if (NOTE_MAP[i].voice < 0 || NOTE_MAP[i].voice >= SIMIAN_VOICES) allmapped = false;
        ok(allmapped, "every note map entry names a real voice");
        ok(note_to_voice(NOTE_PEDAL_HIHAT) < 0, "pedal hihat plays nothing (it only chokes)");
        ok(note_to_voice(NOTE_RIDE) < 0, "ride plays nothing (it only chokes the cymbal)");
        bool primaries = true;
        for (int v = 0; v < SIMIAN_VOICES; v++)
            if (note_to_voice(VOICES[v].note) != v) primaries = false;
        ok(primaries, "each voice's own note routes to it");
    }

    /* ================= instance ================= */
    void *inst = api->create_instance(".", NULL);
    ok(inst != NULL, "create_instance");
    if (!inst) return 1;
    simian_t *S = (simian_t *)inst;

    /* Faust prunes a control nothing reads, so a zone that silently failed to
     * resolve is a knob that turns and does nothing. Check them all. */
    printf("\nzones:\n");
    {
        int unresolved = 0;
        for (int i = 0; i < VP_COUNT; i++)
            if (VOICE_PARAMS[i].zone && !S->voice[0].zone[i]) {
                printf("        unresolved voice zone: %s (%s)\n",
                       VOICE_PARAMS[i].key, VOICE_PARAMS[i].zone);
                unresolved++;
            }
        ok(unresolved == 0, "every declared voice zone resolved in drum.dsp");
        int gun = 0;
        for (int g = 0; g < GP_COUNT; g++)
            if (GLOBAL_PARAMS[g].zone && !S->gzone[g]) {
                printf("        unresolved global zone: %s (%s)\n",
                       GLOBAL_PARAMS[g].key, GLOBAL_PARAMS[g].zone);
                gun++;
            }
        ok(gun == 0, "every declared global zone resolved in output.dsp");
        ok(S->voice[0].z_trigger && S->voice[0].z_choke && S->voice[0].z_key,
           "Trigger / Choke / key resolved");
    }

    /* ================= params ================= */
    printf("\nparams:\n");
    api->set_param(inst, "voice1_pitch", "55");
    ok(get(api, inst, "voice1_pitch") == "55.000", "prefixed key round-trips");
    ok(*S->voice[V_KICK].zone[find_voice_param("pitch")] == 55.0f, "…and reaches the Faust zone");
    api->set_param(inst, "voice10_pitch", "1000");
    ok(get(api, inst, "voice10_pitch") == "1000.000", "two-digit voice index parses");
    ok(get(api, inst, "voice1_pitch") == "55.000", "…without touching voice1");

    api->set_param(inst, "ui_current_voice", "3");
    ok(get(api, inst, "ui_current_voice") == "3", "focus is 1-based on the wire");
    api->set_param(inst, "decay", "777");
    ok(get(api, inst, "voice3_decay") == "777.000", "a bare key edits the FOCUSED voice");

    api->set_param(inst, "voice1_pitch", "99999");
    ok(get(api, inst, "voice1_pitch") == "4400.000", "out-of-range is clamped, not wrapped");
    ok(api->get_param(inst, "no_such_param", (char *)"", 0) < 0 ||
       get(api, inst, "no_such_param") == "<err>",
       "an unknown key answers NEGATIVE, never an empty value");
    ok(get(api, inst, "voice11_pitch") == "<err>", "an out-of-range voice index is not a param");
    ok(get(api, inst, "voice0_pitch") == "<err>", "voice0 does not exist (ids are 1-based)");

    /* ================= presets ================= */
    printf("\npresets:\n");
    ok(get(api, inst, "preset_count") == "35", "35 kits served");
    api->set_param(inst, "preset", "0");
    ok(get(api, inst, "preset_name") == "Basic", "kit 0 is Basic — upstream's init kit");
    float basic_kick = S->voice[V_KICK].value[find_voice_param("pitch")];
    api->set_param(inst, "preset", "5");
    float other_kick = S->voice[V_KICK].value[find_voice_param("pitch")];
    ok(basic_kick != other_kick, "a different kit is a different kick");
    ok(*S->voice[V_KICK].zone[find_voice_param("pitch")] == other_kick,
       "loading a kit writes the Faust zones, not just the shadow values");

    /* ================= state ================= */
    printf("\nstate:\n");
    api->set_param(inst, "voice6_cutoff", "1234");
    api->set_param(inst, "gain", "-12");
    std::string blob = get(api, inst, "state");
    ok(blob.find("\"voice6_cutoff\":1234") != std::string::npos, "state carries every voice key");
    ok(blob.find("\"gain\":-12") != std::string::npos, "state carries the globals");
    {
        void *b = api->create_instance(".", blob.c_str());
        ok(get(api, b, "voice6_cutoff") == "1234.000", "state restores a per-voice value");
        ok(get(api, b, "gain") == "-12.000", "state restores a global");
        ok(get(api, b, "ui_current_voice") == "3", "state restores the focused voice");
        api->destroy_instance(b);
    }

    /* ================= published contracts ================= */
    printf("\ncontracts:\n");
    std::string sv = get(api, inst, "split_voices");
    {
        bool allthere = true;
        for (int v = 0; v < SIMIAN_VOICES; v++) {
            std::string want = std::string("\"id\":\"") + VOICES[v].id + "\"";
            if (sv.find(want) == std::string::npos) allthere = false;
        }
        ok(allthere, "split_voices publishes all ten ids");
        /* Order is the buffer contract: entry i IS buffer i. */
        size_t at = 0; bool ordered = true;
        for (int v = 0; v < SIMIAN_VOICES; v++) {
            std::string want = std::string("\"id\":\"") + VOICES[v].id + "\"";
            size_t p = sv.find(want, at);
            if (p == std::string::npos) { ordered = false; break; }
            at = p;
        }
        ok(ordered, "split_voices is in VOICES[] order — entry i is buffer i");
    }
    {
        /* Each declared send template, substituted with each id, must address
         * a real parameter. This is the check that catches a base-off-by-one. */
        std::string tpl = get(api, inst, "voice_send_params");
        ok(tpl == "[\"{id}_send_a\",\"{id}_send_b\"]", "two send templates, A then B");
        bool resolves = true;
        for (int v = 0; v < SIMIAN_VOICES; v++)
            for (const char *s : {"_send_a", "_send_b"}) {
                std::string k = std::string(VOICES[v].id) + s;
                if (get(api, inst, k.c_str()) == "<err>") resolves = false;
            }
        ok(resolves, "every {id}_send_a / _send_b substitution addresses a real param");
    }

    /* ================= hierarchy ================= */
    printf("\nhierarchy:\n");
    {
        std::string h = get(api, inst, "ui_hierarchy");
        ok(h.find("\"pad_layout\":\"drums\"") != std::string::npos, "pad_layout says drums");
        /* 🔴 The note map must be on exactly ONE level: voicesOf emits a voice
         * per child of every level that declares one, so two levels publish
         * twenty voices and seat two copies of the kit — and the pages still
         * render perfectly. */
        int notemaps = 0;
        for (size_t p = h.find("child_notes"); p != std::string::npos;
             p = h.find("child_notes", p + 1)) notemaps++;
        for (size_t p = h.find("child_note_base"); p != std::string::npos;
             p = h.find("child_note_base", p + 1)) notemaps++;
        ok(notemaps == 1, "exactly one level declares a note map");

        /* ⚠ A key repeated across levels makes the loader drop ALL of its
         * metadata, which silently kills the per-voice sends. */
        std::vector<std::string> seen;
        int dup = 0;
        for (int i = 0; i < VP_COUNT; i++) {
            std::string pat = std::string("\"") + VOICE_PARAMS[i].key + "\"";
            int levels_with = 0;
            const char *lv[] = {"\"voices\"", "\"voice_noise\"", "\"voice_mix\""};
            for (int L = 0; L < 3; L++) {
                size_t start = h.find(lv[L]);
                if (start == std::string::npos) continue;
                /* the level's own span: up to the next level key or the end */
                size_t end = h.size();
                for (int M = 0; M < 3; M++) {
                    size_t o = h.find(lv[M], start + 1);
                    if (o != std::string::npos && o < end) end = o;
                }
                size_t o = h.find("\"output\"", start + 1);
                if (o != std::string::npos && o < end) end = o;
                std::string span = h.substr(start, end - start);
                /* child_copy_keys lists every key on every level by design. */
                size_t ck = span.find("child_copy_keys");
                if (ck != std::string::npos) {
                    size_t cke = span.find(']', ck);
                    span.erase(ck, cke - ck);
                }
                if (span.find(pat) != std::string::npos) levels_with++;
            }
            if (levels_with > 1) { dup++; printf("        duplicated key: %s\n", VOICE_PARAMS[i].key); }
        }
        ok(dup == 0, "no voice key appears on two levels");

        /* Every knob must name a declared param, or it is a dead control. */
        int dead = 0;
        for (size_t p = h.find("\"knobs\":["); p != std::string::npos;
             p = h.find("\"knobs\":[", p + 1)) {
            size_t e = h.find(']', p);
            std::string arr = h.substr(p + 9, e - p - 9);
            size_t q = 0;
            while ((q = arr.find('"', q)) != std::string::npos) {
                size_t r = arr.find('"', q + 1);
                std::string k = arr.substr(q + 1, r - q - 1);
                q = r + 1;
                if (k.empty()) continue;             /* a deliberate gap */
                if (k == "ui_current_voice") continue;
                if (find_voice_param(k.c_str()) < 0 && find_global_param(k.c_str()) < 0) {
                    printf("        knob with no param: %s\n", k.c_str());
                    dead++;
                }
            }
        }
        ok(dead == 0, "every knob names a declared parameter");
    }

    /* ================= module.json agreement ================= */
    printf("\nmodule.json:\n");
    {
        std::string mj = read_file("src/module.json");
        if (mj.empty()) {
            printf("  skip  src/module.json not readable from this cwd\n");
        } else {
            std::string cp = get(api, inst, "chain_params");
            /* The generator reproduces build_chain_params' output from the
             * same tables; if that reproduction drifts, modulation targets go
             * silently dead. Compare key by key. */
            int missing = 0;
            for (int g = 0; g < GP_COUNT; g++) {
                std::string pat = std::string("\"key\": \"") + GLOBAL_PARAMS[g].key + "\"";
                if (mj.find(pat) == std::string::npos) missing++;
            }
            for (int v = 0; v < SIMIAN_VOICES; v++)
                for (int i = 0; i < VP_COUNT; i++) {
                    std::string pat = std::string("\"key\": \"") + VOICES[v].id + "_" +
                                      VOICE_PARAMS[i].key + "\"";
                    if (mj.find(pat) == std::string::npos) missing++;
                }
            okf(missing == 0, "module.json chain_params covers every key (%d missing)", missing);
            ok(mj.size() <= 65536, "module.json is inside the host's 64 KB limit");
            ok(mj.find("\"ui_hierarchy\"") != std::string::npos,
               "module.json carries an inline ui_hierarchy");
            (void)cp;
        }
    }

    /* ================= audio ================= */
    printf("\naudio:\n");
    api->set_param(inst, "preset", "0");
    ok(render_peak(api, inst, 200) < 0.001f, "silent with nothing played");
    note_on(api, inst, 36, 100);
    float kick = render_peak(api, inst, 400);
    okf(kick > 0.01f, "a kick makes a sound (peak %.3f)", kick);

    /* The whole reason for the forced edge: two hits in a row must both
     * sound, even when nothing renders between the note-off and the next
     * note-on. */
    {
        note_on(api, inst, 36, 100);
        render_peak(api, inst, 400);
        note_on(api, inst, 36, 100);
        float second = render_peak(api, inst, 50);
        okf(second > 0.01f, "a repeated note retriggers (peak %.3f)", second);
    }

    /* Velocity has to reach the envelope, or the kit plays flat. */
    {
        note_on(api, inst, 36, 127);
        float loud = render_peak(api, inst, 400);
        note_on(api, inst, 36, 20);
        float soft = render_peak(api, inst, 400);
        okf(soft < loud, "velocity changes level (%.3f soft vs %.3f loud)", soft, loud);
    }

    /* Silence tracking: a voice that has decayed stops being computed. */
    {
        note_on(api, inst, 36, 100);
        render_peak(api, inst, 50);
        ok(S->voice[V_KICK].active, "a struck voice is active");
        render_peak(api, inst, 4000);
        ok(!S->voice[V_KICK].active, "a decayed voice goes inactive and stops costing CPU");
    }

    /* Choke: the closed hat silences the open one. */
    {
        api->set_param(inst, "voice8_decay", "2000");   /* HH Open, long */
        note_on(api, inst, 46, 120);
        render_peak(api, inst, 100);
        float ringing = render_peak(api, inst, 50);
        note_on(api, inst, 42, 1);                      /* HH Closed, quietest */
        api->set_param(inst, "voice6_volume", "-60");   /* …and inaudible */
        float after = render_peak(api, inst, 60);
        okf(after < ringing, "a closed hat chokes the open one (%.3f -> %.3f)", ringing, after);
    }

    /* An unmapped note is not ours. */
    {
        render_peak(api, inst, 3000);
        note_on(api, inst, 60, 127);
        ok(render_peak(api, inst, 200) < 0.001f, "a note outside the map plays nothing");
    }

    /* ================= split render ================= */
    printf("\nsplit render:\n");
    {
        const int frames = 128;
        std::vector<int16_t> bus(frames * 2), main(frames * 2);
        int16_t *vo[SIMIAN_VOICES];
        /* Kick on its own bus; everything else on main, which is how the host
         * hands it over — an unrouted voice gets the main buffer. */
        for (int v = 0; v < SIMIAN_VOICES; v++) vo[v] = main.data();
        vo[V_KICK] = bus.data();

        api->set_param(inst, "preset", "0");
        render_peak(api, inst, 3000);                   /* settle */
        note_on(api, inst, 36, 120);

        float bus_peak = 0, main_peak = 0;
        for (int b = 0; b < 40; b++) {
            std::fill(bus.begin(), bus.end(), 0);
            std::fill(main.begin(), main.end(), 0);
            move_plugin_render_split(inst, vo, SIMIAN_VOICES, main.data(), frames);
            for (int i = 0; i < frames * 2; i++) {
                float a = fabsf(bus[i] / 32768.0f);  if (a > bus_peak) bus_peak = a;
                float m = fabsf(main[i] / 32768.0f); if (m > main_peak) main_peak = m;
            }
        }
        okf(bus_peak > 0.01f, "a routed voice lands in its own buffer (peak %.3f)", bus_peak);
        okf(main_peak < bus_peak,
            "…and not in main (main %.3f vs bus %.3f)", main_peak, bus_peak);

        /* A voice with no bus is handed main_out, and must still be heard. */
        for (int v = 0; v < SIMIAN_VOICES; v++) vo[v] = main.data();
        note_on(api, inst, 38, 120);
        float m2 = 0;
        for (int b = 0; b < 40; b++) {
            std::fill(main.begin(), main.end(), 0);
            move_plugin_render_split(inst, vo, SIMIAN_VOICES, main.data(), frames);
            for (int i = 0; i < frames * 2; i++) {
                float m = fabsf(main[i] / 32768.0f); if (m > m2) m2 = m;
            }
        }
        okf(m2 > 0.01f, "an unrouted voice still reaches main_out (peak %.3f)", m2);
    }

    /* ================= focus following ================= */
    printf("\nfocus:\n");
    {
        void *f = api->create_instance(".", NULL);
        simian_t *F = (simian_t *)f;
        /* No host has vouched and nothing is sequencing: a bare note is a hand. */
        note_on(api, f, 38, 100);
        ok(F->ui_current_voice == V_SNARE, "with no vouching host, a bare note moves focus");

        /* A vouch arriving FIRST arms, and the next note claims it. This is
         * also what makes the host a VOUCHING host from here on, which is why
         * it comes before the late-vouch case: after it, a bare note no
         * longer moves focus by itself and the correlation is what is left. */
        F->ui_current_voice = 0;
        api->set_param(f, "ui_live_press", "1");
        note_on(api, f, 49, 100);
        ok(F->ui_current_voice == V_CYMBAL, "an early vouch is claimed by the next note");

        /* A vouch arriving AFTER the note claims the note it belongs to —
         * the usual order, since the note comes straight off MIDI and the
         * vouch crosses a process boundary. */
        note_on(api, f, 42, 100);
        ok(F->ui_current_voice == V_CYMBAL,
           "a bare note alone does not move focus once a host vouches");
        api->set_param(f, "ui_live_press", "1");
        ok(F->ui_current_voice == V_HHCLOSED, "a late vouch matches the note just played");

        /* A host that names the note needs no correlation at all. */
        F->ui_current_voice = 0;
        api->set_param(f, "ui_live_note", "41");
        ok(F->ui_current_voice == V_LOWTOM, "ui_live_note names the voice outright");
        api->set_param(f, "ui_live_note", "60");
        ok(F->ui_current_voice == V_LOWTOM, "an unmapped ui_live_note moves nothing");

        /* ⚠ DR32's bug, tested so it cannot come back: once a host has
         * vouched, bare-note following must come BACK after the vouching
         * stops, or pad-follow works and then silently dies. */
        F->ui_current_voice = 0;
        F->block += SIMIAN_VOUCH_TTL_BLOCKS + 1;
        note_on(api, f, 39, 100);
        ok(F->ui_current_voice == V_CLAP,
           "a host that has gone quiet stops suppressing bare-note follow");

        api->destroy_instance(f);
    }

    /* The hierarchy the DSP actually SERVES, for tools/pages_check.mjs to run
     * upstream's own validator and voice resolver over. The host plans every
     * page from this text, never from module.json, so this is the input that
     * matters. */
    {
        std::string h = get(api, inst, "ui_hierarchy");
        system("mkdir -p dist/tests");
        FILE *f = fopen("dist/tests/served_hierarchy.json", "wb");
        if (f) { fwrite(h.data(), 1, h.size(), f); fclose(f); printf("\nwrote dist/tests/served_hierarchy.json\n"); }
        else   { printf("\nWARNING: could not write dist/tests/served_hierarchy.json\n"); }
    }

    api->destroy_instance(inst);

    printf("\n%d checks, %d failures\n", g_checks, g_fail);
    return g_fail ? 1 : 0;
}
