/*
 * Host-side tests for the SIMIAN wrapper: build native, run on the workstation.
 *
 *   ./scripts/test.sh
 *
 * The wrapper is #included rather than linked so the tests can reach the
 * static tables (PADS, VOICE_PARAMS, SIMIAN_FACTORY) the module's behaviour
 * is defined by. Everything here is reachable without a device: the DSP is
 * deterministic and needs no audio hardware, so "does this pad make a sound
 * on THAT voice" is a real assertion and not a stand-in for playing it.
 */
#include "../src/dsp/simian_plugin.cpp"

#include <cstdio>
#include <cstdarg>
#include <cmath>
#include <string>
#include <vector>
#include <algorithm>

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
    ok(VP_COUNT == 19, "19 shared parameters per voice");
    ok(PP_COUNT == 1, "one parameter belongs to the pad (Tune)");
    ok(SIMIAN_VOICES == 10 && SIMIAN_PADS == 16, "ten voices on sixteen pads");
    ok(SIMIAN_FACTORY_COUNT == 35, "35 factory kits");
    ok(SIMIAN_FACTORY_VPARAMS == VP_COUNT,
       "factory bank width matches VOICE_PARAMS[] — a mismatch loads every value one slot out");
    ok(SIMIAN_FACTORY_GPARAMS == GP_COUNT, "factory bank global width matches GLOBAL_PARAMS[]");
    ok(SIMIAN_FACTORY_VOICES == SIMIAN_VOICES, "factory bank has a row per voice");

    /* The id is what the host substitutes into "{id}_send_a"; if it stops
     * being "pad" + the 1-based index, every send lands on its neighbour. */
    {
        bool idsok = true, notesok = true, voicesok = true;
        for (int d = 0; d < SIMIAN_PADS; d++) {
            char want[16];
            snprintf(want, sizeof(want), "pad%d", d + 1);
            if (strcmp(PADS[d].id, want) != 0) idsok = false;
            if (PADS[d].note != 36 + d) notesok = false;
            if (PADS[d].voice < 0 || PADS[d].voice >= SIMIAN_VOICES) voicesok = false;
        }
        ok(idsok, "pad ids are pad1..pad16, 1-based, matching child_index_base");
        ok(notesok, "pads are notes 36..51, contiguous — Move's drum grid");
        ok(voicesok, "every pad names a real voice");
    }
    {
        int seen[SIMIAN_VOICES] = {0};
        for (int d = 0; d < SIMIAN_PADS; d++) seen[PADS[d].voice]++;
        bool all = true, tally = true;
        for (int v = 0; v < SIMIAN_VOICES; v++) if (!seen[v]) all = false;
        ok(all, "every voice has at least one pad");
        int pairs = 0;
        for (int v = 0; v < SIMIAN_VOICES; v++) {
            if (seen[v] > 2) tally = false;
            if (seen[v] == 2) pairs++;
        }
        ok(tally, "no voice has more than two pads");
        okf(pairs == 6, "six voices carry an alias pad (%d)", pairs);
        ok(seen[V_SNARE] == 2 && seen[V_LOWTOM] == 2 && seen[V_MIDTOM] == 2 &&
           seen[V_HITOM] == 2,
           "the snare and all three toms keep the alias upstream gives them");
    }
    {
        /* The whole point of the re-seating: an alias must be NEXT TO its
         * parent on the 4-wide grid — ±1 within a row, or ±4. */
        bool adjacent = true;
        for (int d = 0; d < SIMIAN_PADS; d++)
            for (int e = d + 1; e < SIMIAN_PADS; e++) {
                if (PADS[d].voice != PADS[e].voice) continue;
                int gap = e - d;
                bool same_row = (gap == 1) && (d / 4 == e / 4);
                bool stacked  = (gap == 4);
                if (!same_row && !stacked) {
                    printf("        %s and %s share a voice but do not touch\n",
                           PADS[d].label, PADS[e].label);
                    adjacent = false;
                }
            }
        ok(adjacent, "every alias pad touches its parent on the grid");
    }
    {
        /* An alias with tune 0 is the duplicate this design exists to avoid. */
        bool tuned = true;
        for (int d = 0; d < SIMIAN_PADS; d++) {
            int owner = voice_owner_pad(PADS[d].voice);
            if (owner != d && PADS[d].tune == 0.0f) {
                printf("        %s is an alias with no default tune\n", PADS[d].label);
                tuned = false;
            }
            if (owner == d && PADS[d].tune != 0.0f) {
                printf("        %s owns its voice but is detuned\n", PADS[d].label);
                tuned = false;
            }
        }
        ok(tuned, "every alias ships with a nonzero Tune, every parent with zero");
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

    /* ================= params: pad vs voice ================= */
    printf("\nparams:\n");
    api->set_param(inst, "pad1_pitch", "55");
    ok(get(api, inst, "pad1_pitch") == "55.000", "prefixed key round-trips");
    ok(*S->voice[V_KICK].zone[find_voice_param("pitch")] == 55.0f, "…and reaches the Faust zone");
    api->set_param(inst, "pad16_pitch", "1000");
    ok(get(api, inst, "pad16_pitch") == "1000.000", "two-digit pad index parses");
    ok(get(api, inst, "pad1_pitch") == "55.000", "…without touching pad1");

    /* The sharing, stated as a test: pad3 and pad4 are one snare. */
    api->set_param(inst, "pad3_decay", "321");
    ok(get(api, inst, "pad4_decay") == "321.000",
       "an alias SHARES its parent's voice parameters (one snare, two pads)");
    api->set_param(inst, "pad4_decay", "654");
    ok(get(api, inst, "pad3_decay") == "654.000", "…and the sharing goes both ways");

    /* Tune is the one thing it does not share. */
    api->set_param(inst, "pad4_tune", "5");
    ok(get(api, inst, "pad4_tune") == "5.000", "tune round-trips");
    ok(get(api, inst, "pad3_tune") == "0.000", "…and belongs to the PAD, not the voice");
    api->set_param(inst, "pad4_tune", "999");
    ok(get(api, inst, "pad4_tune") == "24.000", "tune is clamped to its declared range");

    api->set_param(inst, "ui_current_pad", "12");
    ok(get(api, inst, "ui_current_pad") == "12", "focus is 1-based on the wire");
    api->set_param(inst, "punch", "44");
    ok(get(api, inst, "pad12_punch") == "44.000", "a bare key edits the FOCUSED pad");
    api->set_param(inst, "tune", "3");
    ok(get(api, inst, "pad12_tune") == "3.000", "…including its tune");

    api->set_param(inst, "pad1_pitch", "99999");
    ok(get(api, inst, "pad1_pitch") == "4400.000", "out-of-range is clamped, not wrapped");
    ok(get(api, inst, "no_such_param") == "<err>",
       "an unknown key answers NEGATIVE, never an empty value");
    ok(get(api, inst, "pad17_pitch") == "<err>", "an out-of-range pad index is not a param");
    ok(get(api, inst, "pad0_pitch") == "<err>", "pad0 does not exist (ids are 1-based)");

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
    api->set_param(inst, "pad7_cutoff", "1234");
    api->set_param(inst, "pad15_tune", "-9");
    api->set_param(inst, "gain", "-12");
    std::string blob = get(api, inst, "state");
    ok(blob.find("\"pad7_cutoff\":1234") != std::string::npos,
       "state carries a voice parameter under the pad that owns it");
    ok(blob.find("\"pad8_cutoff\"") == std::string::npos,
       "…and NOT under the alias, which would be one value written twice");
    ok(blob.find("\"pad15_tune\":-9") != std::string::npos, "state carries every pad's tune");
    ok(blob.find("\"gain\":-12") != std::string::npos, "state carries the globals");
    {
        void *b = api->create_instance(".", blob.c_str());
        ok(get(api, b, "pad7_cutoff") == "1234.000", "state restores a shared value");
        ok(get(api, b, "pad8_cutoff") == "1234.000", "…visible from the alias too");
        ok(get(api, b, "pad15_tune") == "-9.000", "state restores a pad tune");
        ok(get(api, b, "gain") == "-12.000", "state restores a global");
        ok(get(api, b, "ui_current_pad") == "12", "state restores the focused pad");
        api->destroy_instance(b);
    }

    /* ================= published contracts ================= */
    printf("\ncontracts:\n");
    std::string sv = get(api, inst, "split_voices");
    {
        bool allthere = true;
        for (int v = 0; v < SIMIAN_VOICES; v++) {
            std::string want = std::string("\"id\":\"") + PADS[voice_owner_pad(v)].id + "\"";
            if (sv.find(want) == std::string::npos) allthere = false;
        }
        ok(allthere, "split_voices publishes the ten owning pads");

        bool no_alias = true;
        for (int d = 0; d < SIMIAN_PADS; d++) {
            if (voice_owner_pad(PADS[d].voice) == d) continue;
            std::string nope = std::string("\"id\":\"") + PADS[d].id + "\"";
            if (sv.find(nope) != std::string::npos) no_alias = false;
        }
        ok(no_alias, "an alias pad is NOT published as a voice of its own");

        size_t at = 0; bool ordered = true;
        for (int v = 0; v < SIMIAN_VOICES; v++) {
            std::string want = std::string("\"id\":\"") + PADS[voice_owner_pad(v)].id + "\"";
            size_t p = sv.find(want, at);
            if (p == std::string::npos) { ordered = false; break; }
            at = p;
        }
        ok(ordered, "split_voices is in VOICES[] order — entry i is buffer i");
    }
    {
        std::string tpl = get(api, inst, "voice_send_params");
        ok(tpl == "[\"{id}_send_a\",\"{id}_send_b\"]", "two send templates, A then B");
        bool resolves = true;
        for (int v = 0; v < SIMIAN_VOICES; v++)
            for (const char *sfx : {"_send_a", "_send_b"}) {
                std::string k = std::string(PADS[voice_owner_pad(v)].id) + sfx;
                if (get(api, inst, k.c_str()) == "<err>") resolves = false;
            }
        ok(resolves, "every {id}_send_a / _send_b substitution addresses a real param");
    }

    /* ================= hierarchy ================= */
    printf("\nhierarchy:\n");
    {
        std::string h = get(api, inst, "ui_hierarchy");
        ok(h.find("\"pad_layout\":\"drums\"") != std::string::npos, "pad_layout says drums");
        int notemaps = 0;
        for (size_t p = h.find("child_notes"); p != std::string::npos;
             p = h.find("child_notes", p + 1)) notemaps++;
        for (size_t p = h.find("child_note_base"); p != std::string::npos;
             p = h.find("child_note_base", p + 1)) notemaps++;
        ok(notemaps == 1, "exactly one level declares a note map");
        ok(h.find("\"child_count\":16") != std::string::npos, "the child level has sixteen pads");

        /* ⚠ A key repeated across levels makes the loader drop ALL of its
         * metadata, which silently kills the per-voice sends. */
        int dup = 0;
        const char *lv[] = {"\"pads\"", "\"pad_noise\"", "\"pad_mix\""};
        for (int i = 0; i < VP_COUNT + PP_COUNT; i++) {
            const char *k = (i < VP_COUNT) ? VOICE_PARAMS[i].key : PAD_PARAMS[i - VP_COUNT].key;
            std::string pat = std::string("\"") + k + "\"";
            int levels_with = 0;
            for (int L = 0; L < 3; L++) {
                size_t start = h.find(lv[L]);
                if (start == std::string::npos) continue;
                size_t end = h.size();
                for (int M = 0; M < 3; M++) {
                    size_t o = h.find(lv[M], start + 1);
                    if (o != std::string::npos && o < end) end = o;
                }
                size_t o = h.find("\"output\"", start + 1);
                if (o != std::string::npos && o < end) end = o;
                std::string span = h.substr(start, end - start);
                size_t ck = span.find("child_copy_keys");   /* lists everything by design */
                if (ck != std::string::npos) span.erase(ck, span.find(']', ck) - ck);
                if (span.find(pat) != std::string::npos) levels_with++;
            }
            if (levels_with > 1) { dup++; printf("        duplicated key: %s\n", k); }
        }
        ok(dup == 0, "no pad or voice key appears on two levels");

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
                if (k.empty() || k == "ui_current_pad") continue;
                if (find_voice_param(k.c_str()) < 0 && find_global_param(k.c_str()) < 0 &&
                    find_pad_param(k.c_str()) < 0) {
                    printf("        knob with no param: %s\n", k.c_str());
                    dead++;
                }
            }
        }
        ok(dead == 0, "every knob names a declared parameter");

        /* child_names must be one per pad, or the grid labels slide. */
        size_t cn = h.find("child_names");
        int names = 0;
        if (cn != std::string::npos) {
            size_t e = h.find(']', cn);
            int quotes = 0;
            for (size_t q = cn; q < e; q++) if (h[q] == '"') quotes++;
            /* find() lands INSIDE the key's quotes, so only its closing one
             * is in the span — subtract that, not both. */
            names = (quotes - 1) / 2;
        }
        okf(names == SIMIAN_PADS, "child_names has one entry per pad (%d)", names);
    }

    /* ================= module.json agreement ================= */
    printf("\nmodule.json:\n");
    {
        std::string mj = read_file("src/module.json");
        if (mj.empty()) {
            printf("  skip  src/module.json not readable from this cwd\n");
        } else {
            int missing = 0;
            for (int g = 0; g < GP_COUNT; g++)
                if (mj.find(std::string("\"key\": \"") + GLOBAL_PARAMS[g].key + "\"") == std::string::npos)
                    missing++;
            for (int d = 0; d < SIMIAN_PADS; d++)
                if (mj.find(std::string("\"key\": \"") + PADS[d].id + "_tune\"") == std::string::npos)
                    missing++;
            for (int v = 0; v < SIMIAN_VOICES; v++)
                for (int i = 0; i < VP_COUNT; i++) {
                    std::string pat = std::string("\"key\": \"") + PADS[voice_owner_pad(v)].id +
                                      "_" + VOICE_PARAMS[i].key + "\"";
                    if (mj.find(pat) == std::string::npos) missing++;
                }
            okf(missing == 0, "module.json chain_params covers every key (%d missing)", missing);
            int entries = 0;
            for (size_t p = mj.find("\"key\":"); p != std::string::npos;
                 p = mj.find("\"key\":", p + 1)) entries++;
            okf(entries <= 256, "chain_params is inside MAX_CHAIN_PARAMS (%d)", entries);
            ok(mj.size() <= 65536, "module.json is inside the host's 64 KB limit");
            ok(mj.find("\"ui_hierarchy\"") != std::string::npos,
               "module.json carries an inline ui_hierarchy");
        }
    }

    /* ================= audio ================= */
    printf("\naudio:\n");
    api->set_param(inst, "preset", "0");
    ok(render_peak(api, inst, 200) < 0.001f, "silent with nothing played");

    /* Every one of the sixteen pads sounds — the thing upstream's two
     * choke-only notes got wrong on a grid. */
    {
        int silent = 0;
        for (int d = 0; d < SIMIAN_PADS; d++) {
            render_peak(api, inst, 2500);              /* let the last one decay */
            note_on(api, inst, PADS[d].note, 110);
            float pk = render_peak(api, inst, 300);
            if (pk <= 0.005f) {
                printf("        %s (note %d) is silent\n", PADS[d].label, PADS[d].note);
                silent++;
            }
        }
        ok(silent == 0, "all sixteen pads make a sound");
    }

    /* A pad's Tune must actually reach the pitch. */
    {
        render_peak(api, inst, 2500);
        api->set_param(inst, "pad4_tune", "0");
        note_on(api, inst, 39, 110);
        float a = render_peak(api, inst, 300);
        render_peak(api, inst, 2500);
        api->set_param(inst, "pad4_tune", "12");
        note_on(api, inst, 39, 110);
        float b = render_peak(api, inst, 300);
        okf(fabsf(a - b) > 0.0005f, "Tune changes what the pad plays (%.4f vs %.4f)", a, b);
    }

    {
        note_on(api, inst, 36, 100);
        render_peak(api, inst, 400);
        note_on(api, inst, 36, 100);
        float second = render_peak(api, inst, 50);
        okf(second > 0.01f, "a repeated note retriggers (peak %.3f)", second);
    }
    {
        note_on(api, inst, 36, 127);
        float loud = render_peak(api, inst, 400);
        note_on(api, inst, 36, 20);
        float soft = render_peak(api, inst, 400);
        okf(soft < loud, "velocity changes level (%.3f soft vs %.3f loud)", soft, loud);
    }
    {
        note_on(api, inst, 36, 100);
        render_peak(api, inst, 50);
        ok(S->voice[V_KICK].active, "a struck voice is active");
        render_peak(api, inst, 4000);
        ok(!S->voice[V_KICK].active, "a decayed voice goes inactive and stops costing CPU");
    }

    /* Chokes follow Move's groups: the three hats silence each other. */
    {
        render_peak(api, inst, 3000);
        api->set_param(inst, "pad11_decay", "2000");    /* HH Open, long */
        note_on(api, inst, 46, 120);
        render_peak(api, inst, 100);
        float ringing = render_peak(api, inst, 50);
        api->set_param(inst, "pad7_volume", "-60");     /* closed hat, inaudible */
        note_on(api, inst, 42, 1);
        float after = render_peak(api, inst, 60);
        okf(after < ringing, "a closed hat chokes the open one (%.3f -> %.3f)", ringing, after);
    }
    {
        /* …and so does the SECOND closed hat, which is the alias. */
        render_peak(api, inst, 3000);
        note_on(api, inst, 46, 120);
        render_peak(api, inst, 100);
        float ringing = render_peak(api, inst, 50);
        note_on(api, inst, 43, 1);
        float after = render_peak(api, inst, 60);
        okf(after < ringing, "the alias hat chokes the open one too (%.3f -> %.3f)", ringing, after);
    }
    {
        /* A pad must never choke its own voice — that would cut the hit. */
        render_peak(api, inst, 3000);
        api->set_param(inst, "pad7_volume", "0");
        note_on(api, inst, 42, 120);
        float pk = render_peak(api, inst, 200);
        okf(pk > 0.005f, "a hat does not choke ITSELF (peak %.3f)", pk);
    }
    {
        render_peak(api, inst, 3000);
        note_on(api, inst, 60, 127);
        ok(render_peak(api, inst, 200) < 0.001f, "a note outside 36..51 plays nothing");
    }

    /* ================= split render ================= */
    printf("\nsplit render:\n");
    {
        const int frames = 128;
        std::vector<int16_t> bus(frames * 2), main(frames * 2);
        int16_t *vo[SIMIAN_VOICES];
        for (int v = 0; v < SIMIAN_VOICES; v++) vo[v] = main.data();
        vo[V_KICK] = bus.data();

        api->set_param(inst, "preset", "0");
        render_peak(api, inst, 3000);
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
        okf(main_peak < bus_peak, "…and not in main (main %.3f vs bus %.3f)", main_peak, bus_peak);

        /* BOTH pads of a pair follow the voice onto its bus — one DSP, so
         * there is nothing to route separately. */
        render_peak(api, inst, 3000);
        vo[V_SNARE] = bus.data();
        note_on(api, inst, 39, 120);                 /* the ALIAS pad */
        float alias_bus = 0;
        for (int b = 0; b < 40; b++) {
            std::fill(bus.begin(), bus.end(), 0);
            std::fill(main.begin(), main.end(), 0);
            move_plugin_render_split(inst, vo, SIMIAN_VOICES, main.data(), frames);
            for (int i = 0; i < frames * 2; i++) {
                float a = fabsf(bus[i] / 32768.0f); if (a > alias_bus) alias_bus = a;
            }
        }
        okf(alias_bus > 0.01f, "an alias pad rides its parent's bus (peak %.3f)", alias_bus);
    }

    /* ================= focus following ================= */
    printf("\nfocus:\n");
    {
        void *f = api->create_instance(".", NULL);
        simian_t *F = (simian_t *)f;
        note_on(api, f, 38, 100);
        ok(F->ui_current_pad == note_to_pad(38), "with no vouching host, a bare note moves focus");

        /* An alias must focus ITS OWN pad, not its parent's — otherwise its
         * Tune knob is unreachable from the grid. */
        note_on(api, f, 39, 100);
        ok(F->ui_current_pad == note_to_pad(39), "an alias pad focuses itself, not its parent");

        F->ui_current_pad = 0;
        api->set_param(f, "ui_live_press", "1");
        note_on(api, f, 49, 100);
        ok(F->ui_current_pad == note_to_pad(49), "an early vouch is claimed by the next note");

        note_on(api, f, 42, 100);
        ok(F->ui_current_pad == note_to_pad(49),
           "a bare note alone does not move focus once a host vouches");
        api->set_param(f, "ui_live_press", "1");
        ok(F->ui_current_pad == note_to_pad(42), "a late vouch matches the note just played");

        F->ui_current_pad = 0;
        api->set_param(f, "ui_live_note", "41");
        ok(F->ui_current_pad == note_to_pad(41), "ui_live_note names the pad outright");
        api->set_param(f, "ui_live_note", "60");
        ok(F->ui_current_pad == note_to_pad(41), "an unmapped ui_live_note moves nothing");

        /* ⚠ DR32's bug, tested so it cannot come back. */
        F->ui_current_pad = 0;
        F->block += SIMIAN_VOUCH_TTL_BLOCKS + 1;
        note_on(api, f, 40, 100);
        ok(F->ui_current_pad == note_to_pad(40),
           "a host that has gone quiet stops suppressing bare-note follow");

        api->destroy_instance(f);
    }

    /* The hierarchy the DSP actually SERVES, for tools/pages_check.mjs. */
    {
        std::string h = get(api, inst, "ui_hierarchy");
        system("mkdir -p dist/tests");
        FILE *f = fopen("dist/tests/served_hierarchy.json", "wb");
        if (f) { fwrite(h.data(), 1, h.size(), f); fclose(f);
                 printf("\nwrote dist/tests/served_hierarchy.json\n"); }
        else   { printf("\nWARNING: could not write dist/tests/served_hierarchy.json\n"); }
    }

    api->destroy_instance(inst);

    printf("\n%d checks, %d failures\n", g_checks, g_fail);
    return g_fail ? 1 : 0;
}
