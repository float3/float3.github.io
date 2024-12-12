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
  <textarea id="in">
// http://www.pouet.net/prod.php?which=57245
// If you intend to reuse this shader, please add credits to 'Danilo Guanabara'
#define t iTime
#define r iResolution.xy
void mainImage( out vec4 fragColor, in vec2 fragCoord ){
	vec3 c;
	float l,z=t;
	for(int i=0;i
		<3;i++) {
		vec2 uv,p=fragCoord.xy/r;
		uv=p;
		p-=.5;
		p.x*=r.x/r.y;
		z+=.07;
		l=length(p);
		uv+=p/l*(sin(z)+1.)*abs(sin(l*9.-z*2.));
		c[i]=.01/length(abs(mod(uv,1.)-.5));
	}
	fragColor=vec4(c/l,t);
}
    </textarea>
  <textarea id="out"></textarea>
</div>
<input id="convert" type="button" value="Convert" />
<input type="checkbox" id="extract" style="margin-left:10%;"></input>Extract properties (Super experimental, might break) <input type="checkbox" id="raymarch" style="margin-left:10%;"></input>Raymarched (Super experimental, might break) <br>
<br>
<div id="links"></div>
<br>
<br>
<input id="download" type="button" value="Download from URL or ID">
<input id="shader" value="https://www.shadertoy.com/view/XsXXDn" style="width:30%">
<br>
<br>
<br>
<link href="./glsl2hlsl.css" rel="stylesheet" type="text/css">
<script src="./glsl2hlsl/bootstrap.js"></script>