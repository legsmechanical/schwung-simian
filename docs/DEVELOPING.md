# Developing SIMIAN

Everything a user needs is in [`README.md`](../README.md). This is the build,
the generated files, and the things that bite.

## Build, test, look

```sh
./scripts/test.sh                  # native tests: 86 C checks + 12 widget checks
node tools/pages_check.mjs         # the HOST's own validator over the served hierarchy
node tools/icon_sheet.mjs          # every Voice cell, as the device draws it
./scripts/build.sh                 # cross-compile for Move (Docker)
./scripts/install.sh               # scp to move.local
./scripts/render.sh out.wav all    # render all 35 kits to a WAV
./scripts/gen_dsp.sh               # ONLY after editing a .dsp/.lib — needs `faust`
```

`pages_check` and `icon_sheet` import from a Schwung checkout; both default to
`../schwung-current` and take `SCHWUNG_SRC`.

## The synth is Faust, not C++

`src/dsp/faust/*.dsp` are Punk Labs' files, unmodified. `scripts/gen_dsp.sh`
compiles them with `-lang cpp` into `src/dsp/generated/`, which is machine
output — **editing a generated header is always wrong**; edit the `.dsp` and
regenerate. Faust is a workstation tool only: the build container has just a
cross compiler.

⚠ SIMIAN's `shared.lib` imports `onetrick.lib` from `onetrick/src/dsp/` in the
upstream tree, and it is a **different, older version** than the one in
`schwung-chonk`. Use SIMIAN's own.

## Generated files

Both are rebuilt by `scripts/build.sh`, outside the container (no node in the
build image).

| file | from | if it goes stale |
|---|---|---|
| `src/module.json` (`chain_params` + inline `ui_hierarchy`) | `tools/gen_module_json.mjs` | modulation targets are silently dead |
| `src/dsp/factory_bank.h` | `tools/gen_factory_bank.mjs` | every preset value past an inserted param loads one slot out |

⚠⚠ **Both generators parse the wrapper's tables with a regex that only reads
NUMERIC LITERALS.** A row whose min/max/default is a named constant is skipped
silently — that is why `UI_PARAMS` says `16` and not `SIMIAN_PADS`.

⚠ `chain_params` is the MODULATION table, not the UI. A sound generator's UI
comes from the plugin's `ui_hierarchy` and nowhere else, but `parse_chain_params()`
reads `module.json` off disk to build the slot's parameter-metadata table, and a
key missing from it cannot be modulated or patched. The miss is silent. Limits:
**256 entries** (214 used) and **64 KB** (48 KB used). Adding a per-voice
parameter costs ten entries.

## Pads and voices are different things

`PADS[16]` is the seating: note, name, the voice it drives, its default Tune,
its choke group. `VOICES[10]` is the DSP. Six pads are aliases.

- **Tune is per pad; the other 19 parameters are per voice** and genuinely
  shared.
- `chain_params` and the state blob key each shared parameter **once**, under
  the pad that owns the voice.
- `split_voices` publishes the ten **owning** pads, because an alias has no
  audio path of its own.
- Every alias is adjacent to its parent on the 4-wide grid (±1 in a row, or
  ±4). Upstream's General MIDI aliases are +2 apart, and ±2 is never adjacent
  in a 4-wide grid — which is the whole reason the notes were reseated.

## The custom Voice widget

`src/canvas.js` draws the Voice cell: the drum's name plus a link mark. The
host loads that file **because `chain_params` declares `viz.kind:
"custom:voicelink"`** — that declaration is the trigger, and nothing else is.

- **The box is 32 x 15** and the context clips to it silently.
- **Every param-page font is uppercase-only** and draws a missing glyph as
  *nothing*, so mixed case means shipping glyphs. The widget carries a 5x7
  font of exactly the 24 characters the ten names use.
- **No `extra_keys`.** An extra key is one more stop in the controller's value
  rotation, so anything read that way lands a rotation after the cell's own
  value — visibly, on the device. The pairing is compile-time constant, so the
  widget derives it from `NAMES[]`.
- An unregistered kind (typo, file missing, older host, or a widget disabled
  after one throw) does not claim the cell, so the plain number is drawn and
  the page still looks right. The only evidence is a line in the device's
  `debug.log`.

⭑ **Do not review 1-bit art from its source.** `tools/icon_sheet.mjs` renders
every cell through the host's own framebuffer and font. An earlier icon set
took three passes at one glyph, each obvious in the render and invisible in the
code, and the same tool caught art being drawn to a 32x22 box the host gives as
32x15.

## Tests

`tests/simian_test.cpp` includes the wrapper so it can reach the static tables.
`tests/widget.test.mjs` is the only place a JS table can be compared against a C
one: two pads on one voice must show one name, every character must have a
glyph, every name must fit 32 px, and the derived link mark must match `PADS[]`.

`tools/pages_check.mjs` runs **upstream's own** contract validator and voice
resolver over the hierarchy the DSP actually serves (written to
`dist/tests/served_hierarchy.json` by the C suite). It is what catches a note
map declared on a second level — which renders perfectly and publishes three
copies of the rack.

## Before a release

- `NOTICES.txt` is upstream's **Rust dependency** list and describes a build
  this port does not do.
- `LICENSE` carries only Punk Labs' copyright line.
- No `help.json` yet.
- Open design question: a drum routed to a bus leaves pre-master and jumps
  about 4 dB (README, "Routing"). The options are internal inserts, a bus per
  voice with the glue duplicated, or pulling the master stage out into its own
  `audio_fx` — the DR32 → Bus Driver move.
