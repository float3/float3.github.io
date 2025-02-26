---
title: glsl2hlsl
date: 2024-12-12
updated: 2025-02-19
tags:
  - shaders
  - wasm
  - tools
  - graphics
  - unity
  - programming
  - rust
---

made with love and rust (compiled to wasm)

written by pema99 https://github.com/pema99/glsl2hlsl/ \
merged changes by Kit https://github.com/cutesthypnotist/glsl2hlsl/ \
and by antiero https://github.com/antiero/glsl2hlsl/ \
and my own improvements \
glsl2hlsl is MIT Licensed
<noscript>This page contains webassembly and javascript content, please enable javascript in your browser.</noscript>

<div class="areas">
  <table>
    <tr>
      <th><h3>Shadertoy code:</h3></th>
      <th><h3>Shaderlab (Unity) code:</h3></th>
    </tr>
    <tr>
      <td><textarea id="in"></textarea></td>
      <td><textarea id="out"></textarea></td>
    </tr>
    <tr>
      <td>
        <h3>Experimental Features, might break:</h3>
        <input type="checkbox" id="extract" style="margin-left:5%;">Extract properties
        <input type="checkbox" id="raymarch" style="margin-left:5%;">Raymarched
      </td>
      <td><input id="convert" type="button" value="Convert"></td>
    </tr>
  </table>
</div>
<br>
<br>
<br>
<div id="links"></div>
<br>
<br>
<input id="download" type="button" value="Download from URL or ID">
<input id="shader" style="width:100%">
<br>
<br>
<br>
<link href="./glsl2hlsl.css" rel="stylesheet" type="text/css">
<script src="/js/glsl2hlsl.js"></script>
