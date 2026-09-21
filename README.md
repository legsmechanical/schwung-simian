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

Sixteen pads, notes 36–51, laid out the way Move's own drum racks are. Ten
voices, so six pads are **aliases**: a second pad on a voice another pad
already owns, at its own pitch offset.

```
 48 HI TOM *   49 CYMBAL     50 CYMBAL *   51 MID TOM *
 44 HI TOM     45 LOW TOM *  46 HH OPEN    47 MID TOM
 40 CLAP       41 LOW TOM    42 HH CL      43 HH CL *
 36 KICK       37 RIM        38 SNARE      39 SNARE *
```

`*` marks an alias. Each one sits **next to its parent**, and each carries its
own **Tune** — that offset is the only thing an alias owns, and it is what
stops a second pad being a duplicate. Everything else is the voice's and is
shared: turning Snare 2's Decay turns the Snare's, because there is one snare.

Snare and all three toms get the one alias upstream gives them. The two extra
are on HH Closed and Cymbal, which is what upstream's two silent notes already
meant — 44 is a pedal hat, 51 a ride. Here every pad sounds, as it does in
Move's kits.

**Chokes follow Move, not GM.** The three hats (42 · 43 · 46) silence each
other; so do the cymbals. A pad never chokes its own voice — a second hit on
one DSP already restarts it.

⚠ **These are not General MIDI notes.** Move's layout is not GM's — Move puts
cymbals at 48–51 where GM puts toms, and a clap at 39 where GM puts one but
upstream does not. Seating the aliases next to their parents moves things
further still. An imported GM drum loop will play the wrong drums.

### Performance controls

- **Mod wheel** sweeps every voice's cutoff three octaves down — upstream's DJ
  filter.
- **Pitch wheel** scales every voice's decay, 20% to 180%. It is a staccato
  control, not a bend, and is deliberately not saved.
- **Vel Sens** compresses velocity toward a fixed 80/127; at 0 the kit plays
  flat.

## Routing

All ten voices are published to the host (`split_voices`), so each can go to
its own bus, and each carries **Send A / Send B** into the host's return
buses. The ids are the pads that OWN each voice — an alias has no audio path
of its own, so both pads of a pair ride one bus.

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
- **Pads are seated, not inherited.** Upstream routes General MIDI, which on a
  16-pad grid means four duplicate pads two slots from their parents and two
  dead ones. Here sixteen pads are declared explicitly, every alias touches
  its parent, and each owns a Tune. The cost is GM compatibility — see the
  warning under *The pads*.

## Licence

**GPL-3.0-or-later.** The DSP, the factory kits and `original source/` are
Copyright (c) 2024 Punk Labs LLC. The cymbal sample is by Kevin Hall, CC0 (see
`src/dsp/faust/samples/LICENSE.txt`). See `LICENSE` and `NOTICES.txt`.
