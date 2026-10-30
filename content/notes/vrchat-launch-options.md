---
title: vrchat launch options
tags:
  - vrchat
  - unity
---

every launch flag i could find for VRChat, as of build 2026-09-17.

VRChat's own flags are C# string literals, and those live in `global-metadata.dat`, which VRChat ships encrypted, so they come from the [official docs](https://docs.vrchat.com/docs/launch-options) and old changelogs instead. the Unity player flags were read straight out of `UnityPlayer.dll`, and a few more out of `launch.exe`.

flags go in Steam's launch options, or after `launch.exe` / `VRChat.exe` on the command line. VRChat's use two dashes, Unity's one.

## vrchat

| argument                                                                   | default                       | explanation                                                                                                                                                          | source                                              |
| -------------------------------------------------------------------------- | ----------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| `--no-vr`                                                                  |                               | start in desktop mode                                                                                                                                                | [docs](https://docs.vrchat.com/docs/launch-options) |
| `--profile=N`                                                              | `0`                           | use local profile N, for running several accounts side by side                                                                                                       | docs                                                |
| `--fps=N`                                                                  | 90 desktop, headset max in VR | frame rate limit                                                                                                                                                     | docs                                                |
| `--osc=inPort:outIP:outPort`                                               | `9000:127.0.0.1:9001`         | OSC ports, see the [OSC docs](https://docs.vrchat.com/docs/osc-overview)                                                                                             | docs                                                |
| `--midi=name`                                                              |                               | pick a MIDI input device by (partial) name                                                                                                                           | docs                                                |
| `--ignore-trackers=serial1,serial2`                                        |                               | ignore these tracker serials                                                                                                                                         | docs                                                |
| `--enable-debug-gui`                                                       | off                           | enable the debug menu keyboard shortcuts                                                                                                                             | docs                                                |
| `--enable-sdk-log-levels`                                                  | off                           | much more verbose logging                                                                                                                                            | docs                                                |
| `--enable-udon-debug-logging`                                              | off                           | log Udon heap and stack on errors                                                                                                                                    | docs                                                |
| `--watch-worlds`                                                           | off                           | rejoin automatically when the SDK builds a new local test world                                                                                                      | docs                                                |
| `--watch-avatars`                                                          | off                           | reload the local test avatar when the SDK rebuilds it                                                                                                                | docs                                                |
| `--skip-registry-install`                                                  | off                           | don't write the `vrchat://` protocol handler to the registry                                                                                                         | docs                                                |
| `--disable-hw-video-decoding`                                              | hardware                      | force software decoding in AVPro                                                                                                                                     | docs                                                |
| `--enable-hw-video-decoding`                                               | hardware                      | force hardware decoding in AVPro                                                                                                                                     | docs                                                |
| `--enable-amd-stutter-workaround`                                          | off                           | workaround for stutter on some AMD GPUs                                                                                                                              | docs                                                |
| `--enforce-world-server-checks`                                            | off                           | only join worlds after the server has validated them                                                                                                                 | docs                                                |
| `--affinity=MASK`                                                          | all cores                     | CPU affinity as a hex bitmask                                                                                                                                        | docs, `launch.exe`                                  |
| `--process-priority=N`                                                     | `0`                           | process priority, -2 to 2                                                                                                                                            | docs, `launch.exe`                                  |
| `--main-thread-priority=N`                                                 | `0`                           | main thread priority, -2 to 2                                                                                                                                        | docs                                                |
| `--user-affinity`                                                          |                               | undocumented, found in `launch.exe` next to `--affinity`                                                                                                             | `launch.exe`                                        |
| `--startup-begin-ts=`                                                      |                               | undocumented; by the name, a startup timestamp `launch.exe` hands to the game                                                                                        | `launch.exe`                                        |
| `vrchat://launch?id=`                                                      |                               | join an instance; the `id` is the world id and instance, as in an invite link                                                                                        |                                                     |
| `--url=create?roomId=N&hidden=true&name=BuildAndRun&url=file:///path.vrcw` |                               | open a local `.vrcw` like the SDK's build and test                                                                                                                   |                                                     |
| `--log-debug-levels="API;All;…"`                                           |                               | extra log categories; known ones were `Always`, `API`, `AssetBundleDownloadManager`, `ContentCreator`, `All`, `NetworkTransport`, `NetworkData`, `NetworkProcessing` | 2022, may be gone                                   |
| `--custom-arm-ratio="0.4537"`                                              | `0.4537`                      | IK 2.0 arm ratio; `0.415` approximates the old arm scale                                                                                                             | IK 2.0 beta changelog, 2022.1.1p3                   |
| `--disable-shoulder-tracking`                                              | off                           | for IMU-only arm trackers                                                                                                                                            | IK 2.0 beta changelog, 2022.1.1p4                   |
| `--calibration-range="0.6"`                                                | `0.6`                         | calibration search radius around binding points, in metres                                                                                                           | IK 2.0 beta changelog, 2022.1.2p4                   |
| `--enable-ik-debug-logging`                                                | off                           | IK debug logging                                                                                                                                                     | IK 2.0 beta changelog, 2022.1.2p4                   |
| `--freeze-tracking-on-disconnect`                                          | off                           | freeze disconnected trackers in place until you recalibrate                                                                                                          | IK 2.0 beta changelog, 2022.1.2p4                   |
| `--enable-avpro-in-proton`                                                 | off                           | allow AVPro under Proton                                                                                                                                             | VRChat Discord announcement                         |

## unity player

names as found in `UnityPlayer.dll`. the [Unity docs](https://docs.unity3d.com/Manual/PlayerCommandLineArguments.html) cover the common ones; the rest are undocumented and the explanation is read off the name.

### window and display

| argument                               | explanation                           |
| -------------------------------------- | ------------------------------------- |
| `-screen-width N` / `-screen-height N` | resolution                            |
| `-screen-fullscreen 0\|1`              | fullscreen off or on                  |
| `-window-mode exclusive\|borderless`   | fullscreen mode                       |
| `-popupwindow`                         | borderless window                     |
| `-monitor N`                           | display, 1-based                      |
| `-adapter N`                           | GPU index                             |
| `-parentHWND HWND`                     | embed in another window               |
| `-screen-quality name`                 | quality level by name                 |
| `-single-instance`                     | only one running copy                 |
| `-no-stereo-rendering`                 | turn off stereo rendering             |
| `-enable-stereoscopic3d`               | stereoscopic 3D on supported displays |
| `-vrmode name`                         | start with the named XR device        |

### graphics api

| argument                                                                                                                   | explanation                                                            |
| -------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| `-force-d3d11`, `-force-d3d12`, `-force-vulkan`, `-force-glcore`, `-force-gles`                                            | pick the graphics API (VRChat is only supported on D3D11)              |
| `-force-glcoreXY`, `-force-glesXY`                                                                                         | a specific GL/GLES version, e.g. `-force-glcore45`, `-force-gles31aep` |
| `-force-feature-level-10-0` … `-force-feature-level-11-1`                                                                  | D3D feature level                                                      |
| `-force-d3d11-singlethreaded`, `-force-d3d11-nothreads`                                                                    | no D3D11 multithreading                                                |
| `-force-d3d11-flip-model`, `-force-d3d11-bitblt-model`                                                                     | D3D11 swapchain present model                                          |
| `-force-d3d11-debug`, `-force-d3d12-debug`, `-force-d3d12-debug-gbv`                                                       | D3D debug layer, GPU-based validation                                  |
| `-force-driver-type-warp`                                                                                                  | WARP software rasterizer                                               |
| `-force-device-index N`, `-force-multigpu`                                                                                 | GPU selection                                                          |
| `-force-gfx-direct`, `-force-gfx-st`, `-force-gfx-mt`, `-force-gfx-jobs`                                                   | graphics threading mode                                                |
| `-gfx-disable-mt-rendering`, `-gfx-enable-gfx-jobs`, `-gfx-enable-native-gfx-jobs`, `-gfx-jobs-sync`                       | the same, finer grained                                                |
| `-disable-gpu-skinning`                                                                                                    | skin meshes on the CPU                                                 |
| `-force-vulkan-layers`, `-force-vulkan-layers-with-renderdoc`, `-vulkan-layers-printf`                                     | Vulkan validation layers                                               |
| `-force-vulkan-onscreen-swapchain`, `-vulkan-disable-secondary-commandbuffers`, `-disable-vulkan-command-buffer-recycling` | Vulkan workarounds                                                     |
| `-vulkan-suballocator-blocks`, `-vulkan-suballocator-threshold`                                                            | Vulkan memory suballocator                                             |
| `-enable-vulkan-performance-validation`                                                                                    | Vulkan performance warnings                                            |
| `-max-async-pso-job-count N`                                                                                               | parallel pipeline state compiles                                       |
| `-strict-shader-variant-matching`                                                                                          | error instead of falling back on a missing shader variant              |
| `-video-allow-hw-acceleration`                                                                                             | hardware decoding for Unity's own video player                         |
| `-xr-latelatching-enabled`, `-xr-latelatchingdebug-enabled`                                                                | XR late latching                                                       |
| `-xr-request-additional-vulkan-graphics-queue`                                                                             | extra Vulkan queue for XR                                              |

### jobs and memory

| argument                               | explanation                                                                                                                                                                  |
| -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-job-worker-count N`                  | job system worker threads                                                                                                                                                    |
| `-background-job-worker-count N`       | background worker threads                                                                                                                                                    |
| `-enlighten-job-worker-count N`        | Enlighten GI worker threads                                                                                                                                                  |
| `-no-main-thread-job-stealing`         | the main thread doesn't run jobs while it waits                                                                                                                              |
| `-job-system-wait-spin-time-ns N`      | how long workers spin before sleeping                                                                                                                                        |
| `-memorysetup-*=N`                     | allocator sizes, e.g. `-memorysetup-main-allocator-block-size`, `-memorysetup-temp-allocator-size-main`, `-memorysetup-gfx-thread-allocator-block-size`; 24 of them in total |
| `-preload-manager-thread-stack-size N` | preload thread stack size                                                                                                                                                    |

### logging, debugging and crashes

| argument                                                                                                | explanation                         |
| ------------------------------------------------------------------------------------------------------- | ----------------------------------- |
| `-logFile path`                                                                                         | log file; `-` for stdout            |
| `-nolog`                                                                                                | no log file                         |
| `-batchmode`, `-nographics`                                                                             | headless                            |
| `-no-dialogs`                                                                                           | suppress error dialogs              |
| `-silent-crashes`                                                                                       | no crash dialog                     |
| `-break-on-crash`                                                                                       | break into the debugger on a crash  |
| `-crash-report-folder path`                                                                             | where crash dumps go                |
| `-show-crash-handler`, `-debug-crash-handler`, `-no-crash-quit`                                         | crash handler behaviour             |
| `-gfx-debug-msg`                                                                                        | graphics API debug messages         |
| `-native-leak-detection`                                                                                | native memory leak detection        |
| `-enable-file-read-metrics`                                                                             | file read metrics                   |
| `-wait-for-native-debugger`, `-wait-for-managed-debugger`                                               | wait for a debugger before starting |
| `-managed-debugger-fixed-port N`                                                                        | managed debugger port               |
| `-profiler-log-file path`, `-profiler-capture-frame-count N`, `-profiler-enable-deep-profiling-support` | profiler                            |
| `-random-seed N`                                                                                        | fixed random seed                   |
