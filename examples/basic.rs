//! Basic functionality example
//!
//! The Logitech gaming LED SDK is full of race conditions. If you call
//! some methods in quick succession, they fail silently and do nothing.
//! To avoid this, we need to add delays everywhere.
//!

extern crate logitech_led as led;

use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut led = led::Led::init().unwrap();
    let red = led::RGBP(100, 0, 0);
    let green = led::RGBP(0, 100, 0);
    let blue = led::RGBP(0, 0, 100);

    sleep(Duration::from_millis(1000));
    led.set_lighting(red).unwrap();

    sleep(Duration::from_millis(1000));
    led.save_lighting().unwrap();
    sleep(Duration::from_millis(1000));

    led.set_lighting(green).unwrap();
    sleep(Duration::from_millis(1000));

    led.set_lighting(blue).unwrap();
    sleep(Duration::from_millis(1000));

    led.restore_lighting().unwrap();
    sleep(Duration::from_millis(1000));

    led.flash_lighting(green, Some(Duration::from_millis(2000)), Duration::from_millis(500)).unwrap();
    sleep(Duration::from_millis(3000));

    // This is broken for some reason.
    /*
    led.pulse_lighting(blue, Some(Duration::from_millis(2000)), Duration::from_millis(500)).unwrap();
    sleep(Duration::from_millis(3000));
    */
}
