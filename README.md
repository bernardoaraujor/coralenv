# trapezia-rs

![crab](https://upload.wikimedia.org/wikipedia/commons/9/95/Trapezia_tigrina_-_Red_Spotted_Guard_Crab.jpg)

Rust drivers for the [Google Coral Environmental Sensor Board](https://coral.ai/docs/enviro-board/datasheet) using the [`linux-embedded-hal`](https://github.com/rust-embedded/linux-embedded-hal) trait.

The crate bundles together drivers that allow readings from:
- **Humidity + Temperature**: [Texas Instruments HDC2010](https://www.ti.com/lit/ds/symlink/hdc2010.pdf) with help from [`hdc20xx`](https://crates.io/crates/hdc20xx) crate.
- **Ambient Light**: [Texas Instruments OPT3002](https://www.ti.com/lit/ds/symlink/opt3002.pdf) with help from [`opt300x`](https://crates.io/crates/opt300x) crate.
- **Barometric Pressure**: [Bosch BMP280](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bmp280-ds001.pdf) with help from [`bmp280`](https://crates.io/crates/bmp280) crate.

It was developed on a Raspberry Pi.
