/* Compile Punk Labs' factory .preset files into src/dsp/factory_bank.h.
 *
 *   node tools/gen_factory_bank.mjs        (run by scripts/build.sh)
 *
 * The 35 factory kits ship with OneTrick SIMIAN 2 as flat JSON keyed by the
 * DISPLAY names its egui UI used ("Hihat Closed Lowpass Freq"). The module
 * stores the same numbers under Schwung keys ("voice6_cutoff"), so this maps
 * one to the other ONCE, here, and emits a table in VOICE_PARAMS[] /
 * GLOBAL_PARAMS[] order.
 *
 * ⚠ Generated rather than hand-copied because the order is the wrapper's, and
 * a hand-written table would go on compiling perfectly after a param is
 * inserted while loading every value one slot out.
 */
import { readFileSync, writeFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const repo = join(here, "..");

/* The module's voice order is NOTE order; upstream's preset keys are named
 * after its own ten voices. Index i here is voice i in the wrapper. */
const UPSTREAM_VOICE = [
  "Kick",         // voice1, note 36
  "Rimshot",      // voice2, note 37
  "Snare",        // voice3, note 38
  "Clap",         // voice4, note 39
  "Low Tom",      // voice5, note 41
  "Hihat Closed", // voice6, note 42
  "Mid Tom",      // voice7, note 45
  "Hihat Open",   // voice8, note 46
  "High Tom",     // voice9, note 48
  "Cymbal",       // voice10, note 49
];

/* Schwung voice key -> the suffix Punk Labs' preset files use. A key absent
 * from this map (send_a / send_b, which are the host's buses and have no
 * upstream counterpart) keeps its default. */
const VOICE_NAME = {
  pitch:    "Osc Pitch",
  wave:     "Osc Waveform",
  bend:     "Osc Env Bend",
  bend_dyn: "Osc Env Bend Dynamics",
  cutoff:   "Lowpass Freq",
  res:      "Lowpass Q",
  lp_bend:  "Lowpass Env Bend",
  lp_dyn:   "Lowpass Env Bend Dynamics",
  decay:    "Env Decay",
  punch:    "Env Punch",
  noise:    "Fade Tone Noise",
  click:    "Fade Click",
  volume:   "Gain",
  pan:      "Pan",
  vel_vol:  "Gain Dynamics",
  sat:      "Saturation",
  reverb:   "Send Reverb",
};

/* Globals, with the aliases upstream renamed along the way: "Basic.preset"
 * predates the rename and still says "Reverb Decay". Listed oldest-last, and
 * the first one present wins. */
const GLOBAL_NAME = {
  gain:       ["Global Gain"],
  drive:      ["Global Drive"],
  master_sat: ["Global Saturation"],
  rev_size:   ["Global Reverb Size", "Global Reverb Decay"],
  rev_gate:   ["Global Reverb Gate"],
  sens:       ["Global Sensitivity"],
  transpose:  ["Global Transpose"],
};

/* Read the two tables out of the wrapper: key, min, max, default. One source
 * of truth for the order, the clamp and the fallback. */
const wrapper = readFileSync(join(repo, "src", "dsp", "simian_plugin.cpp"), "utf8");

function parseTable(name) {
  const block = wrapper.match(new RegExp(`static const \\w+ ${name}\\[\\] = \\{([\\s\\S]*?)\\n\\};`));
  if (!block) { console.error(`could not find ${name}[] in the wrapper`); process.exit(1); }
  const rows = [];
  for (const line of block[1].split("\n")) {
    /* key, name, zone-or-NULL, min, max, def — numeric literals only, which is
     * why the tables never use named constants for a range. */
    const m = line.match(
      /\{\s*"([^"]+)"\s*,\s*"[^"]*"\s*,\s*(?:"[^"]*"|NULL)\s*,\s*(-?[\d.]+)\s*,\s*(-?[\d.]+)\s*,\s*(-?[\d.]+)/);
    if (m) rows.push({ key: m[1], min: +m[2], max: +m[3], def: +m[4] });
  }
  if (!rows.length) { console.error(`${name}[] parsed as empty`); process.exit(1); }
  return rows;
}

const vparams = parseTable("VOICE_PARAMS");
const gparams = parseTable("GLOBAL_PARAMS");

/* Slot 0 is what a fresh instance loads: "Basic" is upstream's own init kit. */
const dir = join(repo, "src", "presets", "factory");
const files = readdirSync(dir).filter((f) => f.endsWith(".preset")).sort();
const first = "Basic.preset";
if (files.includes(first)) files.splice(0, 0, ...files.splice(files.indexOf(first), 1));

const clamp = (v, p) => Math.min(p.max, Math.max(p.min, v));
const missingKeys = new Set();

const rows = [];
for (const f of files) {
  const pr = JSON.parse(readFileSync(join(dir, f), "utf8"));
  const P = pr.params || {};
  const name = f.replace(/\.preset$/, "");

  const globals = gparams.map((p) => {
    const alts = GLOBAL_NAME[p.key] || [];
    const hit = alts.find((a) => a in P);
    if (!hit && alts.length) missingKeys.add(`${name}: ${alts[0]}`);
    return clamp(hit ? P[hit] : p.def, p);
  });

  const voices = UPSTREAM_VOICE.map((vn) =>
    vparams.map((p) => {
      const suffix = VOICE_NAME[p.key];
      if (!suffix) return p.def;               /* send_a / send_b */
      const up = `${vn} ${suffix}`;
      if (!(up in P)) { missingKeys.add(`${name}: ${up}`); return p.def; }
      return clamp(P[up], p);
    }));

  rows.push({ name, desc: (pr.info && pr.info.Description) || "", globals, voices });
}

if (missingKeys.size)
  console.warn(`  ${missingKeys.size} preset value(s) absent upstream, using defaults:\n   ` +
               [...missingKeys].slice(0, 8).join("\n   ") + (missingKeys.size > 8 ? "\n   ..." : ""));

const esc = (s) => s.replace(/\\/g, "\\\\").replace(/"/g, '\\"');
const num = (v) => (Number.isInteger(v) ? `${v}.0f` : `${(+v).toFixed(4)}f`);

let out = `/* GENERATED by tools/gen_factory_bank.mjs — do not edit.
 *
 * OneTrick SIMIAN 2's ${rows.length} factory kits (Punk Labs LLC, GPL-3.0-or-later),
 * converted from src/presets/factory/*.preset. Values are in DISPLAY units and
 * in VOICE_PARAMS[] / GLOBAL_PARAMS[] order; regenerate after any change to
 * either table or to the .preset files.
 */
#ifndef SIMIAN_FACTORY_BANK_H
#define SIMIAN_FACTORY_BANK_H

#define SIMIAN_FACTORY_VOICES  ${UPSTREAM_VOICE.length}
#define SIMIAN_FACTORY_VPARAMS ${vparams.length}
#define SIMIAN_FACTORY_GPARAMS ${gparams.length}

typedef struct {
    const char *name;
    const char *desc;
    float global[SIMIAN_FACTORY_GPARAMS];
    float voice[SIMIAN_FACTORY_VOICES][SIMIAN_FACTORY_VPARAMS];
} simian_preset_t;

static const simian_preset_t SIMIAN_FACTORY[] = {
`;
for (const r of rows) {
  out += `    {"${esc(r.name)}", "${esc(r.desc)}",\n`;
  out += `     {${r.globals.map(num).join(", ")}},\n     {\n`;
  out += r.voices.map((v, i) => `        /* ${UPSTREAM_VOICE[i]} */ {${v.map(num).join(", ")}}`).join(",\n");
  out += `\n     }},\n`;
}
out += `};

#define SIMIAN_FACTORY_COUNT ((int)(sizeof(SIMIAN_FACTORY) / sizeof(SIMIAN_FACTORY[0])))

#endif /* SIMIAN_FACTORY_BANK_H */
`;

writeFileSync(join(repo, "src", "dsp", "factory_bank.h"), out);
console.log(`factory_bank.h: ${rows.length} kits x ${UPSTREAM_VOICE.length} voices x ${vparams.length} params (+${gparams.length} globals)`);
