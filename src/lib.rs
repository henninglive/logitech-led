extern crate logi_led_sys;
extern crate winapi;

use self::winapi::{c_int};
use logi_led_sys::*;

#[derive(Debug)]
pub struct SdkVersion {
    pub major_num: i32,
    pub minor_num: i32,
    pub build_num: i32,
}

pub fn sdk_version() -> Result<SdkVersion, ()> {
    let mut major_num = 0i32;
    let mut minor_num = 0i32;
    let mut build_num = 0i32;
    unsafe {
        match LogiLedGetSdkVersion(&mut major_num as *mut c_int, &mut minor_num as *mut c_int, &mut build_num as *mut c_int) {
            Bool::FALSE => Err(()),
            Bool::TRUE  => Ok(SdkVersion {
                major_num: major_num,
                minor_num: minor_num,
                build_num: build_num,
            }),
        }
    }
}