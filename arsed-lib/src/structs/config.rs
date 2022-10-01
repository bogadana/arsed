use nom_derive::{NomLE, Parse};
use nom::{error::Error, IResult, bytes::complete::take};
use num_enum::{TryFromPrimitive, IntoPrimitive};
use proc_bitfield::{bitfield, BitRange};
use crate::parser::primitive::u8_bool;

#[repr(align(4))]
#[derive(PartialEq, Debug, NomLE)]
pub struct ResConfig {
    imsi: Imsi,
    locale: Locale,
    screen_type: ScreenType,
    input: Input,
    screen_size: ScreenSize,
    version: Version,
    screen_config: ScreenConfig,
    screen_size_dp: ScreenSizeDp,
    locale_script: [u8; 4],
    locale_variant: [u8; 8],
    screen_config_2: ScreenConfig2,
    #[nom(Parse(u8_bool))]
    locale_script_was_computed: bool,
    locale_numbering_system: [u8; 8],
}

#[derive(PartialEq, Debug, NomLE)]
pub struct Imsi {
    mcc: u16,
    mnc: u16, 
}

#[derive(PartialEq, Debug, NomLE)]
pub struct Locale {
    language: String,
    country: String
}

#[repr(C)]
#[derive(PartialEq, Debug, NomLE)]
pub struct ScreenType {
    orientation: Orientation,
    touchscreen: Touchscreen,
    density: Density,
}

#[repr(u8)]
#[derive(PartialEq, Debug, NomLE)]
pub enum Orientation {
    Any,
    Port,
    Land,
    Square,
}


#[repr(u8)]
#[derive(PartialEq, Debug, NomLE)]
pub enum Touchscreen {
    Any,
    NoTouch,
    Stylus,
    Finger,
}

#[repr(u16)]
#[derive(PartialEq, Debug, NomLE)]
pub enum Density {
    Default,
    Low = 120,
    Medium = 160,
    TV = 213,
    High = 240,
    XHigh = 320,
    XXHigh = 480,
    XXXHigh = 640,
    Any = 0xfffe,
    None = 0xffff,
}

#[repr(C)]
#[derive(PartialEq, Debug, NomLE)]
pub struct Input {
    keyboard: Keyboard,
    navigation: Navigation,
    flags: InputFlagsBit,
}

#[repr(u8)]
#[derive(PartialEq, Debug, NomLE)]
pub enum Keyboard {
    Any,
    NoKeys,
    Qwerty,
    TwelveKey,
}

#[repr(u8)]
#[derive(PartialEq, Debug, NomLE)]
pub enum Navigation {
    Any,
    NoNav,
    Dpad,
    Trackball,
    Wheel,
}

#[repr(u8)]
#[derive(PartialEq, Debug, TryFromPrimitive, IntoPrimitive)]
pub enum KeysHidden {
    Any,
    No,
    Yes,
    Soft,
}

#[repr(u8)]
#[derive(PartialEq, Debug, TryFromPrimitive, IntoPrimitive)]
pub enum NavHidden {
    Any,
    No,
    Yes,
}

bitfield! {
    #[derive(PartialEq, NomLE)]
    pub struct InputFlagsBit(pub u8): Debug {
        pub keys_hidden: u8 [try KeysHidden] @ 6..=7,
        pub nav_hidden: u8 [try NavHidden] @ 4..=5,
    }
}

#[repr(C)]
#[derive(PartialEq, Debug, NomLE)]
pub struct ScreenSize {
    pub width: u16,
    pub height: u16,
}

#[repr(C)]
#[derive(PartialEq, Debug, NomLE)]
pub struct Version {
    pub sdk: u16,
    pub minor: u16,
}

#[repr(C)]
#[derive(PartialEq, Debug, NomLE)]
pub struct ScreenConfig {
    pub screen_layout: ScreenLayoutBit,
    pub ui_mode: UiModeBit,
    pub smallest_screen_width_dp: u16,
}

bitfield! {
    #[derive(PartialEq, NomLE)]
    pub struct ScreenLayoutBit(pub u8): Debug {
        pub screen_size: u8 [try ScreenLayoutSize] @ 4..=7,
        pub screen_long: u8 [try ScreenLong] @ 2..=3,
        pub layout_dir: u8 [try LayoutDir] @ 0..=2
    }
}

#[repr(u8)]
#[derive(PartialEq, Debug, TryFromPrimitive, IntoPrimitive)]
pub enum ScreenLayoutSize {
    Any,
    Small,
    Normal,
    Large,
    XLarge
}

#[repr(u8)]
#[derive(PartialEq, Debug, TryFromPrimitive, IntoPrimitive)]
pub enum ScreenLong {
    Any,
    No,
    Yes,
}

#[repr(u8)]
#[derive(PartialEq, Debug, TryFromPrimitive, IntoPrimitive)]
pub enum LayoutDir {
    Any,
    Ltr,
    Rtl,
}

bitfield! {
    #[derive(PartialEq, NomLE)]
    pub struct UiModeBit(pub u8): Debug {
        pub ui_type: u8 [try UiType] @ 4..=7,
        pub night: u8 [try Night] @ 2..=3,
    }
}

#[repr(u8)]
#[derive(PartialEq, Debug, TryFromPrimitive, IntoPrimitive)]
pub enum UiType {
    Any,
    No,
    Yes,
}

#[repr(u8)]
#[derive(PartialEq, Debug, TryFromPrimitive, IntoPrimitive)]
pub enum Night {
    Any,
    No,
    Yes,
}

#[repr(C)]
#[derive(PartialEq, Debug, NomLE)]
pub struct ScreenSizeDp {
    pub screen_width_dp: u16,
    pub screen_height_dp: u16,
}

#[repr(C)]
#[derive(PartialEq, Debug, NomLE)]
pub struct ScreenConfig2 {
    pub screen_layout_2: u8,
    #[nom(AlignAfter(4))]
    pub screen_height_dp: u8,
}