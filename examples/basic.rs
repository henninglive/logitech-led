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
    let mut driver = led::Driver::init().unwrap();
    let red = led::Color::new(1.0, 0.0, 0.0);
    let green = led::Color::new(0.0, 1.0, 0.0);
    let blue = led::Color::new(0.0, 0.0, 1.0);

    sleep(Duration::from_millis(1000));
    driver.set_lighting(red).unwrap();

    sleep(Duration::from_millis(1000));
    driver.save_lighting().unwrap();
    sleep(Duration::from_millis(1000));

    driver.set_lighting(green).unwrap();
    sleep(Duration::from_millis(1000));

    driver.set_lighting(blue).unwrap();
    sleep(Duration::from_millis(1000));

    driver.restore_lighting().unwrap();
    sleep(Duration::from_millis(1000));

    driver.flash_lighting(blue, Some(Duration::from_millis(2000)), Duration::from_millis(500)).unwrap();
    sleep(Duration::from_millis(3000));

    // Dosen't do anything
    driver.pulse_lighting(green, Some(Duration::from_millis(2000)), Duration::from_millis(1000)).unwrap();
    sleep(Duration::from_millis(3000));
}
