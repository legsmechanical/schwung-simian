// pages_check.mjs — run upstream Schwung's OWN contract validator and voice
// resolver over the hierarchy SIMIAN actually serves, off-device.
//
// The host plans every page from get_param("ui_hierarchy"), not from
// module.json, so the input here is the served text (tests/simian_test.cpp
// writes it to dist/tests/served_hierarchy.json). The library is imported from
// a Schwung checkout — the upstream tree, not a fork — because the rules that
// matter (viz groups on one row, child levels resolving, a note map on exactly
// one level) are the host's, and a re-implementation here would only be a
// second opinion.
//
//   ./scripts/test.sh && node tools/pages_check.mjs
//   SCHWUNG_SRC=/path/to/schwung node tools/pages_check.mjs
import fs from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

const SRC = process.env.SCHWUNG_SRC || path.resolve("..", "schwung-current");
const lib = (f) => pathToFileURL(path.join(SRC, "src", "shared", "param_pages", f)).href;
if (!fs.existsSync(path.join(SRC, "src", "shared", "param_pages", "voices.mjs"))) {
    console.error(`pages_check: no Schwung checkout at ${SRC} (set SCHWUNG_SRC) — skipped`);
    process.exit(0);
}
const { validateContract } = await import(lib("validate_contract.mjs"));
const { voicesOf, padLayoutOf } = await import(lib("voices.mjs"));
const { planPages, PAGE_KNOBS } = await import(lib("page_plan.mjs"));

const served = "dist/tests/served_hierarchy.json";
if (!fs.existsSync(served)) {
    console.error(`pages_check: ${served} missing — run ./scripts/test.sh first`);
    process.exit(1);
}
let hierarchy;
try {
    hierarchy = JSON.parse(fs.readFileSync(served, "utf8"));
} catch (e) {
    console.error(`pages_check: the SERVED hierarchy is not valid JSON (${e.message}) — the host would plan no pages at all`);
    process.exit(1);
}
const mod = JSON.parse(fs.readFileSync("src/module.json", "utf8"));
const chainParams = mod.capabilities.chain_params;

let fail = 0;
const bad = (m) => { fail++; console.error("  FAIL " + m); };

// ---- what the host would resolve
if (padLayoutOf(hierarchy) !== "drums") bad(`pad_layout resolves to ${padLayoutOf(hierarchy)}, want drums`);

/* 🔴 The count is the check that catches a note map declared on a second
 * level: voicesOf emits a voice per child of EVERY level that declares one, so
 * three copies of a ten-voice rack render perfectly and publish thirty. */
const WANT = [
    ["Kick", 36], ["Rimshot", 37], ["Snare", 38], ["Clap", 39], ["Low Tom", 41],
    ["HH Closed", 42], ["Mid Tom", 45], ["HH Open", 46], ["High Tom", 48], ["Cymbal", 49],
];
const voices = voicesOf(hierarchy);
if (voices.length !== WANT.length) bad(`${voices.length} voices, want ${WANT.length}`);
voices.forEach((v, i) => {
    const w = WANT[i];
    if (!w) return;
    if (v.note !== w[1]) bad(`voice ${i} plays note ${v.note}, want ${w[1]}`);
    if (v.name !== w[0]) bad(`voice ${i} is named "${v.name}", want "${w[0]}"`);
});
console.log(`  voices: ${voices.length}, notes ${voices.map((v) => v.note).join(",")}`);
console.log(`  names:  ${voices.map((v) => v.name).join(", ")}`);

// ---- contract validation, the host's rules
const { findings } = validateContract({ id: "simian", hierarchy, chainParams, capabilities: mod.capabilities });
for (const f of findings) {
    /* A deliberate EMPTY knob slot ("" in a knobs array) names nothing, and
     * upstream's validator cannot know it is intentional. Suppressed narrowly:
     * only when the message names no key at all. */
    if (f.rule === "undeclared-knob-params" && /:\s*$/.test(f.message)) continue;
    const line = `[${f.level}] ${f.rule}: ${f.message}`;
    if (f.level === "error") bad(line); else console.log("  " + line);
}

// ---- the pages, as planned
const { pages } = planPages({ hierarchy, chainParams });
const knobPages = pages.filter((p) => p.kind === PAGE_KNOBS);
console.log(`  ${pages.length} pages planned, ${knobPages.length} knob pages`);
/* EVERY page, not just the knob ones: the host GENERATES pages a module never
 * declares, most importantly the "Selected <child>" picker it emits unless
 * child_index_param is reachable as a knob somewhere. */
for (const p of pages) {
    const what = p.kind === PAGE_KNOBS
        ? (p.keys || []).map((k) => k || "-").join(" ")
        : `<${p.kind}>${p.name ? ` "${p.name}"` : ""}`;
    console.log(`    ${String(p.level ?? "-").padEnd(12)} ${what}`);
}

// ---- a fixture for upstream's preview tools
fs.mkdirSync("dist/tests", { recursive: true });
fs.writeFileSync("dist/tests/simian-fixture.json", JSON.stringify({
    _source: "schwung-simian tools/pages_check.mjs — served hierarchy + module.json chain_params",
    generated_at: new Date().toISOString(),
    module_count: 1,
    not_captured: [],
    modules: [{
        id: "simian", category: "sound_generator", component_key: "synth", status: "ok",
        name: mod.name, version: mod.version,
        ui_hierarchy: hierarchy, chain_params: chainParams, presets: null,
    }],
}, null, 1));

console.log(fail ? `pages_check: FAILED (${fail})` : "pages_check: OK");
process.exit(fail ? 1 : 0);
