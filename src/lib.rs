//! logitech-led
//!
//#![warn(missing_docs)]

extern crate logitech_led_sys as sys;

pub use sys::{
    Key, DeviceType,
    BITMAP_WIDTH, BITMAP_HEIGHT,
    BITMAP_BYTES_PER_KEY, BITMAP_SIZE,
};

use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use std::time::Duration;
use std::os::raw::c_int;
use sys::{Library, DURATION_INFINITE};

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

#[derive(Debug, Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
pub struct RGBP(pub u8, pub u8, pub u8);

#[derive(Debug)]
pub struct Led {
    lib: Library,
}

fn str_to_wchar(s: &str) -> Result<Vec<u16>, Error> {
    let mut v = s.encode_utf16().collect::<Vec<u16>>();
    if v.iter().any(|&val| val == 0) {
        return Err(Error::NullCharacter);
    }
    v.push(0);
    Ok(v)
}

fn duration_to_c_int(d: Duration) -> c_int {
    let n = d.as_secs().checked_mul(1000)
        .and_then(|n| n.checked_add(d.subsec_nanos() as u64 / 1000))
        .expect("Duration to c_int overflow");

    assert!(n <= <c_int>::max_value() as u64, "Duration to c_int overflow");
    n as c_int
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
        let lib = Library::load().map_err(|e| Error::LoadLibrary(e))?;
        
        assert_eq!(INITIALIZED.swap(true, Ordering::SeqCst), false);
        unsafe {
            if !(lib.LogiLedInit)() {
                INITIALIZED.store(false, Ordering::SeqCst);
                return Err(Error::Init);
            }
        }

        Ok(Led {
            lib: lib,
        })
    }

    pub fn set_type(&mut self, device_type: DeviceType) -> Result<(), Error> {
        unsafe {
            if !(self.lib.LogiLedSetTargetDevice)(device_type.bits()) {
                return Err(Error::SetTargetDevice);
            }
        }
        Ok(())
    }

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
       let c = default.clamp();
       let mut red = c.0 as c_int;
       let mut green = c.1 as c_int;
       let mut blue = c.2 as c_int;
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

    pub fn set_lighting(&mut self, color: RGBP) -> Result<(), Error> {
        let c = color.clamp();
        unsafe {
            match (self.lib.LogiLedSetLighting)(c.0 as c_int, c.1 as c_int, c.2 as c_int) {
                false => Err(Error::SetLighting),
                true => Ok(()),
            }
        }
    }

    pub fn save_lighting(&mut self) -> Result<(), Error> {
        unsafe {
            match (self.lib.LogiLedSaveCurrentLighting)() {
                false => Err(Error::SaveCurrentLighting),
                true => Ok(()),
            }
        }
    }

    pub fn restore_lighting(&mut self) -> Result<(), Error> {
        unsafe {
            match (self.lib.LogiLedRestoreLighting)() {
                false => Err(Error::RestoreLighting),
                true => Ok(()),
            }
        }
    }

    pub fn flash_lighting(&mut self, color: RGBP, duration: Option<Duration>, interval: Duration) -> Result<(), Error> {
       let c = color.clamp();
       let d = duration.map(|d| duration_to_c_int(d)).unwrap_or(DURATION_INFINITE);
       let i = duration_to_c_int(interval);
       unsafe {
            match (self.lib.LogiLedFlashLighting)(c.0 as c_int, c.1 as c_int, c.2 as c_int, d, i) {
                false => Err(Error::FlashLighting),
                true => Ok(()),
            }
        }
    }

    pub fn pulse_lighting(&mut self, color: RGBP, duration: Option<Duration>, interval: Duration) -> Result<(), Error> {
       let c = color.clamp();
       let d = duration.map(|d| duration_to_c_int(d)).unwrap_or(DURATION_INFINITE);
       let i = duration_to_c_int(interval);
       unsafe {
            match (self.lib.LogiLedPulseLighting)(c.0 as c_int, c.1 as c_int, c.2 as c_int, d, i) {
                false => Err(Error::PulseLighting),
                true => Ok(()),
            }
        }
    }

    pub fn stop_effects(&mut self) -> Result<(), Error> {
        unsafe {
            match (self.lib.LogiLedStopEffects)() {
                false => Err(Error::StopEffects),
                true => Ok(()),
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

//TODO: use color crate
//TODO: add is_supported
//TODO: build script unix
