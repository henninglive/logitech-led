extern crate winapi;

use self::winapi::{wchar_t, c_int, c_double, c_char};

pub const LOGI_LED_BITMAP_WIDTH: usize = 21;
pub const LOGI_LED_BITMAP_HEIGHT: usize = 6;
pub const LOGI_LED_BITMAP_BYTES_PER_KEY: usize = 4;

pub const LOGI_LED_BITMAP_SIZE: usize = LOGI_LED_BITMAP_WIDTH * LOGI_LED_BITMAP_HEIGHT * LOGI_LED_BITMAP_BYTES_PER_KEY;
pub const LOGI_LED_DURATION_INFINITE: usize = 0;

pub const LOGI_DEVICETYPE_MONOCHROME_ORD: u32 = 0;
pub const LOGI_DEVICETYPE_RGB_ORD: u32 = 1;
pub const LOGI_DEVICETYPE_PERKEY_RGB_ORD: u32 = 2;

pub const LOGI_DEVICETYPE_MONOCHROME: u32 = 1 << LOGI_DEVICETYPE_MONOCHROME_ORD;
pub const LOGI_DEVICETYPE_RGB: u32 = 1 << LOGI_DEVICETYPE_RGB_ORD;
pub const LOGI_DEVICETYPE_PERKEY_RGB: u32 = 1 << LOGI_DEVICETYPE_PERKEY_RGB_ORD;

pub const LOGI_DEVICETYPE_ALL: u32 = LOGI_DEVICETYPE_MONOCHROME | LOGI_DEVICETYPE_RGB | LOGI_DEVICETYPE_PERKEY_RGB;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Bool {
    FALSE = 0,
    TRUE = 1
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LogiLed {
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


#[link(name="LogitechLEDLib")]
extern "C" {
    pub fn LogiLedInit() -> Bool;
    
    pub fn LogiLedGetSdkVersion(majorNum: *mut c_int, minorNum: *mut c_int, buildNum: *mut c_int) -> Bool;
    pub fn LogiLedGetConfigOptionNumber(configPath: *const wchar_t, defaultValue: *mut c_double) -> Bool;
    pub fn LogiLedGetConfigOptionBool(configPath: *const wchar_t, defaultValue: *mut Bool) -> Bool;
    pub fn LogiLedGetConfigOptionColor(configPath: *const wchar_t, defaultRed: *mut c_int, 
        defaultGreen: *mut c_int, defaultBlue: *mut c_int) -> Bool;
    pub fn LogiLedGetConfigOptionKeyInput(configPath: *const wchar_t, defaultValue: *mut wchar_t, bufferSize: c_int) -> Bool;
    pub fn LogiLedSetConfigOptionLabel(configPath: *const wchar_t, label: *mut wchar_t) -> Bool;

    //Generic functions => Apply to any device type.
    pub fn LogiLedSetTargetDevice(targetDevice: c_int) -> Bool;
    pub fn LogiLedSaveCurrentLighting() -> Bool;
    pub fn LogiLedSetLighting(redPercentage: c_int, greenPercentage: c_int, bluePercentage: c_int) -> Bool;
    pub fn LogiLedRestoreLighting() -> Bool;
    pub fn LogiLedFlashLighting(redPercentage: c_int, greenPercentage: c_int, bluePercentage: c_int,
        milliSecondsDuration: c_int, milliSecondsInterval: c_int) -> Bool;
    pub fn LogiLedPulseLighting(redPercentage: c_int, greenPercentage: c_int, bluePercentage: c_int,
        milliSecondsDuration: c_int, milliSecondsInterval: c_int) -> Bool;
    pub fn LogiLedStopEffects() -> Bool;

    //Per-key functions => only apply to LOGI_DEVICETYPE_PERKEY_RGB devices.
    pub fn LogiLedSetLightingFromBitmap(bitmap: *const c_char) -> Bool;
    pub fn LogiLedSetLightingForKeyWithScanCode(keyCode: c_int, redPercentage: c_int, 
        greenPercentage: c_int, bluePercentage: c_int) -> Bool;
    pub fn LogiLedSetLightingForKeyWithHidCode(keyCode: c_int, redPercentage: c_int, 
        greenPercentage: c_int, bluePercentage: c_int) -> Bool;
    pub fn LogiLedSetLightingForKeyWithQuartzCode(keyCode: c_int, redPercentage: c_int, 
        greenPercentage: c_int, bluePercentage: c_int) -> Bool;
    pub fn LogiLedSetLightingForKeyWithKeyName(keyName: LogiLed, redPercentage: c_int, 
        greenPercentage: c_int, bluePercentage: c_int) -> Bool;
    pub fn LogiLedSaveLightingForKey(keyName: LogiLed) -> Bool;
    pub fn LogiLedRestoreLightingForKey(keyName: LogiLed) -> Bool;
    pub fn LogiLedExcludeKeysFromBitmap(keyList: *mut LogiLed, listCount: c_int) -> Bool;

    //Per-key effects => only apply to LOGI_DEVICETYPE_PERKEY_RGB devices.
    pub fn LogiLedFlashSingleKey(keyName: LogiLed, redPercentage: c_int, greenPercentage: c_int, 
        bluePercentage: c_int, msDuration: c_int, msInterval: c_int) -> Bool;
    pub fn LogiLedPulseSingleKey(keyName: LogiLed, startRedPercentage: c_int, startGreenPercentage: c_int, 
        startBluePercentage: c_int, finishRedPercentage: c_int, finishGreenPercentage: c_int, 
        finishBluePercentage: c_int, msDuration: c_int, isInfinite: c_int) -> Bool;
    pub fn LogiLedStopEffectsOnKey(keyName: LogiLed) -> Bool;

    pub fn LogiLedShutdown();
}

impl From<Bool> for bool {
    fn from(b: Bool) -> bool {
        match b {
            Bool::FALSE => false,
            Bool::TRUE => true,
        }
    }
}
