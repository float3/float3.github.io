---
title: you
tags:
  - tools
---

<div class="browser-report">
  <div class="wrap">
    <section class="who" aria-live="polite">
      <p class="who-badge" id="whoBadge">reading</p>
      <h2>who sites think you are</h2>
      <div id="who"></div>
    </section>
    <p class="sub">
      what your browser exposes to javascript. your public ip, and who owns it, come from ipify and ip.sb; nothing else leaves the page. hover a highlighted word to see where it came from.
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
