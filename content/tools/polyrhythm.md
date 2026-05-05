---
title: polyrhythm
date: 2025-02-19
updated: 2026-05-04
tags:
  - wasm
  - tools
  - rust
  - music
---

<link href="./polyrhythm.css" rel="stylesheet" type="text/css">

<p class="wasm-credit">made with love and rust (compiled to wasm)</p>

<section class="polyrhythm-tool" aria-labelledby="polyrhythm-heading">
  <h2 id="polyrhythm-heading">polyrhythm</h2>
  <div id="controls" class="polyrhythm-controls">
    <label class="control" for="base">
      <span>Time signature base</span>
      <input type="number" id="base" value="4" min="1" max="16" inputmode="numeric">
    </label>
    <label class="control" for="tempo">
      <span>Tempo</span>
      <input type="number" id="tempo" value="120" min="20" max="280" inputmode="numeric">
    </label>
    <label class="control" for="subdivisions">
      <span>Subdivisions</span>
      <input type="text" id="subdivisions" value="3:4" inputmode="numeric">
    </label>
    <label class="control" for="pitch">
      <span>Pitch</span>
      <input type="number" id="pitch" value="440" min="80" max="1400" inputmode="numeric">
    </label>
    <div class="polyrhythm-presets" aria-label="Preset subdivisions">
      <button type="button" data-polyrhythm-preset="2:3">2:3</button>
      <button type="button" data-polyrhythm-preset="3:4">3:4</button>
      <button type="button" data-polyrhythm-preset="4:5">4:5</button>
      <button type="button" data-polyrhythm-preset="3:5:7">3:5:7</button>
    </div>
    <div class="buttons">
      <button type="button" id="start-button">Start</button>
      <button type="button" id="stop-button">Stop</button>
    </div>
    <p id="polyrhythm-status" class="polyrhythm-status" role="status"></p>
  </div>
  <div class="polyrhythm-canvases">
    <canvas id="canvas0" width="800" height="150"></canvas>
    <canvas id="canvas1" width="800" height="600"></canvas>
    <canvas id="canvas2" width="800" height="220"></canvas>
  </div>
</section>

<script type="module" src="/js/polyrhythm.js"></script>
