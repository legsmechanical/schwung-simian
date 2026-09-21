# Notices

## What is in the built module

`dist/simian/dsp.so` is compiled from exactly three things:

| | |
|---|---|
| **OneTrick SIMIAN 2 DSP** — `src/dsp/faust/*.dsp`, `*.lib` | Copyright (c) 2024 **Punk Labs LLC**, GPL-3.0-or-later. Unmodified. |
| **Faust standard library** code, inlined into `src/dsp/generated/*.hpp` by the Faust compiler | GRAME. Emitted into the generated C++; the result is distributed under this project's GPL-3.0-or-later. |
| **The wrapper, the widget and the tools** — `src/dsp/simian_plugin.cpp`, `src/canvas.js`, `src/ui.js`, `tools/`, `tests/` | Copyright (c) 2026 **Josh Gaines / legsmechanical**, GPL-3.0-or-later. |

The cymbal wavetable in `src/dsp/faust/samples/` is a sample by **Kevin Hall**,
**CC0 1.0** — see that directory's `LICENSE.txt`.

Nothing else is linked in. `ui.js` and `canvas.js` are plain scripts the host
loads; the module has no other dependencies.

## What is NOT in it

`original source/OneTrickSIMIAN2-Source-v2.0.2/NOTICES.txt` is upstream's own
notices file and runs to 400 KB. **It describes upstream's Rust build** —
nih-plug, egui, baseview and several hundred crates — and **none of that is
built, linked or shipped here**. This port replaces that entire host layer with
`simian_plugin.cpp`. The file is kept where it arrived, as provenance for the
vendored source tree, and is not a description of this module's binary.

A root-level copy of it used to sit here and implied otherwise.

## Licence

GPL-3.0-or-later throughout; see [`LICENSE`](LICENSE). The DSP is Punk Labs'
and this port does not change its terms — *free as in rights, not as in beer*.
If you play this, [buy a copy](https://punklabs.com/ot-simian).
