extern crate coral_env;

fn main() {
    println!("Temperature: {} °C", coralenv::temperature());
    println!("Humidity: {} %", coralenv::humidity());
    println!("Ambient Light: {} lux", coralenv::light());
    println!("Pressure: {} kPa", coralenv::pressure());
}
