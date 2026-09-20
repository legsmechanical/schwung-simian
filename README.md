# SIMIAN

**A ten-voice drum synthesiser for Schwung on the Ableton Move. No samples in it.**

Ported from [**OneTrick SIMIAN 2**](https://punklabs.com/ot-simian) by **Punk Labs
LLC** — the drum engine is entirely theirs. *It is free as in rights, not as in
beer: if you play this, [buy a copy](https://punklabs.com/ot-simian).*

Ten voices, and they are ten copies of one synth. Each is a triangle oscillator
that morphs into a sampled cymbal, a resonant-lowpass noise source, and a click
transient — crossfaded, and swept by one decay envelope that bends pitch and
filter as it falls. Nothing makes a voice a kick or a hat except where you put
its knobs, which is why a kit here is 190 numbers and no audio files.

---

## Manual

### The pads

Ten voices on the drum grid, in note order. The octave shift moves the whole
row.

| Pad | Voice | | Pad | Voice |
|---|---|---|---|---|
| **36** | Kick     | | **42** | HH Closed |
| **37** | Rimshot  | | **45** | Mid Tom   |
| **38** | Snare    | | **46** | HH Open   |
| **39** | Clap     | | **48** | High Tom  |
| **41** | Low Tom  | | **49** | Cymbal    |

Upstream's alias notes still work, and a sequencer that writes General MIDI
lands on the right drum: 35 is the kick, 40 the snare, 43 the low tom, 47 the
mid tom, 50 the high tom. An alias plays the voice **transposed** by the
distance from its home note, which is upstream's chromatic behaviour.

**Two notes play nothing and silence something:** 44 (pedal hihat) chokes both
hats, 51 (ride) chokes the cymbal. The closed hat chokes the open one, as on
any kit.

### Knobs and pages

| Page | |
|---|---|
| **Kit** | the 35 factory kits |
| **Tone** | Voice · Pitch · Wave · Bend · Bend Dyn · Decay · Punch · Tone/Nse |
| **Noise** | Cutoff · Res · LP Bend · LP Dyn · Click |
| **Mix** | Volume · Pan · Vel Vol · Sat · Reverb · Send A · Send B |
| **Master** | Gain · Drive · Sat · Rev Size · Rev Gate · Vel Sens · Transpose |

Tone, Noise and Mix are three views of **one** voice — the **Voice** knob on
Tone moves all three, and the focus follows a pad you hit.

**Wave** is a morph, not a switch: at 0 it is a filtered triangle, at 100 a
sampled cymbal played back at pitch, and everything between is a crossfade.
**Bend** and **LP Bend** are how far the envelope drags pitch and cutoff, in
semitones; the **Dyn** beside each is how much of that bend velocity controls.
**Tone/Nse** crossfades the oscillator against the noise source — a kick is
near 0, a snare near the middle, a hat near 100.

### Performance controls

- **Mod wheel** sweeps every voice's cutoff three octaves down — upstream's DJ
  filter.
- **Pitch wheel** scales every voice's decay, 20% to 180%. It is a staccato
  control, not a bend, and is deliberately not saved.
- **Vel Sens** compresses velocity toward a fixed 80/127; at 0 the kit plays
  flat.

## Routing

Every voice is published to the host (`split_voices`), so each can go to its
own bus, and each carries **Send A / Send B** into the host's return buses.

⚠ **A voice routed to a bus leaves pre-master.** It keeps its own Volume, Pan
and Saturation, but not the global Drive, Saturation, limiter or Gain — those
act on a sum that no longer exists once a voice is pulled out of it. The
internal reverb return and every unrouted voice still pass through the master
stage. Expect a level change when you route a voice, and put shared glue in a
bus rather than leaning on Master.

The reverb send is **global whatever the routing** — it is the send, not the
voice.

## Build

```sh
./scripts/gen_dsp.sh     # only after editing a .dsp/.lib — needs `faust`
./scripts/test.sh        # native tests, no device
node tools/pages_check.mjs   # the host's own validator over the served hierarchy
./scripts/build.sh       # cross-compile for Move (Docker)
./scripts/install.sh     # scp to move.local
./scripts/render.sh out.wav all   # hear all 35 kits
```

**The synth is Faust, not C++.** `src/dsp/faust/*.dsp` are Punk Labs' files;
`scripts/gen_dsp.sh` compiles them with `-lang cpp` into `src/dsp/generated/`,
which is machine output — editing the generated header is always wrong. Faust
is a workstation tool only; the build container has just a cross compiler.

⚠⚠ **Two files are GENERATED and rebuilt by `build.sh`:** `src/module.json`
(its `chain_params` and inline `ui_hierarchy`) and `src/dsp/factory_bank.h`,
which is indexed by `VOICE_PARAMS[]` **position** — inserting a parameter
without regenerating loads every later value in all 35 kits one slot out,
silently. Both generators parse the wrapper's tables with a regex that only
reads **numeric literals**: a row whose min/max/default is a named constant is
skipped, and the only symptom is a short bank.

## What this port changed

Everything audible is upstream's, byte for byte — the `.dsp` files are
unmodified. The changes are all in the wrapper that replaces upstream's
nih-plug host:

- **Every hit forces its own rising edge.** Faust edge-detects `Trigger`, and a
  pad press and release can both land between two render calls, which upstream
  would hear as one value and no hit. The zone drops to 0 and is raised after
  exactly one frame, so a repeated note always sounds.
- **Sensitivity and Transpose are applied here.** Both are declared upstream
  and read by nothing, so Faust prunes them — `Global Transpose` does nothing
  at all in OneTrick SIMIAN 2.0.2. They are applied at note-on instead, which
  is where they were meant to act.
- **Volume's floor is −60 dB**, not upstream's −100: below that the knob is
  scrolling through inaudible.
- **A silent voice stops being computed** after 100 ms, as upstream does.

## Licence

**GPL-3.0-or-later.** The DSP, the factory kits and `original source/` are
Copyright (c) 2024 Punk Labs LLC. The cymbal sample is by Kevin Hall, CC0 (see
`src/dsp/faust/samples/LICENSE.txt`). See `LICENSE` and `NOTICES.txt`.
