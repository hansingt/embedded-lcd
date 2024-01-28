mod parallel_eight_bits;
mod parallel_four_bits;

#[derive(Debug, PartialEq, Eq)]
pub enum Font {
    FiveTimesEightDots = 0b0010_0000,
    FiveTimesTenDots = 0b0010_0100,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Lines {
    One = 0b0010_0000,
    Two = 0b0010_1000,
}

pub trait Interface {
    type Error;

    fn initialize(&mut self, lines: Lines, font: Font) -> Result<(), Self::Error>;
    fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error>;
}

pub use parallel_eight_bits::{Parallel8Bits, Parallel8BitsError};
pub use parallel_four_bits::{Parallel4Bits, Parallel4BitsError};
