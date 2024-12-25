mod i2c;

pub trait Interface {
    type Error;
}

pub trait BlockingInterface: Interface {
    fn initialize(&mut self, lines: Lines, font: Font, delay: &mut impl DelayNs) -> Result<(), Self::Error>;
    fn write_command(&mut self, command: u8, delay: &mut impl DelayNs) -> Result<(), Self::Error>;
    fn write_data(&mut self, command: u8, delay: &mut impl DelayNs) -> Result<(), Self::Error>;
    fn backlight(&mut self, enable: bool) -> Result<(), Self::Error>;
}

use embedded_hal::delay::DelayNs;
// Re-exports
pub use i2c::I2c;
use crate::{Font, Lines};
