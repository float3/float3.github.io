---
title: audiolink
tags:
  - unity
  - shaders
---

[AudioLink](https://github.com/llealloo/audiolink) is the audio-visualization
library for Unity I help maintain — it takes an audio signal apart into bands
and hands them to shaders as a texture, so that a world's materials can move
with the music. This is its example scene, built for WebGL.

It is about 19 MB, so give it a moment. There is no sound input here: the
player runs the scene against AudioLink's built-in test signal.

<div class="unity-player">
  <canvas id="unity-canvas" width="960" height="600" tabindex="-1"></canvas>
  <p id="unity-status" role="status" aria-live="polite">loading…</p>
</div>

<style>
  .unity-player canvas {
    width: 100%;
    max-width: 960px;
    aspect-ratio: 960 / 600;
    background: #231f20;
    border-radius: 5px;
    display: block;
  }

  .unity-player p {
    color: var(--gray);
    font-size: 0.9rem;
    min-height: 1.4em;
  }
</style>

<script type="module" src="/js/audiolink.js"></script>
