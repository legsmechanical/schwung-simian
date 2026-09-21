# SIMIAN

**A ten-voice drum synthesiser for Schwung on the Ableton Move. No samples in it.**

Ported from [**OneTrick SIMIAN 2**](https://punklabs.com/ot-simian) by **Punk Labs
LLC** — the drum engine is entirely theirs. *It is free as in rights, not as in
beer: if you play this, [buy a copy](https://punklabs.com/ot-simian).*

Every sound is built, not played back. One voice is an oscillator that morphs
from a triangle to a struck cymbal, a noise source behind a resonant filter,
and a click — crossfaded together and swept by a single envelope that drags
pitch and filter down as it falls. A kick and a hi-hat are the same synth with
different numbers, which is why a whole kit here is 190 values and no audio
files.

That recipe — a tuned oscillator yanked downward by its own envelope, mixed
against filtered noise — is the sound of early-80s electronic drums, the
hexagonal-pad era that **Simmons** made the sound of a decade. It is why the
toms are the fun part: wind **Bend** up on a low **Pitch** and you are
somewhere in 1983 within one knob. Nothing here is modelled on a specific
machine, and Punk Labs make no such claim; it is that *architecture*, built
honestly, with modern range on every control. Pull Bend back to zero and the
same voice is a perfectly straight modern kick.

---

## Playing it

Sixteen pads, ten drums. Hit a pad and the pages follow it, so the knobs are
always editing the drum you just heard.

```
 48 TomH*   49 Crash   50 Crash*  51 TomM*
 44 TomH    45 TomL*   46 OpHat   47 TomM
 40 Clap    41 TomL    42 ClHat   43 ClHat*
 36 Kick    37 Rim     38 Snare   39 Snare*
```

**Six pads are a second pad on a drum another pad already owns** — marked `*`
above, and marked on screen with a small link under the drum's name. There are
ten drums and sixteen pads, so this is how the grid fills up.

**A linked pad shares everything except Tune.** Turn Snare's Decay and the pad
beside it changes too, because there is one snare. What the two pads differ by
is **Tune**, a semitone offset, and it is the second knob on the Tone page.
That is what makes 39 a second snare rather than a copy of 38.

**Chokes work like a real kit.** The three hats (42, 43, 46) cut each other
off, and so do the two cymbals. A pad never chokes itself — hitting the same
drum twice just retriggers it.

> **Sequencing from elsewhere:** these are *not* General MIDI notes. The layout
> follows Move's own drum racks, and the aliases were put next to their
> parents. An imported GM drum loop will play the wrong drums.

## The screen

Five pages. Jog through them; the first is the kit browser.

| Page | What's on it |
|---|---|
| **Kit** | the 35 factory kits |
| **Tone** | Voice · Tune · Pitch · Wave · Bend · Bend Dyn · Decay · Punch |
| **Noise** | Cutoff · Res · LP Bend · LP Dyn · Tone/Nse · Click |
| **Mix** | Volume · Pan · Vel Vol · Sat · Reverb · Send A · Send B |
| **Master** | Gain · Drive · Sat · Rev Size · Rev Gate · Vel Sens · Transpose |

Tone, Noise and Mix are three views of **one drum**. The **Voice** knob picks
which, and it moves all three pages together. It shows the drum's name, with
the link mark under it when that drum has a second pad.

### Tone — the oscillator

| | |
|---|---|
| **Tune** | This *pad's* pitch offset, ±24 semitones. The only control a linked pad does not share. |
| **Pitch** | Oscillator frequency, 30 Hz – 4.4 kHz. Low for a kick, high for a rim. |
| **Wave** | Morphs the oscillator from a filtered **triangle** (0) to a **sampled cymbal** played at pitch (100). Not a switch — everything between is a real crossfade. |
| **Bend** | How far the envelope drags the pitch, ±36 semitones. A kick is a big downward bend; this is where most of the punch in a drum synth lives. |
| **Bend Dyn** | How much of that bend velocity controls. At 100 a soft hit barely bends and a hard one bends fully. |
| **Decay** | 10 ms – 2 s. Velocity always shortens it, and that is built into the engine rather than a control: the lightest hit decays in **30% of the time you set**, a full-velocity one in all of it. |
| **Punch** | Bends the *shape* of the envelope rather than its length: from an even fall to one that holds and then drops. |

### Noise — the other half of every drum

The noise source has its own resonant lowpass with its own envelope sweep.
Snares, hats and claps are mostly this.

| | |
|---|---|
| **Cutoff** | 440 Hz – 15 kHz. |
| **Res** | Emphasis at the cutoff. Gentle by design — it tops out around Q 2, so it colours the noise rather than ringing into a tone of its own. |
| **LP Bend** | How far the envelope sweeps that cutoff, ±36 semitones. |
| **LP Dyn** | How much of the sweep velocity controls. |
| **Tone/Nse** | The crossfade between oscillator and noise. Near 0 is a kick or a tom, the middle is a snare, near 100 is a hat. **The most defining knob in the module.** |
| **Click** | Fades a short noise transient over the attack. It is the stick, and it is most of what makes a quiet drum audible in a busy mix. |

### Mix — per drum

| | |
|---|---|
| **Volume** | −60 to +6 dB. |
| **Pan** | |
| **Vel Vol** | How much velocity controls level. At 0 every hit is equally loud. |
| **Sat** | Saturation on this drum alone, before the master stage. |
| **Reverb** | Send into SIMIAN's own reverb — size and gate live on Master. |
| **Send A / Send B** | Sends into the host's return buses. |

### Master — the whole kit

| | |
|---|---|
| **Gain** | Output level. |
| **Drive** | Into the limiter. Defaults to −6 dB; push it for glue and weight. |
| **Sat** | Saturation across the mix. |
| **Rev Size** | The internal reverb's decay. |
| **Rev Gate** | A gate across the reverb return, as a threshold. At the bottom (−100 dB) the reverb is never gated; raise it and the tail is cut off the moment the reverb falls below it — the gated-reverb snare. |
| **Vel Sens** | How much velocity does anything at all. At 0 the kit plays flat. |
| **Transpose** | ±12 semitones across every drum. |

## Performance

- **Mod wheel** sweeps every drum's filter cutoff down by up to 32 semitones —
  a DJ filter across the whole kit.
- **Pitch wheel** scales every decay from 20% to 180%. A staccato control, not
  a bend, and deliberately not saved with the kit.

## Kits

35 factory kits, all from OneTrick SIMIAN 2, on the **Kit** page. They are only
parameter sets, so anything you hear you can reach and change. Kit 0, *Basic*,
is the init kit.

## Routing

Each of the ten drums can go to its own bus in the host, and each carries Send
A / Send B into the return buses. Both pads of a linked pair follow their drum
onto one bus — one voice, nothing to separate.

> ⚠ **A drum routed to a bus leaves before the master stage.** It keeps its own
> Volume, Pan and Saturation but not the global Drive, Saturation, limiter or
> Gain, because those act on a mix that no longer contains it. **Expect it to
> get louder** — about 4 dB at the default Drive. The reverb return and every
> unrouted drum still pass through Master. Shared glue over routed drums
> belongs in the bus, not on Master.

The Reverb send stays global whatever the routing: it is the send, not the drum.

## Differences from OneTrick SIMIAN 2

The engine is upstream's, unmodified. These are what a OneTrick user would
notice:

- **The pad layout is Move's, not General MIDI's** — see the note above.
- **Global Transpose works.** Upstream declares it and reads it nowhere, so in
  2.0.2 it does nothing at all. Same for Sensitivity. Both act here.
- **Volume bottoms out at −60 dB** instead of −100, where the last 40 dB of
  knob travel are all equally inaudible.
- **Every pad sounds.** Upstream's pedal-hat and ride notes only choked and
  made no sound of their own; here they are a second hat and a second cymbal,
  and they still choke.
- **A repeated note always sounds**, including two hits inside one audio block,
  which upstream can drop.

## Licence

**GPL-3.0-or-later.** The DSP, the factory kits and `original source/` are
Copyright (c) 2024 Punk Labs LLC. The cymbal sample is by Kevin Hall, CC0 (see
`src/dsp/faust/samples/LICENSE.txt`). The wrapper, widget and tooling are
Copyright (c) 2026 Josh Gaines / legsmechanical, same licence. Full breakdown
of what is and is not in the binary: [`NOTICES.md`](NOTICES.md).

Building, testing and the porting notes: [`docs/DEVELOPING.md`](docs/DEVELOPING.md).
