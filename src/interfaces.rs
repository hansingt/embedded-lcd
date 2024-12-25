mod i2c;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum InterfaceWidth {
    FourBit,
    EightBit,
}

pub trait Interface {
    type Error;

    fn interface_width() -> InterfaceWidth;
}

pub trait BlockingInterface: Interface {
    fn write_command(&mut self, command: u8, delay: &mut impl DelayNs) -> Result<(), Self::Error>;
    fn write_data(&mut self, command: u8, delay: &mut impl DelayNs) -> Result<(), Self::Error>;

    fn backlight(&mut self, enable: bool) -> Result<(), Self::Error>;
}

use embedded_hal::delay::DelayNs;
// Re-exports
pub use i2c::I2c;
