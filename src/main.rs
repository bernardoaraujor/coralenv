extern crate linux_embedded_hal as hal;
extern crate hdc20xx;
extern crate opt300x;
extern crate nb;

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();

    let hdc2010_addr = hdc20xx::SlaveAddr::default();
    let mut hdc2010_sensor = hdc20xx::Hdc20xx::new(dev, hdc2010_addr);
    
    let opt3002_addr = opt300x::SlaveAddr::Alternative(false, true);
    let mut opt3002_sensor = opt300x::Opt300x::new_opt3002(dev, opt3002_addr);

    let hdc2010_data = nb::block!(hdc2010_sensor.read()).unwrap();
    let opt3002_data = nb::block!(opt3002_sensor.read_lux()).unwrap();

    println!("Temperature: {}°C", hdc2010_data.temperature);
    println!("Humidity: {:2}%", hdc2010_data.humidity.unwrap());
    println!("Ambient Light: {} lux", opt3002_data.result);
}
