---
title: "flashing the firmware and bootloader of a Steam Controller Dongle"
tags:
  - steamvr
  - vr
  - programming
---

this post is based on this [guide](https://github.com/ykeara/SteamVR-Dongle-Flash) by ykeara
and Instructions on how to flash the bootloader provided by [Ben Jackson](https://ben.com)

# flashing the firmware and bootloader of a Steam Controller Dongle

## requirements

- a SteamVR controller dongle. although the original dongles are no longer available directly from Steam,
  alternative options are available. these include dongles from [Virtual Builds](https://www.virtualbuilds.com/product-page/usb-wireless-receiver-dongle)
  and [Tundra Labs](https://tundra-labs.com/shop/vive-dongle), which should serve the purpose well.
- SteamVR installed on your computer.

## warning

it's important to approach this process with caution:

- assume that this change is permanent, at least if you decide to flash the bootloader as well.
- this procedure will flash ALL connected dongles. be sure to remove any dongles that you do not wish to flash.

## flashing the firmware

1. remove ALL SteamVR dongles as well as the Head-Mounted Display (HMD).
2. Exit Steam.
3. connect your Steam Controller Dongle to a USB port.
4. run the commands below. (you will have to adjust these paths for non-standard installation paths for steam and steamvr)

### Windows

<pre style="background-color: #000000; padding: 10px;"><code>
<span style="color: cyan;">cd</span> <span style="color: blue;">`C:\Program Files (x86)\Steam\steamapps\common\SteamVR\tools\lighthouse`</span>
<span style="color: cyan;">bin\win32\lighthouse_watchman_update.exe</span> <span style="color: green;">-D</span> <span style="color: red;">firmware\vr_controller\archive\htc_vrc_dongle_1461100729_2016_04_19.bin</span>
</code></pre>

### Linux

looks like flashing it on Linux doesn't work natively, the executable just prints `Can't convert dongle on non-Windows platform:  Can't tell if conflicting Steam is running.`
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

### issues

this procedure changes the runtime firmware but not the bootloader.
if you enter the bootloader mode (e.g., through a firmware update),
the device will revert to a Steam controller dongle state,
necessitating a repeat of the process.

for a permanent change, we need to flash the bootloader as well.

## flashing the bootloader

1. follow https://partner.steamgames.com/vrlicensing (it's a clickthrough sign-up don't worry about not being accepted)
2. follow the [Flashing the Firmware Guide](#flashing-the-firmware) but use the commands below in the last step (again you may have to adjust the paths if you isntalled the HDK in a differnt directory)
3. or download the [watchman_dongle_combined.bin](/misc/blobs/watchman_dongle_combined.bin) here

### Windows

<pre style="background-color: black; padding: 10px;"><code>
<span style="color: cyan;">cd</span> <span style="color: blue;">`C:\Program Files (x86)\Steam\steamapps\common\`</span>
<span style="color: cyan;">SteamVR\tools\lighthouse\bin\win32\lighthouse_watchman_update.exe</span> <span style="color: green;">-D</span> <span style="color: red;">SteamVR\ Tracking\ HDK/firmware/dongle/watchman_dongle_combined.bin</span>
</code></pre>

## notes

- while it's possible to flash multiple dongles simultaneously, it might be safer to do them one at a time.

## what to expect

upon running the command

### firmware flash

<pre style="background-color: black; color: white; padding: 10px;">
<code>
Attempting to update Watchman Dongles to version 1461100729...

converting steam controller dongle to watchman dongle. switching to bootloader

sending reset into bootloader command

looking for 1 bootloaders.

HID opened: VID 28de PID 1042 serial (null) seq 1 if 0

attempting to convert Steam Controller dongle into Watchman Dongle...

Walve Nordic bootloader version 19.0

2vvX........vvvvvvvvP3X........VVVVVVVVP4X.. .vvvvvvvvP5X........vvvvvvvvP6X........vvvvvvvvP7X........vvvvvvvvP8X........VVVVVVVVP BX........vvvvvvvvP10X........VVVVVVVVP11X........vvvvvvvvP12X........vvvvvvvvP13X........vvvvvvvvP14X........۷۷۷۷۷vvvP15X........۷۷۷۷۷۷

wvP16X........vvvvvvvvP17X........vvvvvvvvP18X........vvvvvvvvP19X........vvvvvvvvP20X........vvvvvvvvP21X........vvvvvvvvP22X........VV wvvvvvP23X........VVVVVVVvP24X........vvvvvvvvP25X........۷۷۷۷۷۷vvP26X........vvvvvvvvP27X........vvvvvvvvP28X........vvvvvvvvP29X......

..vvvvvvvvP30X........vvvvvvvvP31X........vvvvvvvvP32X........vvvvvvvvP33X........vvvvvvvvP34X........vvvvvvvvP35X........VVVVVVVVP1vvvv

wvvvP0X........VVVVVVVV

successfully converted firmware.

found all expected bootloaders
</code></pre>

### bootloader flash

TODO
