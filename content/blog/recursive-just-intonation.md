---
title: Why you should buy 12 Pianos | Recursive Just Intonation
date: 2022-11-29
updated: 2026-05-05
tags:
  - music
  - programming
---

## What is this about?

Recursive Just Intonation is a novel tuning system I invented up with during my highschool physics classes, while trying to come up with a solution
to the dissonance of 12TET. Before i explain what it is I'll give a little background on tuning systems in general.

## Equal Temperament vs Just Intonation

Equal temperament gives us one keyboard. Every C# is the same C#, every G is
the same G, and every semitone is the same distance from the last one. That is
very convenient, at the cost of shaving almost every interval a little. The
intervals are close enough to simple ratios that they work, but most of them
are not exact.

Just intonation goes the other way. It starts from simple frequency ratios:

- octave: `2/1`
- perfect fifth: `3/2`
- perfect fourth: `4/3`
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

The experiment here is:

> Keep the roots on a C-based just-intonation keyboard, but give every chord
> root its own just-intonation keyboard.

I think of it as 12 pianos: one just piano rooted on C, one on C#/Db, one on D,
and so on. The root of each piano is taken from the original C just-intonation
scale. Once a chord chooses a root, all of its notes come from the piano rooted
on that note.

This is "recursive" in the simple algorithmic sense: use a just-ratio table to
choose the chord root, then use the same ratio table again inside that root.

For a C-based just-intonation scale the 12 pitch
classes are:

| pitch | ratio from C |
| ----- | -----------: |
| C     |        `1/1` |
| C#/Db |      `17/16` |
| D     |        `9/8` |
| D#/Eb |      `19/16` |
| E     |        `5/4` |
| F     |        `4/3` |
| F#/Gb |      `45/32` |
| G     |        `3/2` |
| G#/Ab |      `51/32` |
| A     |      `27/16` |
| A#/Bb |      `57/32` |
| B     |       `15/8` |

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

### What The Waves Look Like

Nice mathematical ratios are pleasant to our ears.
`x + 2*x`, where `x` is some frequency, sounds nice because it has a short
period:

<figure class="wave-figure">
  <figcaption>A tone and its octave: two sine waves at a base frequency <code>f</code> and <code>2f</code>. The whole pattern repeats every <code>1/f</code> seconds, so the ear can lock onto it easily.</figcaption>
  <iframe class="no-input" tabindex="-1" width="850" height="500" src="https://graphtoy.com/?f1(x,t)=sin(x+5*t)+sin(2*(x+5*t))&v1=true&f2(x,t)=&v2=false&f3(x,t)=&v3=false&f4(x,t)=&v4=false&f5(x,t)=&v5=false&f6(x,t)=&v6=false&grid=1&coords=0,-3,12">
  </iframe>
</figure>

While, for example, `x + 13/12*x` has a much longer period:

<figure class="wave-figure">
  <figcaption>A tone and a narrow nearby step: two sine waves at <code>f</code> and <code>13/12 f</code>. The combined wave needs <code>12/f</code> seconds to repeat, so it takes much longer to settle than the octave example.</figcaption>
  <iframe class="no-input" tabindex="-1" width="850" height="500" src="https://graphtoy.com/?f1(x,t)=sin(x+5*t)+sin((13/12)*(x+5*t))&v1=true&f2(x,t)=&v2=false&f3(x,t)=&v3=false&f4(x,t)=&v4=false&f5(x,t)=&v5=false&f6(x,t)=&v6=false&grid=1&coords=0,-3,12">
  </iframe>
</figure>

A just major chord is `4:5:6`, or `1:1.25:1.5`. In 12-TET, the same chord is
closer to `500:630:749`, or `1:1.260:1.498`.

<figure class="wave-figure">
  <figcaption>Two major chords: the just version uses exact <code>4:5:6</code> ratios, while the 12-TET version uses the familiar piano/guitar approximation. They are close, but the 12-TET peaks do not quite return to the same places.</figcaption>
  <iframe class="no-input" tabindex="-1" width="850" height="500" src="https://graphtoy.com/?f1(x,t)=sin(x+5*t)+sin(1.25*(x+5*t))+sin(1.5*(x+5*t))&v1=true&f2(x,t)=sin(x+5*t)+sin(1.26*(x+5*t))+sin(1.4983*(x+5*t))&v2=true&f3(x,t)=&v3=false&f4(x,t)=&v4=false&f5(x,t)=&v5=false&f6(x,t)=&v6=true&grid=1&coords=0,-3,12">
  </iframe>
</figure>

## 12 Just Pianos | Recursive Just Intonation

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

| local root |                                                                                                                                   root `1/1` |                                                                                                                                       `17/16` |                                                                                                                                         `9/8` |                                                                                                                                      `19/16` |                                                                                                                             major third `5/4` |                                                                                                                                 fourth `4/3` |                                                                                                                                       `45/32` |                                                                                                                                  fifth `3/2` |                                                                                                                                       `51/32` |                                                                                                                                       `27/16` |                                                                                                                                      `57/32` |                                                                                                                                        `15/8` |
| ---------- | -------------------------------------------------------------------------------------------------------------------------------------------: | --------------------------------------------------------------------------------------------------------------------------------------------: | --------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | --------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | --------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | --------------------------------------------------------------------------------------------------------------------------------------------: | --------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | --------------------------------------------------------------------------------------------------------------------------------------------: |
| C          |            <span class="recursive-note-cell note-c" data-note="C"><code>130.813 Hz</code><small class="tet-cents">0.000 cents</small></span> |  <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>138.989 Hz</code><small class="tet-cents">+4.955 cents</small></span> |            <span class="recursive-note-cell note-d" data-note="D"><code>147.164 Hz</code><small class="tet-cents">+3.910 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>155.340 Hz</code><small class="tet-cents">-2.487 cents</small></span> |           <span class="recursive-note-cell note-e" data-note="E"><code>163.516 Hz</code><small class="tet-cents">-13.686 cents</small></span> |           <span class="recursive-note-cell note-f" data-note="F"><code>174.417 Hz</code><small class="tet-cents">-1.955 cents</small></span> |  <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>183.955 Hz</code><small class="tet-cents">-9.776 cents</small></span> |           <span class="recursive-note-cell note-g" data-note="G"><code>196.219 Hz</code><small class="tet-cents">+1.955 cents</small></span> |  <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>208.483 Hz</code><small class="tet-cents">+6.910 cents</small></span> |            <span class="recursive-note-cell note-a" data-note="A"><code>220.747 Hz</code><small class="tet-cents">+5.865 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>233.010 Hz</code><small class="tet-cents">-0.532 cents</small></span> |           <span class="recursive-note-cell note-b" data-note="B"><code>245.274 Hz</code><small class="tet-cents">-11.731 cents</small></span> |
| C#/Db      | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>138.989 Hz</code><small class="tet-cents">+4.955 cents</small></span> |            <span class="recursive-note-cell note-d" data-note="D"><code>147.675 Hz</code><small class="tet-cents">+9.911 cents</small></span> |  <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>156.362 Hz</code><small class="tet-cents">+8.865 cents</small></span> |           <span class="recursive-note-cell note-e" data-note="E"><code>165.049 Hz</code><small class="tet-cents">+2.468 cents</small></span> |            <span class="recursive-note-cell note-f" data-note="F"><code>173.736 Hz</code><small class="tet-cents">-8.731 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>185.318 Hz</code><small class="tet-cents">+3.000 cents</small></span> |            <span class="recursive-note-cell note-g" data-note="G"><code>195.453 Hz</code><small class="tet-cents">-4.821 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>208.483 Hz</code><small class="tet-cents">+6.910 cents</small></span> |           <span class="recursive-note-cell note-a" data-note="A"><code>221.513 Hz</code><small class="tet-cents">+11.866 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>234.543 Hz</code><small class="tet-cents">+10.820 cents</small></span> |           <span class="recursive-note-cell note-b" data-note="B"><code>247.573 Hz</code><small class="tet-cents">+4.423 cents</small></span> |            <span class="recursive-note-cell note-c" data-note="C"><code>260.604 Hz</code><small class="tet-cents">-6.776 cents</small></span> |
| D          |           <span class="recursive-note-cell note-d" data-note="D"><code>147.164 Hz</code><small class="tet-cents">+3.910 cents</small></span> |  <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>156.362 Hz</code><small class="tet-cents">+8.865 cents</small></span> |            <span class="recursive-note-cell note-e" data-note="E"><code>165.560 Hz</code><small class="tet-cents">+7.820 cents</small></span> |           <span class="recursive-note-cell note-f" data-note="F"><code>174.758 Hz</code><small class="tet-cents">+1.423 cents</small></span> |  <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>183.955 Hz</code><small class="tet-cents">-9.776 cents</small></span> |           <span class="recursive-note-cell note-g" data-note="G"><code>196.219 Hz</code><small class="tet-cents">+1.955 cents</small></span> |  <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>206.950 Hz</code><small class="tet-cents">-5.866 cents</small></span> |           <span class="recursive-note-cell note-a" data-note="A"><code>220.747 Hz</code><small class="tet-cents">+5.865 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>234.543 Hz</code><small class="tet-cents">+10.820 cents</small></span> |            <span class="recursive-note-cell note-b" data-note="B"><code>248.340 Hz</code><small class="tet-cents">+9.775 cents</small></span> |           <span class="recursive-note-cell note-c" data-note="C"><code>262.137 Hz</code><small class="tet-cents">+3.378 cents</small></span> |  <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>275.933 Hz</code><small class="tet-cents">-7.821 cents</small></span> |
| D#/Eb      | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>155.340 Hz</code><small class="tet-cents">-2.487 cents</small></span> |            <span class="recursive-note-cell note-e" data-note="E"><code>165.049 Hz</code><small class="tet-cents">+2.468 cents</small></span> |            <span class="recursive-note-cell note-f" data-note="F"><code>174.758 Hz</code><small class="tet-cents">+1.423 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>184.466 Hz</code><small class="tet-cents">-4.974 cents</small></span> |           <span class="recursive-note-cell note-g" data-note="G"><code>194.175 Hz</code><small class="tet-cents">-16.173 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>207.120 Hz</code><small class="tet-cents">-4.442 cents</small></span> |           <span class="recursive-note-cell note-a" data-note="A"><code>218.447 Hz</code><small class="tet-cents">-12.263 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>233.010 Hz</code><small class="tet-cents">-0.532 cents</small></span> |            <span class="recursive-note-cell note-b" data-note="B"><code>247.573 Hz</code><small class="tet-cents">+4.423 cents</small></span> |            <span class="recursive-note-cell note-c" data-note="C"><code>262.137 Hz</code><small class="tet-cents">+3.378 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>276.700 Hz</code><small class="tet-cents">-3.019 cents</small></span> |           <span class="recursive-note-cell note-d" data-note="D"><code>291.263 Hz</code><small class="tet-cents">-14.218 cents</small></span> |
| E          |          <span class="recursive-note-cell note-e" data-note="E"><code>163.516 Hz</code><small class="tet-cents">-13.686 cents</small></span> |            <span class="recursive-note-cell note-f" data-note="F"><code>173.736 Hz</code><small class="tet-cents">-8.731 cents</small></span> |  <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>183.955 Hz</code><small class="tet-cents">-9.776 cents</small></span> |          <span class="recursive-note-cell note-g" data-note="G"><code>194.175 Hz</code><small class="tet-cents">-16.173 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>204.395 Hz</code><small class="tet-cents">-27.373 cents</small></span> |          <span class="recursive-note-cell note-a" data-note="A"><code>218.021 Hz</code><small class="tet-cents">-15.641 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>229.944 Hz</code><small class="tet-cents">-23.463 cents</small></span> |          <span class="recursive-note-cell note-b" data-note="B"><code>245.274 Hz</code><small class="tet-cents">-11.731 cents</small></span> |            <span class="recursive-note-cell note-c" data-note="C"><code>260.604 Hz</code><small class="tet-cents">-6.776 cents</small></span> |  <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>275.933 Hz</code><small class="tet-cents">-7.821 cents</small></span> |          <span class="recursive-note-cell note-d" data-note="D"><code>291.263 Hz</code><small class="tet-cents">-14.218 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>306.592 Hz</code><small class="tet-cents">-25.418 cents</small></span> |
| F          |           <span class="recursive-note-cell note-f" data-note="F"><code>174.417 Hz</code><small class="tet-cents">-1.955 cents</small></span> |  <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>185.318 Hz</code><small class="tet-cents">+3.000 cents</small></span> |            <span class="recursive-note-cell note-g" data-note="G"><code>196.219 Hz</code><small class="tet-cents">+1.955 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>207.120 Hz</code><small class="tet-cents">-4.442 cents</small></span> |           <span class="recursive-note-cell note-a" data-note="A"><code>218.021 Hz</code><small class="tet-cents">-15.641 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>232.556 Hz</code><small class="tet-cents">-3.910 cents</small></span> |           <span class="recursive-note-cell note-b" data-note="B"><code>245.274 Hz</code><small class="tet-cents">-11.731 cents</small></span> |            <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> |  <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>277.977 Hz</code><small class="tet-cents">+4.955 cents</small></span> |            <span class="recursive-note-cell note-d" data-note="D"><code>294.329 Hz</code><small class="tet-cents">+3.910 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>310.680 Hz</code><small class="tet-cents">-2.487 cents</small></span> |           <span class="recursive-note-cell note-e" data-note="E"><code>327.032 Hz</code><small class="tet-cents">-13.686 cents</small></span> |
| F#/Gb      | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>183.955 Hz</code><small class="tet-cents">-9.776 cents</small></span> |            <span class="recursive-note-cell note-g" data-note="G"><code>195.453 Hz</code><small class="tet-cents">-4.821 cents</small></span> |  <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>206.950 Hz</code><small class="tet-cents">-5.866 cents</small></span> |          <span class="recursive-note-cell note-a" data-note="A"><code>218.447 Hz</code><small class="tet-cents">-12.263 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>229.944 Hz</code><small class="tet-cents">-23.463 cents</small></span> |          <span class="recursive-note-cell note-b" data-note="B"><code>245.274 Hz</code><small class="tet-cents">-11.731 cents</small></span> |           <span class="recursive-note-cell note-c" data-note="C"><code>258.687 Hz</code><small class="tet-cents">-19.553 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>275.933 Hz</code><small class="tet-cents">-7.821 cents</small></span> |            <span class="recursive-note-cell note-d" data-note="D"><code>293.179 Hz</code><small class="tet-cents">-2.866 cents</small></span> |  <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>310.425 Hz</code><small class="tet-cents">-3.911 cents</small></span> |          <span class="recursive-note-cell note-e" data-note="E"><code>327.671 Hz</code><small class="tet-cents">-10.308 cents</small></span> |           <span class="recursive-note-cell note-f" data-note="F"><code>344.917 Hz</code><small class="tet-cents">-21.508 cents</small></span> |
| G          |           <span class="recursive-note-cell note-g" data-note="G"><code>196.219 Hz</code><small class="tet-cents">+1.955 cents</small></span> |  <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>208.483 Hz</code><small class="tet-cents">+6.910 cents</small></span> |            <span class="recursive-note-cell note-a" data-note="A"><code>220.747 Hz</code><small class="tet-cents">+5.865 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>233.010 Hz</code><small class="tet-cents">-0.532 cents</small></span> |           <span class="recursive-note-cell note-b" data-note="B"><code>245.274 Hz</code><small class="tet-cents">-11.731 cents</small></span> |            <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> |  <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>275.933 Hz</code><small class="tet-cents">-7.821 cents</small></span> |           <span class="recursive-note-cell note-d" data-note="D"><code>294.329 Hz</code><small class="tet-cents">+3.910 cents</small></span> |  <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>312.724 Hz</code><small class="tet-cents">+8.865 cents</small></span> |            <span class="recursive-note-cell note-e" data-note="E"><code>331.120 Hz</code><small class="tet-cents">+7.820 cents</small></span> |           <span class="recursive-note-cell note-f" data-note="F"><code>349.515 Hz</code><small class="tet-cents">+1.423 cents</small></span> |  <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>367.911 Hz</code><small class="tet-cents">-9.776 cents</small></span> |
| G#/Ab      | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>208.483 Hz</code><small class="tet-cents">+6.910 cents</small></span> |           <span class="recursive-note-cell note-a" data-note="A"><code>221.513 Hz</code><small class="tet-cents">+11.866 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>234.543 Hz</code><small class="tet-cents">+10.820 cents</small></span> |           <span class="recursive-note-cell note-b" data-note="B"><code>247.573 Hz</code><small class="tet-cents">+4.423 cents</small></span> |            <span class="recursive-note-cell note-c" data-note="C"><code>260.604 Hz</code><small class="tet-cents">-6.776 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>277.977 Hz</code><small class="tet-cents">+4.955 cents</small></span> |            <span class="recursive-note-cell note-d" data-note="D"><code>293.179 Hz</code><small class="tet-cents">-2.866 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>312.724 Hz</code><small class="tet-cents">+8.865 cents</small></span> |           <span class="recursive-note-cell note-e" data-note="E"><code>332.270 Hz</code><small class="tet-cents">+13.821 cents</small></span> |           <span class="recursive-note-cell note-f" data-note="F"><code>351.815 Hz</code><small class="tet-cents">+12.775 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>371.360 Hz</code><small class="tet-cents">+6.378 cents</small></span> |            <span class="recursive-note-cell note-g" data-note="G"><code>390.905 Hz</code><small class="tet-cents">-4.821 cents</small></span> |
| A          |           <span class="recursive-note-cell note-a" data-note="A"><code>220.747 Hz</code><small class="tet-cents">+5.865 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>234.543 Hz</code><small class="tet-cents">+10.820 cents</small></span> |            <span class="recursive-note-cell note-b" data-note="B"><code>248.340 Hz</code><small class="tet-cents">+9.775 cents</small></span> |           <span class="recursive-note-cell note-c" data-note="C"><code>262.137 Hz</code><small class="tet-cents">+3.378 cents</small></span> |  <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>275.933 Hz</code><small class="tet-cents">-7.821 cents</small></span> |           <span class="recursive-note-cell note-d" data-note="D"><code>294.329 Hz</code><small class="tet-cents">+3.910 cents</small></span> |  <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>310.425 Hz</code><small class="tet-cents">-3.911 cents</small></span> |           <span class="recursive-note-cell note-e" data-note="E"><code>331.120 Hz</code><small class="tet-cents">+7.820 cents</small></span> |           <span class="recursive-note-cell note-f" data-note="F"><code>351.815 Hz</code><small class="tet-cents">+12.775 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>372.510 Hz</code><small class="tet-cents">+11.730 cents</small></span> |           <span class="recursive-note-cell note-g" data-note="G"><code>393.205 Hz</code><small class="tet-cents">+5.333 cents</small></span> |  <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>413.900 Hz</code><small class="tet-cents">-5.866 cents</small></span> |
| A#/Bb      | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>233.010 Hz</code><small class="tet-cents">-0.532 cents</small></span> |            <span class="recursive-note-cell note-b" data-note="B"><code>247.573 Hz</code><small class="tet-cents">+4.423 cents</small></span> |            <span class="recursive-note-cell note-c" data-note="C"><code>262.137 Hz</code><small class="tet-cents">+3.378 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>276.700 Hz</code><small class="tet-cents">-3.019 cents</small></span> |           <span class="recursive-note-cell note-d" data-note="D"><code>291.263 Hz</code><small class="tet-cents">-14.218 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>310.680 Hz</code><small class="tet-cents">-2.487 cents</small></span> |           <span class="recursive-note-cell note-e" data-note="E"><code>327.671 Hz</code><small class="tet-cents">-10.308 cents</small></span> |           <span class="recursive-note-cell note-f" data-note="F"><code>349.515 Hz</code><small class="tet-cents">+1.423 cents</small></span> |  <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>371.360 Hz</code><small class="tet-cents">+6.378 cents</small></span> |            <span class="recursive-note-cell note-g" data-note="G"><code>393.205 Hz</code><small class="tet-cents">+5.333 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>415.050 Hz</code><small class="tet-cents">-1.064 cents</small></span> |           <span class="recursive-note-cell note-a" data-note="A"><code>436.894 Hz</code><small class="tet-cents">-12.263 cents</small></span> |
| B          |          <span class="recursive-note-cell note-b" data-note="B"><code>245.274 Hz</code><small class="tet-cents">-11.731 cents</small></span> |            <span class="recursive-note-cell note-c" data-note="C"><code>260.604 Hz</code><small class="tet-cents">-6.776 cents</small></span> |  <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>275.933 Hz</code><small class="tet-cents">-7.821 cents</small></span> |          <span class="recursive-note-cell note-d" data-note="D"><code>291.263 Hz</code><small class="tet-cents">-14.218 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>306.592 Hz</code><small class="tet-cents">-25.418 cents</small></span> |          <span class="recursive-note-cell note-e" data-note="E"><code>327.032 Hz</code><small class="tet-cents">-13.686 cents</small></span> |           <span class="recursive-note-cell note-f" data-note="F"><code>344.917 Hz</code><small class="tet-cents">-21.508 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>367.911 Hz</code><small class="tet-cents">-9.776 cents</small></span> |            <span class="recursive-note-cell note-g" data-note="G"><code>390.905 Hz</code><small class="tet-cents">-4.821 cents</small></span> |  <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>413.900 Hz</code><small class="tet-cents">-5.866 cents</small></span> |          <span class="recursive-note-cell note-a" data-note="A"><code>436.894 Hz</code><small class="tet-cents">-12.263 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>459.889 Hz</code><small class="tet-cents">-23.463 cents</small></span> |

So the system is not a 12-note tuning system anymore. It is a chord-contextual
tuning system. Pitch classes split according to harmonic function.

### What It Sounds Like

The audio examples use the same chord progression three ways:

```text
C -> E -> G#/Ab -> C -> F -> A -> D -> G7 -> C -> E -> F -> C
```

I picked a progression that visits chords where fixed-C just intonation has
audible trouble. In the recursive version, each chord retunes around its own
root.

<script type="module" src="/js/audiooscilloscope.js"></script>

Here is the same recursive progression as pure sine waves, without the extra
harmonics used in the examples below:

<figure class="audio-figure" data-oscilloscope>
  <figcaption>Pure sine recursive just intonation: the same chord progression, with each chord retuned around its own C-derived root.</figcaption>
  <audio controls src="/blog/recursive-just-intonation/recursive-ji-sine-progression.wav"></audio>
</figure>

<figure class="audio-figure" data-oscilloscope>
  <figcaption>12-TET: stable pitch classes, compromised intervals.</figcaption>
  <audio controls src="/blog/recursive-just-intonation/twelve-tet-progression.wav"></audio>
</figure>

<figure class="audio-figure" data-oscilloscope>
  <figcaption>Fixed C just intonation: C is beautiful, but remote chords start leaning hard.</figcaption>
  <audio controls src="/blog/recursive-just-intonation/fixed-c-ji-progression.wav"></audio>
</figure>

<figure class="audio-figure" data-oscilloscope>
  <figcaption>Recursive just intonation: each chord is tuned from its own C-derived root.</figcaption>
  <audio controls src="/blog/recursive-just-intonation/recursive-ji-progression.wav"></audio>
</figure>

There is also a stripped-down example that alternates a fixed-C pitch with its
recursive chord-local version, then plays both at once so the beating is easier
to hear:

<figure class="audio-figure" data-oscilloscope>
  <figcaption>Pitch-name splits: same nominal note, different chord context.</figcaption>
  <audio controls src="/blog/recursive-just-intonation/recursive-ji-note-splits.wav"></audio>
</figure>

Some of the generated split points:

| chord context | note  |   fixed C JI | recursive JI |      difference |
| ------------- | ----- | -----------: | -----------: | --------------: |
| E major       | G#/Ab | `208.483 Hz` | `204.395 Hz` | `-34.283 cents` |
| F major       | A     | `220.747 Hz` | `218.021 Hz` | `-21.506 cents` |
| A major       | C#/Db | `277.977 Hz` | `275.933 Hz` | `-12.777 cents` |
| G7            | F     | `348.834 Hz` | `349.515 Hz` |  `+3.378 cents` |

### Why This Is Nice

The nice part is that every major chord can be made into a clean `4:5:6`
relationship, even if the chord root is not C. E major does not inherit C's
G#/Ab; it gets its own G#/Ab. F major does not inherit C's A; it gets its own A.

That lines up with how I hear harmony. When a chord arrives, the ear can accept
the chord root as a local center. Recursive just intonation follows that local
center instead of forcing every chord to negotiate with one global keyboard.

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

A few consequences fall out of that:

- A melody can wobble if a held pitch is reinterpreted by the next chord.
- Enharmonic spelling starts to matter, but a 12-key interface usually hides it.
- Modulation becomes a negotiation between smooth voice-leading and pure local
  chords.
- Instruments with fixed frets, keys, or holes cannot do this without pitch
  bending or multiple samples per pitch class.

So this is not a replacement for equal temperament. Equal temperament is still
the practical compromise that lets every key share one physical instrument.

Recursive just intonation is mostly a listening tool for me. It makes the
root-relationship audible again, instead of flattening every chord tone onto one
global keyboard.

### Implementation Notes

The Rust renderer lives in `tools/site/src/recursive_ji.rs`. It does not use
MIDI, because standard MIDI note numbers assume fixed pitch classes unless you
add extra tuning messages. Instead it writes 16-bit mono WAV files directly.

Generate the files with:

```sh
cargo run --manifest-path tools/site/Cargo.toml -- recursive-ji-music
```

The same command also writes a CSV frequency report to
`content/blog/recursive-just-intonation/recursive-ji-frequencies.csv`.

The renderer supports three tunings:

- `12-TET`: `C4 * 2^(n/12)`
- `Fixed C just intonation`: `C4 * J[pitch_class] * octave`
- `Recursive just intonation`: `C4 * J[root] * J[chord_degree] * octave`

## My Other Music Work

- [Play around with different tuning systems and your computer keyboard](/tools/tuningplayground.md)
- [Visualize and listen to polyrhythms](/tools/polyrhythm.md)
- [music21-rs](https://hilll.dev/music21-rs/)

### Visualize and Listen to Polyrhythms in a Shader

<iframe width="640" height="360" frameborder="0" allowfullscreen="allowfullscreen" src="https://www.shadertoy.com/embed/7tV3WV?gui=true&t=10&paused=false&muted=false"></iframe>
