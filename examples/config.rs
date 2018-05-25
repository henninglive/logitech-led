extern crate logitech_led as led;

fn main() {
    let mut led = led::Led::init().unwrap();
    println!("{:?}", led.config_option_bool("logitech-led/bool", true).unwrap());
    println!("{:?}", led.config_option_num("logitech-led/num", 1.0).unwrap());
}