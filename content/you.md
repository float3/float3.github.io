---
title: you
tags:
  - tools
---

<div class="browser-report">
  <div class="wrap">
    <p class="sub">
      what your browser exposes to javascript. your public ip comes from ipify, or ip.sb if that fails; nothing else leaves the page.
    </p>
    <div class="toolbar">
      <button id="refreshBtn">refresh</button>
      <button id="expandBtn">expand all</button>
      <button id="collapseBtn">collapse all</button>
      <button id="copyBtn">copy JSON</button>
    </div>
    <div class="summary-grid" id="summary"></div>
    <div id="sections"></div>
  </div>
  <canvas id="fingerprintCanvas" width="480" height="180"></canvas>
</div>

<link href="./you.css" rel="stylesheet" type="text/css">
<script type="module" src="/js/you.js"></script>
