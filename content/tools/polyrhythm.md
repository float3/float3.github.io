---
title: polyrhythm
tags:
  - wasm
  - tools
  - rust
  - music
---

<link href="./polyrhythm.css" rel="stylesheet" type="text/css">

<div class="tool polyrhythm">
<p class="wasm-credit">made with rust compiled to wasm</p>
<div id="controls" class="tool-bar" role="group" aria-label="Polyrhythm settings">
<label class="tool-field"><span>Time signature base</span><input type="number" id="base" value="4" min="1" max="16" inputmode="numeric" /></label>
<label class="tool-field"><span>Tempo</span><input type="number" id="tempo" value="120" min="20" max="280" inputmode="numeric" /></label>
<label class="tool-field"><span>Subdivisions</span><input type="text" id="subdivisions" value="3:4" inputmode="numeric" /></label>
<label class="tool-field"><span>Pitch</span><input type="number" id="pitch" value="440" min="80" max="1400" inputmode="numeric" /></label>
<div class="tool-actions"><button type="button" id="start-button" class="is-primary">Start</button><button type="button" id="stop-button">Stop</button></div>
</div>
<div class="tool-actions polyrhythm-presets" aria-label="Preset subdivisions"><span class="tool-hint">presets</span><button type="button" data-polyrhythm-preset="2:3">2:3</button><button type="button" data-polyrhythm-preset="3:4">3:4</button><button type="button" data-polyrhythm-preset="4:5">4:5</button><button type="button" data-polyrhythm-preset="3:5:7">3:5:7</button></div>
<p id="polyrhythm-status" class="tool-status" role="status"></p>
<div class="tool-panel polyrhythm-canvases"><canvas id="canvas0" width="800" height="150"></canvas><canvas id="canvas1" width="800" height="600"></canvas><canvas id="canvas2" width="800" height="220"></canvas></div>
</div>

<script type="module" src="/js/polyrhythm.js"></script>
