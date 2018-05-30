//! Configuration example
//!
//! I these think methods are supposed to add configuration options for this 
//! executable in the logitech gaming software, but i can't find the options
//! after running this example.
//!
//! The Logitech gaming LED SDK is full of race conditions. If you call
//! some methods in quick succession, they fail silently and do nothing.
//! To avoid this, we need to add delays everywhere.
//!

extern crate logitech_led as led;

fn main() {
    let mut led = led::Led::init().unwrap();
    let red = led::RGBP(100, 0, 0);

    std::thread::sleep(std::time::Duration::from_secs(1));

    println!("{:?}", led.config_option_bool("logitech-led/bool", true).unwrap());
    println!("{:?}", led.config_option_num("logitech-led/num", 1.0).unwrap());
    println!("{:?}", led.config_option_color("logitech-led/color", red).unwrap());

    std::thread::sleep(std::time::Duration::from_secs(10));
}