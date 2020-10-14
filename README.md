# Rusty Coral Envinronmental Sensors

[<img src="trapezia.jpg">](https://en.wikipedia.org/wiki/Trapezia)

Rust drivers for the [Google Coral Environmental Sensor Board](https://coral.ai/docs/enviro-board/datasheet). Developed on a Raspberry Pi.

The crate bundles together drivers that allow readings from:
- **Humidity + Temperature**: [Texas Instruments HDC2010](https://www.ti.com/lit/ds/symlink/hdc2010.pdf) with help from [`hdc20xx`](https://crates.io/crates/hdc20xx) crate.
- **Ambient Light**: [Texas Instruments OPT3002](https://www.ti.com/lit/ds/symlink/opt3002.pdf) with help from [`opt300x`](https://crates.io/crates/opt300x) crate.
- **Barometric Pressure**: [Bosch BMP280](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bmp280-ds001.pdf) with help from [`bmp280`](https://crates.io/crates/bmp280) crate.

**Warning** ⚠️

I'm a total Rust n00b. This crate is just a small experiment, and things are silly and ugly.
Real credit goes to [natemara](https://github.com/natemara) and [eldruin](https://github.com/eldruin) for their work in the original drivers.
