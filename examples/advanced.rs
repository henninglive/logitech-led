//! Basic functionality example
//!
//! The Logitech gaming LED SDK is full of race conditions. If you call
//! some methods in quick succession, they fail silently and do nothing.
//! To avoid this, we need to add delays everywhere.
//!

extern crate logitech_led as led;

use led::{Driver, Color, Key};
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let mut driver = Driver::init().unwrap();

    sleep(Duration::from_secs(1));

    let white = Color::new(1.0, 1.0, 1.0);
    let black = Color::new(0.0, 0.0, 0.0);

    let buffer = (0..led::BITMAP_WIDTH).flat_map(|i| {
            let h = (i as f32) *
                (360.0 / led::BITMAP_WIDTH as f32);

            // I wish we had into_iter for arrays.
            <[u8; 4]>::from(Color::from_hsv(h, 1.0, 1.0))
                .iter()
                .cloned()
                .collect::<Vec<_>>()
        })
        .cycle()
        .take(led::BITMAP_SIZE)
        .collect::<Vec<_>>();

    driver.exclude_keys_from_bitmap(&[Key::Q]).unwrap();
    driver.set_lighting_from_bitmap(&buffer[..]).unwrap();
    sleep(Duration::from_secs(1));

    driver.set_lighting_for_key(Key::W, white).unwrap();
    driver.set_lighting_for_key(Key::A, white).unwrap();
    driver.set_lighting_for_key(Key::S, white).unwrap();
    driver.set_lighting_for_key(Key::D, white).unwrap();

    sleep(Duration::from_secs(1));
    driver.save_lighting_for_key(Key::W).unwrap();
    sleep(Duration::from_secs(1));
    driver.set_lighting_for_key(Key::W, black).unwrap();
    sleep(Duration::from_secs(1));

    // Dosen't do anything
    driver.restore_lighting_for_key(Key::W).unwrap();
    sleep(Duration::from_secs(1));

    driver.flash_single_key(Key::Q, white, Some(Duration::from_millis(1000)), Duration::from_millis(300)).unwrap();
    sleep(Duration::from_secs(2));

    driver.pulse_single_key(Key::Q, black, white, Duration::from_millis(1000), true).unwrap();
    sleep(Duration::from_secs(10));
}
