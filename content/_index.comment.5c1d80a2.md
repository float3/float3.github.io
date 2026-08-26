---
parent: "_index.md"
date: "2026-08-24T13:10:00.000Z"
author: "float3"
authorId: 86748455
history:
  - date: "2026-08-24T13:10:00.000Z"
---

Comments here can carry a `<script>`, so here is DOOM in my comment section — [iam-medvedev/wasm-doom](https://github.com/iam-medvedev/wasm-doom), which is the shareware WAD and the engine compiled together to WebAssembly by way of [wasm-fizzbuzz](https://github.com/diekmann/wasm-fizzbuzz). It brings its own WAD, so there is nothing to supply. Arrows to move, `Ctrl` to fire, `Space` to open doors.

It is 6.8 MB, so it waits behind the button rather than loading with the page.

<div id="doom"></div>
<script type="module" src="/js/doom.js"></script>
<style>
  #doom { margin: 0.5rem 0; }
  .doom-start { font: inherit; padding: 0.4rem 0.9rem; }
  .doom-screen { display: block; height: auto; image-rendering: pixelated; max-width: 100%; }
  .doom-screen:focus-visible { outline: 2px solid #4a8fd6; outline-offset: 2px; }
</style>
