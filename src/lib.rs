extern crate logi_led_sys;

use logi_led_sys::*;
use std::os::raw::c_int;

#[derive(Debug)]
pub struct SdkVersion {
    pub major_num: i32,
    pub minor_num: i32,
    pub build_num: i32,
}

pub fn sdk_version() -> Result<SdkVersion, ()> {
    let mut major_num:c_int = 0;
    let mut minor_num:c_int = 0;
    let mut build_num:c_int = 0;
    unsafe {
        match LogiLedGetSdkVersion(&mut major_num as *mut c_int, &mut minor_num as *mut c_int, &mut build_num as *mut c_int) {
            false => Err(()),
            true  => Ok(SdkVersion {
                major_num: major_num as i32,
                minor_num: minor_num as i32,
                build_num: build_num as i32,
            }),
        }
    }
}