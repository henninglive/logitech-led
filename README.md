# logitech-led
Rust bindings for the Logitech Gaming LED Illumination SDK

## Overview
The Logitech Gaming LED SDK enables applications such as games to control the backlight LEDs on
supported Logitech gaming mice and keyboards.

### Requirements
- [Logitech Gaming Software 8.55+][LGS]

### Supported Devices
[Features of lighting-capable Logitech Gaming mice and keyboards](DEVICES.md)

### Available colors
Different devices have different capabilities. They range from full single-key RGB support to single color
only. Details for supported devices are found further below in “Features of lighting-capable Logitech Gaming
mice and keyboards”. The SDK has a single function to set the backlighting color and takes values for R(ed), G(reen), B(lue).
The way it deals with single color devices is to take whichever of the R, G, and B values is the highest
and apply it. This is important to remember, because if for example rotating through colors, the game
should make sure to alternate the maximum numbers as it rotates so that the effect on a single color
device would be noticeable too.

### Dynamic Loading
This crate will try to locate and load `LogitechLed.dll` at runtime.
We start by looking up the `CLSID` in the Windows registry,
if it’s found we load the library with a call to [`LoadLibrary()`][LoadLibrary]
with the full path. If it fails we call [`LoadLibrary()`][LoadLibrary] with just the DLL name.
This will search your `PATH` for the library.

### Multiple clients using the SDK at the same time
The SDK allows only one client to control backlighting at any given time. In case two applications try to
initialize the SDK, the latest one will take over control.

## License
#### Bindings
Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

[SDK]: http://gaming.logitech.com/en-us/developers
[LGS]: http://support.logitech.com/en_us/software/lgs
[LoadLibrary]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms684175.aspx
