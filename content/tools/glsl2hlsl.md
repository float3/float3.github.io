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

written by pema99 https://github.com/pema99/glsl2hlsl/ \
merged changes by Kit https://github.com/cutesthypnotist/glsl2hlsl/ \
and by antiero https://github.com/antiero/glsl2hlsl/ \
and my own improvements
<noscript>This page contains webassembly and javascript content, please enable javascript in your browser.</noscript>
<div class="areas">
  <table>
    <tr>
      <th><h2>Shadertoy code:</h2></th>
      <th><h2>Shaderlab (Unity) code:</h2></th>
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
<script src="./glsl2hlsl/bootstrap.js"></script>
