use crate::interfaces::{AsyncInterface, BlockingInterface, Interface};
use crate::{Font, Lines};
use core::fmt::{Debug, Formatter};
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{ErrorType, OutputPin};
use embedded_hal_async::delay::DelayNs as AsyncDelayNs;

pub enum Parallel4BitsError<
    D7: ErrorType,
    D6: ErrorType,
    D5: ErrorType,
    D4: ErrorType,
    E: ErrorType,
    RS: ErrorType,
> {
    EError(E::Error),
    RSError(RS::Error),
    D7Error(D7::Error),
    D6Error(D6::Error),
    D5Error(D5::Error),
    D4Error(D4::Error),
}

impl<D7, D6, D5, D4, E, RS> Debug for Parallel4BitsError<D7, D6, D5, D4, E, RS>
where
    D7: ErrorType,
    D6: ErrorType,
    D5: ErrorType,
    D4: ErrorType,
    E: ErrorType,
    RS: ErrorType,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Parallel4BitsError::EError(e) => write!(f, "{:?}", e),
            Parallel4BitsError::RSError(e) => write!(f, "{:?}", e),
            Parallel4BitsError::D7Error(e) => write!(f, "{:?}", e),
            Parallel4BitsError::D6Error(e) => write!(f, "{:?}", e),
            Parallel4BitsError::D5Error(e) => write!(f, "{:?}", e),
            Parallel4BitsError::D4Error(e) => write!(f, "{:?}", e),
        }
    }
}

pub struct Parallel4Bits<D7, D6, D5, D4, E, RS, DELAY>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    E: OutputPin,
    RS: OutputPin,
{
    d7: D7,
    d6: D6,
    d5: D5,
    d4: D4,
    e: E,
    rs: RS,
    delay: DELAY,
}

impl<D7, D6, D5, D4, E, RS, DELAY> Parallel4Bits<D7, D6, D5, D4, E, RS, DELAY>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    E: OutputPin,
    RS: OutputPin,
{
    fn set_outputs(&mut self, data: u8) -> Result<(), Parallel4BitsError<D7, D6, D5, D4, E, RS>> {
        #[cfg(feature = "log")]
        log::trace!("Writing nibble {:#06b}", data & 0x0F);
        // Set the data bits
        match data & 0b1000 {
            0 => self.d7.set_low().map_err(Parallel4BitsError::D7Error),
            _ => self.d7.set_high().map_err(Parallel4BitsError::D7Error),
        }?;
        match data & 0b0100 {
            0 => self.d6.set_low().map_err(Parallel4BitsError::D6Error),
            _ => self.d6.set_high().map_err(Parallel4BitsError::D6Error),
        }?;
        match data & 0b0010 {
            0 => self.d5.set_low().map_err(Parallel4BitsError::D5Error),
            _ => self.d5.set_high().map_err(Parallel4BitsError::D5Error),
        }?;
        match data & 0b0001 {
            0 => self.d4.set_low().map_err(Parallel4BitsError::D4Error),
            _ => self.d4.set_high().map_err(Parallel4BitsError::D4Error),
        }
    }
}

impl<D7, D6, D5, D4, E, RS, DELAY> Parallel4Bits<D7, D6, D5, D4, E, RS, DELAY>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    E: OutputPin,
    RS: OutputPin,
    DELAY: DelayNs,
{
    pub fn new(d7: D7, d6: D6, d5: D5, d4: D4, e: E, rs: RS, delay: DELAY) -> Self {
        Parallel4Bits {
            d7,
            d6,
            d5,
            d4,
            e,
            rs,
            delay,
        }
    }

    fn write_nibble(&mut self, data: u8) -> Result<(), Parallel4BitsError<D7, D6, D5, D4, E, RS>> {
        // Set the output pin levels
        self.set_outputs(data)?;
        // Open the latch
        self.e.set_high().map_err(Parallel4BitsError::EError)?;
        self.delay.delay_us(1);
        // Close the latch
        self.e.set_low().map_err(Parallel4BitsError::EError)?;
        Ok(())
    }
}

impl<D7, D6, D5, D4, E, RS, DELAY> Parallel4Bits<D7, D6, D5, D4, E, RS, DELAY>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    E: OutputPin,
    RS: OutputPin,
    DELAY: AsyncDelayNs,
{
    pub fn new_async(d7: D7, d6: D6, d5: D5, d4: D4, e: E, rs: RS, delay: DELAY) -> Self {
        Parallel4Bits {
            d7,
            d6,
            d5,
            d4,
            e,
            rs,
            delay,
        }
    }

    async fn write_nibble_async(
        &mut self,
        data: u8,
    ) -> Result<(), Parallel4BitsError<D7, D6, D5, D4, E, RS>> {
        // Set the output pin levels
        self.set_outputs(data)?;
        // Open the latch
        self.e.set_high().map_err(Parallel4BitsError::EError)?;
        // Wait for the controller to fetch the data
        self.delay.delay_ns(500).await;
        // Close the latch
        self.e.set_low().map_err(Parallel4BitsError::EError)?;
        // Wait until we can send the next data
        self.delay.delay_ns(500).await;
        Ok(())
    }
}

impl<D7, D6, D5, D4, E, RS, DELAY> Interface for Parallel4Bits<D7, D6, D5, D4, E, RS, DELAY>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    E: OutputPin,
    RS: OutputPin,
{
    type Error = Parallel4BitsError<D7, D6, D5, D4, E, RS>;
}

impl<D7, D6, D5, D4, E, RS, DELAY> DelayNs for Parallel4Bits<D7, D6, D5, D4, E, RS, DELAY>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    E: OutputPin,
    RS: OutputPin,
    DELAY: DelayNs,
{
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        self.delay.delay_ns(ns);
    }
}

impl<D7, D6, D5, D4, E, RS, DELAY> BlockingInterface for Parallel4Bits<D7, D6, D5, D4, E, RS, DELAY>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    E: OutputPin,
    RS: OutputPin,
    DELAY: DelayNs,
{
    fn initialize(&mut self, lines: Lines, font: Font) -> Result<(), Self::Error> {
        self.write_nibble(0b0011)?;
        self.delay.delay_us(4500);
        self.write_nibble(0b0011)?;
        self.delay.delay_us(150);
        self.write_nibble(0b0011)?;
        self.write_nibble(0b0010)?;

        let function_set = match font {
            Font::_5x10 => 0b0010_0100,
            Font::_5x8 => match lines {
                Lines::_1 => 0b0010_0000,
                Lines::_2 => 0b0010_1000,
            },
        };
        self.write(function_set, true)
    }

    fn write(
        &mut self,
        data: u8,
        command: bool,
    ) -> Result<(), Parallel4BitsError<D7, D6, D5, D4, E, RS>> {
        #[cfg(feature = "log")]
        log::debug!("Writing '{:#010b}' to LCD Display", data);
        // Enable data mode
        match command {
            true => self.rs.set_low().map_err(Parallel4BitsError::RSError),
            false => self.rs.set_high().map_err(Parallel4BitsError::RSError),
        }?;
        // Wait for the address to settle
        self.delay.delay_ns(60);
        // Write the data in two nibbles in MSB first order.
        self.write_nibble(data >> 4)?;
        self.write_nibble(data)?;
        Ok(())
    }
}

impl<D7, D6, D5, D4, E, RS, DELAY> AsyncDelayNs for Parallel4Bits<D7, D6, D5, D4, E, RS, DELAY>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    E: OutputPin,
    RS: OutputPin,
    DELAY: AsyncDelayNs,
{
    #[inline]
    async fn delay_ns(&mut self, ns: u32) {
        self.delay.delay_ns(ns).await;
    }
}

impl<D7, D6, D5, D4, E, RS, DELAY> AsyncInterface for Parallel4Bits<D7, D6, D5, D4, E, RS, DELAY>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    E: OutputPin,
    RS: OutputPin,
    DELAY: AsyncDelayNs,
{
    async fn initialize(&mut self, lines: Lines, font: Font) -> Result<(), Self::Error> {
        self.write_nibble_async(0b0011).await?;
        self.delay.delay_us(4500).await;
        self.write_nibble_async(0b0011).await?;
        self.delay.delay_us(150).await;
        self.write_nibble_async(0b0011).await?;
        self.write_nibble_async(0b0010).await?;

        let function_set = match font {
            Font::_5x10 => 0b0010_0100,
            Font::_5x8 => match lines {
                Lines::_1 => 0b0010_0000,
                Lines::_2 => 0b0010_1000,
            },
        };
        self.write(function_set, true).await
    }

    async fn write(
        &mut self,
        data: u8,
        command: bool,
    ) -> Result<(), Parallel4BitsError<D7, D6, D5, D4, E, RS>> {
        #[cfg(feature = "log")]
        log::debug!("Writing '{:#010b}' to LCD Display", data);
        // Enable data mode
        match command {
            true => self.rs.set_low().map_err(Parallel4BitsError::RSError),
            false => self.rs.set_high().map_err(Parallel4BitsError::RSError),
        }?;
        // Wait for the address to settle
        self.delay.delay_ns(60).await;
        // Write the data in two nibbles in MSB first order.
        self.write_nibble_async(data >> 4).await?;
        self.write_nibble_async(data).await?;
        Ok(())
    }
}
