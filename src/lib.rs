#![cfg_attr(not(test), no_std)]

pub mod interfaces;

mod display;
pub use display::Display;

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Font {
    #[default]
    Font5x8 = 0b0000_0000,
    Font5x10 = 0b0000_0100,
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Lines {
    #[default]
    One = 0b0000_0000,
    Two = 0b0000_1000,
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ShiftDirection {
    #[default]
    Right = 0b0000_0110,
    Left = 0b0000_0100,
}
