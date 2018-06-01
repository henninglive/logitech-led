//! logitech-led
//!
//#![warn(missing_docs)]

extern crate logitech_led_sys as sys;

mod color;

pub use color::{Color, BGRA};

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

    pub fn config_option_color(&mut self, config_path: &str, default: Color) -> Result<Color, Error> {
       let ws = str_to_wchar(config_path)?;
       let mut c = color::to_precent(default);
       unsafe {
            match (self.lib.LogiGetConfigOptionColor)(ws.as_ptr(),
                (&mut c.0) as *mut _, (&mut c.1) as *mut _, (&mut c.2) as *mut _)
            {
                false => Err(Error::GetConfigOptionColor),
                true  => Ok(color::from_precent(c)),
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

    pub fn set_lighting(&mut self, color: Color) -> Result<(), Error> {
        let c = color::to_precent(color);
        unsafe {
            match (self.lib.LogiLedSetLighting)(c.0, c.1, c.2) {
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

    pub fn flash_lighting(&mut self, color: Color, duration: Option<Duration>, interval: Duration) -> Result<(), Error> {
       let c = color::to_precent(color);
       let d = duration.map(|d| duration_to_c_int(d)).unwrap_or(DURATION_INFINITE);
       let i = duration_to_c_int(interval);
       unsafe {
            match (self.lib.LogiLedFlashLighting)(c.0, c.1, c.2, d, i) {
                false => Err(Error::FlashLighting),
                true => Ok(()),
            }
        }
    }

    pub fn pulse_lighting(&mut self, color: Color, duration: Option<Duration>, interval: Duration) -> Result<(), Error> {
       let c = color::to_precent(color);
       let d = duration.map(|d| duration_to_c_int(d)).unwrap_or(DURATION_INFINITE);
       let i = duration_to_c_int(interval);
       unsafe {
            match (self.lib.LogiLedPulseLighting)(c.0, c.1, c.2, d, i) {
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

    pub fn set_lighting_from_bitmap(&mut self, bitmap: &[u8]) -> Result<(), Error> {
        assert_eq!(bitmap.len(), BITMAP_SIZE);
        unsafe {
            match (self.lib.LogiLedSetLightingFromBitmap)(bitmap.as_ptr()) {
                false => Err(Error::SetLightingFromBitmap),
                true => Ok(()),
            }
        }
    }

    pub fn set_lighting_for_key(&mut self, key: Key, color: Color) -> Result<(), Error> {
        let c = color::to_precent(color);
        unsafe {
            match (self.lib.LogiLedSetLightingForKeyWithKeyName)(key, c.0, c.1, c.2) {
                false => Err(Error::SetLightingForKeyWithKeyName),
                true => Ok(()),
            }
        }
    }

    pub fn save_lighting_for_key(&mut self, key: Key) -> Result<(), Error> {
       unsafe {
            match (self.lib.LogiLedSaveLightingForKey)(key) {
                false => Err(Error::SaveLightingForKey),
                true => Ok(()),
            }
        }
    }

    pub fn restore_lighting_for_key(&mut self, key: Key) -> Result<(), Error> {
       unsafe {
            match (self.lib.LogiLedRestoreLightingForKey)(key) {
                false => Err(Error::RestoreLightingForKey),
                true => Ok(()),
            }
        }
    }

    pub fn exclude_keys_from_bitmap(&mut self, keys: &[Key]) -> Result<(), Error> {
       unsafe {
            match (self.lib.LogiLedExcludeKeysFromBitmap)(keys.as_ptr(), keys.len() as c_int) {
                false => Err(Error::ExcludeKeysFromBitmap),
                true => Ok(()),
            }
        }
    }

    pub fn flash_single_key(&mut self, key: Key, color: Color, duration: Option<Duration>, interval: Duration)
        -> Result<(), Error>
    {
        let c = color::to_precent(color);
        let d = duration.map(|d| duration_to_c_int(d)).unwrap_or(DURATION_INFINITE);
        let i = duration_to_c_int(interval);
        unsafe {
            match (self.lib.LogiLedFlashSingleKey)(key, c.0, c.1, c.2, d, i) {
                false => Err(Error::FlashSingleKey),
                true => Ok(()),
            }
        }
    }

    pub fn pulse_single_key(&mut self, key: Key, start: Color, finish: Color,
        duration: Option<Duration>, infinite: bool) -> Result<(), Error>
    {
        let s = color::to_precent(start);
        let f = color::to_precent(finish);
        let d = duration.map(|d| duration_to_c_int(d)).unwrap_or(DURATION_INFINITE);
        unsafe {
            match (self.lib.LogiLedPulseSingleKey)(
                    key,
                    s.0, s.1, s.2,
                    f.0, f.1, f.2,
                    d,
                    infinite as c_int,
                )
            {
                false => Err(Error::PulseSingleKey),
                true => Ok(()),
            }
        }
    }

    pub fn stop_effects_on_key(&mut self, key: Key)
        -> Result<(), Error>
    {
       unsafe {
            match (self.lib.LogiLedStopEffectsOnKey)(key) {
                false => Err(Error::StopEffectsOnKey),
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

//TODO: add is_supported
//TODO: build script unix
//TODO: docs
