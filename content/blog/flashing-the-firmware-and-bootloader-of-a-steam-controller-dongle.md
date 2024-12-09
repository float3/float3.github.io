---
title: "Flashing the Firmware and Bootloader of a Steam Controller Dongle"
date: 2024-02-14
updated: 2023-02-14
tags:
  - steamvr
  - vr
  - programming
---

This post is based on this [guide](https://github.com/ykeara/SteamVR-Dongle-Flash) by ykeara
and Instructions on how to flash the bootloader provided by [Ben Jackson](https://ben.com)

# Flashing the Firmware and Bootloader of a Steam Controller Dongle

## Requirements

- A SteamVR controller dongle. Although the original dongles are no longer available directly from Steam,
  alternative options are available. These include dongles from [Virtual Builds](https://www.virtualbuilds.com/product-page/usb-wireless-receiver-dongle)
  and [Tundra Labs](https://tundra-labs.com/shop/vive-dongle), which should serve the purpose well.
- SteamVR installed on your computer.

## Warning

It's important to approach this process with caution:

- Assume that this change is permanent, at least if you decide to flash the bootloader as well.
- This procedure will flash ALL connected dongles. Be sure to remove any dongles that you do not wish to flash.

## Flashing the Firmware

1. Remove ALL SteamVR dongles as well as the Head-Mounted Display (HMD).
2. Exit Steam.
3. Connect your Steam Controller Dongle to a USB port.
4. Run the commands below. (you will have to adjust these paths for non-standard installation paths for steam and steamvr)

### Windows

<pre style="background-color: #000000; padding: 10px;"><code>
<span style="color: cyan;">cd</span> <span style="color: blue;">`C:\Program Files (x86)\Steam\steamapps\common\SteamVR\tools\lighthouse`</span>
<span style="color: cyan;">bin\win32\lighthouse_watchman_update.exe</span> <span style="color: green;">-D</span> <span style="color: red;">firmware\vr_controller\archive\htc_vrc_dongle_1461100729_2016_04_19.bin</span>
</code></pre>

### Linux

Looks like flashing it on Linux doesn't work natively, the executable just prints `Can't convert dongle on non-Windows platform:  Can't tell if conflicting Steam is running.`
This is stupid. so let's use wine.

you need these dlls

```sh
winetricks ole32
winetricks winmm
winetricks oleaut32
```

you can find the lighthouse_watchman_update.exe [here](/misc/blobs/lighthouse_watchman_update.exe) and the vrcameral_api.dll [here](/misc/blobs/vrcamera_api.dll)
or in a steamvr windows installation at `C:\Program Files (x86)\Steam\steamapps\common\SteamVR\tools\lighthouse\bin\win32` (they aren't shipped on linux)

then just follow the windows instructions but with wine

### Issues

This procedure changes the runtime firmware but not the bootloader.
If you enter the bootloader mode (e.g., through a firmware update),
the device will revert to a Steam controller dongle state,
necessitating a repeat of the process.

For a permanent change, we need to flash the bootloader as well.

## Flashing the Bootloader

1. Follow https://partner.steamgames.com/vrlicensing (it's a clickthrough sign-up don't worry about not being accepted)
2. Follow the [Flashing the Firmware Guide](#flashing-the-firmware) but use the commands below in the last step (again you may have to adjust the paths if you isntalled the HDK in a differnt directory)
3. Or download the [watchman_dongle_combined.bin](/misc/blobs/watchman_dongle_combined.bin) here

### Windows

<pre style="background-color: black; padding: 10px;"><code>
<span style="color: cyan;">cd</span> <span style="color: blue;">`C:\Program Files (x86)\Steam\steamapps\common\`</span>
<span style="color: cyan;">SteamVR\tools\lighthouse\bin\win32\lighthouse_watchman_update.exe</span> <span style="color: green;">-D</span> <span style="color: red;">SteamVR\ Tracking\ HDK/firmware/dongle/watchman_dongle_combined.bin</span>
</code></pre>

## Notes

- While it's possible to flash multiple dongles simultaneously, it might be safer to do them one at a time.

## What to Expect

Upon running the command

### Firmware Flash

<pre style="background-color: black; color: white; padding: 10px;">
<code>
Attempting to update Watchman Dongles to version 1461100729...

Converting steam controller dongle to watchman dongle. Switching to bootloader

Sending reset into bootloader command

Looking for 1 bootloaders.

HID opened: VID 28de PID 1042 serial (null) seq 1 if 0

Attempting to convert Steam Controller dongle into Watchman Dongle...

Walve Nordic bootloader version 19.0

2vvX........vvvvvvvvP3X........VVVVVVVVP4X.. .vvvvvvvvP5X........vvvvvvvvP6X........vvvvvvvvP7X........vvvvvvvvP8X........VVVVVVVVP BX........vvvvvvvvP10X........VVVVVVVVP11X........vvvvvvvvP12X........vvvvvvvvP13X........vvvvvvvvP14X........۷۷۷۷۷vvvP15X........۷۷۷۷۷۷

wvP16X........vvvvvvvvP17X........vvvvvvvvP18X........vvvvvvvvP19X........vvvvvvvvP20X........vvvvvvvvP21X........vvvvvvvvP22X........VV wvvvvvP23X........VVVVVVVvP24X........vvvvvvvvP25X........۷۷۷۷۷۷vvP26X........vvvvvvvvP27X........vvvvvvvvP28X........vvvvvvvvP29X......

..vvvvvvvvP30X........vvvvvvvvP31X........vvvvvvvvP32X........vvvvvvvvP33X........vvvvvvvvP34X........vvvvvvvvP35X........VVVVVVVVP1vvvv

wvvvP0X........VVVVVVVV

Successfully converted firmware.

Found all expected bootloaders
</code></pre>

### Bootloader Flash

TODO
