---
title: glsl2hlsl
tags:
  - shaders
  - wasm
  - tools
  - graphics
  - unity
  - programming
  - rust
---

<link href="./glsl2hlsl.css" rel="stylesheet" type="text/css">

<div class="tool glsl">
<p class="wasm-credit">made with rust compiled to wasm</p>
<p class="tool-description">Turns a Shadertoy shader into a Unity Shaderlab shader. Written by <a href="https://github.com/pema99/glsl2hlsl/">pema99</a>, with changes merged from <a href="https://github.com/cutesthypnotist/glsl2hlsl/">Kit</a> and <a href="https://github.com/antiero/glsl2hlsl/">antiero</a> and my own; MIT licensed.</p>
<noscript>This page contains webassembly and javascript content, please enable javascript in your browser.</noscript>
<div class="tool-bar">
<label class="tool-field tool-field-wide"><span>Shadertoy URL or ID</span><input id="shader" placeholder="https://www.shadertoy.com/view/…" spellcheck="false" /></label>
<div class="tool-actions"><button id="download" type="button">Fetch and download</button></div>
</div>
<div class="tool-panes">
<label class="tool-field"><span>Shadertoy (GLSL)</span><textarea id="in" spellcheck="false" rows="18"></textarea></label>
<label class="tool-field"><span>Shaderlab (Unity)</span><textarea id="out" spellcheck="false" rows="18"></textarea></label>
</div>
<div class="tool-bar">
<label class="tool-check"><input type="checkbox" id="extract" /> Extract properties</label>
<label class="tool-check"><input type="checkbox" id="raymarch" /> Raymarched</label>
<label class="tool-check"><input type="checkbox" id="preview-bg" /> Render as this page's background</label>
<span class="tool-hint">the first two are experimental and may break</span>
<div class="tool-actions glsl-convert"><button id="convert" type="button" class="is-primary">Convert</button></div>
</div>
<p id="preview-status" class="tool-status glsl-preview-status" role="status" hidden></p>
<div id="links" class="glsl-links"></div>
</div>

<script src="/js/glsl2hlsl.js"></script>
