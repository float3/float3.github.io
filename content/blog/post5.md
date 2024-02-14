
+++
title = "Flashing the Firmware of a SteamVR Dongle"
date = 2024-02-14
updated = 2023-02-14
+++

This blog is based on this [guide](https://github.com/ykeara/SteamVR-Dongle-Flash) by ykeara;
Instructions on how to flash the bootloader provided by [Ben Jackson](https://ben.com)

# Repurposing Steam Controller Dongles for SteamVR

## Requirements

- A SteamVR controller dongle. Although the original dongles are no longer available directly from Steam,
   alternative options are available. These include dongles from [Virtual Builds](https://www.virtualbuilds.com/product-page/usb-wireless-receiver-dongle) 
   and [Tundra Labs](https://tundra-labs.com/shop/vive-dongle), which should serve the purpose well.
- SteamVR installed on your computer.

## Warning

It's important to approach this process with caution:
- Assume that this change is permanent.
- This procedure will flash ALL connected dongles. Be sure to remove any dongles that you do not wish to flash.

## Flashing the Firmware

1. Remove ALL SteamVR dongles as well as the Head-Mounted Display (HMD).
2. Exit Steam.
3. Connect your Steam Controller Dongle to a USB port.
4. Run the commands below. (you will have to adjust these paths for non-standard installation paths for steam and steamvr)

### Windows
```bat
cd `C:\Program Files (x86)\Steam\steamapps\common\SteamVR\tools\lighthouse`
bin\win32\lighthouse_watchman_update.exe -D firmware\vr_controller\archive\htc_vrc_dongle_1461100729_2016_04_19.bin
```

### Linux
Looks like flashing it on Linux doesn't work `Can't convert dongle on non-Windows platform:  Can't tell if conflicting Steam is running.`

## Issues

This procedure changes the runtime firmware but not the bootloader. If you enter the bootloader mode (e.g., through a firmware update), the device will revert to a Steam controller dongle state, necessitating a repeat of the process.

For a permanent change, we need to flash the bootloader as well. 

## Flashing the Bootloader

1. follow https://partner.steamgames.com/vrlicensing (it's a clickthrough sign-up don't worry about not being accepted)
2. Follow the [Flashing the Firmware Guide](#flashing-the-firmware) but use the commands below in the last step (again you may have to adjust the paths if you isntalled the HDK in a differnt directory)
3. Or download the [watchman_dongle_combined.bin](/watchman_dongle_combined.bin) here
   
### Windows
```bat
cd `C:\Program Files (x86)\Steam\steamapps\common\`
SteamVR\tools\lighthouse\bin\win32\lighthouse_watchman_update.exe -D SteamVR\ Tracking\ HDK/firmware/dongle/watchman_dongle_combined.bin
```
## Notes

- While it's possible to flash multiple dongles simultaneously, it's safer to do them one at a time.
- The modification should be considered permanent. Although it's theoretically possible to revert the firmware, successful cases are rare.

## What to Expect

Upon running the command

```
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
```

