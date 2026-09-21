/* Inject the wrapper's ui_hierarchy and chain_params into src/module.json.
 *
 *   node tools/gen_module_json.mjs        (run by scripts/build.sh)
 *
 * WHY: the host fills a slot's param table from module.json ON DISK at load
 * (parse_chain_params), and the modulation / patch / smoothing paths strcmp
 * against that table and silently do nothing on a miss. The UI does not come
 * from here — a sound_generator's ui_hierarchy is served by the plugin — so a
 * stale or missing block here looks perfect on screen and is a set of dead
 * modulation destinations.
 *
 * Generated, not hand-copied: the wrapper's kUiHierarchy, PADS[],
 * VOICE_PARAMS[], PAD_PARAMS[] and GLOBAL_PARAMS[] are the single source of
 * truth, and a duplicate would drift
 * the moment a level is renamed or a parameter added. tests/simian_test.cpp
 * asserts that what lands here is byte-identical to what the compiled wrapper
 * serves, which is the guard against this file's own parsing going stale.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const repo = join(here, "..");
const src = readFileSync(join(repo, "src", "dsp", "simian_plugin.cpp"), "utf8");

/* ---- ui_hierarchy: a C string literal, with one #define spliced in ---- */
function literal(match) {
  let out = "";
  for (const lit of match.matchAll(/"((?:[^"\\]|\\.)*)"/g))
    out += lit[1].replace(/\\"/g, '"').replace(/\\\\/g, "\\").replace(/\\n/g, "\n");
  return out;
}

const childCommon = src.match(/#define CHILD_COMMON([\s\S]*?)\n\n/);
if (!childCommon) { console.error("could not find CHILD_COMMON in the wrapper"); process.exit(1); }
const childCommonText = literal(childCommon[1]);

const h = src.match(/static const char \*kUiHierarchy\s*=\s*([\s\S]*?);\s*\n/);
if (!h) { console.error("could not find kUiHierarchy in the wrapper"); process.exit(1); }
/* The literal's own text, with CHILD_COMMON expanded exactly as cpp would. */
const hierarchyText = h[1]
  .split("CHILD_COMMON")
  .map((chunk) => literal(chunk))
  .join(childCommonText);

let uih;
try { uih = JSON.parse(hierarchyText); }
catch (e) { console.error("wrapper ui_hierarchy is not valid JSON:", e.message); process.exit(1); }

/* ---- chain_params: rebuilt from the same two tables build_chain_params
 * walks, in the same order. ---- */
function parseTable(name) {
  const block = src.match(new RegExp(`static const \\w+ ${name}\\[\\] = \\{([\\s\\S]*?)\\n\\};`));
  if (!block) { console.error(`could not find ${name}[] in the wrapper`); process.exit(1); }
  const rows = [];
  for (const line of block[1].split("\n")) {
    const m = line.match(
      /\{\s*"([^"]+)"\s*,\s*"([^"]*)"\s*,\s*(?:"[^"]*"|NULL)\s*,\s*(-?[\d.]+)\s*,\s*(-?[\d.]+)\s*,\s*(-?[\d.]+)\s*,\s*(?:"([^"]*)"|NULL)/);
    if (m) rows.push({ key: m[1], name: m[2], min: +m[3], max: +m[4], def: +m[5], unit: m[6] });
  }
  if (!rows.length) { console.error(`${name}[] parsed as empty`); process.exit(1); }
  return rows;
}

/* PADS[] carries the ids, and an id is what every parameter key is prefixed
 * with. Sixteen pads over ten voices: the alias pads share their parent's
 * voice parameters and own only `tune`. */
const pads = [...src.matchAll(
  /\{"(pad\d+)",\s*"([^"]+)",\s*(V_\w+),\s*(\d+),\s*(-?[\d.]+)f,\s*(\d+)\}/g)]
  .map((m) => ({ id: m[1], label: m[2], voice: m[3], note: +m[4], tune: +m[5] }));
if (pads.length !== 16) { console.error(`expected 16 pads, parsed ${pads.length}`); process.exit(1); }

/* The voice LABELS, in declaration order, so a shared parameter is named
 * after the drum rather than after whichever pad happens to own it. */
const voiceBlock = src.match(/static const voice_def_t VOICES\[SIMIAN_VOICES\] = \{([\s\S]*?)\n\};/);
if (!voiceBlock) { console.error("could not find VOICES[] in the wrapper"); process.exit(1); }
const voiceLabels = [...voiceBlock[1].matchAll(/\{"([^"]+)",\s*"([^"]+)"\}/g)].map((m) => m[1]);
if (voiceLabels.length !== 10) {
  console.error(`expected 10 voices, parsed ${voiceLabels.length}`); process.exit(1);
}
/* V_KICK.. in the order VOICES[] declares them — the enum the pad table
 * points through. */
const VOICE_ORDER = ["V_KICK", "V_RIM", "V_SNARE", "V_CLAP", "V_LOWTOM",
                     "V_HHCLOSED", "V_MIDTOM", "V_HHOPEN", "V_HITOM", "V_CYMBAL"];

/* Mirrors VOICE_VIZ in the wrapper. The Voice knob is drawn by src/canvas.js
 * (the drum's name, plus a link mark when two pads share the voice), and
 * declaring a "custom:" kind is also what makes the host load that file.
 * No extra_keys: an extra key is an extra stop in the value rotation, and the
 * mark visibly trailed the name because of it. */
const VOICE_VIZ = { kind: "custom:voicelink" };

const entry = (key, name, p, def = p.def) => {
  const o = { key, name, type: "int", min: Math.trunc(p.min), max: Math.trunc(p.max),
              default: Math.trunc(def) };
  if (p.unit) o.unit = p.unit;
  if (key === "ui_current_voice") o.viz = VOICE_VIZ;
  return o;
};

/* Exactly build_chain_params' order: globals, the focus knob, every pad's
 * tune, then each voice's parameters once under the pad that owns it. */
const chainParams = [
  ...parseTable("GLOBAL_PARAMS").map((p) => entry(p.key, p.name, p)),
  ...parseTable("UI_PARAMS").map((p) => entry(p.key, p.name, p)),
];
const pparams = parseTable("PAD_PARAMS");
for (const d of pads)
  for (const p of pparams) chainParams.push(entry(`${d.id}_${p.key}`, `${d.label} ${p.name}`, p, d.tune));

const vparams = parseTable("VOICE_PARAMS");
for (let v = 0; v < VOICE_ORDER.length; v++) {
  const owner = pads.find((d) => d.voice === VOICE_ORDER[v]);
  if (!owner) { console.error(`no pad owns ${VOICE_ORDER[v]}`); process.exit(1); }
  for (const p of vparams)
    chainParams.push(entry(`${owner.id}_${p.key}`, `${voiceLabels[v]} ${p.name}`, p));
}

/* The host refuses a module.json over 64 KB and stops building the table at
 * MAX_CHAIN_PARAMS (256). Both are silent in production; fail here instead. */
if (chainParams.length > 256) {
  console.error(`chain_params has ${chainParams.length} entries, over the host's 256`);
  process.exit(1);
}

const jsonPath = join(repo, "src", "module.json");
const mod = JSON.parse(readFileSync(jsonPath, "utf8"));
mod.capabilities.chain_params = chainParams;
mod.capabilities.ui_hierarchy = uih;
const text = JSON.stringify(mod, null, 2) + "\n";
if (Buffer.byteLength(text) > 65536) {
  console.error(`module.json is ${Buffer.byteLength(text)} bytes, over the host's 64 KB limit`);
  process.exit(1);
}
writeFileSync(jsonPath, text);

const levels = Object.keys(uih.levels || {});
console.log(`module.json: ${chainParams.length} chain_params, ` +
            `${levels.length} levels (${levels.join(", ")}), ` +
            `${Buffer.byteLength(text)} bytes`);
