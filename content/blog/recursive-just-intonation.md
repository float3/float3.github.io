---
title: Recursive Just Intonation
date: 2022-11-29
updated: 2026-05-04
tags:
  - music
  - programming
---

## Recursive Just Intonation

Equal temperament gives us one keyboard. Every C# is the same C#, every G is
the same G, and every semitone is the same distance from the last one. That is
extremely useful. It is also a compromise: the intervals are close enough to
simple ratios that they work, but most of them are not exact.

Just intonation goes the other way. It starts from simple frequency ratios:

- octave: `2/1`
- perfect fifth: `3/2`
- perfect fourth: `4/3`
- major third: `5/4`
- major chord: `4:5:6`, or `1/1`, `5/4`, `3/2`

Those ratios sound still and locked-in because their waveforms repeat against
each other quickly. A just major chord is not only "close" to consonant; it is
consonant by construction.

The catch is that just intonation normally needs a root. A `5/4` major third
above C is E. A `5/4` major third above E is G#/Ab. Those two facts cannot both
fit into one fixed 12-note keyboard unless we allow the same pitch name to mean
different frequencies in different harmonic contexts.

That is the idea of this experiment:

> Keep the roots on a C-based just-intonation keyboard, but give every chord
> root its own just-intonation keyboard.

I think of it as 12 pianos: one just piano rooted on C, one on C#/Db, one on D,
and so on. The root of each piano is taken from the original C just-intonation
scale. Once a chord chooses a root, all of its notes come from the piano rooted
on that note.

This is "recursive" in the simple algorithmic sense: use a just-ratio table to
choose the chord root, then use the same ratio table again inside that root.

## 12-TET

In 12-tone equal temperament, the ratio between adjacent semitones is:

```text
2^(1/12) = 1.059463...
```

The frequency of a note `n` semitones above some reference note is:

```text
frequency(n) = reference * 2^(n/12)
```

The nice property is composability:

```text
2^(1/12) * 2^(1/12) = 2^(2/12)
```

Going up two semitones one step at a time lands at the same frequency as
jumping up two semitones directly. This is why transposition is easy in equal
temperament. There is only one global grid.

## One C Just Scale

For the C-based just-intonation scale used in my tuning playground, the 12 pitch
classes are:

| pitch | ratio from C |
| --- | ---: |
| C | `1/1` |
| C#/Db | `17/16` |
| D | `9/8` |
| D#/Eb | `19/16` |
| E | `5/4` |
| F | `4/3` |
| F#/Gb | `45/32` |
| G | `3/2` |
| G#/Ab | `51/32` |
| A | `27/16` |
| A#/Bb | `57/32` |
| B | `15/8` |

This already makes a C major chord exact:

```text
C = 1/1
E = 5/4
G = 3/2
```

But an E major chord on the same fixed C just keyboard has a problem:

```text
E      = 5/4
G#/Ab  = 51/32
B      = 15/8
```

Relative to E, the G#/Ab is:

```text
(51/32) / (5/4) = 51/40 = 1.275
```

A just major third should be `5/4 = 1.25`. So the E major chord has a fifth that
works and a third that is too high by about `34.3` cents. That is not a tiny
rounding error. It is enough to make the chord feel tense.

## 12 Just Pianos

Recursive just intonation changes only one rule: after choosing a root, restart
the ratio table at that root.

For an E major chord:

```text
E      = C * 5/4
G#/Ab  = E * 5/4 = C * 25/16
B      = E * 3/2 = C * 15/8
```

Now the E major chord is internally just:

```text
E : G# : B = 1 : 5/4 : 3/2 = 4 : 5 : 6
```

The cost is that `G#/Ab` is no longer globally stable. Fixed-C just intonation
puts G#/Ab at `51/32` from C. Recursive just intonation puts the G#/Ab inside E
major at `25/16` from C.

```text
fixed C just G#/Ab    = 51/32 = 1.59375
recursive E-major G#  = 25/16 = 1.5625
```

Those are different notes hiding under the same name.

The general formula is:

```text
recursive_frequency(root, degree) =
    C_frequency * J[root] * J[degree]
```

where `J[x]` is the just-ratio table above, with octave correction whenever the
index crosses C again.

So the system is not a 12-note tuning system anymore. It is a chord-contextual
tuning system. Pitch classes split according to harmonic function.

## What It Sounds Like

I wrote a small Rust renderer for this post. It creates the same chord
progression three ways:

```text
C -> E -> G#/Ab -> C -> F -> A -> D -> G7 -> C -> E -> F -> C
```

The progression deliberately visits chords where fixed-C just intonation has
audible trouble. In the recursive version, each chord gets to retune itself
around its own root.

Generate the files with:

```sh
cargo run --manifest-path tools/site/Cargo.toml -- recursive-ji-music
```

The generator writes WAV examples and a CSV frequency report into
`content/blog/recursive-just-intonation/`.

<figure>
  <audio controls src="/blog/recursive-just-intonation/twelve-tet-progression.wav"></audio>
  <figcaption>12-TET: stable pitch classes, compromised intervals.</figcaption>
</figure>

<figure>
  <audio controls src="/blog/recursive-just-intonation/fixed-c-ji-progression.wav"></audio>
  <figcaption>Fixed C just intonation: C is beautiful, but remote chords start leaning hard.</figcaption>
</figure>

<figure>
  <audio controls src="/blog/recursive-just-intonation/recursive-ji-progression.wav"></audio>
  <figcaption>Recursive just intonation: each chord is tuned from its own C-derived root.</figcaption>
</figure>

There is also a more clinical example that alternates a fixed-C pitch with its
recursive chord-local version, then plays both at once so the beating is easier
to hear:

<figure>
  <audio controls src="/blog/recursive-just-intonation/recursive-ji-note-splits.wav"></audio>
  <figcaption>Pitch-name splits: same nominal note, different chord context.</figcaption>
</figure>

Some of the generated split points:

| chord context | note | fixed C JI | recursive JI | difference |
| --- | --- | ---: | ---: | ---: |
| E major | G#/Ab | `208.483 Hz` | `204.395 Hz` | `-34.283 cents` |
| F major | A | `220.747 Hz` | `218.021 Hz` | `-21.506 cents` |
| A major | C#/Db | `277.977 Hz` | `275.933 Hz` | `-12.777 cents` |
| G7 | F | `348.834 Hz` | `349.515 Hz` | `+3.378 cents` |

## Why This Is Nice

The upside is direct: every major chord can be made into a clean `4:5:6`
relationship, even if the chord root is not C. E major does not inherit C's
G#/Ab; it gets its own G#/Ab. F major does not inherit C's A; it gets its own A.

This matches how harmony often behaves perceptually. When a chord arrives, the
ear can accept the chord root as a local gravitational center. Recursive just
intonation uses that local center instead of forcing every chord to negotiate
with one global keyboard.

It is also a useful programming model. A chord can be rendered as:

```text
root_frequency = base_frequency * global_just_ratio[root]
note_frequency = root_frequency * local_just_ratio[chord_degree]
```

The same pure function works for any root.

## Why This Is Bad

The downside is also direct: the same note name can jump when the chord changes.

In 12-TET, G#/Ab is one frequency per octave. In fixed C just intonation, G#/Ab
is also one frequency per octave, just a different one. In recursive just
intonation, G#/Ab depends on why you are playing it.

That creates several problems:

- A melody can wobble if a held pitch is reinterpreted by the next chord.
- Enharmonic spelling starts to matter, but a 12-key interface usually hides it.
- Modulation becomes a negotiation between smooth voice-leading and pure local
  chords.
- Instruments with fixed frets, keys, or holes cannot do this without pitch
  bending or multiple samples per pitch class.

So this is not a replacement for equal temperament. Equal temperament is still
the heroic compromise that lets every key share one physical instrument.

Recursive just intonation is more like a harmonic microscope. It lets us hear
what equal temperament flattens: a chord tone is not only a key on a keyboard,
it is also a relationship to a root.

## Implementation Notes

The Rust generator lives in `tools/site/src/recursive_ji.rs`. It does not use
MIDI, because standard MIDI note numbers assume fixed pitch classes unless you
add extra tuning messages. Instead it writes 16-bit mono WAV files directly.

The renderer supports three tunings:

- `12-TET`: `C4 * 2^(n/12)`
- `Fixed C just intonation`: `C4 * J[pitch_class] * octave`
- `Recursive just intonation`: `C4 * J[root] * J[chord_degree] * octave`

The current implementation intentionally uses the same 12-entry table as the
blog post and tuning playground. That means the sound of a "minor third" depends
on the chosen chromatic just table. A future version could swap in a 5-limit or
7-limit table per chord quality, but this version keeps the experiment clean:
one C just scale, reused recursively as 12 local pianos.

## My Other Music Work

[Play around with different tuning systems and your computer keyboard](/tools/tuningplayground.md)

### Visualize and Listen to Polyrhythms in a Shader

<iframe width="640" height="360" frameborder="0" allowfullscreen="allowfullscreen" src="https://www.shadertoy.com/embed/7tV3WV?gui=true&t=10&paused=false&muted=false"></iframe>
