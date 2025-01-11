use crate::{Font, Lines};
use core::future::Future;
use embedded_hal::delay::DelayNs;
use embedded_hal_async::delay::DelayNs as AsyncDelayNs;

pub trait Interface {
    type Error;
}

pub trait BlockingInterface: Interface + DelayNs {
    fn initialize(&mut self, lines: Lines, font: Font) -> Result<(), Self::Error>;
    fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error>;
    fn backlight(&mut self, enable: bool) -> Result<(), Self::Error>;
}

pub trait AsyncInterface: Interface + AsyncDelayNs {
    fn initialize(
        &mut self,
        lines: Lines,
        font: Font,
    ) -> impl Future<Output = Result<(), Self::Error>>;
    fn write(&mut self, data: u8, command: bool) -> impl Future<Output = Result<(), Self::Error>>;
    fn backlight(&mut self, enable: bool) -> impl Future<Output = Result<(), Self::Error>>;
}

// Re-exports
mod i2c;
mod parallel_eight_bits;
mod parallel_four_bits;

pub use i2c::I2c;
pub use parallel_eight_bits::*;
pub use parallel_four_bits::*;
