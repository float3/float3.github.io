---
title: "trolley"
date: 2024-12-21
updated: 2026-05-10
tags:
  - trolley
---

<link href="/photography.css" rel="stylesheet">
<script type="module" src="/js/trolley.js"></script>

<p class="wasm-credit">made with love and rust (compiled to wasm)</p>

<div class="photo-page trolley-page">
<section class="photo-gallery-section" aria-labelledby="trolley-heading">
<div class="photo-gallery-header">
<h2 id="trolley-heading">problem</h2>
<p id="trolley-gallery-count"></p>
</div>
<div id="trolley-gallery" class="photo-gallery trolley-gallery" aria-live="polite"></div>
</section>
<dialog id="trolley-lightbox" class="photo-lightbox" aria-label="trolley problem viewer">
<button class="photo-lightbox-close" type="button" aria-label="close">x</button>
<button class="photo-lightbox-prev" type="button" aria-label="previous trolley problem">prev</button>
<figure>
<figcaption></figcaption>
</figure>
<button class="photo-lightbox-next" type="button" aria-label="next trolley problem">next</button>
</dialog>
</div>
