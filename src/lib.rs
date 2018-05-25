//! logitech-led
//!
//#![warn(missing_docs)]

extern crate logitech_led_sys as sys;

pub use sys::{LogiLed, LogiDeviceType,
    LOGI_LED_BITMAP_WIDTH,
    LOGI_LED_BITMAP_HEIGHT,
    LOGI_LED_BITMAP_BYTES_PER_KEY,
    LOGI_LED_BITMAP_SIZE,
    LOGI_LED_DURATION_INFINITE,
};

use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use std::os::raw::c_int;
use sys::LogitechLed;


static INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

#[derive(Debug)]
pub enum Error {
    Init,
    GetSdkVersion,
    GetConfigOptionNumber,
    GetConfigOptionBool,
    GetConfigOptionColor,
    GetConfigOptionKeyInput,
    SetConfigOptionLabel,
    SetTargetDevice,
    SaveCurrentLighting,
    SetLighting,
    RestoreLighting,
    FlashLighting,
    PulseLighting,
    StopEffects,
    SetLightingFromBitmap,
    SetLightingForKeyWithScanCode,
    SetLightingForKeyWithHidCode,
    SetLightingForKeyWithQuartzCode,
    SetLightingForKeyWithKeyName,
    SaveLightingForKey,
    RestoreLightingForKey,
    ExcludeKeysFromBitmap,
    FlashSingleKey,
    PulseSingleKey,
    StopEffectsOnKey,
    Shutdown,
    /// Unexpected NULL character
    NullCharacter,
    /// Failed to load LogitechLed.dll.
    LoadLibrary(std::io::Error),
    Utf16(std::string::FromUtf16Error),
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SdkVersion {
    pub major_num: usize,
    pub minor_num: usize,
    pub build_num: usize,
}

pub struct RGBP(pub u8, pub u8, pub u8);

#[derive(Debug)]
pub struct Led {
    lib: LogitechLed,
    led_type: LogiDeviceType,
}

fn str_to_wchar(s: &str) -> Result<Vec<u16>, Error> {
    let mut v = s.encode_utf16().collect::<Vec<u16>>();
    if v.iter().any(|&val| val == 0) {
        return Err(Error::NullCharacter);
    }
    v.push(0);
    Ok(v)
}

impl RGBP {
    pub fn clamp(self) -> RGBP {
        RGBP(
            if self.0 > 100 { 100 } else { self.0 },
            if self.1 > 100 { 100 } else { self.1 },
            if self.2 > 100 { 100 } else { self.2 },
        )
    }

    pub fn red(&self) -> u8 { self.0 }
    pub fn green(&self) -> u8 { self.1 }
    pub fn blue(&self) -> u8 { self.2 }
}

impl Led {
    pub fn init() -> Result<Led, Error> {
        Self::init_type(LogiDeviceType::ALL)
    }

    pub fn init_type(led_type: LogiDeviceType) -> Result<Led, Error> {
        let lib = LogitechLed::load().map_err(|e| Error::LoadLibrary(e))?;
        
        assert_eq!(INITIALIZED.swap(true, Ordering::SeqCst), false);
        unsafe {
            if !(lib.LogiLedInit)() {
                INITIALIZED.store(false, Ordering::SeqCst);
                return Err(Error::Init);
            }
        }

        let mut led = Led {
            lib: lib,
            led_type: LogiDeviceType::ALL,
        };

        led.set_type(led_type)?;

        Ok(led)
    }

    pub fn set_type(&mut self, led_type: LogiDeviceType) -> Result<(), Error> {
        unsafe {
            if !(self.lib.LogiLedSetTargetDevice)(led_type.bits()) {
                return Err(Error::SetTargetDevice);
            }
        }
        self.led_type = led_type;
        Ok(())
    }

    /*
    pub fn sdk_version(&self) -> Result<SdkVersion, Error> {
        let mut major_num:c_int = 0;
        let mut minor_num:c_int = 0;
        let mut build_num:c_int = 0;
        unsafe {
            match (self.lib.LogiLedGetSdkVersion)(
                &mut major_num as *mut c_int, &mut minor_num as *mut c_int, &mut build_num as *mut c_int)
            {
                false => Err(Error::GetSdkVersion),
                true  => Ok(SdkVersion {
                    major_num: major_num as usize,
                    minor_num: minor_num as usize,
                    build_num: build_num as usize,
                }),
            }
        }
    }
    */

    pub fn config_option_num(&mut self, config_path: &str, default: f64) -> Result<f64, Error> {
        let ws = str_to_wchar(config_path)?;
        let mut val = default;
        unsafe {
            match (self.lib.LogiGetConfigOptionNumber)(ws.as_ptr(), (&mut val) as *mut _) {
                false => Err(Error::GetConfigOptionNumber),
                true  => Ok(val),
            }
        }
    }

    pub fn config_option_bool(&mut self, config_path: &str, default: bool) -> Result<bool, Error> {
        let ws = str_to_wchar(config_path)?;
        let mut val = default;
        unsafe {
            match (self.lib.LogiGetConfigOptionBool)(ws.as_ptr(), (&mut val) as *mut _) {
                false => Err(Error::GetConfigOptionBool),
                true  => Ok(val),
            }
        }
    }

    pub fn config_option_color(&mut self, config_path: &str, default: RGBP) -> Result<RGBP, Error> {
       let ws = str_to_wchar(config_path)?;
       let mut red = default.0 as c_int;
       let mut green = default.1 as c_int;
       let mut blue = default.2 as c_int;
       unsafe {
            match (self.lib.LogiGetConfigOptionColor)(ws.as_ptr(),
                (&mut red) as *mut _, (&mut green) as *mut _, (&mut blue) as *mut _)
            {
                false => Err(Error::GetConfigOptionColor),
                true  => Ok(RGBP(red as u8, green as u8, blue as u8).clamp()),
            }
        }
    }

    pub fn config_option_label(&mut self, config_path: &str, lable: &str) -> Result<String, Error> {
        let path_ws = str_to_wchar(config_path)?;
        let mut lable_ws = str_to_wchar(lable)?;
        
        unsafe {
            // This might write over lable_ws, we unfortunately do not know the size of that string,
            // let's hope it's smaller then this.
            const LABEL_LEN: usize = 512;
            lable_ws.reserve(LABEL_LEN);
            lable_ws.set_len(LABEL_LEN);
            match (self.lib.LogiSetConfigOptionLabel)(path_ws.as_ptr(), lable_ws.as_mut_ptr())
            {
                false => Err(Error::SetConfigOptionLabel),
                true  => {
                    // Find the \0
                    let mut n = 0;
                    while lable_ws[n] != 0 {
                        n += 1;
                        if n >= LABEL_LEN {
                            return Err(Error::NullCharacter)
                        }
                    }

                    // To string without the \0
                    String::from_utf16(&lable_ws[..(n - 1)]).map_err(|e| Error::Utf16(e))
                },
            }
        }
    }
}

impl Drop for Led {
    /// Kills the applet and frees memory used by the SDK
    fn drop(&mut self) {
        unsafe {
            (self.lib.LogiLedShutdown)();
        }
        INITIALIZED.store(false, Ordering::SeqCst);
    }
}
