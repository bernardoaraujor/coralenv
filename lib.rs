extern crate linux_embedded_hal as hal;
extern crate hdc20xx;
extern crate opt300x;
extern crate bmp280;
extern crate i2cdev;
extern crate nb;

use i2cdev::sensors::{Barometer};

#[no_mangle]
pub extern "C" fn temperature() -> f32 {
    let dev_i2c = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let hdc2010_addr = hdc20xx::SlaveAddr::default();
    let mut hdc2010_sensor = hdc20xx::Hdc20xx::new(dev_i2c, hdc2010_addr);
    let hdc2010_data = nb::block!(hdc2010_sensor.read()).unwrap();

    return hdc2010_data.temperature;
}

#[no_mangle]
pub extern "C" fn humidity() -> f32 {
    let dev_i2c = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let hdc2010_addr = hdc20xx::SlaveAddr::default();
    let mut hdc2010_sensor = hdc20xx::Hdc20xx::new(dev_i2c, hdc2010_addr);
    let hdc2010_data = nb::block!(hdc2010_sensor.read()).unwrap();

    return hdc2010_data.humidity.unwrap();
}

#[no_mangle]
pub extern "C" fn light() -> f32 {
    let dev_i2c = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let opt3002_addr = opt300x::SlaveAddr::Alternative(false, true);
    let mut opt3002_sensor = opt300x::Opt300x::new_opt3002(dev_i2c, opt3002_addr);

    let opt3002_data = nb::block!(opt3002_sensor.read_lux()).unwrap();

    return opt3002_data.result;
}

#[no_mangle]
pub extern "C" fn pressure() -> f32 {
    let mut dev = bmp280::Bmp280Builder::new()
        .path("/dev/i2c-1".to_string())
        .address(0x76)
        .build()
        .expect("Failed to build device");

    dev.zero().expect("failed to zero");

    return dev.pressure_kpa().unwrap();
}
