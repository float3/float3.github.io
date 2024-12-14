---
title: glsl2hlsl
date: 2024-12-09
tags:
  - shaders
  - wasm
  - tools
  - graphics
  - unity
  - programming
---

made with love and rust (compiled to wasm)

written by pema99 https://github.com/pema99/glsl2hlsl/
<noscript>This page contains webassembly and javascript content, please enable javascript in your browser.</noscript>

<div class="areas">
  <h2>Shadertoy code:</h2>
  <h2>Shaderlab (Unity) code:</h2>
  <textarea id="in"></textarea>
  <textarea id="out"></textarea>
</div>
<input id="convert" type="button" value="Convert" />
<input type="checkbox" id="extract" style="margin-left:10%;"></input>Extract properties (Super experimental, might break) <input type="checkbox" id="raymarch" style="margin-left:10%;"></input>Raymarched (Super experimental, might break) <br>
<br>
<div id="links"></div>
<br>
<br>
<input id="download" type="button" value="Download from URL or ID">
<input id="shader" value="https://www.shadertoy.com/view/ld3Gz2" style="width:100%">
<br>
<br>
<br>
<link href="./glsl2hlsl.css" rel="stylesheet" type="text/css">
<script src="./glsl2hlsl/bootstrap.js"></script>
