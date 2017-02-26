extern crate logi_led_sys;

use logi_led_sys::*;
use std::os::raw::c_int;

#[derive(Debug)]
pub enum LedError {
    GetSdkVersion,
    GetConfigOptionNumber,
    GetConfigOptionBool,
    GetConfigOptionColor,
    GetConfigOptionKeyInput,
    SetConfigOptionLabel,
    NullCharacter,
}

#[derive(Debug)]
pub struct SdkVersion {
    pub major_num: i32,
    pub minor_num: i32,
    pub build_num: i32,
}

fn str_to_wchar(s: &str) -> Result<Vec<u16>, LedError> {
    let mut v = s.encode_utf16().collect::<Vec<u16>>();

    if v.iter().any(|&val| val == 0) {
        return Err(LedError::NullCharacter);
    }

    v.push(0);

    Ok(v)
}

pub fn sdk_version() -> Result<SdkVersion, LedError> {
    let mut major_num:c_int = 0;
    let mut minor_num:c_int = 0;
    let mut build_num:c_int = 0;
    unsafe {
        match LogiLedGetSdkVersion(&mut major_num as *mut c_int, &mut minor_num as *mut c_int, &mut build_num as *mut c_int) {
            false => Err(LedError::GetSdkVersion),
            true  => Ok(SdkVersion {
                major_num: major_num as i32,
                minor_num: minor_num as i32,
                build_num: build_num as i32,
            }),
        }
    }
}

pub fn config_option_number(config_path: &str, default_value: &mut f64) -> Result<(), LedError> {
    let ws = str_to_wchar(config_path)?;
    unsafe {
        match LogiLedGetConfigOptionNumber(ws.as_ptr(), default_value as *mut f64) {
            false => Err(LedError::GetConfigOptionNumber),
            true  => Ok(()),
        }
    }
}

pub fn config_option_bool(config_path: &str, default_value: &mut bool) -> Result<(), LedError> {
    let ws = str_to_wchar(config_path)?;
    unsafe {
        match LogiLedGetConfigOptionBool(ws.as_ptr(), default_value as *mut bool) {
            false => Err(LedError::GetConfigOptionBool),
            true  => Ok(()),
        }
    }
}