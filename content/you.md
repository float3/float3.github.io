---
title: you
tags:
  - tools
---

<div class="browser-report">
  <div class="wrap">
    <h1>Browser Information Report</h1>
    <p class="sub">
      This page collects browser-exposed information only. Some values are hidden by browser privacy protections.
      Public IP requires a network request to an external endpoint.
    </p>
    <div class="toolbar">
      <button id="refreshBtn">Refresh</button>
      <button id="expandBtn">Expand all</button>
      <button id="collapseBtn">Collapse all</button>
      <button id="copyBtn">Copy JSON</button>
    </div>
    <div class="summary-grid" id="summary"></div>
    <div id="sections"></div>
    <div class="footer">
    </div>
  </div>
  <canvas id="fingerprintCanvas" width="480" height="180"></canvas>
</div>

<link href="./you.css" rel="stylesheet" type="text/css">
<script type="module" src="./you.js"></script>
