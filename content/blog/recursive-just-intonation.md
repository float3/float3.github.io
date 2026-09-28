---
title: Why you should buy 12 Pianos | Recursive Just Intonation
tags:
  - music
  - programming
---

## What is this about?

Recursive just intonation is a toy tuning system I came up with during my
highschool physics classes, while trying to find a solution to the dissonance of
12TET. I was frustrated with 12TET and watched a video on just intonation, I
immediately realized the impracticality of it, so I decided to make my own even
less practical version.

It's very easy to predict why it won't become popular. That said I find it
interesting and both mathematically and musically beautiful, so I decided to
write this blogpost ([listening examples further below](#listening-examples)).
Before I explain what it is I'll give a little background on tuning systems in
general.

## What makes one interval nice and another unpleasant

Nice mathematical ratios are pleasant to our ears. `x + 2*x`, where `x` is some
frequency, is gonna sound nice, because it has a short period:

<figure class="wave-figure">
  <iframe class="no-input" tabindex="-1" width="850" height="500" src="https://graphtoy.com/?f1(x,t)=sin(x+5*t)+sin(2*(x+5*t))&v1=true&f2(x,t)=&v2=false&f3(x,t)=&v3=false&f4(x,t)=&v4=false&f5(x,t)=&v5=false&f6(x,t)=&v6=false&grid=1&coords=0,-3,12">
  </iframe>
  <figcaption>A tone and its octave: sine waves at <code>f</code> and <code>2f</code>. The pattern repeats every <code>1/f</code> seconds.</figcaption>
</figure>

while for example `x + 13/12*x` has a much longer period:

<figure class="wave-figure">
  <iframe class="no-input" tabindex="-1" width="850" height="500" src="https://graphtoy.com/?f1(x,t)=sin(x+5*t)+sin((13/12)*(x+5*t))&v1=true&f2(x,t)=&v2=false&f3(x,t)=&v3=false&f4(x,t)=&v4=false&f5(x,t)=&v5=false&f6(x,t)=&v6=false&grid=1&coords=0,-3,12">
  </iframe>
  <figcaption>Sine waves at <code>f</code> and <code>13/12 f</code>. The combined wave repeats every <code>12/f</code> seconds.</figcaption>
</figure>

Waves that are nice to look at are nice to the ear.

## 12TET: the current standard

In 12TET the ratio between two neighbouring tones is

```text
2^(1/12) = 1.059463...
```

so a note `n` semitones above some reference note has the frequency

```text
frequency(n) = reference * 2^(n/12)
```

This serves the purpose of making sure all steps have the same size, relative
to their base frequency (every step is 100 cents). E.g. multiplying a frequency
by `2^(1/12)` 7 times in a row is the same as going 7 steps at once:

```text
(2^(1/12))^7 = 2^(7/12)
```

which is a nice property that's true only for equal temperament systems. It's
why a piano can play in every key without retuning. The price is that no
interval except the octave is a nice mathematical ratio: the fifth is 2 cents
off `3/2` and the major third is 13.7 cents off `5/4`.

## Just Intonation

In just intonation every note is a nice mathematical ratio to a root. The
five-limit scale on C builds all twelve notes out of octaves (`2`), perfect
fifths (`3/2`) and major thirds (`5/4`):

| pitch | ratio from C | built from      | from 12TET |
| ----- | -----------: | --------------- | ---------: |
| C     |        `1/1` | `1`             |     `0.0¢` |
| C#/Db |      `16/15` | `4/3 ÷ 5/4`     |   `+11.7¢` |
| D     |        `9/8` | `3/2 × 3/2 ÷ 2` |    `+3.9¢` |
| D#/Eb |        `6/5` | `3/2 ÷ 5/4`     |   `+15.6¢` |
| E     |        `5/4` | `5/4`           |   `-13.7¢` |
| F     |        `4/3` | `2 ÷ 3/2`       |    `-2.0¢` |
| F#/Gb |      `64/45` | `16/15 × 4/3`   |    `+9.8¢` |
| G     |        `3/2` | `3/2`           |    `+2.0¢` |
| G#/Ab |        `8/5` | `2 ÷ 5/4`       |   `+13.7¢` |
| A     |        `5/3` | `4/3 × 5/4`     |   `-15.6¢` |
| A#/Bb |       `16/9` | `4/3 × 4/3`     |    `-3.9¢` |
| B     |       `15/8` | `3/2 × 5/4`     |   `-11.7¢` |
| C     |        `2/1` | `2`             |     `0.0¢` |

### Why Just Intonation is good

Just intonation is nice because intervals have nice mathematical ratios. For
example, a major chord is `4:5:6` (`1:1.25:1.5`), while in 12TET a major chord
is `500:630:749` (`1:1.260:1.498`). The following graph shows the difference
between the just intonated major chord and the 12TET major chord:

<figure class="wave-figure">
<iframe class="no-input" tabindex="-1" width="850" height="500" src="https://graphtoy.com/?f1(x,t)=sin(x+5*t)+sin((5/4)*(x+5*t))+sin((3/2)*(x+5*t))&v1=true&f2(x,t)=sin(x+5*t)+sin((2^(4/12))*(x+5*t))+sin((2^(7/12))*(x+5*t))&v2=true&f3(x,t)=&v3=false&f4(x,t)=&v4=false&f5(x,t)=&v5=false&f6(x,t)=&v6=false&grid=1&coords=0,-3,12">
</iframe>
  <figcaption>The just major chord at exactly <code>4:5:6</code>, and the one a piano plays. The piano's peaks do not return to the same places.</figcaption>
</figure>

### Why Just Intonation is bad

```text
(16/15)^2 = 256/225 ≠ 9/8
```

but

```text
(2^(1/12))^2 = 2^(2/12)
```

Now while just intonated intervals are nicer, all of these intervals are in
relation to C, our root. While a major third (`4:5`) and a perfect fifth (`2:3`)
on their own sound good, if we keep going up the steps one by one (`16/15`), we
don't end up at the same place that we would end up if we skipped a step
(`9/8`). I.e. just intonation doesn't have the property mentioned earlier.

You hear this as soon as you play a chord that isn't built on C. C major is
exact, but E major on the same keyboard is:

```text
E      = 5/4
G#/Ab  = 8/5
B      = 15/8
```

The fifth, `(15/8) / (5/4) = 3/2`, is fine. The third is
`(8/5) / (5/4) = 32/25 = 1.28` where it should be `5/4 = 1.25`, which is 41.1
cents too high, three times worse than the piano's third. The table's G#/Ab is
really an Ab, a major third below C. The G# that E major needs is a major third
above E, at `25/16`, and a keyboard with twelve keys has no key left for it.

## 12 Just Pianos | Recursive Just Intonation

So here's the idea:

> Keep the roots on a C-based just intonation keyboard, but give every chord
> root its own just intonated keyboard.

I think of it as 12 pianos: one just piano rooted on C, one on C#/Db, one on D,
and so on. The root of each piano is taken from the C just intonation scale
above. Once a chord chooses a root, all of its notes come from the piano rooted
on that note. It's recursive because the same ratio table is used twice, once
to pick the root and once again inside that root:

```text
frequency(root, degree) = C * J[root] * J[degree]
```

where `J` is the ratio table above, dropped an octave whenever `root + degree`
goes past the next C.

For E major:

```text
E  = C * 5/4
G# = E * 5/4 = C * 25/16
B  = E * 3/2 = C * 15/8
```

and now `E : G# : B = 4 : 5 : 6`. The same works for every major chord on every
root.

Here are all 12 pianos written out as frequencies, with C at `130.813 Hz`:

- every row is one piano, named by its root
- every column is a pitch name, and the cell is the frequency that pitch has
  on that row's piano (same pitch name, same colour)
- every row starts at its root and goes up an octave, so pitches below the root
  are an octave higher
- the small number is how many cents that frequency is away from 12TET

E major is the `E`, `G#/Ab` and `B` cells of the E row: `163.516 Hz`,
`204.395 Hz` and `245.274 Hz`. For another octave multiply or divide the whole
row by `2`.

| local root | `C` | `C#/Db` | `D` | `D#/Eb` | `E` | `F` | `F#/Gb` | `G` | `G#/Ab` | `A` | `A#/Bb` | `B` |
| ---------- | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------: |
| C | <span class="recursive-note-cell note-c" data-note="C"><code>130.813 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>139.534 Hz</code><small class="tet-cents">11.731 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>147.164 Hz</code><small class="tet-cents">3.910 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>156.975 Hz</code><small class="tet-cents">15.641 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>163.516 Hz</code><small class="tet-cents">-13.686 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>174.417 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>186.045 Hz</code><small class="tet-cents">9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>196.219 Hz</code><small class="tet-cents">1.955 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>209.300 Hz</code><small class="tet-cents">13.686 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>218.021 Hz</code><small class="tet-cents">-15.641 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>232.556 Hz</code><small class="tet-cents">-3.910 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>245.274 Hz</code><small class="tet-cents">-11.731 cents</small></span> |
| C#/Db | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>139.534 Hz</code><small class="tet-cents">11.731 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>148.836 Hz</code><small class="tet-cents">23.463 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>156.975 Hz</code><small class="tet-cents">15.641 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>167.440 Hz</code><small class="tet-cents">27.373 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>174.417 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>186.045 Hz</code><small class="tet-cents">9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>198.448 Hz</code><small class="tet-cents">21.508 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>209.300 Hz</code><small class="tet-cents">13.686 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>223.254 Hz</code><small class="tet-cents">25.418 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>232.556 Hz</code><small class="tet-cents">-3.910 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>248.060 Hz</code><small class="tet-cents">7.821 cents</small></span> |
| D | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>275.933 Hz</code><small class="tet-cents">-7.821 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>147.164 Hz</code><small class="tet-cents">3.910 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>156.975 Hz</code><small class="tet-cents">15.641 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>165.560 Hz</code><small class="tet-cents">7.820 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>176.597 Hz</code><small class="tet-cents">19.551 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>183.955 Hz</code><small class="tet-cents">-9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>196.219 Hz</code><small class="tet-cents">1.955 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>209.300 Hz</code><small class="tet-cents">13.686 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>220.747 Hz</code><small class="tet-cents">5.865 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>235.463 Hz</code><small class="tet-cents">17.596 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>245.274 Hz</code><small class="tet-cents">-11.731 cents</small></span> |
| D#/Eb | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>279.067 Hz</code><small class="tet-cents">11.731 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>294.329 Hz</code><small class="tet-cents">3.910 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>156.975 Hz</code><small class="tet-cents">15.641 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>167.440 Hz</code><small class="tet-cents">27.373 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>176.597 Hz</code><small class="tet-cents">19.551 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>188.370 Hz</code><small class="tet-cents">31.283 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>196.219 Hz</code><small class="tet-cents">1.955 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>209.300 Hz</code><small class="tet-cents">13.686 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>223.254 Hz</code><small class="tet-cents">25.418 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>235.463 Hz</code><small class="tet-cents">17.596 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>251.161 Hz</code><small class="tet-cents">29.328 cents</small></span> |
| E | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>272.527 Hz</code><small class="tet-cents">-29.328 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>290.695 Hz</code><small class="tet-cents">-17.596 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>306.592 Hz</code><small class="tet-cents">-25.418 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>163.516 Hz</code><small class="tet-cents">-13.686 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>174.417 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>183.955 Hz</code><small class="tet-cents">-9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>196.219 Hz</code><small class="tet-cents">1.955 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>204.395 Hz</code><small class="tet-cents">-27.373 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>218.021 Hz</code><small class="tet-cents">-15.641 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>232.556 Hz</code><small class="tet-cents">-3.910 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>245.274 Hz</code><small class="tet-cents">-11.731 cents</small></span> |
| F | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>279.067 Hz</code><small class="tet-cents">11.731 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>290.695 Hz</code><small class="tet-cents">-17.596 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>310.075 Hz</code><small class="tet-cents">-5.865 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>327.032 Hz</code><small class="tet-cents">-13.686 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>174.417 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>186.045 Hz</code><small class="tet-cents">9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>196.219 Hz</code><small class="tet-cents">1.955 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>209.300 Hz</code><small class="tet-cents">13.686 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>218.021 Hz</code><small class="tet-cents">-15.641 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>232.556 Hz</code><small class="tet-cents">-3.910 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>248.060 Hz</code><small class="tet-cents">7.821 cents</small></span> |
| F#/Gb | <span class="recursive-note-cell note-c" data-note="C"><code>264.597 Hz</code><small class="tet-cents">19.553 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>279.067 Hz</code><small class="tet-cents">11.731 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>297.672 Hz</code><small class="tet-cents">23.463 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>310.075 Hz</code><small class="tet-cents">-5.865 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>330.746 Hz</code><small class="tet-cents">5.866 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>348.834 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>186.045 Hz</code><small class="tet-cents">9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>198.448 Hz</code><small class="tet-cents">21.508 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>209.300 Hz</code><small class="tet-cents">13.686 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>223.254 Hz</code><small class="tet-cents">25.418 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>232.556 Hz</code><small class="tet-cents">-3.910 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>248.060 Hz</code><small class="tet-cents">7.821 cents</small></span> |
| G | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>279.067 Hz</code><small class="tet-cents">11.731 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>294.329 Hz</code><small class="tet-cents">3.910 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>313.951 Hz</code><small class="tet-cents">15.641 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>327.032 Hz</code><small class="tet-cents">-13.686 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>348.834 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>367.911 Hz</code><small class="tet-cents">-9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>196.219 Hz</code><small class="tet-cents">1.955 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>209.300 Hz</code><small class="tet-cents">13.686 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>220.747 Hz</code><small class="tet-cents">5.865 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>235.463 Hz</code><small class="tet-cents">17.596 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>245.274 Hz</code><small class="tet-cents">-11.731 cents</small></span> |
| G#/Ab | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>279.067 Hz</code><small class="tet-cents">11.731 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>297.672 Hz</code><small class="tet-cents">23.463 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>313.951 Hz</code><small class="tet-cents">15.641 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>334.881 Hz</code><small class="tet-cents">27.373 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>348.834 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>372.090 Hz</code><small class="tet-cents">9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>392.438 Hz</code><small class="tet-cents">1.955 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>209.300 Hz</code><small class="tet-cents">13.686 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>223.254 Hz</code><small class="tet-cents">25.418 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>235.463 Hz</code><small class="tet-cents">17.596 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>251.161 Hz</code><small class="tet-cents">29.328 cents</small></span> |
| A | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>272.527 Hz</code><small class="tet-cents">-29.328 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>290.695 Hz</code><small class="tet-cents">-17.596 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>310.075 Hz</code><small class="tet-cents">-5.865 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>327.032 Hz</code><small class="tet-cents">-13.686 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>348.834 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>363.369 Hz</code><small class="tet-cents">-31.283 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>387.593 Hz</code><small class="tet-cents">-19.551 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>408.790 Hz</code><small class="tet-cents">-27.373 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>218.021 Hz</code><small class="tet-cents">-15.641 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>232.556 Hz</code><small class="tet-cents">-3.910 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>245.274 Hz</code><small class="tet-cents">-11.731 cents</small></span> |
| A#/Bb | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>279.067 Hz</code><small class="tet-cents">11.731 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>290.695 Hz</code><small class="tet-cents">-17.596 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>310.075 Hz</code><small class="tet-cents">-5.865 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>330.746 Hz</code><small class="tet-cents">5.866 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>348.834 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>372.090 Hz</code><small class="tet-cents">9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>387.593 Hz</code><small class="tet-cents">-19.551 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>413.433 Hz</code><small class="tet-cents">-7.820 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>436.043 Hz</code><small class="tet-cents">-15.641 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>232.556 Hz</code><small class="tet-cents">-3.910 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>248.060 Hz</code><small class="tet-cents">7.821 cents</small></span> |
| B | <span class="recursive-note-cell note-c" data-note="C"><code>261.626 Hz</code><small class="tet-cents">0.000 cents</small></span> | <span class="recursive-note-cell note-c-sharp" data-note="C#/Db"><code>275.933 Hz</code><small class="tet-cents">-7.821 cents</small></span> | <span class="recursive-note-cell note-d" data-note="D"><code>294.329 Hz</code><small class="tet-cents">3.910 cents</small></span> | <span class="recursive-note-cell note-d-sharp" data-note="D#/Eb"><code>306.592 Hz</code><small class="tet-cents">-25.418 cents</small></span> | <span class="recursive-note-cell note-e" data-note="E"><code>327.032 Hz</code><small class="tet-cents">-13.686 cents</small></span> | <span class="recursive-note-cell note-f" data-note="F"><code>348.834 Hz</code><small class="tet-cents">-1.955 cents</small></span> | <span class="recursive-note-cell note-f-sharp" data-note="F#/Gb"><code>367.911 Hz</code><small class="tet-cents">-9.776 cents</small></span> | <span class="recursive-note-cell note-g" data-note="G"><code>392.438 Hz</code><small class="tet-cents">1.955 cents</small></span> | <span class="recursive-note-cell note-g-sharp" data-note="G#/Ab"><code>408.790 Hz</code><small class="tet-cents">-27.373 cents</small></span> | <span class="recursive-note-cell note-a" data-note="A"><code>436.043 Hz</code><small class="tet-cents">-15.641 cents</small></span> | <span class="recursive-note-cell note-a-sharp" data-note="A#/Bb"><code>459.889 Hz</code><small class="tet-cents">-23.463 cents</small></span> | <span class="recursive-note-cell note-b" data-note="B"><code>245.274 Hz</code><small class="tet-cents">-11.731 cents</small></span> |

If you read down the `G#/Ab` column you can see the catch: eight pianos have
`209.300 Hz`, the E piano has `204.395 Hz`, the A and B pianos have that same
pitch an octave up, and the A#/Bb piano has a third one. Which G# you get
depends on which chord it's in.

## Listening examples

<script type="module" src="/js/audiooscilloscope.js"></script>

I picked a progression that goes to chords that fixed C just intonation gets
wrong. The first four chords, C, E, Ab and back to C, go up in major thirds,
which is exactly where E major's G# and the C piano's Ab clash.

<figure class="abc-figure">
  <div class="abc-notation" data-recursive-ji-abc="progression"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 382 237" class="engraved-notation" role="img" aria-label="The twelve-chord progression, written on a treble staff"><title>The twelve-chord progression, written on a treble staff</title><defs><path id="rji-prog-clef" d="M9.69 -37.41c0.09 -0.09 0.24 -0.06 0.36 0c0.12 0.09 0.57 0.6 0.96 1.11c1.77 2.34 3.21 5.85 3.57 8.73c0.21 1.56 0.03 3.27 -0.45 4.86c-0.69 2.31 -1.92 4.47 -4.23 7.44c-0.3 0.39 -0.57 0.72 -0.6 0.75c-0.03 0.06 0 0.15 0.18 0.78c0.54 1.68 1.38 4.44 1.68 5.49l0.09 0.42l0.39 0c1.47 0.09 2.76 0.51 3.96 1.29c1.83 1.23 3.06 3.21 3.39 5.52c0.09 0.45 0.12 1.29 0.06 1.74c-0.09 1.02 -0.33 1.83 -0.75 2.73c-0.84 1.71 -2.28 3.06 -4.02 3.72l-0.33 0.12l0.03 1.26c0 1.74 -0.06 3.63 -0.21 4.62c-0.45 3.06 -2.19 5.49 -4.47 6.21c-0.57 0.18 -0.9 0.21 -1.59 0.21c-0.69 0 -1.02 -0.03 -1.65 -0.21c-1.14 -0.27 -2.13 -0.84 -2.94 -1.65c-0.99 -0.99 -1.56 -2.16 -1.71 -3.54c-0.09 -0.81 0.06 -1.53 0.45 -2.13c0.63 -0.99 1.83 -1.56 3 -1.53c1.5 0.09 2.64 1.32 2.73 2.94c0.06 1.47 -0.93 2.7 -2.37 2.97c-0.45 0.06 -0.84 0.03 -1.29 -0.09l-0.21 -0.09l0.09 0.12c0.39 0.54 0.78 0.93 1.32 1.26c1.35 0.87 3.06 1.02 4.35 0.36c1.44 -0.72 2.52 -2.28 2.97 -4.35c0.15 -0.66 0.24 -1.5 0.3 -3.03c0.03 -0.84 0.03 -2.94 0 -3c-0.03 0 -0.18 0 -0.36 0.03c-0.66 0.12 -0.99 0.12 -1.83 0.12c-1.05 0 -1.71 -0.06 -2.61 -0.3c-4.02 -0.99 -7.11 -4.35 -7.8 -8.46c-0.12 -0.66 -0.12 -0.99 -0.12 -1.83c0 -0.84 0 -1.14 0.15 -1.92c0.36 -2.28 1.41 -4.62 3.3 -7.29l2.79 -3.6c0.54 -0.66 0.96 -1.2 0.96 -1.23c0 -0.03 -0.09 -0.33 -0.18 -0.69c-0.96 -3.21 -1.41 -5.28 -1.59 -7.68c-0.12 -1.38 -0.15 -3.09 -0.06 -3.96c0.33 -2.67 1.38 -5.07 3.12 -7.08c0.36 -0.42 0.99 -1.05 1.17 -1.14zm2.01 4.71c-0.15 -0.3 -0.3 -0.54 -0.3 -0.54c-0.03 0 -0.18 0.09 -0.3 0.21c-2.4 1.74 -3.87 4.2 -4.26 7.11c-0.06 0.54 -0.06 1.41 -0.03 1.89c0.09 1.29 0.48 3.12 1.08 5.22c0.15 0.42 0.24 0.78 0.24 0.81c0 0.03 0.84 -1.11 1.23 -1.68c1.89 -2.73 2.88 -5.07 3.15 -7.53c0.09 -0.57 0.12 -1.74 0.06 -2.37c-0.09 -1.23 -0.27 -1.92 -0.87 -3.12zm-2.94 20.7c-0.21 -0.72 -0.39 -1.32 -0.42 -1.32c0 0 -1.2 1.47 -1.86 2.37c-2.79 3.63 -4.02 6.3 -4.35 9.3c-0.03 0.21 -0.03 0.69 -0.03 1.08c0 0.69 0 0.75 0.06 1.11c0.12 0.54 0.27 0.99 0.51 1.47c0.69 1.38 1.83 2.55 3.42 3.42c0.96 0.54 2.07 0.9 3.21 1.08c0.78 0.12 2.04 0.12 2.94 -0.03c0.51 -0.06 0.45 -0.03 0.42 -0.3c-0.24 -3.33 -0.72 -6.33 -1.62 -10.08c-0.09 -0.39 -0.18 -0.75 -0.18 -0.78c-0.03 -0.03 -0.42 0 -0.81 0.09c-0.9 0.18 -1.65 0.57 -2.22 1.14c-0.72 0.72 -1.08 1.65 -1.05 2.64c0.06 0.96 0.48 1.83 1.23 2.58c0.36 0.36 0.72 0.63 1.17 0.9c0.33 0.18 0.36 0.21 0.42 0.33c0.18 0.42 -0.18 0.9 -0.6 0.87c-0.18 -0.03 -0.84 -0.36 -1.26 -0.63c-0.78 -0.51 -1.38 -1.11 -1.86 -1.83c-1.77 -2.7 -0.99 -6.42 1.71 -8.19c0.3 -0.21 0.81 -0.48 1.17 -0.63c0.3 -0.09 1.02 -0.3 1.14 -0.3c0.06 0 0.09 0 0.09 -0.03c0.03 -0.03 -0.51 -1.92 -1.23 -4.26zm3.78 7.41c-0.18 -0.03 -0.36 -0.06 -0.39 -0.06c-0.03 0 0 0.21 0.18 1.02c0.75 3.18 1.26 6.3 1.5 9.09c0.06 0.72 0 0.69 0.51 0.42c0.78 -0.36 1.44 -0.96 1.98 -1.77c1.08 -1.62 1.2 -3.69 0.3 -5.55c-0.81 -1.62 -2.31 -2.79 -4.08 -3.15z"/><path id="rji-prog-whole" d="M6.51 -4.05c0.51 -0.03 2.01 0 2.52 0.03c1.41 0.18 2.64 0.51 3.72 1.08c1.2 0.63 1.95 1.41 2.19 2.31c0.09 0.33 0.09 0.9 0 1.23c-0.24 0.9 -0.99 1.68 -2.19 2.31c-1.08 0.57 -2.28 0.9 -3.75 1.08c-0.66 0.06 -2.31 0.06 -2.97 0c-1.47 -0.18 -2.67 -0.51 -3.75 -1.08c-1.2 -0.63 -1.95 -1.41 -2.19 -2.31c-0.09 -0.33 -0.09 -0.9 0 -1.23c0.24 -0.9 0.99 -1.68 2.19 -2.31c1.2 -0.63 2.61 -0.99 4.23 -1.11zm0.57 0.66c-0.87 -0.15 -1.53 0 -2.04 0.51c-0.15 0.15 -0.24 0.27 -0.33 0.48c-0.24 0.51 -0.36 1.08 -0.33 1.77c0.03 0.69 0.18 1.26 0.42 1.77c0.6 1.17 1.74 1.98 3.18 2.22c1.11 0.21 1.95 -0.15 2.34 -0.99c0.24 -0.51 0.36 -1.08 0.33 -1.8c-0.06 -1.11 -0.45 -2.04 -1.17 -2.76c-0.63 -0.63 -1.47 -1.05 -2.4 -1.2z"/><path id="rji-prog-sharp" d="M5.73 -11.19c0.21 -0.12 0.54 -0.03 0.66 0.24c0.06 0.12 0.06 0.21 0.06 2.31c0 1.23 0 2.22 0.03 2.22c0 0 0.27 -0.12 0.6 -0.24c0.69 -0.27 0.78 -0.3 0.96 -0.15c0.21 0.15 0.21 0.18 0.21 1.38c0 1.02 0 1.11 -0.06 1.2c-0.03 0.06 -0.09 0.12 -0.12 0.15c-0.06 0.03 -0.42 0.21 -0.84 0.36l-0.75 0.33l-0.03 2.43c0 1.32 0 2.43 0.03 2.43c0 0 0.27 -0.12 0.6 -0.24c0.69 -0.27 0.78 -0.3 0.96 -0.15c0.21 0.15 0.21 0.18 0.21 1.38c0 1.02 0 1.11 -0.06 1.2c-0.03 0.06 -0.09 0.12 -0.12 0.15c-0.06 0.03 -0.42 0.21 -0.84 0.36l-0.75 0.33l-0.03 2.52c0 2.28 -0.03 2.55 -0.06 2.64c-0.21 0.36 -0.72 0.36 -0.93 0c-0.03 -0.09 -0.06 -0.33 -0.06 -2.43l0 -2.31l-1.29 0.51l-1.26 0.51l0 2.43c0 2.58 0 2.52 -0.15 2.67c-0.06 0.09 -0.27 0.18 -0.36 0.18c-0.12 0 -0.33 -0.09 -0.39 -0.18c-0.15 -0.15 -0.15 -0.09 -0.15 -2.43c0 -1.23 0 -2.22 -0.03 -2.22c0 0 -0.27 0.12 -0.6 0.24c-0.69 0.27 -0.78 0.3 -0.96 0.15c-0.21 -0.15 -0.21 -0.18 -0.21 -1.38c0 -1.02 0 -1.11 0.06 -1.2c0.03 -0.06 0.09 -0.12 0.12 -0.15c0.06 -0.03 0.42 -0.21 0.84 -0.36l0.78 -0.33l0 -2.43c0 -1.32 0 -2.43 -0.03 -2.43c0 0 -0.27 0.12 -0.6 0.24c-0.69 0.27 -0.78 0.3 -0.96 0.15c-0.21 -0.15 -0.21 -0.18 -0.21 -1.38c0 -1.02 0 -1.11 0.06 -1.2c0.03 -0.06 0.09 -0.12 0.12 -0.15c0.06 -0.03 0.42 -0.21 0.84 -0.36l0.78 -0.33l0 -2.52c0 -2.28 0.03 -2.55 0.06 -2.64c0.21 -0.36 0.72 -0.36 0.93 0c0.03 0.09 0.06 0.33 0.06 2.43l0.03 2.31l1.26 -0.51l1.26 -0.51l0 -2.43c0 -2.28 0 -2.43 0.06 -2.55c0.06 -0.12 0.12 -0.18 0.27 -0.24zm-0.33 10.65l0 -2.43l-1.29 0.51l-1.26 0.51l0 2.46l0 2.43l0.09 -0.03c0.06 -0.03 0.63 -0.27 1.29 -0.51l1.17 -0.48l0 -2.46z"/><path id="rji-prog-flat" d="M-0.36 -14.07c0.33 -0.06 0.87 0 1.08 0.15c0.06 0.03 0.06 0.36 -0.03 5.25c-0.06 2.85 -0.09 5.19 -0.09 5.19c0 0.03 0.12 -0.03 0.24 -0.12c0.63 -0.42 1.41 -0.66 2.19 -0.72c0.81 -0.03 1.47 0.21 2.04 0.78c0.57 0.54 0.87 1.26 0.93 2.04c0.03 0.57 -0.09 1.08 -0.36 1.62c-0.42 0.81 -1.02 1.38 -2.82 2.61c-1.14 0.78 -1.44 1.02 -1.8 1.44c-0.18 0.18 -0.39 0.39 -0.45 0.42c-0.27 0.18 -0.57 0.15 -0.81 -0.06c-0.06 -0.09 -0.12 -0.18 -0.15 -0.27c-0.03 -0.06 -0.09 -3.27 -0.18 -8.34c-0.09 -4.53 -0.15 -8.58 -0.18 -9.03l0 -0.78l0.12 -0.06c0.06 -0.03 0.18 -0.09 0.27 -0.12zm3.18 11.01c-0.21 -0.12 -0.54 -0.15 -0.81 -0.06c-0.54 0.15 -0.99 0.63 -1.17 1.26c-0.06 0.3 -0.12 2.88 -0.06 3.87c0.03 0.42 0.03 0.81 0.06 0.9l0.03 0.12l0.45 -0.39c0.63 -0.54 1.26 -1.17 1.56 -1.59c0.3 -0.42 0.6 -0.99 0.72 -1.41c0.18 -0.69 0.09 -1.47 -0.18 -2.07c-0.15 -0.3 -0.33 -0.51 -0.6 -0.63z"/></defs><g fill="currentColor" stroke="currentColor" stroke-linecap="square"><line x1="6.00" y1="30.00" x2="376.00" y2="30.00" stroke-width="0.9"/><line x1="6.00" y1="37.75" x2="376.00" y2="37.75" stroke-width="0.9"/><line x1="6.00" y1="45.50" x2="376.00" y2="45.50" stroke-width="0.9"/><line x1="6.00" y1="53.25" x2="376.00" y2="53.25" stroke-width="0.9"/><line x1="6.00" y1="61.00" x2="376.00" y2="61.00" stroke-width="0.9"/><use href="#rji-prog-clef" x="10.00" y="53.25"/><line x1="6.00" y1="30.00" x2="6.00" y2="61.00" stroke-width="1.1"/><text x="75.00" y="20.00" font-size="11" class="notation-chord">C</text><line x1="64.31" y1="68.75" x2="85.69" y2="68.75" stroke-width="1"/><use href="#rji-prog-whole" x="67.51" y="68.75"/><use href="#rji-prog-whole" x="67.51" y="61.00"/><use href="#rji-prog-whole" x="67.51" y="53.25"/><line x1="118.00" y1="30.00" x2="118.00" y2="61.00" stroke-width="1.1"/><text x="161.00" y="20.00" font-size="11" class="notation-chord">E</text><use href="#rji-prog-whole" x="153.51" y="61.00"/><use href="#rji-prog-sharp" x="143.76" y="53.25"/><use href="#rji-prog-whole" x="153.51" y="53.25"/><use href="#rji-prog-whole" x="153.51" y="45.50"/><line x1="204.00" y1="30.00" x2="204.00" y2="61.00" stroke-width="1.1"/><text x="247.00" y="20.00" font-size="11" class="notation-chord">Ab</text><use href="#rji-prog-flat" x="231.26" y="49.38"/><use href="#rji-prog-whole" x="239.51" y="49.38"/><use href="#rji-prog-whole" x="239.51" y="41.62"/><use href="#rji-prog-flat" x="231.26" y="33.88"/><use href="#rji-prog-whole" x="239.51" y="33.88"/><line x1="290.00" y1="30.00" x2="290.00" y2="61.00" stroke-width="1.1"/><text x="333.00" y="20.00" font-size="11" class="notation-chord">C</text><line x1="322.31" y1="68.75" x2="343.69" y2="68.75" stroke-width="1"/><use href="#rji-prog-whole" x="325.51" y="68.75"/><use href="#rji-prog-whole" x="325.51" y="61.00"/><use href="#rji-prog-whole" x="325.51" y="53.25"/><line x1="376.00" y1="30.00" x2="376.00" y2="61.00" stroke-width="1.1"/><line x1="6.00" y1="109.00" x2="376.00" y2="109.00" stroke-width="0.9"/><line x1="6.00" y1="116.75" x2="376.00" y2="116.75" stroke-width="0.9"/><line x1="6.00" y1="124.50" x2="376.00" y2="124.50" stroke-width="0.9"/><line x1="6.00" y1="132.25" x2="376.00" y2="132.25" stroke-width="0.9"/><line x1="6.00" y1="140.00" x2="376.00" y2="140.00" stroke-width="0.9"/><use href="#rji-prog-clef" x="10.00" y="132.25"/><line x1="6.00" y1="109.00" x2="6.00" y2="140.00" stroke-width="1.1"/><text x="75.00" y="99.00" font-size="11" class="notation-chord">F</text><use href="#rji-prog-whole" x="67.51" y="136.12"/><use href="#rji-prog-whole" x="67.51" y="128.38"/><use href="#rji-prog-whole" x="67.51" y="120.62"/><line x1="118.00" y1="109.00" x2="118.00" y2="140.00" stroke-width="1.1"/><text x="161.00" y="99.00" font-size="11" class="notation-chord">A</text><use href="#rji-prog-whole" x="153.51" y="128.38"/><use href="#rji-prog-sharp" x="143.76" y="120.62"/><use href="#rji-prog-whole" x="153.51" y="120.62"/><use href="#rji-prog-whole" x="153.51" y="112.88"/><line x1="204.00" y1="109.00" x2="204.00" y2="140.00" stroke-width="1.1"/><text x="247.00" y="99.00" font-size="11" class="notation-chord">D</text><use href="#rji-prog-whole" x="239.51" y="143.88"/><use href="#rji-prog-sharp" x="229.76" y="136.12"/><use href="#rji-prog-whole" x="239.51" y="136.12"/><use href="#rji-prog-whole" x="239.51" y="128.38"/><line x1="290.00" y1="109.00" x2="290.00" y2="140.00" stroke-width="1.1"/><text x="333.00" y="99.00" font-size="11" class="notation-chord">G7</text><line x1="322.31" y1="147.75" x2="343.69" y2="147.75" stroke-width="1"/><line x1="322.31" y1="155.50" x2="343.69" y2="155.50" stroke-width="1"/><use href="#rji-prog-whole" x="325.51" y="159.38"/><line x1="322.31" y1="147.75" x2="343.69" y2="147.75" stroke-width="1"/><use href="#rji-prog-whole" x="325.51" y="151.62"/><use href="#rji-prog-whole" x="325.51" y="143.88"/><use href="#rji-prog-whole" x="325.51" y="136.12"/><line x1="376.00" y1="109.00" x2="376.00" y2="140.00" stroke-width="1.1"/><line x1="6.00" y1="188.00" x2="376.00" y2="188.00" stroke-width="0.9"/><line x1="6.00" y1="195.75" x2="376.00" y2="195.75" stroke-width="0.9"/><line x1="6.00" y1="203.50" x2="376.00" y2="203.50" stroke-width="0.9"/><line x1="6.00" y1="211.25" x2="376.00" y2="211.25" stroke-width="0.9"/><line x1="6.00" y1="219.00" x2="376.00" y2="219.00" stroke-width="0.9"/><use href="#rji-prog-clef" x="10.00" y="211.25"/><line x1="6.00" y1="188.00" x2="6.00" y2="219.00" stroke-width="1.1"/><text x="75.00" y="178.00" font-size="11" class="notation-chord">C</text><line x1="64.31" y1="226.75" x2="85.69" y2="226.75" stroke-width="1"/><use href="#rji-prog-whole" x="67.51" y="226.75"/><use href="#rji-prog-whole" x="67.51" y="219.00"/><use href="#rji-prog-whole" x="67.51" y="211.25"/><line x1="118.00" y1="188.00" x2="118.00" y2="219.00" stroke-width="1.1"/><text x="161.00" y="178.00" font-size="11" class="notation-chord">E</text><use href="#rji-prog-whole" x="153.51" y="219.00"/><use href="#rji-prog-sharp" x="143.76" y="211.25"/><use href="#rji-prog-whole" x="153.51" y="211.25"/><use href="#rji-prog-whole" x="153.51" y="203.50"/><line x1="204.00" y1="188.00" x2="204.00" y2="219.00" stroke-width="1.1"/><text x="247.00" y="178.00" font-size="11" class="notation-chord">F</text><use href="#rji-prog-whole" x="239.51" y="215.12"/><use href="#rji-prog-whole" x="239.51" y="207.38"/><use href="#rji-prog-whole" x="239.51" y="199.62"/><line x1="290.00" y1="188.00" x2="290.00" y2="219.00" stroke-width="1.1"/><text x="333.00" y="178.00" font-size="11" class="notation-chord">C</text><line x1="322.31" y1="226.75" x2="343.69" y2="226.75" stroke-width="1"/><use href="#rji-prog-whole" x="325.51" y="226.75"/><use href="#rji-prog-whole" x="325.51" y="219.00"/><use href="#rji-prog-whole" x="325.51" y="211.25"/><line x1="371.00" y1="188.00" x2="371.00" y2="219.00" stroke-width="1.1"/><line x1="374.40" y1="188.00" x2="374.40" y2="219.00" stroke-width="3.2"/></g></svg></div>
</figure>

Every tuning is played three times: as sine waves, with a simple harmonic
timbre (the overtones make it easier to hear when something is out of tune), and
over a C drone (so you can hear how far each chord drifts from C).

The last row is a variant of the 12 pianos: every piano is still a just piano,
but its root is where 12TET puts it instead of where the C just scale does:

```text
frequency(root, degree) = C * 2^(root / 12) * J[degree]
```

<div class="oscilloscope-matrix">
  <table>
    <thead>
      <tr>
        <th scope="col">tuning system</th>
        <th scope="col">sine waves</th>
        <th scope="col">harmonic timbre</th>
        <th scope="col">over a C drone</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <th scope="row">12TET</th>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/twelve-tet-sine-progression.wav"></audio>
            <figcaption>One frequency per pitch; every third is 13.7 cents too wide.</figcaption>
          </figure>
        </td>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/twelve-tet-progression.wav"></audio>
            <figcaption>The same, with overtones.</figcaption>
          </figure>
        </td>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/twelve-tet-c-drone-progression.wav"></audio>
            <figcaption>12TET against a fixed C.</figcaption>
          </figure>
        </td>
      </tr>
      <tr>
        <th scope="row">fixed C just intonation</th>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/fixed-c-ji-sine-progression.wav"></audio>
            <figcaption>C major is exact; E major's third is 41 cents too wide.</figcaption>
          </figure>
        </td>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/fixed-c-ji-progression.wav"></audio>
            <figcaption>The overtones make the bad chords easier to pick out.</figcaption>
          </figure>
        </td>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/fixed-c-ji-c-drone-progression.wav"></audio>
            <figcaption>Every note agrees with the drone, whatever chord it's in.</figcaption>
          </figure>
        </td>
      </tr>
      <tr>
        <th scope="row">recursive just intonation</th>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/recursive-ji-sine-progression.wav"></audio>
            <figcaption>Every chord is exact, on a root from the C just scale.</figcaption>
          </figure>
        </td>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/recursive-ji-progression.wav"></audio>
            <figcaption>The same, with overtones.</figcaption>
          </figure>
        </td>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/recursive-ji-c-drone-progression.wav"></audio>
            <figcaption>You can hear when a chord's notes move away from the C scale.</figcaption>
          </figure>
        </td>
      </tr>
      <tr>
        <th scope="row">just chords on 12TET roots</th>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/twelve-tet-rooted-ji-sine-progression.wav"></audio>
            <figcaption>Every chord is exact, on a root a piano would play.</figcaption>
          </figure>
        </td>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/twelve-tet-rooted-ji-progression.wav"></audio>
            <figcaption>The same, with overtones.</figcaption>
          </figure>
        </td>
        <td>
          <figure class="audio-figure" data-oscilloscope>
            <audio controls src="/misc/media/twelve-tet-rooted-ji-c-drone-progression.wav"></audio>
            <figcaption>The roots follow the piano, not the drone, so most chords beat against it.</figcaption>
          </figure>
        </td>
      </tr>
    </tbody>
  </table>
</div>

Here's a stripped-down example: every line plays a note the way fixed C just
intonation tunes it, then the way recursive just intonation tunes it inside the
chord, then both at once so you can hear them beat:

<figure class="abc-figure">
  <div class="abc-notation" data-recursive-ji-abc="note-splits"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 554 404" class="engraved-notation" role="img" aria-label="Each split pitch written three times: fixed, recursive, and both together"><title>Each split pitch written three times: fixed, recursive, and both together</title><defs><path id="rji-split-clef" d="M9.69 -37.41c0.09 -0.09 0.24 -0.06 0.36 0c0.12 0.09 0.57 0.6 0.96 1.11c1.77 2.34 3.21 5.85 3.57 8.73c0.21 1.56 0.03 3.27 -0.45 4.86c-0.69 2.31 -1.92 4.47 -4.23 7.44c-0.3 0.39 -0.57 0.72 -0.6 0.75c-0.03 0.06 0 0.15 0.18 0.78c0.54 1.68 1.38 4.44 1.68 5.49l0.09 0.42l0.39 0c1.47 0.09 2.76 0.51 3.96 1.29c1.83 1.23 3.06 3.21 3.39 5.52c0.09 0.45 0.12 1.29 0.06 1.74c-0.09 1.02 -0.33 1.83 -0.75 2.73c-0.84 1.71 -2.28 3.06 -4.02 3.72l-0.33 0.12l0.03 1.26c0 1.74 -0.06 3.63 -0.21 4.62c-0.45 3.06 -2.19 5.49 -4.47 6.21c-0.57 0.18 -0.9 0.21 -1.59 0.21c-0.69 0 -1.02 -0.03 -1.65 -0.21c-1.14 -0.27 -2.13 -0.84 -2.94 -1.65c-0.99 -0.99 -1.56 -2.16 -1.71 -3.54c-0.09 -0.81 0.06 -1.53 0.45 -2.13c0.63 -0.99 1.83 -1.56 3 -1.53c1.5 0.09 2.64 1.32 2.73 2.94c0.06 1.47 -0.93 2.7 -2.37 2.97c-0.45 0.06 -0.84 0.03 -1.29 -0.09l-0.21 -0.09l0.09 0.12c0.39 0.54 0.78 0.93 1.32 1.26c1.35 0.87 3.06 1.02 4.35 0.36c1.44 -0.72 2.52 -2.28 2.97 -4.35c0.15 -0.66 0.24 -1.5 0.3 -3.03c0.03 -0.84 0.03 -2.94 0 -3c-0.03 0 -0.18 0 -0.36 0.03c-0.66 0.12 -0.99 0.12 -1.83 0.12c-1.05 0 -1.71 -0.06 -2.61 -0.3c-4.02 -0.99 -7.11 -4.35 -7.8 -8.46c-0.12 -0.66 -0.12 -0.99 -0.12 -1.83c0 -0.84 0 -1.14 0.15 -1.92c0.36 -2.28 1.41 -4.62 3.3 -7.29l2.79 -3.6c0.54 -0.66 0.96 -1.2 0.96 -1.23c0 -0.03 -0.09 -0.33 -0.18 -0.69c-0.96 -3.21 -1.41 -5.28 -1.59 -7.68c-0.12 -1.38 -0.15 -3.09 -0.06 -3.96c0.33 -2.67 1.38 -5.07 3.12 -7.08c0.36 -0.42 0.99 -1.05 1.17 -1.14zm2.01 4.71c-0.15 -0.3 -0.3 -0.54 -0.3 -0.54c-0.03 0 -0.18 0.09 -0.3 0.21c-2.4 1.74 -3.87 4.2 -4.26 7.11c-0.06 0.54 -0.06 1.41 -0.03 1.89c0.09 1.29 0.48 3.12 1.08 5.22c0.15 0.42 0.24 0.78 0.24 0.81c0 0.03 0.84 -1.11 1.23 -1.68c1.89 -2.73 2.88 -5.07 3.15 -7.53c0.09 -0.57 0.12 -1.74 0.06 -2.37c-0.09 -1.23 -0.27 -1.92 -0.87 -3.12zm-2.94 20.7c-0.21 -0.72 -0.39 -1.32 -0.42 -1.32c0 0 -1.2 1.47 -1.86 2.37c-2.79 3.63 -4.02 6.3 -4.35 9.3c-0.03 0.21 -0.03 0.69 -0.03 1.08c0 0.69 0 0.75 0.06 1.11c0.12 0.54 0.27 0.99 0.51 1.47c0.69 1.38 1.83 2.55 3.42 3.42c0.96 0.54 2.07 0.9 3.21 1.08c0.78 0.12 2.04 0.12 2.94 -0.03c0.51 -0.06 0.45 -0.03 0.42 -0.3c-0.24 -3.33 -0.72 -6.33 -1.62 -10.08c-0.09 -0.39 -0.18 -0.75 -0.18 -0.78c-0.03 -0.03 -0.42 0 -0.81 0.09c-0.9 0.18 -1.65 0.57 -2.22 1.14c-0.72 0.72 -1.08 1.65 -1.05 2.64c0.06 0.96 0.48 1.83 1.23 2.58c0.36 0.36 0.72 0.63 1.17 0.9c0.33 0.18 0.36 0.21 0.42 0.33c0.18 0.42 -0.18 0.9 -0.6 0.87c-0.18 -0.03 -0.84 -0.36 -1.26 -0.63c-0.78 -0.51 -1.38 -1.11 -1.86 -1.83c-1.77 -2.7 -0.99 -6.42 1.71 -8.19c0.3 -0.21 0.81 -0.48 1.17 -0.63c0.3 -0.09 1.02 -0.3 1.14 -0.3c0.06 0 0.09 0 0.09 -0.03c0.03 -0.03 -0.51 -1.92 -1.23 -4.26zm3.78 7.41c-0.18 -0.03 -0.36 -0.06 -0.39 -0.06c-0.03 0 0 0.21 0.18 1.02c0.75 3.18 1.26 6.3 1.5 9.09c0.06 0.72 0 0.69 0.51 0.42c0.78 -0.36 1.44 -0.96 1.98 -1.77c1.08 -1.62 1.2 -3.69 0.3 -5.55c-0.81 -1.62 -2.31 -2.79 -4.08 -3.15z"/><path id="rji-split-sharp" d="M5.73 -11.19c0.21 -0.12 0.54 -0.03 0.66 0.24c0.06 0.12 0.06 0.21 0.06 2.31c0 1.23 0 2.22 0.03 2.22c0 0 0.27 -0.12 0.6 -0.24c0.69 -0.27 0.78 -0.3 0.96 -0.15c0.21 0.15 0.21 0.18 0.21 1.38c0 1.02 0 1.11 -0.06 1.2c-0.03 0.06 -0.09 0.12 -0.12 0.15c-0.06 0.03 -0.42 0.21 -0.84 0.36l-0.75 0.33l-0.03 2.43c0 1.32 0 2.43 0.03 2.43c0 0 0.27 -0.12 0.6 -0.24c0.69 -0.27 0.78 -0.3 0.96 -0.15c0.21 0.15 0.21 0.18 0.21 1.38c0 1.02 0 1.11 -0.06 1.2c-0.03 0.06 -0.09 0.12 -0.12 0.15c-0.06 0.03 -0.42 0.21 -0.84 0.36l-0.75 0.33l-0.03 2.52c0 2.28 -0.03 2.55 -0.06 2.64c-0.21 0.36 -0.72 0.36 -0.93 0c-0.03 -0.09 -0.06 -0.33 -0.06 -2.43l0 -2.31l-1.29 0.51l-1.26 0.51l0 2.43c0 2.58 0 2.52 -0.15 2.67c-0.06 0.09 -0.27 0.18 -0.36 0.18c-0.12 0 -0.33 -0.09 -0.39 -0.18c-0.15 -0.15 -0.15 -0.09 -0.15 -2.43c0 -1.23 0 -2.22 -0.03 -2.22c0 0 -0.27 0.12 -0.6 0.24c-0.69 0.27 -0.78 0.3 -0.96 0.15c-0.21 -0.15 -0.21 -0.18 -0.21 -1.38c0 -1.02 0 -1.11 0.06 -1.2c0.03 -0.06 0.09 -0.12 0.12 -0.15c0.06 -0.03 0.42 -0.21 0.84 -0.36l0.78 -0.33l0 -2.43c0 -1.32 0 -2.43 -0.03 -2.43c0 0 -0.27 0.12 -0.6 0.24c-0.69 0.27 -0.78 0.3 -0.96 0.15c-0.21 -0.15 -0.21 -0.18 -0.21 -1.38c0 -1.02 0 -1.11 0.06 -1.2c0.03 -0.06 0.09 -0.12 0.12 -0.15c0.06 -0.03 0.42 -0.21 0.84 -0.36l0.78 -0.33l0 -2.52c0 -2.28 0.03 -2.55 0.06 -2.64c0.21 -0.36 0.72 -0.36 0.93 0c0.03 0.09 0.06 0.33 0.06 2.43l0.03 2.31l1.26 -0.51l1.26 -0.51l0 -2.43c0 -2.28 0 -2.43 0.06 -2.55c0.06 -0.12 0.12 -0.18 0.27 -0.24zm-0.33 10.65l0 -2.43l-1.29 0.51l-1.26 0.51l0 2.46l0 2.43l0.09 -0.03c0.06 -0.03 0.63 -0.27 1.29 -0.51l1.17 -0.48l0 -2.46z"/><path id="rji-split-quarter" d="M6.09 -4.05c0.36 -0.03 1.2 0 1.53 0.06c1.17 0.24 1.89 0.84 2.16 1.83c0.06 0.18 0.06 0.3 0.06 0.66c0 0.45 0 0.63 -0.15 1.08c-0.66 2.04 -3.06 3.93 -5.52 4.38c-0.54 0.09 -1.44 0.09 -1.83 0.03c-1.23 -0.27 -1.98 -0.87 -2.25 -1.86c-0.06 -0.18 -0.06 -0.3 -0.06 -0.66c0 -0.45 0 -0.63 0.15 -1.08c0.24 -0.78 0.75 -1.53 1.44 -2.22c1.2 -1.2 2.85 -2.01 4.47 -2.22z"/></defs><g fill="currentColor" stroke="currentColor" stroke-linecap="square"><line x1="6.00" y1="52.00" x2="548.00" y2="52.00" stroke-width="0.9"/><line x1="6.00" y1="59.75" x2="548.00" y2="59.75" stroke-width="0.9"/><line x1="6.00" y1="67.50" x2="548.00" y2="67.50" stroke-width="0.9"/><line x1="6.00" y1="75.25" x2="548.00" y2="75.25" stroke-width="0.9"/><line x1="6.00" y1="83.00" x2="548.00" y2="83.00" stroke-width="0.9"/><use href="#rji-split-clef" x="10.00" y="75.25"/><line x1="6.00" y1="52.00" x2="6.00" y2="83.00" stroke-width="1.1"/><text x="118.00" y="26.00" font-size="9.5" class="notation-annotation">E major G#/Ab</text><text x="118.00" y="38.00" font-size="9.5" class="notation-annotation">fixed +0.000c</text><use href="#rji-split-sharp" x="103.34" y="75.25"/><use href="#rji-split-quarter" x="113.09" y="75.25"/><line x1="122.41" y1="75.25" x2="122.41" y2="48.12" stroke-width="1.1"/><text x="290.00" y="26.00" font-size="9.5" class="notation-annotation">recursive</text><text x="290.00" y="38.00" font-size="9.5" class="notation-annotation">-41.059c</text><use href="#rji-split-sharp" x="275.35" y="75.25"/><use href="#rji-split-quarter" x="285.10" y="75.25"/><line x1="294.40" y1="75.25" x2="294.40" y2="48.12" stroke-width="1.1"/><text x="462.00" y="26.00" font-size="9.5" class="notation-annotation">together</text><text x="462.00" y="38.00" font-size="9.5" class="notation-annotation">+0.000c / -41.059c</text><use href="#rji-split-sharp" x="447.35" y="75.25"/><use href="#rji-split-quarter" x="457.10" y="75.25"/><line x1="466.40" y1="75.25" x2="466.40" y2="48.12" stroke-width="1.1"/><line x1="543.00" y1="52.00" x2="543.00" y2="83.00" stroke-width="1.1"/><line x1="546.40" y1="52.00" x2="546.40" y2="83.00" stroke-width="3.2"/><line x1="6.00" y1="153.00" x2="548.00" y2="153.00" stroke-width="0.9"/><line x1="6.00" y1="160.75" x2="548.00" y2="160.75" stroke-width="0.9"/><line x1="6.00" y1="168.50" x2="548.00" y2="168.50" stroke-width="0.9"/><line x1="6.00" y1="176.25" x2="548.00" y2="176.25" stroke-width="0.9"/><line x1="6.00" y1="184.00" x2="548.00" y2="184.00" stroke-width="0.9"/><use href="#rji-split-clef" x="10.00" y="176.25"/><line x1="6.00" y1="153.00" x2="6.00" y2="184.00" stroke-width="1.1"/><text x="118.00" y="127.00" font-size="9.5" class="notation-annotation">A major C#/Db</text><text x="118.00" y="139.00" font-size="9.5" class="notation-annotation">fixed +0.000c</text><use href="#rji-split-sharp" x="103.34" y="164.62"/><use href="#rji-split-quarter" x="113.09" y="164.62"/><line x1="113.59" y1="164.62" x2="113.59" y2="191.75" stroke-width="1.1"/><text x="290.00" y="127.00" font-size="9.5" class="notation-annotation">recursive</text><text x="290.00" y="139.00" font-size="9.5" class="notation-annotation">-41.059c</text><use href="#rji-split-sharp" x="275.35" y="164.62"/><use href="#rji-split-quarter" x="285.10" y="164.62"/><line x1="285.60" y1="164.62" x2="285.60" y2="191.75" stroke-width="1.1"/><text x="462.00" y="127.00" font-size="9.5" class="notation-annotation">together</text><text x="462.00" y="139.00" font-size="9.5" class="notation-annotation">+0.000c / -41.059c</text><use href="#rji-split-sharp" x="447.35" y="164.62"/><use href="#rji-split-quarter" x="457.10" y="164.62"/><line x1="457.60" y1="164.62" x2="457.60" y2="191.75" stroke-width="1.1"/><line x1="543.00" y1="153.00" x2="543.00" y2="184.00" stroke-width="1.1"/><line x1="546.40" y1="153.00" x2="546.40" y2="184.00" stroke-width="3.2"/><line x1="6.00" y1="254.00" x2="548.00" y2="254.00" stroke-width="0.9"/><line x1="6.00" y1="261.75" x2="548.00" y2="261.75" stroke-width="0.9"/><line x1="6.00" y1="269.50" x2="548.00" y2="269.50" stroke-width="0.9"/><line x1="6.00" y1="277.25" x2="548.00" y2="277.25" stroke-width="0.9"/><line x1="6.00" y1="285.00" x2="548.00" y2="285.00" stroke-width="0.9"/><use href="#rji-split-clef" x="10.00" y="277.25"/><line x1="6.00" y1="254.00" x2="6.00" y2="285.00" stroke-width="1.1"/><text x="118.00" y="228.00" font-size="9.5" class="notation-annotation">D major F#/Gb</text><text x="118.00" y="240.00" font-size="9.5" class="notation-annotation">fixed +0.000c</text><use href="#rji-split-sharp" x="103.34" y="281.12"/><use href="#rji-split-quarter" x="113.09" y="281.12"/><line x1="122.41" y1="281.12" x2="122.41" y2="254.00" stroke-width="1.1"/><text x="290.00" y="228.00" font-size="9.5" class="notation-annotation">recursive</text><text x="290.00" y="240.00" font-size="9.5" class="notation-annotation">-19.553c</text><use href="#rji-split-sharp" x="275.35" y="281.12"/><use href="#rji-split-quarter" x="285.10" y="281.12"/><line x1="294.40" y1="281.12" x2="294.40" y2="254.00" stroke-width="1.1"/><text x="462.00" y="228.00" font-size="9.5" class="notation-annotation">together</text><text x="462.00" y="240.00" font-size="9.5" class="notation-annotation">+0.000c / -19.553c</text><use href="#rji-split-sharp" x="447.35" y="281.12"/><use href="#rji-split-quarter" x="457.10" y="281.12"/><line x1="466.40" y1="281.12" x2="466.40" y2="254.00" stroke-width="1.1"/><line x1="543.00" y1="254.00" x2="543.00" y2="285.00" stroke-width="1.1"/><line x1="546.40" y1="254.00" x2="546.40" y2="285.00" stroke-width="3.2"/><line x1="6.00" y1="355.00" x2="548.00" y2="355.00" stroke-width="0.9"/><line x1="6.00" y1="362.75" x2="548.00" y2="362.75" stroke-width="0.9"/><line x1="6.00" y1="370.50" x2="548.00" y2="370.50" stroke-width="0.9"/><line x1="6.00" y1="378.25" x2="548.00" y2="378.25" stroke-width="0.9"/><line x1="6.00" y1="386.00" x2="548.00" y2="386.00" stroke-width="0.9"/><use href="#rji-split-clef" x="10.00" y="378.25"/><line x1="6.00" y1="355.00" x2="6.00" y2="386.00" stroke-width="1.1"/><text x="118.00" y="329.00" font-size="9.5" class="notation-annotation">D major A</text><text x="118.00" y="341.00" font-size="9.5" class="notation-annotation">fixed +0.000c</text><use href="#rji-split-quarter" x="113.09" y="374.38"/><line x1="122.41" y1="374.38" x2="122.41" y2="347.25" stroke-width="1.1"/><text x="290.00" y="329.00" font-size="9.5" class="notation-annotation">recursive</text><text x="290.00" y="341.00" font-size="9.5" class="notation-annotation">+21.506c</text><use href="#rji-split-quarter" x="285.10" y="374.38"/><line x1="294.40" y1="374.38" x2="294.40" y2="347.25" stroke-width="1.1"/><text x="462.00" y="329.00" font-size="9.5" class="notation-annotation">together</text><text x="462.00" y="341.00" font-size="9.5" class="notation-annotation">+0.000c / +21.506c</text><use href="#rji-split-quarter" x="457.10" y="374.38"/><line x1="466.40" y1="374.38" x2="466.40" y2="347.25" stroke-width="1.1"/><line x1="543.00" y1="355.00" x2="543.00" y2="386.00" stroke-width="1.1"/><line x1="546.40" y1="355.00" x2="546.40" y2="386.00" stroke-width="3.2"/></g></svg></div>
</figure>

<figure class="audio-figure" data-oscilloscope>
  <audio controls src="/misc/media/recursive-ji-note-splits.wav"></audio>
  <figcaption>Same note name, different chord.</figcaption>
</figure>

| chord context | note  |   fixed C JI | recursive JI |      difference |
| ------------- | ----- | -----------: | -----------: | --------------: |
| E major       | G#/Ab | `209.300 Hz` | `204.395 Hz` | `-41.059 cents` |
| A major       | C#/Db | `279.067 Hz` | `272.527 Hz` | `-41.059 cents` |
| D major       | F#/Gb | `186.045 Hz` | `183.955 Hz` | `-19.553 cents` |
| D major       | A     | `218.021 Hz` | `220.747 Hz` | `+21.506 cents` |

The first two are the same 41.1 cents as E major's third above. D major moves
two notes: its F# by `2048/2025` (19.6 cents) and its A by the syntonic comma
(`81/80`, 21.5 cents).

more audio examples:

<figure class="audio-figure" data-oscilloscope>
  <audio controls src="/misc/media/mozart-dies-irae-recursive-just-intonation-piano.wav"></audio>
  <figcaption>mozarts dies irae.</figcaption>
</figure>

<figure class="audio-figure" data-oscilloscope>
  <audio controls src="/misc/media/recursive-just-intonation-composition.wav"></audio>
  <figcaption>some composition I came up with for this blog post.</figcaption>
</figure>

## Why Recursive Just Intonation is good

Every major chord is an exact `4:5:6`, whatever its root. E major doesn't take
C's Ab, it gets its own G#. D major doesn't take C's A, it gets its own.

That's also how I hear harmony: when a chord arrives, its root becomes the
local center, and recursive just intonation tunes to that center instead of to
one keyboard for the whole piece.

It's also really easy to program. A chord is two table lookups:

```text
root_frequency = base_frequency * J[root]
note_frequency = root_frequency * J[degree]
```

## Why Recursive Just Intonation is bad

The same note name can move. In 12TET, G# is one frequency per octave. In fixed
C just intonation it's also one frequency, just a different one. In recursive
just intonation it depends on which chord you're in:

- A held note can have to move when the chord changes. In the progression
  above, E major's G# and the next chord's Ab are the same key on a piano, and
  41 cents apart here.
- G# and Ab are different frequencies now, but a keyboard only has one key for
  both.
- Keeping every chord pure and keeping held notes still pull in opposite
  directions.
- Instruments with frets, keys or holes can't do this without pitch bending or
  several samples per note.

So this isn't going to replace 12TET. 12TET is the compromise that lets every
key share one instrument.

## Practical Uses

One day I will make a keyboard on which with your left hand you can determine
the current key/context and with your right hand you play notes that are
dynamically retuned according to the table, until then the practical
applications remain few.

In the meantime, the [tuning playground](/tools/tuningplayground.md) has
recursive just intonation as one of its scales, and I later realized this is
just one instance of a more general idea, which I wrote about in
[Recursive Tuning](/blog/recursive-tuning.md).

## My other music related work

blog posts:

- [Recursive Tuning](/blog/recursive-tuning.md): the generalization of this post,
  pair any two tuning systems and listen to the matrix
- [a Breadboard for your Fretboard](/blog/a-breadboard-for-your-breadboard.md):
  learning electronics by building a guitar pedal

tools:

- [Play around with different tuning systems and your computer keyboard](/tools/tuningplayground.md)
- [Visualize and listen to polyrhythms](/tools/polyrhythm.md)
- [AudioLink](/audiolink): an audio reactive example shader running in the browser

projects:

- [music21-rs](https://hilll.dev/music21-rs/): a rust music theory library
  inspired by python's music21, which the tuning playground and this post are
  built on ([github](https://github.com/float3/music21-rs),
  [crates.io](https://crates.io/crates/music21-rs))
- [my pull requests to music21](https://github.com/cuthbertLab/music21/pulls?q=author%3Afloat3)
- [AudioLink](https://audiolink.dev): I'm a maintainer on the core team of
  AudioLink, an audio-visualization library for Unity
  ([github](https://github.com/llealloo/audiolink),
  [demo](https://traeumerei.dev))
- [an audio reactive screen-space shader using AudioLink](https://github.com/float3/ShaderArchive/blob/master/Misc/AudioLinkScreenSpaceNaNMarching.shader)
- [Music21TS](https://github.com/float3/Music21TS) and
  [Music21.NET](https://github.com/float3/Music21.NET): my earlier, archived
  attempts at porting music21 to TypeScript and C#

### Visualize and listen to Polyrhythms in a Shader

<iframe width="640" height="360" frameborder="0" allowfullscreen="allowfullscreen" src="https://www.shadertoy.com/embed/7tV3WV?gui=true&t=10&paused=false&muted=false"></iframe>
