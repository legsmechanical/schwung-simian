/* The custom Voice widget, checked the way only node can.
 *
 *   node tests/widget.test.mjs        (run by scripts/test.sh)
 *
 * The C suite cannot reach canvas.js and canvas.js cannot reach PADS[], so
 * the one thing that can go silently wrong — the two tables disagreeing about
 * which pad is which drum — has to be checked from here. A pad reseated in
 * the wrapper would otherwise draw the WRONG DRUM, and nothing anywhere would
 * complain: the widget would still render a perfectly good icon.
 */
import fs from "node:fs";

let fail = 0;
const ok = (cond, what) => {
    if (cond) console.log(`  ok    ${what}`);
    else { console.log(`  FAIL  ${what}`); fail++; }
};

console.log("widget.test:");

/* ---- the wrapper's view ---- */
const wrapper = fs.readFileSync("src/dsp/simian_plugin.cpp", "utf8");
const pads = [...wrapper.matchAll(/\{"(pad\d+)",\s*"([^"]+)",\s*(V_\w+),\s*(\d+),/g)]
    .map((m) => ({ id: m[1], label: m[2], voice: m[3], note: +m[4] }));
ok(pads.length === 16, `PADS[] parsed (${pads.length} pads)`);

/* ---- the widget's view ---- */
/* Resolve against this file's own URL — pathToFileURL on a .pathname
 * double-encodes a directory with a space in it, which this repo has. */
await import(new URL("../src/canvas.js", import.meta.url).href);
const overlay = globalThis.canvas_overlay;
ok(!!overlay, "canvas.js defines canvas_overlay");
ok(overlay && overlay.widgetKind === "custom:voicelink",
   "it declares the kind chain_params asks for");
ok(overlay && typeof overlay.drawCell === "function", "it exports drawCell");

const src = fs.readFileSync("src/canvas.js", "utf8");
const names = overlay._names;
ok(Array.isArray(names) && names.length === 16,
   `NAMES[] has one entry per pad (${names ? names.length : "none"})`);

/* THE CHECK: two pads on one voice must show the same name, and two pads on
 * different voices must not — which is exactly "the tables agree". */
let mismatched = 0;
for (let i = 0; i < pads.length; i++)
    for (let j = i + 1; j < pads.length; j++) {
        const sameVoice = pads[i].voice === pads[j].voice;
        const sameName = names[i] === names[j];
        if (sameVoice !== sameName) {
            console.log(`        pad ${i + 1} (${names[i]}) vs pad ${j + 1} (${names[j]}): ` +
                        `voice ${sameVoice ? "same" : "differs"}, name ${sameName ? "same" : "differs"}`);
            mismatched++;
        }
    }
ok(mismatched === 0, "every name matches its pad's voice, and only its voice");

/* ⚠ A character with no glyph draws NOTHING — silently, exactly as the
 * device font does for lowercase. This is the check that the shipped font
 * actually covers the shipped names. */
const glyphs = new Set(
    [...src.matchAll(/^\s{4}(\w): \[/gm)].map((m) => m[1]));
const missing = new Set();
for (const n of names) for (const ch of n) if (!glyphs.has(ch)) missing.add(ch);
ok(missing.size === 0, `every character in every name has a glyph${missing.size ? `: missing ${[...missing].join(" ")}` : ""}`);

/* The box is 32 wide at an advance of 6, so a 6-character name would be
 * clipped by the context without a word of complaint. */
const tooWide = names.filter((n) => n.length * 6 - 1 > 32);
ok(tooWide.length === 0, `every name fits 32px${tooWide.length ? `: ${tooWide.join(" ")}` : ""}`);

/* The wrapper must serve the extra key the widget reads, or the chain never
 * draws and the page looks finished. */
ok(/ui_voice_link/.test(wrapper), "the wrapper serves ui_voice_link");
const mj = JSON.parse(fs.readFileSync("src/module.json", "utf8"));
const voiceParam = mj.capabilities.chain_params.find((p) => p.key === "ui_current_voice");
ok(voiceParam && voiceParam.viz && voiceParam.viz.kind === "custom:voicelink",
   "module.json declares the custom kind on the Voice knob");
ok(voiceParam && (voiceParam.viz.extra_keys || []).includes("ui_voice_link"),
   "…and asks for ui_voice_link as an extra value");

/* Every icon named must exist, or the cell silently draws nothing. */
const drawn = new Set();
const probe = {
    width: 32, height: 22,
    fillRect() {}, setPixel() {}, line() {}, print() {}, textWidth: () => 0,
};
for (let i = 0; i < 16; i++) {
    let threw = null;
    try { overlay.drawCell(probe, { values: { ui_current_voice: i + 1, ui_voice_link: 0 },
                                    group: { keys: ["ui_current_voice"] } }); }
    catch (e) { threw = e; }
    if (threw) console.log(`        pad ${i + 1} threw: ${threw}`);
    else drawn.add(i + 1);
}
ok(drawn.size === 16, `every pad draws without throwing (${drawn.size}/16)`);
/* A throw is not merely a missing picture: the host disables the widget for
 * the whole session on the FIRST one. */

console.log(`\n${fail ? `widget.test: FAILED (${fail})` : "widget.test: OK"}`);
process.exit(fail ? 1 : 0);
