---
title: Why you should buy 12 Pianos | Recursive Just Intonation
tags:
  - music
  - programming
---

## What is this about?

Recursive just intonation is a novel toy-tuning system that I came up with during my high school physics classes, It's very easy to predict why it won't become popular. That said I find it interesting and both mathematically and musically beautiful, so I decided to write this blogpost (listening examples further below).

## Equal Temperament vs Just Intonation

Equal temperament gives us one frequency table. Every C# is the same C#, every G is the
same G, and every semitone is the same distance from the last one. That is very
convenient, at the cost of slightly altering almost every interval. The intervals
are close enough to simple ratios that they work, but all of them are not
exact.

Just intonation goes the other way. It treats notes as relationships to a root,
then builds those relationships from simple frequency ratios:

- octave: `2/1`
- perfect fifth: `3/2`
- major third: `5/4`
- major chord: `4:5:6`, or `1/1`, `5/4`, `3/2`

Those ratios sound still and locked-in because their waveforms repeat against
each other quickly. In a just major chord, the consonance comes directly from
the exact `4:5:6` relationship.

### 12-TET

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

### Just Intonation

The annoying part is that just intonation normally needs a root. A `5/4` major
third above C is E. A `5/4` major third above E is G#/Ab. Those two facts cannot
both fit into one fixed 12-note keyboard unless we allow the same pitch name to
mean different frequencies in different harmonic contexts.

For a C-based just-intonation scale, the 12 pitch classes could be (if chosen from the overtone series):

| pitch | ratio from C | Nth overtone |
| ----- | -----------: | ------------ |
| C     |        `1/1` | 0th          |
| C#/Db |      `17/16` | 16th         |
| D     |        `9/8` | 8th          |
| D#/Eb |      `19/16` | 18th         |
| E     |        `5/4` | 4th          |
| F     |      `21/16` | 21st         |
| F#/Gb |       `11/8` | 10th         |
| G     |        `3/2` | 3rd          |
| G#/Ab |       `13/8` | 12th         |
| A     |      `27/16` | 26th         |
| A#/Bb |        `7/4` | 6th          |
| B     |       `15/8` | 14th         |
| C     |        `2/1` | 1st          |

This already makes a C major chord exact:

```text
C = 1/1
E = 5/4
G = 3/2
```

But an E major chord on the same fixed C just keyboard has a problem:

```text
E      = 5/4
G#/Ab  = 13/8
B      = 15/8
```

Relative to E, the G#/Ab is:

```text
(13/8) / (5/4) = 13/10 = 1.3
```

A just major third should be `5/4 = 1.25`. So the E major chord has a fifth that
works and a third that is too high by about `34.3` cents. That is not a tiny
rounding error. It is enough to make the chord feel tense.

### What The Waves Look Like

Nice mathematical ratios are pleasant to our ears.
`x + 2*x`, where `x` is some frequency, sounds nice because it has a short
period:

<figure class="wave-figure">
  <iframe class="no-input" tabindex="-1" width="850" height="500" src="https://graphtoy.com/?f1(x,t)=sin(x+5*t)+sin(2*(x+5*t))&v1=true&f2(x,t)=&v2=false&f3(x,t)=&v3=false&f4(x,t)=&v4=false&f5(x,t)=&v5=false&f6(x,t)=&v6=false&grid=1&coords=0,-3,12">
  </iframe>
  <figcaption>A tone and its octave: two sine waves at a base frequency <code>f</code> and <code>2f</code>. The whole pattern repeats every <code>1/f</code> seconds, so the ear can lock onto it easily.</figcaption>
</figure>

While, for example, `x + 13/12*x` has a much longer period:

<figure class="wave-figure">
  <iframe class="no-input" tabindex="-1" width="850" height="500" src="https://graphtoy.com/?f1(x,t)=sin(x+5*t)+sin((13/12)*(x+5*t))&v1=true&f2(x,t)=&v2=false&f3(x,t)=&v3=false&f4(x,t)=&v4=false&f5(x,t)=&v5=false&f6(x,t)=&v6=false&grid=1&coords=0,-3,12">
  </iframe>
  <figcaption>A tone and a narrow nearby step: two sine waves at <code>f</code> and <code>13/12 f</code>. The combined wave needs <code>12/f</code> seconds to repeat, so it takes much longer to settle than the octave example.</figcaption>
</figure>

A just major chord is `4:5:6`, or `1:1.25:1.5`. In 12-TET, the same chord is
closer to `500:630:749`, or `1:1.260:1.498`.

<figure class="wave-figure">
<iframe class="no-input" tabindex="-1" width="850" height="500" src="https://graphtoy.com/?f1(x,t)=sin(x+5*t)+sin((5/4)*(x+5*t))+sin((3/2)*(x+5*t))&v1=true&f2(x,t)=sin(x+5*t)+sin((2^(4/12))*(x+5*t))+sin((2^(7/12))*(x+5*t))&v2=true&f3(x,t)=&v3=false&f4(x,t)=&v4=false&f5(x,t)=&v5=false&f6(x,t)=&v6=false&grid=1&coords=0,-3,12">
</iframe>
  <figcaption>Two major chords: the just version uses exact <code>4:5:6</code> ratios, while the 12-TET version uses the familiar piano/guitar approximation. They are close, but the 12-TET peaks do not quite return to the same places.</figcaption>
</figure>

## 12 Just Pianos | Recursive Just Intonation

Here is what I call recursive just intonation:

> Keep the roots on a C-based just-intonation keyboard, but give every chord
> root its own just-intonated keyboard.

I think of it as 12 pianos: one just piano rooted on C, one on C#/Db, one on D,
and so on. The root of each piano is taken from the original C just-intonation
scale. Once a chord chooses a root, all of its notes come from the piano rooted
on that note.

This is "recursive" in the simple algorithmic sense: use a just-ratio table to
choose the chord root, then use the same ratio table again inside that root.

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
fixed C just G#/Ab    = 13/8 = 1.625
recursive E-major G#  = 25/16 = 1.5625
```

Those are different frequencies sharing the same name.

The general formula is:

```text
recursive_frequency(root, degree) =
    C_frequency * J[root] * J[degree]
```

where `J[x]` is the just-ratio table above, with octave correction whenever the
index crosses C again.

The table below is the "12 pianos" idea written out as frequencies. To keep the
numbers concrete, I set the C root to `130.813 Hz`.

How to read it:

- The left column chooses the chord root, or "which piano" you are using.
- The top row chooses the interval above that root. These are ratios, not note
  names.
- The cell tells you the frequency to play for that local interval.
- The color and small label inside the cell show the resulting pitch name.
  Cells with the same pitch name share a color.
- The cents line shows how far that frequency is from 12-TET for the same
  pitch name.

For example, an E major chord uses the `E` row and the `1/1`, `5/4`, and `3/2`
columns. That gives `163.516 Hz`, `204.395 Hz`, and `245.274 Hz`. In another
octave, multiply or divide the whole row by `2`.

| local root | `C` | `C#/Db` | `D` | `D#/Eb` | `E` | `F` | `F#/Gb` | `G` | `G#/Ab` | `A` | `A#/Bb` | `B` |
| ---------- | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: |
| C | <span class="recursive-note-cell note-c" data-note="C"><code>130.813 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>139.534 Hz</code><small class="tet-cents">11.731 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>147.164 Hz</code><small class="tet-cents">3.910 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>156.975 Hz</code><small class="tet-cents">15.641 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>163.516 Hz</code><small class="tet-cents">-13.686 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>174.417 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>186.045 Hz</code><small class="tet-cents">9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>196.219 Hz</code><small class="tet-cents">1.955 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>209.300 Hz</code><small class="tet-cents">13.686 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>218.021 Hz</code><small class="tet-cents">-15.641 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>232.556 Hz</code><small class="tet-cents">-3.910 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>245.274 Hz</code><small class="tet-cents">-11.731 cents</small></span> |
| C#/Db | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>139.534 Hz</code><small class="tet-cents">11.731 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>148.836 Hz</code><small class="tet-cents">23.463 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>156.975 Hz</code><small class="tet-cents">15.641 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>167.440 Hz</code><small class="tet-cents">27.373 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>174.417 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>186.045 Hz</code><small class="tet-cents">9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>198.448 Hz</code><small class="tet-cents">21.508 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>209.300 Hz</code><small class="tet-cents">13.686 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>223.254 Hz</code><small class="tet-cents">25.418 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>232.556 Hz</code><small class="tet-cents">-3.910 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>248.060 Hz</code><small class="tet-cents">7.821 cents</small></span> |
| D | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>275.933 Hz</code><small class="tet-cents">-7.821 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>147.164 Hz</code><small class="tet-cents">3.910 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>156.975 Hz</code><small class="tet-cents">15.641 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>165.560 Hz</code><small class="tet-cents">7.820 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>176.597 Hz</code><small class="tet-cents">19.551 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>183.956 Hz</code><small class="tet-cents">-9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>196.219 Hz</code><small class="tet-cents">1.955 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>209.300 Hz</code><small class="tet-cents">13.686 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>220.747 Hz</code><small class="tet-cents">5.865 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>235.463 Hz</code><small class="tet-cents">17.596 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>245.274 Hz</code><small class="tet-cents">-11.731 cents</small></span> |
| D#/Eb | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>279.067 Hz</code><small class="tet-cents">11.731 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>294.329 Hz</code><small class="tet-cents">3.910 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>156.975 Hz</code><small class="tet-cents">15.641 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>167.440 Hz</code><small class="tet-cents">27.373 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>176.597 Hz</code><small class="tet-cents">19.551 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>188.370 Hz</code><small class="tet-cents">31.283 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>196.219 Hz</code><small class="tet-cents">1.955 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>209.300 Hz</code><small class="tet-cents">13.686 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>223.254 Hz</code><small class="tet-cents">25.418 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>235.463 Hz</code><small class="tet-cents">17.596 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>251.161 Hz</code><small class="tet-cents">29.328 cents</small></span> |
| E | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>272.527 Hz</code><small class="tet-cents">-29.328 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>290.695 Hz</code><small class="tet-cents">-17.596 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>306.593 Hz</code><small class="tet-cents">-25.418 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>163.516 Hz</code><small class="tet-cents">-13.686 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>174.417 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>183.956 Hz</code><small class="tet-cents">-9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>196.219 Hz</code><small class="tet-cents">1.955 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>204.395 Hz</code><small class="tet-cents">-27.373 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>218.021 Hz</code><small class="tet-cents">-15.641 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>232.556 Hz</code><small class="tet-cents">-3.910 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>245.274 Hz</code><small class="tet-cents">-11.731 cents</small></span> |
| F | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>279.067 Hz</code><small class="tet-cents">11.731 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>290.695 Hz</code><small class="tet-cents">-17.596 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>310.075 Hz</code><small class="tet-cents">-5.865 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>327.032 Hz</code><small class="tet-cents">-13.686 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>174.417 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>186.045 Hz</code><small class="tet-cents">9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>196.219 Hz</code><small class="tet-cents">1.955 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>209.300 Hz</code><small class="tet-cents">13.686 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>218.021 Hz</code><small class="tet-cents">-15.641 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>232.556 Hz</code><small class="tet-cents">-3.910 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>248.060 Hz</code><small class="tet-cents">7.821 cents</small></span> |
| F#/Gb | <span class="recursive-note-cell note-c" data-note="C"><code>264.597 Hz</code><small class="tet-cents">19.553 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>279.067 Hz</code><small class="tet-cents">11.731 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>297.672 Hz</code><small class="tet-cents">23.463 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>310.075 Hz</code><small class="tet-cents">-5.865 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>330.746 Hz</code><small class="tet-cents">5.866 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>348.834 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>186.045 Hz</code><small class="tet-cents">9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>198.448 Hz</code><small class="tet-cents">21.508 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>209.300 Hz</code><small class="tet-cents">13.686 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>223.254 Hz</code><small class="tet-cents">25.418 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>232.556 Hz</code><small class="tet-cents">-3.910 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>248.060 Hz</code><small class="tet-cents">7.821 cents</small></span> |
| G | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>279.067 Hz</code><small class="tet-cents">11.731 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>294.329 Hz</code><small class="tet-cents">3.910 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>313.951 Hz</code><small class="tet-cents">15.641 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>327.032 Hz</code><small class="tet-cents">-13.686 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>348.834 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>367.911 Hz</code><small class="tet-cents">-9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>196.219 Hz</code><small class="tet-cents">1.955 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>209.300 Hz</code><small class="tet-cents">13.686 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>220.747 Hz</code><small class="tet-cents">5.865 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>235.463 Hz</code><small class="tet-cents">17.596 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>245.274 Hz</code><small class="tet-cents">-11.731 cents</small></span> |
| G#/Ab | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>279.067 Hz</code><small class="tet-cents">11.731 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>297.672 Hz</code><small class="tet-cents">23.463 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>313.951 Hz</code><small class="tet-cents">15.641 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>334.881 Hz</code><small class="tet-cents">27.373 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>348.834 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>372.090 Hz</code><small class="tet-cents">9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>392.438 Hz</code><small class="tet-cents">1.955 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>209.300 Hz</code><small class="tet-cents">13.686 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>223.254 Hz</code><small class="tet-cents">25.418 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>235.463 Hz</code><small class="tet-cents">17.596 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>251.161 Hz</code><small class="tet-cents">29.328 cents</small></span> |
| A | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>272.527 Hz</code><small class="tet-cents">-29.328 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>290.695 Hz</code><small class="tet-cents">-17.596 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>310.075 Hz</code><small class="tet-cents">-5.865 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>327.032 Hz</code><small class="tet-cents">-13.686 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>348.834 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>363.369 Hz</code><small class="tet-cents">-31.283 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>387.593 Hz</code><small class="tet-cents">-19.551 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>408.790 Hz</code><small class="tet-cents">-27.373 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>218.021 Hz</code><small class="tet-cents">-15.641 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>232.556 Hz</code><small class="tet-cents">-3.910 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>245.274 Hz</code><small class="tet-cents">-11.731 cents</small></span> |
| A#/Bb | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>279.067 Hz</code><small class="tet-cents">11.731 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>290.695 Hz</code><small class="tet-cents">-17.596 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>310.075 Hz</code><small class="tet-cents">-5.865 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>330.746 Hz</code><small class="tet-cents">5.866 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>348.834 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>372.090 Hz</code><small class="tet-cents">9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>387.593 Hz</code><small class="tet-cents">-19.551 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>413.433 Hz</code><small class="tet-cents">-7.820 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>436.043 Hz</code><small class="tet-cents">-15.641 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>232.556 Hz</code><small class="tet-cents">-3.910 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>248.060 Hz</code><small class="tet-cents">7.821 cents</small></span> |
| B | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>275.933 Hz</code><small class="tet-cents">-7.821 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>294.329 Hz</code><small class="tet-cents">3.910 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>306.593 Hz</code><small class="tet-cents">-25.418 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>327.032 Hz</code><small class="tet-cents">-13.686 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>348.834 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>367.911 Hz</code><small class="tet-cents">-9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>392.438 Hz</code><small class="tet-cents">1.955 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>408.790 Hz</code><small class="tet-cents">-27.373 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>436.043 Hz</code><small class="tet-cents">-15.641 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>459.889 Hz</code><small class="tet-cents">-23.463 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>245.274 Hz</code><small class="tet-cents">-11.731 cents</small></span> |

We now have a chord-contextual tuning system. Pitch classes split according to harmonic function.

### What It Sounds Like

I picked a progression that visits chords where fixed-C just intonation has
audible trouble. In the recursive version, each chord retunes around its own
root.

<script type="module" src="/js/audiooscilloscope.js"></script>

<figure class="abc-figure">
  <div class="abc-notation" data-recursive-ji-abc="progression"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 382 237" class="engraved-notation" role="img" aria-label="The twelve-chord progression, written on a treble staff"><title>The twelve-chord progression, written on a treble staff</title><defs><path id="rji-prog-clef" d="M9.69 -37.41c0.09 -0.09 0.24 -0.06 0.36 0c0.12 0.09 0.57 0.6 0.96 1.11c1.77 2.34 3.21 5.85 3.57 8.73c0.21 1.56 0.03 3.27 -0.45 4.86c-0.69 2.31 -1.92 4.47 -4.23 7.44c-0.3 0.39 -0.57 0.72 -0.6 0.75c-0.03 0.06 0 0.15 0.18 0.78c0.54 1.68 1.38 4.44 1.68 5.49l0.09 0.42l0.39 0c1.47 0.09 2.76 0.51 3.96 1.29c1.83 1.23 3.06 3.21 3.39 5.52c0.09 0.45 0.12 1.29 0.06 1.74c-0.09 1.02 -0.33 1.83 -0.75 2.73c-0.84 1.71 -2.28 3.06 -4.02 3.72l-0.33 0.12l0.03 1.26c0 1.74 -0.06 3.63 -0.21 4.62c-0.45 3.06 -2.19 5.49 -4.47 6.21c-0.57 0.18 -0.9 0.21 -1.59 0.21c-0.69 0 -1.02 -0.03 -1.65 -0.21c-1.14 -0.27 -2.13 -0.84 -2.94 -1.65c-0.99 -0.99 -1.56 -2.16 -1.71 -3.54c-0.09 -0.81 0.06 -1.53 0.45 -2.13c0.63 -0.99 1.83 -1.56 3 -1.53c1.5 0.09 2.64 1.32 2.73 2.94c0.06 1.47 -0.93 2.7 -2.37 2.97c-0.45 0.06 -0.84 0.03 -1.29 -0.09l-0.21 -0.09l0.09 0.12c0.39 0.54 0.78 0.93 1.32 1.26c1.35 0.87 3.06 1.02 4.35 0.36c1.44 -0.72 2.52 -2.28 2.97 -4.35c0.15 -0.66 0.24 -1.5 0.3 -3.03c0.03 -0.84 0.03 -2.94 0 -3c-0.03 0 -0.18 0 -0.36 0.03c-0.66 0.12 -0.99 0.12 -1.83 0.12c-1.05 0 -1.71 -0.06 -2.61 -0.3c-4.02 -0.99 -7.11 -4.35 -7.8 -8.46c-0.12 -0.66 -0.12 -0.99 -0.12 -1.83c0 -0.84 0 -1.14 0.15 -1.92c0.36 -2.28 1.41 -4.62 3.3 -7.29l2.79 -3.6c0.54 -0.66 0.96 -1.2 0.96 -1.23c0 -0.03 -0.09 -0.33 -0.18 -0.69c-0.96 -3.21 -1.41 -5.28 -1.59 -7.68c-0.12 -1.38 -0.15 -3.09 -0.06 -3.96c0.33 -2.67 1.38 -5.07 3.12 -7.08c0.36 -0.42 0.99 -1.05 1.17 -1.14zm2.01 4.71c-0.15 -0.3 -0.3 -0.54 -0.3 -0.54c-0.03 0 -0.18 0.09 -0.3 0.21c-2.4 1.74 -3.87 4.2 -4.26 7.11c-0.06 0.54 -0.06 1.41 -0.03 1.89c0.09 1.29 0.48 3.12 1.08 5.22c0.15 0.42 0.24 0.78 0.24 0.81c0 0.03 0.84 -1.11 1.23 -1.68c1.89 -2.73 2.88 -5.07 3.15 -7.53c0.09 -0.57 0.12 -1.74 0.06 -2.37c-0.09 -1.23 -0.27 -1.92 -0.87 -3.12zm-2.94 20.7c-0.21 -0.72 -0.39 -1.32 -0.42 -1.32c0 0 -1.2 1.47 -1.86 2.37c-2.79 3.63 -4.02 6.3 -4.35 9.3c-0.03 0.21 -0.03 0.69 -0.03 1.08c0 0.69 0 0.75 0.06 1.11c0.12 0.54 0.27 0.99 0.51 1.47c0.69 1.38 1.83 2.55 3.42 3.42c0.96 0.54 2.07 0.9 3.21 1.08c0.78 0.12 2.04 0.12 2.94 -0.03c0.51 -0.06 0.45 -0.03 0.42 -0.3c-0.24 -3.33 -0.72 -6.33 -1.62 -10.08c-0.09 -0.39 -0.18 -0.75 -0.18 -0.78c-0.03 -0.03 -0.42 0 -0.81 0.09c-0.9 0.18 -1.65 0.57 -2.22 1.14c-0.72 0.72 -1.08 1.65 -1.05 2.64c0.06 0.96 0.48 1.83 1.23 2.58c0.36 0.36 0.72 0.63 1.17 0.9c0.33 0.18 0.36 0.21 0.42 0.33c0.18 0.42 -0.18 0.9 -0.6 0.87c-0.18 -0.03 -0.84 -0.36 -1.26 -0.63c-0.78 -0.51 -1.38 -1.11 -1.86 -1.83c-1.77 -2.7 -0.99 -6.42 1.71 -8.19c0.3 -0.21 0.81 -0.48 1.17 -0.63c0.3 -0.09 1.02 -0.3 1.14 -0.3c0.06 0 0.09 0 0.09 -0.03c0.03 -0.03 -0.51 -1.92 -1.23 -4.26zm3.78 7.41c-0.18 -0.03 -0.36 -0.06 -0.39 -0.06c-0.03 0 0 0.21 0.18 1.02c0.75 3.18 1.26 6.3 1.5 9.09c0.06 0.72 0 0.69 0.51 0.42c0.78 -0.36 1.44 -0.96 1.98 -1.77c1.08 -1.62 1.2 -3.69 0.3 -5.55c-0.81 -1.62 -2.31 -2.79 -4.08 -3.15z"/><path id="rji-prog-whole" d="M6.51 -4.05c0.51 -0.03 2.01 0 2.52 0.03c1.41 0.18 2.64 0.51 3.72 1.08c1.2 0.63 1.95 1.41 2.19 2.31c0.09 0.33 0.09 0.9 0 1.23c-0.24 0.9 -0.99 1.68 -2.19 2.31c-1.08 0.57 -2.28 0.9 -3.75 1.08c-0.66 0.06 -2.31 0.06 -2.97 0c-1.47 -0.18 -2.67 -0.51 -3.75 -1.08c-1.2 -0.63 -1.95 -1.41 -2.19 -2.31c-0.09 -0.33 -0.09 -0.9 0 -1.23c0.24 -0.9 0.99 -1.68 2.19 -2.31c1.2 -0.63 2.61 -0.99 4.23 -1.11zm0.57 0.66c-0.87 -0.15 -1.53 0 -2.04 0.51c-0.15 0.15 -0.24 0.27 -0.33 0.48c-0.24 0.51 -0.36 1.08 -0.33 1.77c0.03 0.69 0.18 1.26 0.42 1.77c0.6 1.17 1.74 1.98 3.18 2.22c1.11 0.21 1.95 -0.15 2.34 -0.99c0.24 -0.51 0.36 -1.08 0.33 -1.8c-0.06 -1.11 -0.45 -2.04 -1.17 -2.76c-0.63 -0.63 -1.47 -1.05 -2.4 -1.2z"/><path id="rji-prog-sharp" d="M5.73 -11.19c0.21 -0.12 0.54 -0.03 0.66 0.24c0.06 0.12 0.06 0.21 0.06 2.31c0 1.23 0 2.22 0.03 2.22c0 0 0.27 -0.12 0.6 -0.24c0.69 -0.27 0.78 -0.3 0.96 -0.15c0.21 0.15 0.21 0.18 0.21 1.38c0 1.02 0 1.11 -0.06 1.2c-0.03 0.06 -0.09 0.12 -0.12 0.15c-0.06 0.03 -0.42 0.21 -0.84 0.36l-0.75 0.33l-0.03 2.43c0 1.32 0 2.43 0.03 2.43c0 0 0.27 -0.12 0.6 -0.24c0.69 -0.27 0.78 -0.3 0.96 -0.15c0.21 0.15 0.21 0.18 0.21 1.38c0 1.02 0 1.11 -0.06 1.2c-0.03 0.06 -0.09 0.12 -0.12 0.15c-0.06 0.03 -0.42 0.21 -0.84 0.36l-0.75 0.33l-0.03 2.52c0 2.28 -0.03 2.55 -0.06 2.64c-0.21 0.36 -0.72 0.36 -0.93 0c-0.03 -0.09 -0.06 -0.33 -0.06 -2.43l0 -2.31l-1.29 0.51l-1.26 0.51l0 2.43c0 2.58 0 2.52 -0.15 2.67c-0.06 0.09 -0.27 0.18 -0.36 0.18c-0.12 0 -0.33 -0.09 -0.39 -0.18c-0.15 -0.15 -0.15 -0.09 -0.15 -2.43c0 -1.23 0 -2.22 -0.03 -2.22c0 0 -0.27 0.12 -0.6 0.24c-0.69 0.27 -0.78 0.3 -0.96 0.15c-0.21 -0.15 -0.21 -0.18 -0.21 -1.38c0 -1.02 0 -1.11 0.06 -1.2c0.03 -0.06 0.09 -0.12 0.12 -0.15c0.06 -0.03 0.42 -0.21 0.84 -0.36l0.78 -0.33l0 -2.43c0 -1.32 0 -2.43 -0.03 -2.43c0 0 -0.27 0.12 -0.6 0.24c-0.69 0.27 -0.78 0.3 -0.96 0.15c-0.21 -0.15 -0.21 -0.18 -0.21 -1.38c0 -1.02 0 -1.11 0.06 -1.2c0.03 -0.06 0.09 -0.12 0.12 -0.15c0.06 -0.03 0.42 -0.21 0.84 -0.36l0.78 -0.33l0 -2.52c0 -2.28 0.03 -2.55 0.06 -2.64c0.21 -0.36 0.72 -0.36 0.93 0c0.03 0.09 0.06 0.33 0.06 2.43l0.03 2.31l1.26 -0.51l1.26 -0.51l0 -2.43c0 -2.28 0 -2.43 0.06 -2.55c0.06 -0.12 0.12 -0.18 0.27 -0.24zm-0.33 10.65l0 -2.43l-1.29 0.51l-1.26 0.51l0 2.46l0 2.43l0.09 -0.03c0.06 -0.03 0.63 -0.27 1.29 -0.51l1.17 -0.48l0 -2.46z"/><path id="rji-prog-flat" d="M-0.36 -14.07c0.33 -0.06 0.87 0 1.08 0.15c0.06 0.03 0.06 0.36 -0.03 5.25c-0.06 2.85 -0.09 5.19 -0.09 5.19c0 0.03 0.12 -0.03 0.24 -0.12c0.63 -0.42 1.41 -0.66 2.19 -0.72c0.81 -0.03 1.47 0.21 2.04 0.78c0.57 0.54 0.87 1.26 0.93 2.04c0.03 0.57 -0.09 1.08 -0.36 1.62c-0.42 0.81 -1.02 1.38 -2.82 2.61c-1.14 0.78 -1.44 1.02 -1.8 1.44c-0.18 0.18 -0.39 0.39 -0.45 0.42c-0.27 0.18 -0.57 0.15 -0.81 -0.06c-0.06 -0.09 -0.12 -0.18 -0.15 -0.27c-0.03 -0.06 -0.09 -3.27 -0.18 -8.34c-0.09 -4.53 -0.15 -8.58 -0.18 -9.03l0 -0.78l0.12 -0.06c0.06 -0.03 0.18 -0.09 0.27 -0.12zm3.18 11.01c-0.21 -0.12 -0.54 -0.15 -0.81 -0.06c-0.54 0.15 -0.99 0.63 -1.17 1.26c-0.06 0.3 -0.12 2.88 -0.06 3.87c0.03 0.42 0.03 0.81 0.06 0.9l0.03 0.12l0.45 -0.39c0.63 -0.54 1.26 -1.17 1.56 -1.59c0.3 -0.42 0.6 -0.99 0.72 -1.41c0.18 -0.69 0.09 -1.47 -0.18 -2.07c-0.15 -0.3 -0.33 -0.51 -0.6 -0.63z"/></defs><g fill="currentColor" stroke="currentColor" stroke-linecap="square"><line x1="6.00" y1="30.00" x2="376.00" y2="30.00" stroke-width="0.9"/><line x1="6.00" y1="37.75" x2="376.00" y2="37.75" stroke-width="0.9"/><line x1="6.00" y1="45.50" x2="376.00" y2="45.50" stroke-width="0.9"/><line x1="6.00" y1="53.25" x2="376.00" y2="53.25" stroke-width="0.9"/><line x1="6.00" y1="61.00" x2="376.00" y2="61.00" stroke-width="0.9"/><use href="#rji-prog-clef" x="10.00" y="53.25"/><line x1="6.00" y1="30.00" x2="6.00" y2="61.00" stroke-width="1.1"/><text x="75.00" y="20.00" font-size="11" class="notation-chord">C</text><line x1="64.31" y1="68.75" x2="85.69" y2="68.75" stroke-width="1"/><use href="#rji-prog-whole" x="67.51" y="68.75"/><use href="#rji-prog-whole" x="67.51" y="61.00"/><use href="#rji-prog-whole" x="67.51" y="53.25"/><line x1="118.00" y1="30.00" x2="118.00" y2="61.00" stroke-width="1.1"/><text x="161.00" y="20.00" font-size="11" class="notation-chord">E</text><use href="#rji-prog-whole" x="153.51" y="61.00"/><use href="#rji-prog-sharp" x="143.76" y="53.25"/><use href="#rji-prog-whole" x="153.51" y="53.25"/><use href="#rji-prog-whole" x="153.51" y="45.50"/><line x1="204.00" y1="30.00" x2="204.00" y2="61.00" stroke-width="1.1"/><text x="247.00" y="20.00" font-size="11" class="notation-chord">Ab</text><use href="#rji-prog-flat" x="231.26" y="49.38"/><use href="#rji-prog-whole" x="239.51" y="49.38"/><use href="#rji-prog-whole" x="239.51" y="41.62"/><use href="#rji-prog-flat" x="231.26" y="33.88"/><use href="#rji-prog-whole" x="239.51" y="33.88"/><line x1="290.00" y1="30.00" x2="290.00" y2="61.00" stroke-width="1.1"/><text x="333.00" y="20.00" font-size="11" class="notation-chord">C</text><line x1="322.31" y1="68.75" x2="343.69" y2="68.75" stroke-width="1"/><use href="#rji-prog-whole" x="325.51" y="68.75"/><use href="#rji-prog-whole" x="325.51" y="61.00"/><use href="#rji-prog-whole" x="325.51" y="53.25"/><line x1="376.00" y1="30.00" x2="376.00" y2="61.00" stroke-width="1.1"/><line x1="6.00" y1="109.00" x2="376.00" y2="109.00" stroke-width="0.9"/><line x1="6.00" y1="116.75" x2="376.00" y2="116.75" stroke-width="0.9"/><line x1="6.00" y1="124.50" x2="376.00" y2="124.50" stroke-width="0.9"/><line x1="6.00" y1="132.25" x2="376.00" y2="132.25" stroke-width="0.9"/><line x1="6.00" y1="140.00" x2="376.00" y2="140.00" stroke-width="0.9"/><use href="#rji-prog-clef" x="10.00" y="132.25"/><line x1="6.00" y1="109.00" x2="6.00" y2="140.00" stroke-width="1.1"/><text x="75.00" y="99.00" font-size="11" class="notation-chord">F</text><use href="#rji-prog-whole" x="67.51" y="136.12"/><use href="#rji-prog-whole" x="67.51" y="128.38"/><use href="#rji-prog-whole" x="67.51" y="120.62"/><line x1="118.00" y1="109.00" x2="118.00" y2="140.00" stroke-width="1.1"/><text x="161.00" y="99.00" font-size="11" class="notation-chord">A</text><use href="#rji-prog-whole" x="153.51" y="128.38"/><use href="#rji-prog-sharp" x="143.76" y="120.62"/><use href="#rji-prog-whole" x="153.51" y="120.62"/><use href="#rji-prog-whole" x="153.51" y="112.88"/><line x1="204.00" y1="109.00" x2="204.00" y2="140.00" stroke-width="1.1"/><text x="247.00" y="99.00" font-size="11" class="notation-chord">D</text><use href="#rji-prog-whole" x="239.51" y="143.88"/><use href="#rji-prog-sharp" x="229.76" y="136.12"/><use href="#rji-prog-whole" x="239.51" y="136.12"/><use href="#rji-prog-whole" x="239.51" y="128.38"/><line x1="290.00" y1="109.00" x2="290.00" y2="140.00" stroke-width="1.1"/><text x="333.00" y="99.00" font-size="11" class="notation-chord">G7</text><line x1="322.31" y1="147.75" x2="343.69" y2="147.75" stroke-width="1"/><line x1="322.31" y1="155.50" x2="343.69" y2="155.50" stroke-width="1"/><use href="#rji-prog-whole" x="325.51" y="159.38"/><line x1="322.31" y1="147.75" x2="343.69" y2="147.75" stroke-width="1"/><use href="#rji-prog-whole" x="325.51" y="151.62"/><use href="#rji-prog-whole" x="325.51" y="143.88"/><use href="#rji-prog-whole" x="325.51" y="136.12"/><line x1="376.00" y1="109.00" x2="376.00" y2="140.00" stroke-width="1.1"/><line x1="6.00" y1="188.00" x2="376.00" y2="188.00" stroke-width="0.9"/><line x1="6.00" y1="195.75" x2="376.00" y2="195.75" stroke-width="0.9"/><line x1="6.00" y1="203.50" x2="376.00" y2="203.50" stroke-width="0.9"/><line x1="6.00" y1="211.25" x2="376.00" y2="211.25" stroke-width="0.9"/><line x1="6.00" y1="219.00" x2="376.00" y2="219.00" stroke-width="0.9"/><use href="#rji-prog-clef" x="10.00" y="211.25"/><line x1="6.00" y1="188.00" x2="6.00" y2="219.00" stroke-width="1.1"/><text x="75.00" y="178.00" font-size="11" class="notation-chord">C</text><line x1="64.31" y1="226.75" x2="85.69" y2="226.75" stroke-width="1"/><use href="#rji-prog-whole" x="67.51" y="226.75"/><use href="#rji-prog-whole" x="67.51" y="219.00"/><use href="#rji-prog-whole" x="67.51" y="211.25"/><line x1="118.00" y1="188.00" x2="118.00" y2="219.00" stroke-width="1.1"/><text x="161.00" y="178.00" font-size="11" class="notation-chord">E</text><use href="#rji-prog-whole" x="153.51" y="219.00"/><use href="#rji-prog-sharp" x="143.76" y="211.25"/><use href="#rji-prog-whole" x="153.51" y="211.25"/><use href="#rji-prog-whole" x="153.51" y="203.50"/><line x1="204.00" y1="188.00" x2="204.00" y2="219.00" stroke-width="1.1"/><text x="247.00" y="178.00" font-size="11" class="notation-chord">F</text><use href="#rji-prog-whole" x="239.51" y="215.12"/><use href="#rji-prog-whole" x="239.51" y="207.38"/><use href="#rji-prog-whole" x="239.51" y="199.62"/><line x1="290.00" y1="188.00" x2="290.00" y2="219.00" stroke-width="1.1"/><text x="333.00" y="178.00" font-size="11" class="notation-chord">C</text><line x1="322.31" y1="226.75" x2="343.69" y2="226.75" stroke-width="1"/><use href="#rji-prog-whole" x="325.51" y="226.75"/><use href="#rji-prog-whole" x="325.51" y="219.00"/><use href="#rji-prog-whole" x="325.51" y="211.25"/><line x1="371.00" y1="188.00" x2="371.00" y2="219.00" stroke-width="1.1"/><line x1="374.40" y1="188.00" x2="374.40" y2="219.00" stroke-width="3.2"/></g></svg></div>
</figure>

The first two columns use the same progression: once as pure sine waves, then
again with a simple harmonic timbre. The third keeps a sustained C underneath
the progression, so the tradeoff between a global reference pitch and
chord-local purity becomes easier to hear.

<div class="oscilloscope-matrix">
  <table>
    <thead>
      <tr>
        <th scope="col">tuning system</th>
        <th scope="col">sine wave progression</th>
        <th scope="col">harmonic timbre progression</th>
        <th scope="col">C drone progression</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <th scope="row">12-TET</th>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/twelve-tet-sine-progression.wav"></audio>
            <figcaption>Stable pitch classes, heard without extra harmonics.</figcaption>
          </figure>
        </td>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/twelve-tet-progression.wav"></audio>
            <figcaption>Stable pitch classes, compromised intervals.</figcaption>
          </figure>
        </td>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/twelve-tet-c-drone-progression.wav"></audio>
            <figcaption>The equal-tempered progression against a fixed C reference.</figcaption>
          </figure>
        </td>
      </tr>
      <tr>
        <th scope="row">fixed C just intonation</th>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/fixed-c-ji-sine-progression.wav"></audio>
            <figcaption>C sounds pure; remote chords are noticeably out of tune.</figcaption>
          </figure>
        </td>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/fixed-c-ji-progression.wav"></audio>
            <figcaption>C sounds pure; the added harmonics make the mistuning of remote chords easier to hear.</figcaption>
          </figure>
        </td>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/fixed-c-ji-c-drone-progression.wav"></audio>
            <figcaption>Fixed-C tuning remains consistent with the drone as the harmony moves to other keys.</figcaption>
          </figure>
        </td>
      </tr>
      <tr>
        <th scope="row">recursive just intonation</th>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/recursive-ji-sine-progression.wav"></audio>
            <figcaption>Each chord retunes around its own C-derived root.</figcaption>
          </figure>
        </td>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/recursive-ji-progression.wav"></audio>
            <figcaption>Chord-local roots with the simple harmonic timbre.</figcaption>
          </figure>
        </td>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/recursive-ji-c-drone-progression.wav"></audio>
            <figcaption>The drone makes it audible when chord-local roots diverge from global C.</figcaption>
          </figure>
        </td>
      </tr>
    </tbody>
  </table>
</div>

### What If The Roots Come From 12-TET?

Another way to build the 12 pianos is to take the row roots from 12-TET, then
build a just-intoned scale on top of each one:

```text
hybrid_frequency(root, degree) =
    C_frequency * 2^(root / 12) * J[degree]
```

So the root grid keeps equal temperament's transposition symmetry, while each
row still has just local intervals. The tradeoff is that the row roots no
longer come from the original C-based just scale; they are the familiar piano
frequencies with just chords built on them.

<figure class="audio-figure" data-oscilloscope>
  <audio controls src="/misc/media/twelve-tet-rooted-ji-progression.wav"></audio>
  <figcaption>12-TET roots with just-intoned chord tones on each root.</figcaption>
</figure>

There is also a stripped-down example that alternates a fixed-C pitch with its
recursive chord-local version, then plays both at once so the beating is easier
to hear:

<figure class="abc-figure">
  <div class="abc-notation" data-recursive-ji-abc="note-splits"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 554 404" class="engraved-notation" role="img" aria-label="Each split pitch written three times: fixed, recursive, and both together"><title>Each split pitch written three times: fixed, recursive, and both together</title><defs><path id="rji-split-clef" d="M9.69 -37.41c0.09 -0.09 0.24 -0.06 0.36 0c0.12 0.09 0.57 0.6 0.96 1.11c1.77 2.34 3.21 5.85 3.57 8.73c0.21 1.56 0.03 3.27 -0.45 4.86c-0.69 2.31 -1.92 4.47 -4.23 7.44c-0.3 0.39 -0.57 0.72 -0.6 0.75c-0.03 0.06 0 0.15 0.18 0.78c0.54 1.68 1.38 4.44 1.68 5.49l0.09 0.42l0.39 0c1.47 0.09 2.76 0.51 3.96 1.29c1.83 1.23 3.06 3.21 3.39 5.52c0.09 0.45 0.12 1.29 0.06 1.74c-0.09 1.02 -0.33 1.83 -0.75 2.73c-0.84 1.71 -2.28 3.06 -4.02 3.72l-0.33 0.12l0.03 1.26c0 1.74 -0.06 3.63 -0.21 4.62c-0.45 3.06 -2.19 5.49 -4.47 6.21c-0.57 0.18 -0.9 0.21 -1.59 0.21c-0.69 0 -1.02 -0.03 -1.65 -0.21c-1.14 -0.27 -2.13 -0.84 -2.94 -1.65c-0.99 -0.99 -1.56 -2.16 -1.71 -3.54c-0.09 -0.81 0.06 -1.53 0.45 -2.13c0.63 -0.99 1.83 -1.56 3 -1.53c1.5 0.09 2.64 1.32 2.73 2.94c0.06 1.47 -0.93 2.7 -2.37 2.97c-0.45 0.06 -0.84 0.03 -1.29 -0.09l-0.21 -0.09l0.09 0.12c0.39 0.54 0.78 0.93 1.32 1.26c1.35 0.87 3.06 1.02 4.35 0.36c1.44 -0.72 2.52 -2.28 2.97 -4.35c0.15 -0.66 0.24 -1.5 0.3 -3.03c0.03 -0.84 0.03 -2.94 0 -3c-0.03 0 -0.18 0 -0.36 0.03c-0.66 0.12 -0.99 0.12 -1.83 0.12c-1.05 0 -1.71 -0.06 -2.61 -0.3c-4.02 -0.99 -7.11 -4.35 -7.8 -8.46c-0.12 -0.66 -0.12 -0.99 -0.12 -1.83c0 -0.84 0 -1.14 0.15 -1.92c0.36 -2.28 1.41 -4.62 3.3 -7.29l2.79 -3.6c0.54 -0.66 0.96 -1.2 0.96 -1.23c0 -0.03 -0.09 -0.33 -0.18 -0.69c-0.96 -3.21 -1.41 -5.28 -1.59 -7.68c-0.12 -1.38 -0.15 -3.09 -0.06 -3.96c0.33 -2.67 1.38 -5.07 3.12 -7.08c0.36 -0.42 0.99 -1.05 1.17 -1.14zm2.01 4.71c-0.15 -0.3 -0.3 -0.54 -0.3 -0.54c-0.03 0 -0.18 0.09 -0.3 0.21c-2.4 1.74 -3.87 4.2 -4.26 7.11c-0.06 0.54 -0.06 1.41 -0.03 1.89c0.09 1.29 0.48 3.12 1.08 5.22c0.15 0.42 0.24 0.78 0.24 0.81c0 0.03 0.84 -1.11 1.23 -1.68c1.89 -2.73 2.88 -5.07 3.15 -7.53c0.09 -0.57 0.12 -1.74 0.06 -2.37c-0.09 -1.23 -0.27 -1.92 -0.87 -3.12zm-2.94 20.7c-0.21 -0.72 -0.39 -1.32 -0.42 -1.32c0 0 -1.2 1.47 -1.86 2.37c-2.79 3.63 -4.02 6.3 -4.35 9.3c-0.03 0.21 -0.03 0.69 -0.03 1.08c0 0.69 0 0.75 0.06 1.11c0.12 0.54 0.27 0.99 0.51 1.47c0.69 1.38 1.83 2.55 3.42 3.42c0.96 0.54 2.07 0.9 3.21 1.08c0.78 0.12 2.04 0.12 2.94 -0.03c0.51 -0.06 0.45 -0.03 0.42 -0.3c-0.24 -3.33 -0.72 -6.33 -1.62 -10.08c-0.09 -0.39 -0.18 -0.75 -0.18 -0.78c-0.03 -0.03 -0.42 0 -0.81 0.09c-0.9 0.18 -1.65 0.57 -2.22 1.14c-0.72 0.72 -1.08 1.65 -1.05 2.64c0.06 0.96 0.48 1.83 1.23 2.58c0.36 0.36 0.72 0.63 1.17 0.9c0.33 0.18 0.36 0.21 0.42 0.33c0.18 0.42 -0.18 0.9 -0.6 0.87c-0.18 -0.03 -0.84 -0.36 -1.26 -0.63c-0.78 -0.51 -1.38 -1.11 -1.86 -1.83c-1.77 -2.7 -0.99 -6.42 1.71 -8.19c0.3 -0.21 0.81 -0.48 1.17 -0.63c0.3 -0.09 1.02 -0.3 1.14 -0.3c0.06 0 0.09 0 0.09 -0.03c0.03 -0.03 -0.51 -1.92 -1.23 -4.26zm3.78 7.41c-0.18 -0.03 -0.36 -0.06 -0.39 -0.06c-0.03 0 0 0.21 0.18 1.02c0.75 3.18 1.26 6.3 1.5 9.09c0.06 0.72 0 0.69 0.51 0.42c0.78 -0.36 1.44 -0.96 1.98 -1.77c1.08 -1.62 1.2 -3.69 0.3 -5.55c-0.81 -1.62 -2.31 -2.79 -4.08 -3.15z"/><path id="rji-split-sharp" d="M5.73 -11.19c0.21 -0.12 0.54 -0.03 0.66 0.24c0.06 0.12 0.06 0.21 0.06 2.31c0 1.23 0 2.22 0.03 2.22c0 0 0.27 -0.12 0.6 -0.24c0.69 -0.27 0.78 -0.3 0.96 -0.15c0.21 0.15 0.21 0.18 0.21 1.38c0 1.02 0 1.11 -0.06 1.2c-0.03 0.06 -0.09 0.12 -0.12 0.15c-0.06 0.03 -0.42 0.21 -0.84 0.36l-0.75 0.33l-0.03 2.43c0 1.32 0 2.43 0.03 2.43c0 0 0.27 -0.12 0.6 -0.24c0.69 -0.27 0.78 -0.3 0.96 -0.15c0.21 0.15 0.21 0.18 0.21 1.38c0 1.02 0 1.11 -0.06 1.2c-0.03 0.06 -0.09 0.12 -0.12 0.15c-0.06 0.03 -0.42 0.21 -0.84 0.36l-0.75 0.33l-0.03 2.52c0 2.28 -0.03 2.55 -0.06 2.64c-0.21 0.36 -0.72 0.36 -0.93 0c-0.03 -0.09 -0.06 -0.33 -0.06 -2.43l0 -2.31l-1.29 0.51l-1.26 0.51l0 2.43c0 2.58 0 2.52 -0.15 2.67c-0.06 0.09 -0.27 0.18 -0.36 0.18c-0.12 0 -0.33 -0.09 -0.39 -0.18c-0.15 -0.15 -0.15 -0.09 -0.15 -2.43c0 -1.23 0 -2.22 -0.03 -2.22c0 0 -0.27 0.12 -0.6 0.24c-0.69 0.27 -0.78 0.3 -0.96 0.15c-0.21 -0.15 -0.21 -0.18 -0.21 -1.38c0 -1.02 0 -1.11 0.06 -1.2c0.03 -0.06 0.09 -0.12 0.12 -0.15c0.06 -0.03 0.42 -0.21 0.84 -0.36l0.78 -0.33l0 -2.43c0 -1.32 0 -2.43 -0.03 -2.43c0 0 -0.27 0.12 -0.6 0.24c-0.69 0.27 -0.78 0.3 -0.96 0.15c-0.21 -0.15 -0.21 -0.18 -0.21 -1.38c0 -1.02 0 -1.11 0.06 -1.2c0.03 -0.06 0.09 -0.12 0.12 -0.15c0.06 -0.03 0.42 -0.21 0.84 -0.36l0.78 -0.33l0 -2.52c0 -2.28 0.03 -2.55 0.06 -2.64c0.21 -0.36 0.72 -0.36 0.93 0c0.03 0.09 0.06 0.33 0.06 2.43l0.03 2.31l1.26 -0.51l1.26 -0.51l0 -2.43c0 -2.28 0 -2.43 0.06 -2.55c0.06 -0.12 0.12 -0.18 0.27 -0.24zm-0.33 10.65l0 -2.43l-1.29 0.51l-1.26 0.51l0 2.46l0 2.43l0.09 -0.03c0.06 -0.03 0.63 -0.27 1.29 -0.51l1.17 -0.48l0 -2.46z"/><path id="rji-split-quarter" d="M6.09 -4.05c0.36 -0.03 1.2 0 1.53 0.06c1.17 0.24 1.89 0.84 2.16 1.83c0.06 0.18 0.06 0.3 0.06 0.66c0 0.45 0 0.63 -0.15 1.08c-0.66 2.04 -3.06 3.93 -5.52 4.38c-0.54 0.09 -1.44 0.09 -1.83 0.03c-1.23 -0.27 -1.98 -0.87 -2.25 -1.86c-0.06 -0.18 -0.06 -0.3 -0.06 -0.66c0 -0.45 0 -0.63 0.15 -1.08c0.24 -0.78 0.75 -1.53 1.44 -2.22c1.2 -1.2 2.85 -2.01 4.47 -2.22z"/></defs><g fill="currentColor" stroke="currentColor" stroke-linecap="square"><line x1="6.00" y1="52.00" x2="548.00" y2="52.00" stroke-width="0.9"/><line x1="6.00" y1="59.75" x2="548.00" y2="59.75" stroke-width="0.9"/><line x1="6.00" y1="67.50" x2="548.00" y2="67.50" stroke-width="0.9"/><line x1="6.00" y1="75.25" x2="548.00" y2="75.25" stroke-width="0.9"/><line x1="6.00" y1="83.00" x2="548.00" y2="83.00" stroke-width="0.9"/><use href="#rji-split-clef" x="10.00" y="75.25"/><line x1="6.00" y1="52.00" x2="6.00" y2="83.00" stroke-width="1.1"/><text x="118.00" y="26.00" font-size="9.5" class="notation-annotation">E major G#/Ab</text><text x="118.00" y="38.00" font-size="9.5" class="notation-annotation">fixed +0.000c</text><use href="#rji-split-sharp" x="103.34" y="75.25"/><use href="#rji-split-quarter" x="113.09" y="75.25"/><line x1="122.41" y1="75.25" x2="122.41" y2="48.12" stroke-width="1.1"/><text x="290.00" y="26.00" font-size="9.5" class="notation-annotation">recursive</text><text x="290.00" y="38.00" font-size="9.5" class="notation-annotation">-41.059c</text><use href="#rji-split-sharp" x="275.35" y="75.25"/><use href="#rji-split-quarter" x="285.10" y="75.25"/><line x1="294.40" y1="75.25" x2="294.40" y2="48.12" stroke-width="1.1"/><text x="462.00" y="26.00" font-size="9.5" class="notation-annotation">together</text><text x="462.00" y="38.00" font-size="9.5" class="notation-annotation">+0.000c / -41.059c</text><use href="#rji-split-sharp" x="447.35" y="75.25"/><use href="#rji-split-quarter" x="457.10" y="75.25"/><line x1="466.40" y1="75.25" x2="466.40" y2="48.12" stroke-width="1.1"/><line x1="543.00" y1="52.00" x2="543.00" y2="83.00" stroke-width="1.1"/><line x1="546.40" y1="52.00" x2="546.40" y2="83.00" stroke-width="3.2"/><line x1="6.00" y1="153.00" x2="548.00" y2="153.00" stroke-width="0.9"/><line x1="6.00" y1="160.75" x2="548.00" y2="160.75" stroke-width="0.9"/><line x1="6.00" y1="168.50" x2="548.00" y2="168.50" stroke-width="0.9"/><line x1="6.00" y1="176.25" x2="548.00" y2="176.25" stroke-width="0.9"/><line x1="6.00" y1="184.00" x2="548.00" y2="184.00" stroke-width="0.9"/><use href="#rji-split-clef" x="10.00" y="176.25"/><line x1="6.00" y1="153.00" x2="6.00" y2="184.00" stroke-width="1.1"/><text x="118.00" y="127.00" font-size="9.5" class="notation-annotation">F major A</text><text x="118.00" y="139.00" font-size="9.5" class="notation-annotation">fixed +0.000c</text><use href="#rji-split-quarter" x="113.09" y="172.38"/><line x1="122.41" y1="172.38" x2="122.41" y2="145.25" stroke-width="1.1"/><text x="290.00" y="127.00" font-size="9.5" class="notation-annotation">recursive</text><text x="290.00" y="139.00" font-size="9.5" class="notation-annotation">+0.000c</text><use href="#rji-split-quarter" x="285.10" y="172.38"/><line x1="294.40" y1="172.38" x2="294.40" y2="145.25" stroke-width="1.1"/><text x="462.00" y="127.00" font-size="9.5" class="notation-annotation">together</text><text x="462.00" y="139.00" font-size="9.5" class="notation-annotation">+0.000c / +0.000c</text><use href="#rji-split-quarter" x="457.10" y="172.38"/><line x1="466.40" y1="172.38" x2="466.40" y2="145.25" stroke-width="1.1"/><line x1="543.00" y1="153.00" x2="543.00" y2="184.00" stroke-width="1.1"/><line x1="546.40" y1="153.00" x2="546.40" y2="184.00" stroke-width="3.2"/><line x1="6.00" y1="254.00" x2="548.00" y2="254.00" stroke-width="0.9"/><line x1="6.00" y1="261.75" x2="548.00" y2="261.75" stroke-width="0.9"/><line x1="6.00" y1="269.50" x2="548.00" y2="269.50" stroke-width="0.9"/><line x1="6.00" y1="277.25" x2="548.00" y2="277.25" stroke-width="0.9"/><line x1="6.00" y1="285.00" x2="548.00" y2="285.00" stroke-width="0.9"/><use href="#rji-split-clef" x="10.00" y="277.25"/><line x1="6.00" y1="254.00" x2="6.00" y2="285.00" stroke-width="1.1"/><text x="118.00" y="228.00" font-size="9.5" class="notation-annotation">A major C#/Db</text><text x="118.00" y="240.00" font-size="9.5" class="notation-annotation">fixed +0.000c</text><use href="#rji-split-sharp" x="103.34" y="265.62"/><use href="#rji-split-quarter" x="113.09" y="265.62"/><line x1="113.59" y1="265.62" x2="113.59" y2="292.75" stroke-width="1.1"/><text x="290.00" y="228.00" font-size="9.5" class="notation-annotation">recursive</text><text x="290.00" y="240.00" font-size="9.5" class="notation-annotation">-41.059c</text><use href="#rji-split-sharp" x="275.35" y="265.62"/><use href="#rji-split-quarter" x="285.10" y="265.62"/><line x1="285.60" y1="265.62" x2="285.60" y2="292.75" stroke-width="1.1"/><text x="462.00" y="228.00" font-size="9.5" class="notation-annotation">together</text><text x="462.00" y="240.00" font-size="9.5" class="notation-annotation">+0.000c / -41.059c</text><use href="#rji-split-sharp" x="447.35" y="265.62"/><use href="#rji-split-quarter" x="457.10" y="265.62"/><line x1="457.60" y1="265.62" x2="457.60" y2="292.75" stroke-width="1.1"/><line x1="543.00" y1="254.00" x2="543.00" y2="285.00" stroke-width="1.1"/><line x1="546.40" y1="254.00" x2="546.40" y2="285.00" stroke-width="3.2"/><line x1="6.00" y1="355.00" x2="548.00" y2="355.00" stroke-width="0.9"/><line x1="6.00" y1="362.75" x2="548.00" y2="362.75" stroke-width="0.9"/><line x1="6.00" y1="370.50" x2="548.00" y2="370.50" stroke-width="0.9"/><line x1="6.00" y1="378.25" x2="548.00" y2="378.25" stroke-width="0.9"/><line x1="6.00" y1="386.00" x2="548.00" y2="386.00" stroke-width="0.9"/><use href="#rji-split-clef" x="10.00" y="378.25"/><line x1="6.00" y1="355.00" x2="6.00" y2="386.00" stroke-width="1.1"/><text x="118.00" y="329.00" font-size="9.5" class="notation-annotation">G7 F</text><text x="118.00" y="341.00" font-size="9.5" class="notation-annotation">fixed +0.000c</text><use href="#rji-split-quarter" x="113.09" y="382.12"/><line x1="122.41" y1="382.12" x2="122.41" y2="355.00" stroke-width="1.1"/><text x="290.00" y="329.00" font-size="9.5" class="notation-annotation">recursive</text><text x="290.00" y="341.00" font-size="9.5" class="notation-annotation">+0.000c</text><use href="#rji-split-quarter" x="285.10" y="382.12"/><line x1="294.40" y1="382.12" x2="294.40" y2="355.00" stroke-width="1.1"/><text x="462.00" y="329.00" font-size="9.5" class="notation-annotation">together</text><text x="462.00" y="341.00" font-size="9.5" class="notation-annotation">+0.000c / +0.000c</text><use href="#rji-split-quarter" x="457.10" y="382.12"/><line x1="466.40" y1="382.12" x2="466.40" y2="355.00" stroke-width="1.1"/><line x1="543.00" y1="355.00" x2="543.00" y2="386.00" stroke-width="1.1"/><line x1="546.40" y1="355.00" x2="546.40" y2="386.00" stroke-width="3.2"/></g></svg></div>
  <figcaption>The stripped-down pitch split demo: fixed C just intonation first, recursive just intonation second, then both together with the recursive offset marked in cents.</figcaption>
</figure>

<figure class="audio-figure" data-oscilloscope>
  <audio controls src="/misc/media/recursive-ji-note-splits.wav"></audio>
  <figcaption>Pitch-name splits: same nominal note, different chord context.</figcaption>
</figure>

more audio examples:

<figure class="audio-figure" data-oscilloscope>
  <audio controls src="/misc/media/mozart-dies-irae-recursive-just-intonation-piano.wav"></audio>
  <figcaption>mozarts dies irae.</figcaption>
</figure>

<figure class="audio-figure" data-oscilloscope>
  <audio controls src="/misc/media/recursive-just-intonation-composition.wav"></audio>
  <figcaption>some composition I came up with for this blog post.</figcaption>
</figure>

Some split points:

| chord context | note  |   fixed C JI | recursive JI |      difference |
| ------------- | ----- | -----------: | -----------: | --------------: |
| E major       | G#/Ab | `212.571 Hz` | `204.395 Hz` | `-67.900 cents` |
| F major       | A     | `220.747 Hz` | `214.615 Hz` | `-48.770 cents` |
| A major       | C#/Db | `277.978 Hz` | `275.934 Hz` | `-12.777 cents` |

### Why This Is Nice

The nice part is that every major chord can be made into a clean `4:5:6`
relationship, even if the chord root is not C. E major does not inherit C's
G#/Ab; it gets its own G#/Ab. F major does not inherit C's A; it gets its own A.

That lines up with how I hear harmony. When a chord arrives, the ear can accept
the chord root as a local center. Recursive just intonation uses that local
center instead of constraining every chord to one global keyboard.

It is also a useful programming model. A chord can be rendered as:

```text
root_frequency = base_frequency * global_just_ratio[root]
note_frequency = root_frequency * local_just_ratio[chord_degree]
```

The same pure function works for any root.

### Why This Is Bad

The bad part shows up as soon as the chord changes: the same note name can move.

In 12-TET, G#/Ab is one frequency per octave. In fixed C just intonation, G#/Ab
is also one frequency per octave, just a different one. In recursive just
intonation, G#/Ab depends on why you are playing it.

A few consequences of that:

- A melody can shift pitch if a held note is reinterpreted by the next chord.
- Enharmonic spelling starts to matter, but a 12-key interface usually hides it.
- Modulation requires balancing smooth voice-leading against pure local
  chords.
- Instruments with fixed frets, keys, or holes cannot do this without pitch
  bending or multiple samples per pitch class.

So this is not a replacement for equal temperament. Equal temperament is still
the practical compromise that lets every key share one physical instrument.

### Practical Uses

One day I will make a keyboard on which with your left hand you can determine the current key/context and with your right hand you play notes that are dynamically retuned according to the table, until then the practical applications remain few.

## My Other Music Work

- [Play around with different tuning systems and your computer keyboard](/tools/tuningplayground.md)
- [Visualize and listen to polyrhythms](/tools/polyrhythm.md)
- [music21-rs](https://hilll.dev/music21-rs/)

### Visualize and Listen to Polyrhythms in a Shader

<iframe width="640" height="360" frameborder="0" allowfullscreen="allowfullscreen" src="https://www.shadertoy.com/embed/7tV3WV?gui=true&t=10&paused=false&muted=false"></iframe>