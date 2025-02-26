---
title: polyrhythm
date: 2025-02-19
updated: 2025-02-19
tags:
  - wasm
  - tools
  - rust
  - music
---

<body>
<div id="controls">
  <div class="control">
    <label for="base">Time Signature (Base):</label>
    <input type="number" id="base" value="4">
  </div>
  <div class="control">
    <label for="tempo">Tempo (BPM):</label>
    <input type="number" id="tempo" value="120">
  </div>
  <div class="control">
    <label for="subdivisions">Subdivisions (colon separated):</label>
    <input type="text" id="subdivisions" value="3:4">
  </div>
  <div class="control">
    <label for="pitch">Pitch (Hz):</label>
    <input type="number" id="pitch" value="440">
  </div>
  <div class="buttons">
    <button id="start-button">Start</button>
    <button id="stop-button">Stop</button>
   </div>
</div>
  <canvas id="canvas1" style="color: black" width="800" height="600" ></canvas>
  <canvas id="canvas0" style="color: black" width="800" height="200" ></canvas>
  <canvas id="canvas2" style="color: black" width="800" height="300" ></canvas>
  <canvas id="canvas3" style="color: black" width="800" height="600" ></canvas>
  <script type="module" src="/js/polyrhythm.js"></script>
  <link href="./polyrhythm.css" rel="stylesheet" type="text/css">
</body>
