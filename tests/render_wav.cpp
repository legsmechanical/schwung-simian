/*
 * Audition render: plays a beat through the module and writes a WAV, so a
 * change to the port can be HEARD without a device in front of you.
 *
 *   ./scripts/render.sh                  -> build/simian_demo.wav (kit 0)
 *   ./scripts/render.sh out.wav 12       -> kit 12
 *   ./scripts/render.sh out.wav all      -> two bars of every one of the 35
 *
 * Also prints a realtime factor. That number is measured on the workstation,
 * NOT on Move's ARM core — it is useful for spotting a change that made the
 * DSP ten times more expensive, and useless as an absolute budget. The only
 * real CPU answer comes from the device.
 */
#include "../src/dsp/simian_plugin.cpp"

#include <cstdio>
#include <cstring>
#include <ctime>
#include <vector>

static plugin_api_v2_t *g_api = NULL;
static void *g_inst = NULL;
static std::vector<int16_t> g_pcm;

static void render_ms(int ms) {
    const int frames = 128;
    int blocks = (MOVE_SAMPLE_RATE * ms / 1000) / frames;
    int16_t buf[frames * 2];
    for (int b = 0; b < blocks; b++) {
        g_api->render_block(g_inst, buf, frames);
        g_pcm.insert(g_pcm.end(), buf, buf + frames * 2);
    }
}

static void hit(int note, int vel) {
    uint8_t m[3] = {0x90, (uint8_t)note, (uint8_t)vel};
    g_api->on_midi(g_inst, m, 3, 0);
}

/* Two bars of a plain sixteenth-note pattern at 120 BPM: 125 ms a step.
 * Deliberately ordinary — the point is to hear the KIT, not the groove. */
static void play_two_bars(void) {
    static const struct { int step; int note; int vel; } PAT[] = {
        { 0, 36, 118}, { 0, 42,  70},
        { 2, 42,  50},
        { 4, 38, 110}, { 4, 42,  70},
        { 6, 42,  50}, { 7, 36,  80},
        { 8, 36, 112}, { 8, 42,  70},
        {10, 42,  50}, {10, 37,  60},
        {12, 38, 110}, {12, 42,  70},
        {14, 46,  90},
        {15, 39,  75},
    };
    const int STEPS = 16, STEP_MS = 125;
    for (int bar = 0; bar < 2; bar++)
        for (int s = 0; s < STEPS; s++) {
            for (size_t i = 0; i < sizeof(PAT) / sizeof(PAT[0]); i++)
                if (PAT[i].step == s) hit(PAT[i].note, PAT[i].vel);
            /* The last step of the last bar gets the crash and a long tail. */
            if (bar == 1 && s == 15) hit(49, 100);
            render_ms(STEP_MS);
        }
    render_ms(1200);
}

static void write_wav(const char *path) {
    FILE *f = fopen(path, "wb");
    if (!f) { printf("cannot write %s\n", path); return; }
    uint32_t data_bytes = (uint32_t)(g_pcm.size() * 2);
    uint32_t rate = MOVE_SAMPLE_RATE, byte_rate = rate * 2 * 2;
    uint16_t ch = 2, bits = 16, fmt = 1, align = 4;
    uint32_t riff = 36 + data_bytes, fmt_size = 16;
    fwrite("RIFF", 1, 4, f); fwrite(&riff, 4, 1, f); fwrite("WAVE", 1, 4, f);
    fwrite("fmt ", 1, 4, f); fwrite(&fmt_size, 4, 1, f);
    fwrite(&fmt, 2, 1, f); fwrite(&ch, 2, 1, f); fwrite(&rate, 4, 1, f);
    fwrite(&byte_rate, 4, 1, f); fwrite(&align, 2, 1, f); fwrite(&bits, 2, 1, f);
    fwrite("data", 1, 4, f); fwrite(&data_bytes, 4, 1, f);
    fwrite(g_pcm.data(), 1, data_bytes, f);
    fclose(f);
    printf("wrote %s (%.1f s, %u bytes)\n", path, data_bytes / (float)byte_rate, data_bytes);
}

int main(int argc, char **argv) {
    const char *path = (argc > 1) ? argv[1] : "build/simian_demo.wav";
    const char *which = (argc > 2) ? argv[2] : "0";
    int all = (strcmp(which, "all") == 0);

    g_api = move_plugin_init_v2(NULL);
    g_inst = g_api->create_instance(".", NULL);
    if (!g_inst) { printf("create_instance failed\n"); return 1; }

    clock_t t0 = clock();
    if (all) {
        for (int k = 0; k < SIMIAN_FACTORY_COUNT; k++) {
            char v[8]; snprintf(v, sizeof(v), "%d", k);
            g_api->set_param(g_inst, "preset", v);
            printf("  %2d  %s\n", k, SIMIAN_FACTORY[k].name);
            play_two_bars();
        }
    } else {
        g_api->set_param(g_inst, "preset", which);
        printf("  kit %s: %s\n", which, SIMIAN_FACTORY[atoi(which)].name);
        play_two_bars();
    }
    double cpu = (double)(clock() - t0) / CLOCKS_PER_SEC;
    double audio = (double)(g_pcm.size() / 2) / MOVE_SAMPLE_RATE;
    printf("rendered %.1f s of audio in %.2f s — %.0fx realtime (workstation, not Move)\n",
           audio, cpu, audio / (cpu > 0 ? cpu : 1e-9));

    write_wav(path);
    g_api->destroy_instance(g_inst);
    return 0;
}
