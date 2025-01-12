use crate::private::Sealed;

pub trait BusWidth: Sealed {
    const WIDTH: u8;
}
#[derive(Debug)]
pub struct FourBitBus {}

impl Sealed for FourBitBus {}
impl BusWidth for FourBitBus {
    const WIDTH: u8 = 0b0010_0000;
}

#[derive(Debug)]
pub struct EightBitBus {}

impl Sealed for EightBitBus {}
impl BusWidth for EightBitBus {
    const WIDTH: u8 = 0b0011_0000;
}

pub trait ErrorType {
    type Error;
}

// -------------------------------------------------------------------------------------------------
// BLOCKING INTERFACE
// -------------------------------------------------------------------------------------------------
pub trait BlockingBus<Width: BusWidth>: ErrorType + embedded_hal::delay::DelayNs {
    fn initialize(&mut self) -> Result<(), Self::Error>;
    fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error>;
}

pub trait BlockingInterface<Width: BusWidth>: BlockingBus<Width> {
    fn backlight(&mut self, enable: bool) -> Result<(), Self::Error>;
}

pub trait Blocking4BitBus: ErrorType + embedded_hal::delay::DelayNs {
    fn write_nibble(&mut self, data: u8) -> Result<(), Self::Error>;
    fn set_command_mode(&mut self, command: bool) -> Result<(), Self::Error>;

    #[inline]
    fn write_nibble_logged(&mut self, data: u8) -> Result<(), Self::Error> {
        #[cfg(feature = "log")]
        log::trace!("Writing nibble {:#06b}", data & 0x0F);
        self.write_nibble(data)
    }

    fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error> {
        #[cfg(feature = "log")]
        log::debug!("Writing '{:#010b}' to LCD Display", data);
        // Set command / data mode
        self.set_command_mode(command)?;
        // Write the data in two nibbles in MSB first order.
        self.write_nibble_logged(data >> 4)?;
        self.write_nibble_logged(data)
    }

    fn initialize(&mut self) -> Result<(), Self::Error> {
        self.write_nibble_logged(0b0011)?;
        self.delay_us(4500);
        self.write_nibble_logged(0b0011)?;
        self.delay_us(150);
        self.write_nibble_logged(0b0011)?;
        self.write_nibble_logged(0b0010)
    }
}
impl<T: Blocking4BitBus> BlockingBus<FourBitBus> for T {
    #[inline]
    fn initialize(&mut self) -> Result<(), Self::Error> {
        Blocking4BitBus::initialize(self)
    }

    #[inline]
    fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error> {
        Blocking4BitBus::write(self, data, command)
    }
}

pub trait Blocking8BitBus: ErrorType + embedded_hal::delay::DelayNs {
    fn write_byte(&mut self, data: u8) -> Result<(), Self::Error>;

    fn set_command_mode(&mut self, command: bool) -> Result<(), Self::Error>;

    fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error> {
        #[cfg(feature = "log")]
        log::debug!("Writing '{:#010b}' to LCD Display", data);
        // Set command / data mode
        self.set_command_mode(command)?;
        self.delay_ns(60);

        // Write the data
        self.write_byte(data)
    }

    fn initialize(&mut self) -> Result<(), Self::Error> {
        self.write(0b0011_0000, true)?;
        self.delay_us(4500);
        self.write(0b0011_0000, true)?;
        self.delay_us(150);
        self.write(0b0011_0000, true)
    }
}
impl<T: Blocking8BitBus> BlockingBus<EightBitBus> for T {
    #[inline]
    fn initialize(&mut self) -> Result<(), Self::Error> {
        Blocking8BitBus::initialize(self)
    }

    #[inline]
    fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error> {
        Blocking8BitBus::write(self, data, command)
    }
}
// -------------------------------------------------------------------------------------------------
// ASYNC INTERFACE
// -------------------------------------------------------------------------------------------------
pub trait AsyncBus<Width: BusWidth>: ErrorType + embedded_hal_async::delay::DelayNs {
    async fn initialize(&mut self) -> Result<(), Self::Error>;
    async fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error>;
}

pub trait AsyncInterface<Width: BusWidth>: AsyncBus<Width> {
    async fn backlight(&mut self, enable: bool) -> Result<(), Self::Error>;
}

pub trait Async4BitBus: ErrorType + embedded_hal_async::delay::DelayNs {
    async fn write_nibble(&mut self, data: u8) -> Result<(), Self::Error>;
    async fn set_command_mode(&mut self, command: bool) -> Result<(), Self::Error>;

    #[inline]
    async fn write_nibble_logged(&mut self, data: u8) -> Result<(), Self::Error> {
        #[cfg(feature = "log")]
        log::trace!("Writing nibble {:#06b}", data & 0x0F);
        self.write_nibble(data).await
    }
    async fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error> {
        #[cfg(feature = "log")]
        log::debug!("Writing '{:#010b}' to LCD Display", data);
        // Set command / data mode
        self.set_command_mode(command).await?;
        // Write the data in two nibbles in MSB first order.
        self.write_nibble_logged(data >> 4).await?;
        self.write_nibble_logged(data).await?;
        Ok(())
    }

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        self.write_nibble_logged(0b0011).await?;
        self.delay_us(4500).await;
        self.write_nibble_logged(0b0011).await?;
        self.delay_us(150).await;
        self.write_nibble_logged(0b0011).await?;
        self.write_nibble_logged(0b0010).await
    }
}

impl<T: Async4BitBus> AsyncBus<FourBitBus> for T {
    async fn initialize(&mut self) -> Result<(), Self::Error> {
        Async4BitBus::initialize(self).await
    }

    async fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error> {
        Async4BitBus::write(self, data, command).await
    }
}
pub trait Async8BitBus: ErrorType + embedded_hal_async::delay::DelayNs {
    async fn write_byte(&mut self, data: u8) -> Result<(), Self::Error>;

    async fn set_command_mode(&mut self, command: bool) -> Result<(), Self::Error>;

    async fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error> {
        #[cfg(feature = "log")]
        log::debug!("Writing '{:#010b}' to LCD Display", data);
        // Set command / data mode
        self.set_command_mode(command).await?;
        // Write the data
        self.write_byte(data).await
    }

    async fn initialize(&mut self) -> Result<(), Self::Error> {
        self.write_byte(0b0011_0000).await?;
        self.delay_us(4500).await;
        self.write_byte(0b0011_0000).await?;
        self.delay_us(150).await;
        self.write_byte(0b0011_0000).await
    }
}
impl<T: Async8BitBus> AsyncBus<EightBitBus> for T {
    #[inline]
    async fn initialize(&mut self) -> Result<(), Self::Error> {
        Async8BitBus::initialize(self).await
    }

    #[inline]
    async fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error> {
        Async8BitBus::write(self, data, command).await
    }
}

// Re-exports
mod i2c;
mod parallel_eight_bits;
mod parallel_four_bits;

pub use i2c::I2c;
pub use parallel_eight_bits::*;
pub use parallel_four_bits::*;
