---
title: tuningplayground
tags:
  - music
  - wasm
  - tools
  - rust
---

<link href="./tuningplayground.css" rel="stylesheet" type="text/css">

<div id="tuningPlayground" class="tool tp">
<p class="wasm-credit">made with rust compiled to wasm</p>
<noscript>hey this page needs javascript</noscript>
<div id="tuningPlaygroundStatus" class="tool-status" role="status" data-state="loading">Loading tuning playground…</div>
<div class="tp-library">
<div class="tool-bar">
<label class="tool-field tool-field-wide"><span>Tuning</span><input id="tuningSearch" type="search" placeholder="Search tuning systems, temperaments, equal divisions and the Scala archive" autocomplete="off" spellcheck="false" /></label>
<button id="browseToggle" type="button" aria-expanded="false" aria-controls="libraryPanel">Browse</button>
</div>
<div id="libraryPanel" class="tp-panel" hidden>
<div id="libraryChips" class="tp-chips" role="group" aria-label="Kind of tuning">
<button type="button" class="is-active" data-kind="all">All</button>
<button type="button" data-kind="system">Systems <small></small></button>
<button type="button" data-kind="temperament">Temperaments <small></small></button>
<button type="button" data-kind="equal">Equal <small></small></button>
<button type="button" data-kind="scala">Scala <small></small></button>
<button type="button" data-kind="mine">Yours</button>
</div>
<div id="libraryList" class="tp-list"></div>
</div>
</div>
<div class="tp-head">
<p class="tp-title" id="tuningTitle"></p>
<p class="tp-facts" id="tuningFacts"></p>
<div class="tool-actions tp-sizes" id="tuningSizes"></div>
<p id="tuningDescription" class="tool-description"></p>
</div>
<div id="output" class="chord-staff" aria-label="notation of the held chord"></div>
<div id="keyboard" class="tp-keyboard" role="group" aria-label="Playable keyboard"></div>
<p class="tool-hint">click or tap the keys, or play them from the computer keyboard; shift-click marks a key so a chord can be shared.</p>
<div class="tool-bar" role="group" aria-label="Sound">
<label class="tool-field"><span>Engine</span><select id="soundMethod" name="soundMethod"><option value="sample">Sample</option><option value="native">Synth</option></select></label>
<label class="tool-field"><span>Waveform</span><select id="waveform" name="waveform"><option value="sine">Sine</option><option value="triangle">Triangle</option><option value="square">Square</option><option value="sawtooth">Saw</option></select></label>
<label class="tool-field"><span>Volume</span><input type="range" id="volumeSlider" min="0" max="1" step="0.01" value="0.4" /></label>
<label class="tool-field"><span>Octave</span><input type="number" id="octave" value="0" min="-4" max="4" step="1" /></label>
<div class="tool-actions"><button id="playScale" type="button" class="is-primary">Play scale</button><button id="stopAll" type="button">Stop</button><button id="shareLink" type="button">Copy link</button></div>
</div>
<details class="tool-more" id="degreesDetails">
<summary>Degrees</summary>
<div class="tp-table-wrap"><table class="tp-degrees"><thead><tr><th>Degree</th><th>Name</th><th>Ratio</th><th>Hz</th><th>From equal</th></tr></thead><tbody id="degreesBody"></tbody></table></div>
</details>
<details class="tool-more">
<summary>More controls</summary>
<div class="tool-more-grid">
<label class="tool-field"><span>Computer keyboard</span><select id="keymapSelect" name="keymapSelect"><option value="auto">Auto</option><option value="us">Piano, US QWERTY</option><option value="us-extended">Piano, US QWERTY extended</option><option value="qwertz">Piano, QWERTZ</option><option value="azerty">Piano, AZERTY</option><option value="rows">Rows of degrees</option><option value="periods">A period per row</option></select></label>
<div class="tool-field"><span>MIDI device</span><div class="tool-actions"><button id="midiButton" type="button">Connect a MIDI device</button></div><span id="midiStatus" class="tool-hint">plays through the tuning; the browser asks once.</span></div>
<div class="tool-field"><span>MIDI file</span><div class="tool-actions"><input type="file" id="fileInput" accept=".midi,.mid" /><button id="playButton" type="button">Play</button><button id="stopButton" type="button">Stop</button></div><span class="tool-hint">example file: <a href="/misc/blobs/jm_mozdi.mid" download="mozart_dies_irae.mid">Mozart, Dies Irae</a>.</span></div>
<div class="tool-field"><span>Name a chord</span><div class="tool-actions"><input id="chordInput" placeholder="C E G or C Eb G Bb" /><button id="nameChord" type="button">Name</button><button id="clearChord" type="button">Clear</button></div><div id="chordNameOutput" class="tp-chord-name"></div><div id="chordDetailsOutput" class="tool-hint"></div></div>
</div>
</details>
<div id="markedButtons" class="tool-actions" hidden><button id="playMarked" type="button">Play marked notes</button><button id="shareMarked" type="button">Share marked notes</button></div>
<div id="logContainer" class="tool-log" aria-live="polite"></div>
</div>

<script src="/js/tuningplayground.js"></script>
