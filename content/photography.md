---
title: photography
date: 2026-05-05
updated: 2026-05-07
tags:
  - photography
---

<link href="./photography.css" rel="stylesheet">
<script type="module" src="/js/photography.js"></script>

<p class="wasm-credit">made with love and rust (compiled to wasm)</p>

<div class="photo-page">
<section class="photo-equipment" aria-labelledby="equipment-heading">
<h2 id="equipment-heading">equipment</h2>
<div class="photo-gear-sections">
<section class="photo-gear-group">
<h3>35mm cameras</h3>
<div class="photo-gear-list">
<article class="photo-gear-item">
<strong>Minolta X-500</strong>
<p>manual-focus 35mm SLR, also sold as the X-570.</p>
</article>
<article class="photo-gear-item">
<strong>Nikon F100</strong>
<p>autofocus F-mount 35mm SLR.</p>
</article>
<article class="photo-gear-item">
<strong>Nikon FA</strong>
<p>F-mount SLR with Nikon's early Matrix Metering system.</p>
</article>
</div>
</section>
<section class="photo-gear-group photo-gear-group-wide">
<h3>lenses</h3>
<div class="photo-lens-columns">
<section>
<h4>auto focus compatible</h4>
<div class="photo-gear-list">
<article class="photo-gear-item">
<strong>AF Zoom-Nikkor 28-70mm f/3.5-4.5D</strong>
<p>compact AF-D standard zoom.</p>
</article>
<article class="photo-gear-item">
<strong>AF-S DX VR Zoom-Nikkor 18-200mm f/3.5-5.6G IF-ED</strong>
<p>DX all-in-one zoom.</p>
</article>
<article class="photo-gear-item">
<strong>AF-S NIKKOR 35mm f/1.8G ED</strong>
<p>full-frame F-mount prime.</p>
</article>
<article class="photo-gear-item">
<strong>AF-S NIKKOR 50mm f/1.4G</strong>
<p>full-frame F-mount prime.</p>
</article>
<article class="photo-gear-item">
<strong>AF-S NIKKOR 50mm f/1.8G</strong>
<p>full-frame F-mount prime.</p>
</article>
<article class="photo-gear-item">
<strong>AF Micro-Nikkor 105mm f/2.8</strong>
<p>non-D macro lens marked 1:2.8 on the barrel. used with the NIK F-L adapter for film scanning.</p>
</article>
</div>
</section>
<section>
<h4>not auto focus compatible</h4>
<div class="photo-gear-list">
<article class="photo-gear-item">
<strong>NIKKOR-S Auto 50mm f/1.4</strong>
<p>Nippon Kogaku Japan pre-AI standard prime.</p>
</article>
<article class="photo-gear-item">
<strong>Sigma Zoom-α III 35-135mm f/3.5-4.5</strong>
<p>manual-focus multi-coated zoom, marked 55mm.</p>
</article>
</div>
</section>
</div>
</section>
<section class="photo-gear-group">
<h3>film</h3>
<div class="photo-gear-list">
<article class="photo-gear-item">
<strong>Fujifilm, Kodak, and ILFORD HP5 PLUS</strong>
<p>my default film speeds are ISO 200 and 400, in both black-and-white and color. photos will eventually be tagged by the film stock they were shot on.</p>
</article>
</div>
</section>
<section class="photo-gear-group">
<h3>flashes and other equipment</h3>
<div class="photo-gear-list">
<article class="photo-gear-item">
<strong>Nikon Speedlight SB-15</strong>
<p>compact TTL-capable Nikon flash.</p>
</article>
<article class="photo-gear-item">
<strong>ROWI Skylight 1A 55mm</strong>
<p>skylight filter mounted on the Sigma zoom.</p>
</article>
</div>
</section>
<section class="photo-gear-group">
<h3>scanning equipment</h3>
<div class="photo-gear-list">
<article class="photo-gear-item">
<strong>Panasonic LUMIX S5 (DC-S5)</strong>
<p>full-frame mirrorless camera used for camera scanning.</p>
</article>
<article class="photo-gear-item">
<strong>K&amp;F Concept Nikon F to L-mount adapter</strong>
<p>F-mount to L-mount adapter used on the Lumix for film scanning.</p>
</article>
<article class="photo-gear-item">
<strong>VALOI easy35</strong>
<p>35mm camera-scanning system.</p>
</article>
</div>
</section>
</div>
</section>
<section class="photo-gallery-section" aria-labelledby="gallery-heading">
<div class="photo-gallery-header">
<h2 id="gallery-heading">gallery</h2>
<p id="photo-gallery-count"></p>
</div>
<div id="photo-gallery" class="photo-gallery" aria-live="polite"></div>
</section>
<dialog id="photo-lightbox" class="photo-lightbox" aria-label="photo viewer">
<button class="photo-lightbox-close" type="button" aria-label="close">x</button>
<button class="photo-lightbox-prev" type="button" aria-label="previous photo">prev</button>
<figure>
<img alt="">
<figcaption></figcaption>
</figure>
<button class="photo-lightbox-next" type="button" aria-label="next photo">next</button>
</dialog>
</div>
