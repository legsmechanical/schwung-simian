# Simian

Simian is a module for [Schwung](https://github.com/charlesvestal/schwung) on Ableton Move.

**Status:** scaffold only — purpose, signal path and parameters are not yet defined.

## Layout

| path | holds |
|---|---|
| `src/module.json` | module manifest (id, params, UI hierarchy) |
| `src/ui.js` | module UI |
| `src/dsp/` | DSP source, cross-compiled to `dsp.so` |
| `src/host/` | host API headers, copied in from the host at build time |
| `scripts/` | `build.sh`, `install.sh`, `Dockerfile` |
| `docs/` | design notes |
| `release.json` | release manifest consumed by the host's module browser |

## TODO before this builds

- Decide `capabilities.component_type` (`midi_fx` / `audio_fx` / instrument) — currently `"TBD"`.
- Copy the matching host API header into `src/host/`.
- Port `scripts/build.sh` + `Dockerfile` from a sibling module.
