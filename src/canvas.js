/*
 * SIMIAN's custom knob widget: the Voice cell names the drum.
 *
 * WHAT PROBLEM THIS SOLVES. Sixteen pads sit on ten voices, so six pads are a
 * SECOND pad on a voice another pad already owns. Everything on these pages
 * except Tune belongs to the voice, which means editing Snare 2's Decay edits
 * Snare's — there is one snare. Nothing on screen said so: the cell drew a
 * bare index, and a number cannot tell you that two of them are the same
 * drum. (Josh, from the device: "it's not clear that editing params on one
 * besides tune also affects the others", and "numbers don't mean anything
 * anyway to the user when the pad you press is what you edit and hear".)
 *
 * So the cell draws the drum's NAME, and a LINK MARK when the voice has a
 * second pad. Both pads of a pair draw the same name, which is the honest
 * picture: they are one drum.
 *
 * ⚠⚠ WHY THIS FILE CARRIES A FONT. Every font the device ships is
 * UPPERCASE-ONLY — font4x5's CHARS has no lowercase at all, and tamzen and
 * font5x3 are missing the same fourteen letters. A missing glyph renders as
 * NOTHING, silently, so "Snare" through the host font would draw as "S".
 * Mixed case therefore has to bring its own glyphs, and this is them: only
 * the 24 characters these ten names use, at 5x7 with one descender row.
 *
 * HOW IT IS REACHED. `ui_current_voice`'s chain_params entry declares
 * viz.kind "custom:voicelink", which is also what makes the host load this
 * file at all. If it fails to load, or an older host has never heard of the
 * kind, the kind does not claim the cell and the plain number is drawn
 * instead — the page stays correct, and the failure is named in the device's
 * debug.log rather than showing as a hole.
 *
 * THE VALUES. `ui_current_voice` is the cell's own key, 1..16.
 * `ui_voice_link` is declared in the same viz as an extra_key: the wrapper
 * answers the OTHER pad on this voice, 1-based, or 0 for a voice with one
 * pad. One extra read per value rotation, which is why it asks for one.
 *
 * THE BOX IS 32 x 15. render_page_movy hands a single-slot widget
 * { w: cellW, h: lblY - rowY } = 32 x 15, and the context CLIPS to it
 * silently. Everything below is laid out against those two numbers.
 *
 * DRAWING BUDGET. A page render is 1.68 ms for eight cells and every ctx call
 * is ~490 ns. Glyphs are compiled to horizontal RUNS once at load, so a
 * five-letter name costs ~30 fillRects rather than ~150 setPixels.
 *
 * ⚠ NAMES[] IS INDEXED BY PAD and must agree with PADS[] in
 * src/dsp/simian_plugin.cpp: two pads on one voice must show one name.
 * tests/widget.test.mjs parses both and fails if they drift — otherwise a pad
 * reseated in the wrapper would name the wrong drum and nothing would notice.
 *
 * GPL-3.0-or-later (DSP: Punk Labs LLC)
 */

/* Pad 1..16 -> the drum's name. Five characters is the budget: the box is 32
 * wide and the advance is 6, so a 5-letter name at 29 px is the widest that
 * fits. */
const NAMES = [
    "Kick", "Rim", "Snare", "Snare",
    "Clap", "TomL", "ClHat", "ClHat",
    "TomH", "TomL", "OpHat", "TomM",
    "TomH", "Crash", "Crash", "TomM",
];

/*
 * The font: 5 wide, cap height 7, x-height 5, one descender row.
 *
 * Row 0 is the cap line and row 6 the baseline, so a lowercase letter starts
 * two rows down and both cases sit on one baseline with no per-glyph offset.
 * Row 7 exists for 'p' alone.
 *
 * Only the characters these ten names use. An unknown one draws blank — the
 * same failure mode the device font has, caught by the test rather than by
 * squinting at the device.
 */
const GLYPHS = {
    C: [".###.", "#...#", "#....", "#....", "#....", "#...#", ".###."],
    H: ["#...#", "#...#", "#...#", "#####", "#...#", "#...#", "#...#"],
    K: ["#...#", "#..#.", "#.#..", "##...", "#.#..", "#..#.", "#...#"],
    L: ["#....", "#....", "#....", "#....", "#....", "#....", "#####"],
    M: ["#...#", "##.##", "#.#.#", "#.#.#", "#...#", "#...#", "#...#"],
    O: [".###.", "#...#", "#...#", "#...#", "#...#", "#...#", ".###."],
    P: ["####.", "#...#", "#...#", "####.", "#....", "#....", "#...."],
    R: ["####.", "#...#", "#...#", "####.", "#.#..", "#..#.", "#...#"],
    S: [".####", "#....", "#....", ".###.", "....#", "....#", "####."],
    T: ["#####", "..#..", "..#..", "..#..", "..#..", "..#..", "..#.."],

    a: [".....", ".....", ".###.", "....#", ".####", "#...#", ".####"],
    c: [".....", ".....", ".###.", "#....", "#....", "#....", ".###."],
    e: [".....", ".....", ".###.", "#...#", "#####", "#....", ".###."],
    h: ["#....", "#....", "#.##.", "##..#", "#...#", "#...#", "#...#"],
    i: ["..#..", ".....", "..#..", "..#..", "..#..", "..#..", "..#.."],
    k: ["#....", "#....", "#..#.", "#.#..", "##...", "#.#..", "#..#."],
    l: [".##..", "..#..", "..#..", "..#..", "..#..", "..#..", "..###"],
    m: [".....", ".....", "##.#.", "#.#.#", "#.#.#", "#.#.#", "#.#.#"],
    n: [".....", ".....", "#.##.", "##..#", "#...#", "#...#", "#...#"],
    o: [".....", ".....", ".###.", "#...#", "#...#", "#...#", ".###."],
    p: [".....", ".....", "####.", "#...#", "#...#", "####.", "#....", "#...."],
    r: [".....", ".....", "#.##.", "##..#", "#....", "#....", "#...."],
    s: [".....", ".....", ".####", "#....", ".###.", "....#", "####."],
    t: [".#...", ".#...", "####.", ".#...", ".#...", ".#...", "..##."],
};

/*
 * Josh's link mark, 7 x 3: two rings joined by a bridge, at the size he drew
 * it. Small on purpose — it is a status mark beside a name, not a second
 * thing to read. The bridge is the load-bearing pixel: two rings without it
 * read as "oo".
 */
const LINK = [
    "###.###",
    "#.###.#",
    "###.###",
];

const ADVANCE = 6, TEXT_H = 8;

/* Rows of "#" compiled to [x, y, w] runs, ONCE at load. Drawing a glyph then
 * costs one fillRect per run — two or three a row — instead of one call per
 * lit pixel. See the budget note at the top. */
function compile(rows) {
    const runs = [];
    for (let y = 0; y < rows.length; y++) {
        const row = rows[y];
        let x = 0;
        while (x < row.length) {
            if (row[x] !== "#") { x++; continue; }
            let w = 0;
            while (x + w < row.length && row[x + w] === "#") w++;
            runs.push([x, y, w]);
            x += w;
        }
    }
    return runs;
}

const RUNS = {};
for (const ch of Object.keys(GLYPHS)) RUNS[ch] = compile(GLYPHS[ch]);
const LINK_RUNS = compile(LINK);
const LINK_W = LINK[0].length, LINK_H = LINK.length;

function blit(ctx, runs, ox, oy, c) {
    for (let i = 0; i < runs.length; i++) {
        const r = runs[i];
        ctx.fillRect(ox + r[0], oy + r[1], r[2], 1, c);
    }
}

function textWidth(s) { return s.length ? s.length * ADVANCE - 1 : 0; }

function drawText(ctx, s, ox, oy, c) {
    for (let i = 0; i < s.length; i++) {
        const runs = RUNS[s[i]];
        if (runs) blit(ctx, runs, ox + i * ADVANCE, oy, c);
    }
}

globalThis.canvas_overlay = {
    widgetKind: "custom:voicelink",

    /* Exported for tests and for tools/icon_sheet.mjs — the names are the
     * thing most likely to drift from the wrapper's pad table. */
    _names: NAMES,

    drawCell(ctx, { values, group }) {
        const idx = Math.round(Number(values[group.keys[0]]) || 1);
        const linked = Math.round(Number(values["ui_voice_link"]) || 0) > 0;
        const name = NAMES[idx - 1];
        if (!name) return;

        const w = ctx.width, h = ctx.height;
        const tx = Math.max(0, (w - textWidth(name)) >> 1);

        /* THE NAME DOES NOT MOVE. It sits at the same y whether or not the
         * pad is linked, so stepping along the row reads as one line of text
         * with a mark appearing over it — not as a layout that jumps. At 3
         * rows for the mark, a blank row under it and 8 for the name, that
         * costs 12 of the 15 and everything fits without shuffling. */
        const ty = Math.max(LINK_H + 1, (h - TEXT_H + 1) >> 1);
        if (linked) blit(ctx, LINK_RUNS, (w - LINK_W) >> 1, 0, 1);
        drawText(ctx, name, tx, ty, 1);
    },
};
