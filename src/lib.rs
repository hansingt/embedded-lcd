#![cfg_attr(not(test), no_std)]

pub mod interfaces;

mod display;

use core::fmt::Formatter;
pub use display::Display;

#[repr(u8)]
#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Font {
    #[default]
    Font5x8 = 0,
    Font5x10 = 4,
}

#[repr(u8)]
#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Lines {
    #[default]
    One = 0,
    Two = 8,
}

impl core::fmt::Display for Lines {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Lines::One => write!(f, "one"),
            Lines::Two => write!(f, "two"),
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
