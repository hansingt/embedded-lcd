#![cfg_attr(not(test), no_std)]

pub mod interfaces;

mod display;

use core::fmt::Formatter;
pub use display::Display;

mod private {
    pub trait Sealed {}
}

pub trait Mode: private::Sealed {}

#[derive(Debug)]
pub struct Blocking {}
impl private::Sealed for Blocking {}
impl Mode for Blocking {}

#[derive(Debug)]
pub struct Async {}
impl private::Sealed for Async {}
impl Mode for Async {}

#[repr(u8)]
#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Font {
    #[default]
    _5x8 = 0,
    _5x10 = 4,
}

#[repr(u8)]
#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Lines {
    #[default]
    _1 = 0,
    _2 = 8,
}

impl core::fmt::Display for Lines {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Lines::_1 => write!(f, "one"),
            Lines::_2 => write!(f, "two"),
        }
    }
}

#[repr(u8)]
#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Shift {
    #[default]
    Cursor = 0,
    Display = 1,
}

impl core::fmt::Display for Shift {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Shift::Cursor => write!(f, "cursor"),
            Shift::Display => write!(f, "display"),
        }
    }
}

#[repr(u8)]
#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ShiftDirection {
    Left = 0,
    #[default]
    Right = 2,
}

impl core::fmt::Display for ShiftDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ShiftDirection::Left => write!(f, "left"),
            ShiftDirection::Right => write!(f, "right"),
        }
    }
}

#[repr(u8)]
#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Cursor {
    #[default]
    Disabled = 0,
    Enabled = 2,
    Blinking = 3,
}
