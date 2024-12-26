use crate::{Font, Lines};

pub trait Interface {
    type Error;
}

pub trait BlockingInterface: Interface {
    fn initialize(&mut self, lines: Lines, font: Font) -> Result<(), Self::Error>;
    fn write_command(&mut self, command: u8) -> Result<(), Self::Error>;
    fn write_data(&mut self, command: u8) -> Result<(), Self::Error>;
    fn backlight(&mut self, enable: bool) -> Result<(), Self::Error>;
}

// Re-exports
mod i2c;
pub use i2c::I2c;
