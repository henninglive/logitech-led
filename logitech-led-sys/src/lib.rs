//! FFI bindings and loader for the Logitech LED SDK
//!
//! [LogitechLed](struct.LogitechLcd.html) will try to locate and load
//! `LogitechLed.dll` at Runtime for dynamic linking. The library
//! will be unloaded if dropped, but it is reference counted by internally
//! Windows.
//!

#![allow(non_camel_case_types, non_snake_case)]

#[macro_use]
extern crate bitflags;

use std::os::raw::{c_int, c_uint, c_double};

pub const BITMAP_WIDTH: usize         = 21;
pub const BITMAP_HEIGHT: usize        = 6;
pub const BITMAP_BYTES_PER_KEY: usize = 4;
pub const BITMAP_SIZE: usize = BITMAP_WIDTH * BITMAP_HEIGHT * BITMAP_BYTES_PER_KEY;

pub const DURATION_INFINITE: c_int = 0;

bitflags! {
    /// Targeted device type.
    ///
    /// This library allows you to target different device types.
    pub struct DeviceType: c_uint {
        const MONOCHROME =  0x1;
        const RGB = 0x2;
        const PERKEY_RGB = 0x4;
        const ALL = Self::MONOCHROME.bits | Self::RGB.bits | Self::PERKEY_RGB.bits;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Key {
    ESC                = 0x01,
    F1                 = 0x3b,
    F2                 = 0x3c,
    F3                 = 0x3d,
    F4                 = 0x3e,
    F5                 = 0x3f,
    F6                 = 0x40,
    F7                 = 0x41,
    F8                 = 0x42,
    F9                 = 0x43,
    F10                = 0x44,
    F11                = 0x57,
    F12                = 0x58,
    PRINT_SCREEN       = 0x137,
    SCROLL_LOCK        = 0x46,
    PAUSE_BREAK        = 0x145,
    TILDE              = 0x29,
    ONE                = 0x02,
    TWO                = 0x03,
    THREE              = 0x04,
    FOUR               = 0x05,
    FIVE               = 0x06,
    SIX                = 0x07,
    SEVEN              = 0x08,
    EIGHT              = 0x09,
    NINE               = 0x0A,
    ZERO               = 0x0B,
    MINUS              = 0x0C,
    EQUALS             = 0x0D,
    BACKSPACE          = 0x0E,
    INSERT             = 0x152,
    HOME               = 0x147,
    PAGE_UP            = 0x149,
    NUM_LOCK           = 0x45,
    NUM_SLASH          = 0x135,
    NUM_ASTERISK       = 0x37,
    NUM_MINUS          = 0x4A,
    TAB                = 0x0F,
    Q                  = 0x10,
    W                  = 0x11,
    E                  = 0x12,
    R                  = 0x13,
    T                  = 0x14,
    Y                  = 0x15,
    U                  = 0x16,
    I                  = 0x17,
    O                  = 0x18,
    P                  = 0x19,
    OPEN_BRACKET       = 0x1A,
    CLOSE_BRACKET      = 0x1B,
    BACKSLASH          = 0x2B,
    KEYBOARD_DELETE    = 0x153,
    END                = 0x14F,
    PAGE_DOWN          = 0x151,
    NUM_SEVEN          = 0x47,
    NUM_EIGHT          = 0x48,
    NUM_NINE           = 0x49,
    NUM_PLUS           = 0x4E,
    CAPS_LOCK          = 0x3A,
    A                  = 0x1E,
    S                  = 0x1F,
    D                  = 0x20,
    F                  = 0x21,
    G                  = 0x22,
    H                  = 0x23,
    J                  = 0x24,
    K                  = 0x25,
    L                  = 0x26,
    SEMICOLON          = 0x27,
    APOSTROPHE         = 0x28,
    ENTER              = 0x1C,
    NUM_FOUR           = 0x4B,
    NUM_FIVE           = 0x4C,
    NUM_SIX            = 0x4D,
    LEFT_SHIFT         = 0x2A,
    Z                  = 0x2C,
    X                  = 0x2D,
    C                  = 0x2E,
    V                  = 0x2F,
    B                  = 0x30,
    N                  = 0x31,
    M                  = 0x32,
    COMMA              = 0x33,
    PERIOD             = 0x34,
    FORWARD_SLASH      = 0x35,
    RIGHT_SHIFT        = 0x36,
    ARROW_UP           = 0x148,
    NUM_ONE            = 0x4F,
    NUM_TWO            = 0x50,
    NUM_THREE          = 0x51,
    NUM_ENTER          = 0x11C,
    LEFT_CONTROL       = 0x1D,
    LEFT_WINDOWS       = 0x15B,
    LEFT_ALT           = 0x38,
    SPACE              = 0x39,
    RIGHT_ALT          = 0x138,
    RIGHT_WINDOWS      = 0x15C,
    APPLICATION_SELECT = 0x15D,
    RIGHT_CONTROL      = 0x11D,
    ARROW_LEFT         = 0x14B,
    ARROW_DOWN         = 0x150,
    ARROW_RIGHT        = 0x14D,
    NUM_ZERO           = 0x52,
    NUM_PERIOD         = 0x53,
    G_1                = 0xFFF1,
    G_2                = 0xFFF2,
    G_3                = 0xFFF3,
    G_4                = 0xFFF4,
    G_5                = 0xFFF5,
    G_6                = 0xFFF6,
    G_7                = 0xFFF7,
    G_8                = 0xFFF8,
    G_9                = 0xFFF9,
    G_LOGO             = 0xFFFF1,
    G_BADGE            = 0xFFFF2
}

#[derive(Debug)]
pub struct Library {
    pub LogiLedInit: unsafe extern "C" fn() -> bool,

    pub LogiGetConfigOptionNumber: unsafe extern "C" fn(configPath: *const u16, defaultValue: *mut c_double) -> bool,
    pub LogiGetConfigOptionBool: unsafe extern "C" fn(configPath: *const u16, defaultValue: *mut bool) -> bool,
    pub LogiGetConfigOptionColor: unsafe extern "C" fn(configPath: *const u16, defaultRed: *mut c_int,
        defaultGreen: *mut c_int, defaultBlue: *mut c_int) -> bool,
    pub LogiGetConfigOptionKeyInput: unsafe extern "C" fn(configPath: *const u16, defaultValue: *mut u16, bufferSize: c_int) -> bool,
    pub LogiSetConfigOptionLabel: unsafe extern "C" fn(configPath: *const u16, label: *mut u16) -> bool,

    // Generic functions => Apply to any device type.
    pub LogiLedSetTargetDevice: unsafe extern "C" fn(targetDevice: c_uint) -> bool,
    pub LogiLedSaveCurrentLighting: unsafe extern "C" fn() -> bool,
    pub LogiLedSetLighting: unsafe extern "C" fn(redPercentage: c_int, greenPercentage: c_int, bluePercentage: c_int) -> bool,
    pub LogiLedRestoreLighting: unsafe extern "C" fn() -> bool,
    pub LogiLedFlashLighting: unsafe extern "C" fn(redPercentage: c_int, greenPercentage: c_int, bluePercentage: c_int,
        milliSecondsDuration: c_int, milliSecondsInterval: c_int) -> bool,
    pub LogiLedPulseLighting: unsafe extern "C" fn(redPercentage: c_int, greenPercentage: c_int, bluePercentage: c_int,
        milliSecondsDuration: c_int, milliSecondsInterval: c_int) -> bool,
    pub LogiLedStopEffects: unsafe extern "C" fn() -> bool,

    // Per-key functions => only apply to LogiDeviceType::PERKEY_RGB devices.
    pub LogiLedSetLightingFromBitmap: unsafe extern "C" fn(bitmap: *const u8) -> bool,
    pub LogiLedSetLightingForKeyWithScanCode: unsafe extern "C" fn(keyCode: c_int, redPercentage: c_int,
        greenPercentage: c_int, bluePercentage: c_int) -> bool,
    pub LogiLedSetLightingForKeyWithHidCode: unsafe extern "C" fn(keyCode: c_int, redPercentage: c_int,
        greenPercentage: c_int, bluePercentage: c_int) -> bool,
    pub LogiLedSetLightingForKeyWithQuartzCode: unsafe extern "C" fn(keyCode: c_int, redPercentage: c_int,
        greenPercentage: c_int, bluePercentage: c_int) -> bool,
    pub LogiLedSetLightingForKeyWithKeyName: unsafe extern "C" fn(keyName: Key, redPercentage: c_int,
        greenPercentage: c_int, bluePercentage: c_int) -> bool,
    pub LogiLedSaveLightingForKey: unsafe extern "C" fn(keyName: Key) -> bool,
    pub LogiLedRestoreLightingForKey: unsafe extern "C" fn(keyName: Key) -> bool,
    pub LogiLedExcludeKeysFromBitmap: unsafe extern "C" fn(keyList: *const Key, listCount: c_int) -> bool,

    // Per-key effects => only apply to LogiDeviceType::PERKEY_RGB devices.
    pub LogiLedFlashSingleKey: unsafe extern "C" fn(keyName: Key, redPercentage: c_int, greenPercentage: c_int,
        bluePercentage: c_int, msDuration: c_int, msInterval: c_int) -> bool,
    pub LogiLedPulseSingleKey: unsafe extern "C" fn(keyName: Key, startRedPercentage: c_int, startGreenPercentage: c_int,
        startBluePercentage: c_int, finishRedPercentage: c_int, finishGreenPercentage: c_int, 
        finishBluePercentage: c_int, msDuration: c_int, isInfinite: c_int) -> bool,
    pub LogiLedStopEffectsOnKey: unsafe extern "C" fn(keyName: Key) -> bool,

    pub LogiLedShutdown: unsafe extern "C" fn(),

    /// Library handle, will be freed on drop
    _handle: platform::Handle,
}

#[cfg(not(target_os = "windows"))]
mod platform {
    use super::Library;
    use std::io::{Error, ErrorKind};

    pub struct Handle(*const ());

    impl Library {
        pub fn load() -> Result<Library, Error> {
            Err(Error::new(ErrorKind::Other, "Unsupported system"))
        }
    }

    unsafe impl Send for Handle {}
}

#[cfg(target_os = "windows")]
mod platform {
    extern crate winapi;
    extern crate kernel32;
    extern crate winreg;

    use super::Library;

    use self::winreg::RegKey;
    use self::winreg::enums::{HKEY_LOCAL_MACHINE, HKEY_CLASSES_ROOT, KEY_READ};
    use self::winapi::minwindef::{HMODULE, FARPROC};

    use std::os::windows::ffi::OsStrExt;
    use std::ffi::OsStr;
    use std::io::Error;
    use std::fmt;

    pub struct Handle(HMODULE);

    const ERROR_MOD_NOT_FOUND: i32 = winapi::winerror::ERROR_MOD_NOT_FOUND as i32;

    /// Find `LogitechLed.dll` in Windows registry using its CLSID
    fn dll_path_clsid() -> Result<Vec<u16>, Error> {
        let hkcl = RegKey::predef(HKEY_CLASSES_ROOT);
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

        let mut dll_path = None;

        #[cfg(target_arch = "x86_64")]
        {
            match hkcl.open_subkey_with_flags(
                "CLSID\\{a6519e67-7632-4375-afdf-caa889744403}\\ServerBinary", KEY_READ)
            {
                Ok(key) => dll_path = key.get_value::<String, &str>("").ok(),
                Err(_) => {},
            }
    
            match hklm.open_subkey_with_flags(
                "SOFTWARE\\Classes\\CLSID\\{a6519e67-7632-4375-afdf-caa889744403}\\ServerBinary", KEY_READ)
            {
                Ok(key) => dll_path = key.get_value::<String, &str>("").ok(),
                Err(_) => {},
            }
        }

        #[cfg(target_arch = "x86")]
        {
            match hkcl.open_subkey_with_flags(
                "Wow6432Node\\CLSID\\{a6519e67-7632-4375-afdf-caa889744403}\\ServerBinary", KEY_READ)
            {
                Ok(key) => dll_path = key.get_value::<String, &str>("").ok(),
                Err(_) => {},
            }
    
            match hklm.open_subkey_with_flags(
                "SOFTWARE\\Classes\\Wow6432Node\\CLSID\\{a6519e67-7632-4375-afdf-caa889744403}\\ServerBinary", KEY_READ)
            {
                Ok(key) => dll_path = key.get_value::<String, &str>("").ok(),
                Err(_) => {},
            }
    
            match hklm.open_subkey_with_flags(
                "SOFTWARE\\Wow6432Node\\Classes\\CLSID\\{a6519e67-7632-4375-afdf-caa889744403}\\ServerBinary", KEY_READ)
            {
                Ok(key) => dll_path = key.get_value::<String, &str>("").ok(),
                Err(_) => {},
            }
        }

        match dll_path {
            // Convert to widestring and terminate with \0\0.
            Some(p) => Ok(OsStr::new(&p[..]).encode_wide().chain(Some(0)).collect::<Vec<u16>>()),
            None => Err(Error::from_raw_os_error(ERROR_MOD_NOT_FOUND)),
        }
    }

    unsafe fn load_lib() -> Result<HMODULE, Error> {
        match dll_path_clsid() {
            Ok(wide_path) => {
                let handle = kernel32::LoadLibraryW(wide_path.as_ptr());
                if handle.is_null() {
                    let error = Error::last_os_error();
                    let ecode = error.raw_os_error().unwrap();
                    // Fallthrough on ERROR_MOD_NOT_FOUND
                    if ecode != ERROR_MOD_NOT_FOUND {
                        return Err(error);
                    }
                } else {
                    return Ok(handle);
                }
            },
            Err(e) => {
                match e.raw_os_error() {
                    Some(ERROR_MOD_NOT_FOUND) => {},
                    _ => return Err(e),
                }
            },
        }

        // Convert to widestring and terminate with \0\0.
        let wide_name = OsStr::new("LogitechLed.dll").encode_wide().chain(Some(0)).collect::<Vec<u16>>();
        let handle = kernel32::LoadLibraryW(wide_name.as_ptr());
        if handle.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(handle)
        }
    }

    impl Library {
        /// Try to locate and load 'LogitechLed.dll'.
        pub fn load() -> Result<Library, Error> {
            use std::mem;

            unsafe {
                let handle = load_lib()?;

                let mut symbols = [
                    ("LogiLedInit\0",                            0 as FARPROC),
                    ("LogiGetConfigOptionNumber\0",              0 as FARPROC),
                    ("LogiGetConfigOptionBool\0",                0 as FARPROC),
                    ("LogiGetConfigOptionColor\0",               0 as FARPROC),
                    ("LogiGetConfigOptionKeyInput\0",            0 as FARPROC),
                    ("LogiSetConfigOptionLabel\0",               0 as FARPROC),
                    ("LogiLedSetTargetDevice\0",                 0 as FARPROC),
                    ("LogiLedSaveCurrentLighting\0",             0 as FARPROC),
                    ("LogiLedSetLighting\0",                     0 as FARPROC),
                    ("LogiLedRestoreLighting\0",                 0 as FARPROC),
                    ("LogiLedFlashLighting\0",                   0 as FARPROC),
                    ("LogiLedPulseLighting\0",                   0 as FARPROC),
                    ("LogiLedStopEffects\0",                     0 as FARPROC),
                    ("LogiLedSetLightingFromBitmap\0",           0 as FARPROC),
                    ("LogiLedSetLightingForKeyWithScanCode\0",   0 as FARPROC),
                    ("LogiLedSetLightingForKeyWithHidCode\0",    0 as FARPROC),
                    ("LogiLedSetLightingForKeyWithQuartzCode\0", 0 as FARPROC),
                    ("LogiLedSetLightingForKeyWithKeyName\0",    0 as FARPROC),
                    ("LogiLedSaveLightingForKey\0",              0 as FARPROC),
                    ("LogiLedRestoreLightingForKey\0",           0 as FARPROC),
                    ("LogiLedExcludeKeysFromBitmap\0",           0 as FARPROC),
                    ("LogiLedFlashSingleKey\0",                  0 as FARPROC),
                    ("LogiLedPulseSingleKey\0",                  0 as FARPROC),
                    ("LogiLedStopEffectsOnKey\0",                0 as FARPROC),
                    ("LogiLedShutdown\0",                        0 as FARPROC),
                ];

                for i in symbols.iter_mut() {
                    i.1 = kernel32::GetProcAddress(handle, i.0.as_ptr() as *const i8);
                    if i.1.is_null() {
                        let error = Error::last_os_error();
                        kernel32::FreeLibrary(handle);
                        return Err(error);
                    }
                }

                Ok(Library {
                    LogiLedInit:                            mem::transmute(symbols[0].1),
                    LogiGetConfigOptionNumber:              mem::transmute(symbols[1].1),
                    LogiGetConfigOptionBool:                mem::transmute(symbols[2].1),
                    LogiGetConfigOptionColor:               mem::transmute(symbols[3].1),
                    LogiGetConfigOptionKeyInput:            mem::transmute(symbols[4].1),
                    LogiSetConfigOptionLabel:               mem::transmute(symbols[5].1),
                    LogiLedSetTargetDevice:                 mem::transmute(symbols[6].1),
                    LogiLedSaveCurrentLighting:             mem::transmute(symbols[7].1),
                    LogiLedSetLighting:                     mem::transmute(symbols[8].1),
                    LogiLedRestoreLighting:                 mem::transmute(symbols[9].1),
                    LogiLedFlashLighting:                   mem::transmute(symbols[10].1),
                    LogiLedPulseLighting:                   mem::transmute(symbols[11].1),
                    LogiLedStopEffects:                     mem::transmute(symbols[12].1),
                    LogiLedSetLightingFromBitmap:           mem::transmute(symbols[13].1),
                    LogiLedSetLightingForKeyWithScanCode:   mem::transmute(symbols[14].1),
                    LogiLedSetLightingForKeyWithHidCode:    mem::transmute(symbols[15].1),
                    LogiLedSetLightingForKeyWithQuartzCode: mem::transmute(symbols[16].1),
                    LogiLedSetLightingForKeyWithKeyName:    mem::transmute(symbols[17].1),
                    LogiLedSaveLightingForKey:              mem::transmute(symbols[18].1),
                    LogiLedRestoreLightingForKey:           mem::transmute(symbols[19].1),
                    LogiLedExcludeKeysFromBitmap:           mem::transmute(symbols[20].1),
                    LogiLedFlashSingleKey:                  mem::transmute(symbols[21].1),
                    LogiLedPulseSingleKey:                  mem::transmute(symbols[22].1),
                    LogiLedStopEffectsOnKey:                mem::transmute(symbols[23].1),
                    LogiLedShutdown:                        mem::transmute(symbols[24].1),
                    _handle: Handle(handle),
                })
            }
        }
    }

    impl fmt::Debug for Handle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Pointer::fmt(&self.0, f)
        }
    }

    impl Drop for Handle {
        fn drop(&mut self) {
            unsafe {
                kernel32::FreeLibrary(self.0);
            }
        }
    }

    unsafe impl Send for Handle {}
}
