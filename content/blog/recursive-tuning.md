---
title: Recursive Tuning
tags:
  - music
  - programming
  - tools
  - wasm
  - rust
---

<link href="./recursive-tuning.css" rel="stylesheet" type="text/css">

I talked about [recursive just intonation](https://hilll.dev/blog/recursive-just-intonation/) in the past, but I now realize that that's just a specific instance of a more general type of adaptive tuningsystem I've discovered which I call "Recursive Tuning Systems" or just "Recursive Tuning".

in this post you'll be able to make a matrix of any two tuningsystems that are supported by my music21-rs library and see how they interact with each other and how they sound.

## From One Table To Two

Recursive just intonation uses one table twice. The five-limit table picks the
root of a chord, and the same table tunes every note above that root:

```text
recursive_frequency(root, degree) =
    C_frequency * J[root] * J[degree]
```

Nothing in that formula needs the two lookups to use the same table, or needs
either table to be just intonation:

```text
frequency(root, degree) =
    root_frequency * G[root] * L[degree]
```

`G` is the **global** table. It places the roots, the way the C-based just
keyboard did. `L` is the **local** table. It tunes every note above a root. The
matrix below has one row per degree of `G` and one column per degree of `L`, so
it is the "12 pianos" table from the other post with any number of pianos of
any size.

Three pairs are already familiar:

- `G` and `L` both five-limit is recursive just intonation.
- `G` 12-TET and `L` five-limit is the hybrid at the end of the other post:
  piano roots with just chords built on them.
- `G` and `L` both 12-TET is plain 12-TET. A step of an equal division above
  another step is the step of their sum, `2^(a/12) * 2^(b/12) = 2^((a+b)/12)`,
  so recursing it moves nothing.

That last property belongs to equal divisions and to nothing else. If
`T[a] * T[b] = T[a+b]` for every pair of degrees, then `T[n] = T[1]^n`, which is
an equal division. So recursion changes every tuning except the equal ones.

## Scales Of Different Sizes

The global and the local scale do not need the same number of notes or the
same period. Every row is as long as the local scale and repeats at the local
scale's period, and there is one row for each degree of the global scale. A
7-note diatonic scale on 12-TET roots is 12 pianos with 7 keys each.
Bohlen-Pierce on five-limit roots is 12 pianos that repeat at `3/1` instead of
the octave.

When the sizes differ, a column is no longer the same note name in every row.
The matrix then compares each note with the nearest note of the global scale,
and the colours show the nearest 12-TET note.

## Try It

<div id="recursiveTuning" class="tool rt">
<p class="wasm-credit">made with rust compiled to wasm</p>
<noscript>hey this page needs javascript</noscript>
<div id="recursiveTuningStatus" class="tool-status" role="status" data-state="loading">Loading the recursive tuning explorer…</div>
<div class="rt-pickers">
<div id="rtGlobal" class="rt-picker">
<div class="rt-search">
<label class="tool-field tool-field-wide"><span>Global scale: places the roots</span><input type="search" placeholder="Search tunings, temperaments, equal divisions and Scala" autocomplete="off" spellcheck="false" aria-expanded="false" /></label>
<div class="rt-results" hidden></div>
</div>
<p class="rt-chosen"></p>
</div>
<div id="rtLocal" class="rt-picker">
<div class="rt-search">
<label class="tool-field tool-field-wide"><span>Local scale: tunes above each root</span><input type="search" placeholder="Search tunings, temperaments, equal divisions and Scala" autocomplete="off" spellcheck="false" aria-expanded="false" /></label>
<div class="rt-results" hidden></div>
</div>
<p class="rt-chosen"></p>
</div>
</div>
<div class="tool-bar">
<label class="tool-check"><input type="checkbox" id="rtLink" checked /> the same scale in both places</label>
<label class="tool-field"><span>Root (Hz)</span><input type="number" id="rtRoot" value="261.6255653005986" min="20" max="2000" step="any" /></label>
<label class="tool-field"><span>Octave</span><input type="number" id="rtOctave" value="0" min="-3" max="3" step="1" /></label>
<label class="tool-field"><span>Timbre</span><select id="rtTimbre"><option value="harmonic">Harmonics</option><option value="sine">Sine</option></select></label>
<label class="tool-field"><span>Volume</span><input type="range" id="rtVolume" min="0" max="1" step="0.01" value="0.5" /></label>
<div class="tool-actions"><button id="rtShare" type="button">Copy link</button></div>
</div>
<div id="rtPairs" class="rt-pairs" role="group" aria-label="Pairs to try">
<button type="button" data-global="FiveLimit" data-local="FiveLimit">recursive just intonation</button>
<button type="button" data-global="EqualTemperament" data-local="FiveLimit">12-TET roots, just chords</button>
<button type="button" data-global="FiveLimit" data-local="EqualTemperament">just roots, 12-TET chords</button>
<button type="button" data-global="PythagoreanTuning" data-local="PythagoreanTuning">Pythagorean, recursed</button>
<button type="button" data-global="CarlosHarmonic" data-local="CarlosHarmonic">the harmonic series, recursed</button>
<button type="button" data-global="QuarterCommaMeantone" data-local="FiveLimit">meantone roots, just chords</button>
<button type="button" data-global="EqualTemperament" data-local="PtolemyIntenseDiatonic">7 just notes on 12 roots</button>
<button type="button" data-global="FiveLimit" data-local="equal:13:3:1">Bohlen-Pierce on just roots</button>
<button type="button" data-global="EqualTemperament" data-local="EqualTemperament">12-TET, recursed</button>
</div>
<p id="rtSummary" class="tool-description"></p>
<h3>Play it</h3>
<div id="rtRoots" class="rt-roots" role="group" aria-label="Roots from the global scale"></div>
<div id="rtNotes" class="rt-notes" role="group" aria-label="The local scale over the chosen root"></div>
<div class="tool-actions"><button id="rtChord" type="button" class="is-primary">Play a major chord on this root</button></div>
<h3>A progression</h3>
<div class="tool-bar">
<div class="tool-actions"><button type="button" data-render="recursive" class="is-primary">Recursive</button><button type="button" data-render="fixed">Global scale alone</button><button type="button" data-render="equal">12-TET</button><button id="rtStop" type="button">Stop</button></div>
<label class="tool-check"><input type="checkbox" id="rtDrone" /> hold the root underneath</label>
</div>
<div id="rtProgression" class="rt-progression"></div>
<h3>The matrix</h3>
<p class="tool-hint">a row per root of the global scale, a column per step of the local scale. The small number in a cell is how far that note is from the global scale's own pitch for it. Click a cell to hear it.</p>
<div id="rtMatrix" class="rt-matrix-wrap"></div>
</div>

<script src="/js/recursivetuning.js"></script>

How to play it:

- The digit row, `1` to `=`, picks one of the first twelve roots of the global
  scale. Clicking a root, or the head of a row in the matrix, picks it too.
- The three letter rows play the local scale over the chosen root, starting at
  `z` and going up one degree per key.
- Notes you are holding move to the new root when you pick another one. That is
  the keyboard from the end of the recursive JI post: one hand picks the
  context, the other plays notes that are retuned to it.
- The progression is the one from the recursive JI post. Under each chord is
  every note that sounds different from the global scale alone, and by how many
  cents. In a scale that is not 12-tone, each root and chord note is the degree
  nearest the 12-TET one.
