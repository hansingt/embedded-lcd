use super::{AsyncInterface, BlockingInterface, Font, Interface, Lines};
use core::fmt::{Debug, Formatter};
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{ErrorType, OutputPin};
use embedded_hal_async::delay::DelayNs as AsyncDelayNs;

pub enum Parallel8BitsError<
    D0: ErrorType,
    D1: ErrorType,
    D2: ErrorType,
    D3: ErrorType,
    D4: ErrorType,
    D5: ErrorType,
    D6: ErrorType,
    D7: ErrorType,
    E: ErrorType,
    RS: ErrorType,
> {
    EError(E::Error),
    RSError(RS::Error),
    D0Error(D0::Error),
    D1Error(D1::Error),
    D2Error(D2::Error),
    D3Error(D3::Error),
    D4Error(D4::Error),
    D5Error(D5::Error),
    D6Error(D6::Error),
    D7Error(D7::Error),
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS> Debug
    for Parallel8BitsError<D0, D1, D2, D3, D4, D5, D6, D7, E, RS>
where
    D0: ErrorType,
    D1: ErrorType,
    D2: ErrorType,
    D3: ErrorType,
    D4: ErrorType,
    D5: ErrorType,
    D6: ErrorType,
    D7: ErrorType,
    E: ErrorType,
    RS: ErrorType,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Parallel8BitsError::EError(e) => write!(f, "{:?}", e),
            Parallel8BitsError::RSError(e) => write!(f, "{:?}", e),
            Parallel8BitsError::D0Error(e) => write!(f, "{:?}", e),
            Parallel8BitsError::D1Error(e) => write!(f, "{:?}", e),
            Parallel8BitsError::D2Error(e) => write!(f, "{:?}", e),
            Parallel8BitsError::D3Error(e) => write!(f, "{:?}", e),
            Parallel8BitsError::D4Error(e) => write!(f, "{:?}", e),
            Parallel8BitsError::D5Error(e) => write!(f, "{:?}", e),
            Parallel8BitsError::D6Error(e) => write!(f, "{:?}", e),
            Parallel8BitsError::D7Error(e) => write!(f, "{:?}", e),
        }
    }
}

pub struct Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, DELAY>
where
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    E: OutputPin,
    RS: OutputPin,
{
    d0: D0,
    d1: D1,
    d2: D2,
    d3: D3,
    d4: D4,
    d5: D5,
    d6: D6,
    d7: D7,
    e: E,
    rs: RS,
    delay: DELAY,
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, DELAY>
    Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, DELAY>
where
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    E: OutputPin,
    RS: OutputPin,
{
    #[allow(clippy::complexity)]
    fn set_outputs(
        &mut self,
        data: u8,
    ) -> Result<(), Parallel8BitsError<D0, D1, D2, D3, D4, D5, D6, D7, E, RS>> {
        // Set the data bits
        match data & 0b1000_0000 {
            0 => self.d7.set_low().map_err(Parallel8BitsError::D7Error),
            _ => self.d7.set_high().map_err(Parallel8BitsError::D7Error),
        }?;
        match data & 0b0100_0000 {
            0 => self.d6.set_low().map_err(Parallel8BitsError::D6Error),
            _ => self.d6.set_high().map_err(Parallel8BitsError::D6Error),
        }?;
        match data & 0b0010_0000 {
            0 => self.d5.set_low().map_err(Parallel8BitsError::D5Error),
            _ => self.d5.set_high().map_err(Parallel8BitsError::D5Error),
        }?;
        match data & 0b0001_0000 {
            0 => self.d4.set_low().map_err(Parallel8BitsError::D4Error),
            _ => self.d4.set_high().map_err(Parallel8BitsError::D4Error),
        }?;
        match data & 0b0000_1000 {
            0 => self.d3.set_low().map_err(Parallel8BitsError::D3Error),
            _ => self.d3.set_high().map_err(Parallel8BitsError::D3Error),
        }?;
        match data & 0b0000_0100 {
            0 => self.d2.set_low().map_err(Parallel8BitsError::D2Error),
            _ => self.d2.set_high().map_err(Parallel8BitsError::D2Error),
        }?;
        match data & 0b0000_0010 {
            0 => self.d1.set_low().map_err(Parallel8BitsError::D1Error),
            _ => self.d1.set_high().map_err(Parallel8BitsError::D1Error),
        }?;
        match data & 0b0000_0001 {
            0 => self.d0.set_low().map_err(Parallel8BitsError::D0Error),
            _ => self.d0.set_high().map_err(Parallel8BitsError::D0Error),
        }
    }
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, DELAY>
    Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, DELAY>
where
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    E: OutputPin,
    RS: OutputPin,
    DELAY: DelayNs,
{
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn new(
        d0: D0,
        d1: D1,
        d2: D2,
        d3: D3,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
        e: E,
        rs: RS,
        delay: DELAY,
    ) -> Self {
        Self {
            d0,
            d1,
            d2,
            d3,
            d4,
            d5,
            d6,
            d7,
            e,
            rs,
            delay,
        }
    }

    #[allow(clippy::complexity)]
    fn write_byte(
        &mut self,
        data: u8,
    ) -> Result<(), Parallel8BitsError<D0, D1, D2, D3, D4, D5, D6, D7, E, RS>> {
        // Set the output pin levels
        self.set_outputs(data)?;
        // Open the latch
        self.e.set_high().map_err(Parallel8BitsError::EError)?;
        // Wait for the controller to fetch the data
        self.delay.delay_ns(500);
        // Close the latch
        self.e.set_low().map_err(Parallel8BitsError::EError)?;
        // Wait until we can send the next data
        self.delay.delay_ns(500);
        Ok(())
    }
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, DELAY> Interface
    for Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, DELAY>
where
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    E: OutputPin,
    RS: OutputPin,
{
    type Error = Parallel8BitsError<D0, D1, D2, D3, D4, D5, D6, D7, E, RS>;
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, DELAY> DelayNs
    for Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, DELAY>
where
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    E: OutputPin,
    RS: OutputPin,
    DELAY: DelayNs,
{
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        self.delay.delay_ns(ns);
    }
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, DELAY> BlockingInterface
    for Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, DELAY>
where
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    E: OutputPin,
    RS: OutputPin,
    DELAY: DelayNs,
{
    fn initialize(&mut self, lines: Lines, font: Font) -> Result<(), Self::Error> {
        self.write(0b0011_0000, true)?;
        self.delay.delay_us(4500);
        self.write(0b0011_0000, true)?;
        self.delay.delay_us(150);
        self.write(0b0011_0000, true)?;
        self.write(0b0011_0000 | lines as u8 | font as u8, true)
    }

    fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error> {
        match command {
            true => self.rs.set_low().map_err(Parallel8BitsError::RSError),
            false => self.rs.set_high().map_err(Parallel8BitsError::RSError),
        }?;
        // Wait for the address to settle
        self.delay.delay_us(60);
        // We want to write data
        self.write_byte(data)
    }
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, DELAY>
    Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, DELAY>
where
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    E: OutputPin,
    RS: OutputPin,
    DELAY: AsyncDelayNs,
{
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn new_async(
        d0: D0,
        d1: D1,
        d2: D2,
        d3: D3,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
        e: E,
        rs: RS,
        delay: DELAY,
    ) -> Self {
        Self {
            d0,
            d1,
            d2,
            d3,
            d4,
            d5,
            d6,
            d7,
            e,
            rs,
            delay,
        }
    }

    async fn write_byte_async(
        &mut self,
        data: u8,
    ) -> Result<(), Parallel8BitsError<D0, D1, D2, D3, D4, D5, D6, D7, E, RS>> {
        // Set the output pin levels
        self.set_outputs(data)?;
        // Open the latch
        self.e.set_high().map_err(Parallel8BitsError::EError)?;
        // Wait for the controller to fetch the data
        self.delay.delay_ns(500).await;
        // Close the latch
        self.e.set_low().map_err(Parallel8BitsError::EError)?;
        // Wait until we can send the next data
        self.delay.delay_ns(500).await;
        Ok(())
    }
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, DELAY> AsyncDelayNs
    for Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, DELAY>
where
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    E: OutputPin,
    RS: OutputPin,
    DELAY: AsyncDelayNs,
{
    #[inline]
    async fn delay_ns(&mut self, ns: u32) {
        self.delay.delay_ns(ns).await;
    }
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, DELAY> AsyncInterface
    for Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, DELAY>
where
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    E: OutputPin,
    RS: OutputPin,
    DELAY: AsyncDelayNs,
{
    async fn initialize(&mut self, lines: Lines, font: Font) -> Result<(), Self::Error> {
        self.write(0b0011_0000, true).await?;
        self.delay.delay_us(4500).await;
        self.write(0b0011_0000, true).await?;
        self.delay.delay_us(150).await;
        self.write(0b0011_0000, true).await?;
        self.write(0b0011_0000 | lines as u8 | font as u8, true)
            .await
    }

    async fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error> {
        match command {
            true => self.rs.set_low().map_err(Parallel8BitsError::RSError),
            false => self.rs.set_high().map_err(Parallel8BitsError::RSError),
        }?;
        // Wait for the address to settle
        self.delay.delay_us(60).await;
        // We want to write data
        self.write_byte_async(data).await
    }
}
