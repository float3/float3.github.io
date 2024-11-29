---
title: Piracy at the British Museum
date: 2023-05-14
updated: 2023-05-23
tags:
  - art
---

## Visiting the British Museum

On the 20th of August 2022 I visited the British Museum with some friends and decided to take some videos:

<div style="display: flex;">
<video controls height="500">
<source src="/misc/media/statue_with_child.mp4">
</video>
<video controls height="500">
<source src="/misc/media/kneeling_statue.mp4">
</video>
<video controls height="500">
<source src="/misc/media/helmet_guy.mp4">
</video>
<video controls height="500">
<source src="/misc/media/laughing_buddha.mp4">
</video>
</div>

## back home

After I returned from my Vacation, I extracted the frames from each video using [FFmpeg](https://github.com/FFmpeg/FFmpeg) using the following command

```sh
ffmpeg -i video.mp4 -vf yadif images/%05d.png
```

the `-vf yadif` flag should extract frames without interlacing.

## and then

I imported the frames into [meshroom](https://github.com/alicevision/meshroom) and let it run on ~~200°C~~ default settings overnight.

## and finally

I imported all the 3D Models and textures into [Blender](https://www.blender.org), cleaned up the meshes, set up my scene hit the render button and voilà we've successfully stolen from the British Museum:

<img src="/misc/media/render.png" width=100%/>

you can find all the assets [here](https://github.com/float3/BritishMuseum3DScans)
