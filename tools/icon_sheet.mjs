/* icon_sheet.mjs — render SIMIAN's custom Voice widget for all sixteen pads,
 * as a PNG, using the DEVICE'S OWN framebuffer and font.
 *
 *   node tools/icon_sheet.mjs                 -> build/icon_sheet.png
 *   SCHWUNG_SRC=/path/to/schwung node tools/icon_sheet.mjs
 *
 * WHY THIS EXISTS. 1-bit art at 32x22 cannot be reviewed from its code — this
 * codebase has already learned that reviewing widgets from source rather than
 * from their render lets real defects through. Every swatch here comes out of
 * the same drawCell the device calls, through the same frameCtx, so a picture
 * that looks right here is the picture the OLED draws.
 *
 * Node-only; nothing here ships to the device.
 */
import fs from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

const SRC = process.env.SCHWUNG_SRC || path.resolve("..", "schwung-current");
const lib = (f) => pathToFileURL(path.join(SRC, "src", "shared", "param_pages", f)).href;
if (!fs.existsSync(path.join(SRC, "src", "shared", "param_pages", "frame_ctx.mjs"))) {
    console.error(`icon_sheet: no Schwung checkout at ${SRC} (set SCHWUNG_SRC) — skipped`);
    process.exit(0);
}
const { frameCtx } = await import(lib("frame_ctx.mjs"));
const harness = pathToFileURL(path.join(SRC, "tools", "param-pages", "harness.mjs")).href;
const { createFramebuffer, drawContext } = await import(harness);

/* Loading canvas.js the way the device does: it assigns globalThis. */
await import(pathToFileURL(path.resolve("src/canvas.js")).href);
const overlay = globalThis.canvas_overlay;
if (!overlay || typeof overlay.drawCell !== "function") {
    console.error("icon_sheet: src/canvas.js did not define canvas_overlay.drawCell");
    process.exit(1);
}

/* THE REAL CELL GEOMETRY, taken from the host: render_page_movy.mjs passes
 * drawVizGroup { w: slotSpan * cellW, h: lblY - rowY } — so a single-slot
 * widget gets CELL_W = 32 by BOX_H = 15, and NOT the 22 this sheet first
 * assumed. Drawing to a taller box than the device gives is how art that
 * looks right here arrives clipped on the OLED. */
const CELL_W = 32, CELL_H = 15, COLS = 4, ROWS = 4, PAD = 2, LABEL_H = 3;
/* No captions: the widget says what it is. LABEL_H is just the gutter that
 * keeps four rows of cells from running together. */
const W = COLS * (CELL_W + PAD) + PAD;
const H = ROWS * (CELL_H + LABEL_H + PAD) + PAD;

const fb = createFramebuffer(W, H);
const ctx = drawContext(fb);

/* The pairs come from PADS[] itself, not from a copy: a third table would be
 * a third thing to keep in step, and this sheet exists to be trusted. */
const wrapper = fs.readFileSync("src/dsp/simian_plugin.cpp", "utf8");
const padVoices = [...wrapper.matchAll(/\{"pad\d+",\s*"[^"]+",\s*(V_\w+),/g)].map((m) => m[1]);
if (padVoices.length !== 16) {
    console.error(`icon_sheet: parsed ${padVoices.length} pads from the wrapper, want 16`);
    process.exit(1);
}
const PARTNER = padVoices.map((v, i) => {
    const j = padVoices.findIndex((o, k) => k !== i && o === v);
    return j < 0 ? 0 : j + 1;
});

for (let i = 0; i < 16; i++) {
    const cx = PAD + (i % COLS) * (CELL_W + PAD);
    const cy = PAD + Math.floor(i / COLS) * (CELL_H + LABEL_H + PAD);
    const cell = frameCtx(ctx, { x: cx, y: cy, w: CELL_W, h: CELL_H });
    overlay.drawCell(cell, {
        values: { ui_current_voice: i + 1, ui_voice_link: PARTNER[i] },
        group: { keys: ["ui_current_voice"] },
    });
}

/* The harness framebuffer writes its own PNG — same encoder the host's widget
 * sheet uses, so this picture is comparable to upstream's. */
fs.mkdirSync("build", { recursive: true });
const out = process.argv[2] || "build/icon_sheet.png";
fs.writeFileSync(out, fb.toPng(Number(process.env.SCALE || 4)));
console.log(`wrote ${out} (${W}x${H}), clipped pixels: ${fb.clipped()}`);
