use crate::async_output_pin::AsyncOutputPin;
use crate::interfaces::{AsyncInterface, BlockingInterface, Interface};
use crate::{Async, Blocking, Font, Lines, Mode};
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use embedded_hal::digital::{ErrorType, OutputPin};

pub enum Parallel4BitsError<
    D7: ErrorType,
    D6: ErrorType,
    D5: ErrorType,
    D4: ErrorType,
    E: ErrorType,
    RS: ErrorType,
    B: ErrorType,
> {
    EError(E::Error),
    RSError(RS::Error),
    D7Error(D7::Error),
    D6Error(D6::Error),
    D5Error(D5::Error),
    D4Error(D4::Error),
    BacklightError(B::Error),
}

impl<D7, D6, D5, D4, E, RS, B> Debug for Parallel4BitsError<D7, D6, D5, D4, E, RS, B>
where
    D7: ErrorType,
    D6: ErrorType,
    D5: ErrorType,
    D4: ErrorType,
    E: ErrorType,
    RS: ErrorType,
    B: ErrorType,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Parallel4BitsError::EError(e) => write!(f, "{:?}", e),
            Parallel4BitsError::RSError(e) => write!(f, "{:?}", e),
            Parallel4BitsError::D7Error(e) => write!(f, "{:?}", e),
            Parallel4BitsError::D6Error(e) => write!(f, "{:?}", e),
            Parallel4BitsError::D5Error(e) => write!(f, "{:?}", e),
            Parallel4BitsError::D4Error(e) => write!(f, "{:?}", e),
            Parallel4BitsError::BacklightError(e) => write!(f, "{:?}", e),
        }
    }
}

#[derive(Debug)]
pub struct Parallel4Bits<D7, D6, D5, D4, E, RS, B, DELAY, M: Mode> {
    d7: D7,
    d6: D6,
    d5: D5,
    d4: D4,
    e: E,
    rs: RS,
    delay: DELAY,
    backlight: Option<B>,
    _mode: PhantomData<M>,
}

impl<D7, D6, D5, D4, E, RS, B, DELAY, M: Mode> Parallel4Bits<D7, D6, D5, D4, E, RS, B, DELAY, M> {
    pub fn with_backlight(mut self, backlight: B) -> Self {
        self.backlight = Some(backlight);
        self
    }
}

impl<D7, D6, D5, D4, E, RS, B, DELAY, M: Mode> Interface
    for Parallel4Bits<D7, D6, D5, D4, E, RS, B, DELAY, M>
where
    D7: ErrorType,
    D6: ErrorType,
    D5: ErrorType,
    D4: ErrorType,
    E: ErrorType,
    RS: ErrorType,
    B: ErrorType,
{
    type Error = Parallel4BitsError<D7, D6, D5, D4, E, RS, B>;
}

// -------------------------------------------------------------------------------------------------
// BLOCKING INTERFACE
// -------------------------------------------------------------------------------------------------
impl<D7, D6, D5, D4, E, RS, B, DELAY> Parallel4Bits<D7, D6, D5, D4, E, RS, B, DELAY, Blocking>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    E: OutputPin,
    RS: OutputPin,
    B: OutputPin,
{
    #[allow(clippy::complexity)]
    fn set_outputs(
        &mut self,
        data: u8,
    ) -> Result<(), Parallel4BitsError<D7, D6, D5, D4, E, RS, B>> {
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

    #[allow(clippy::complexity)]
    #[inline]
    fn _backlight(
        &mut self,
        enable: bool,
    ) -> Result<(), Parallel4BitsError<D7, D6, D5, D4, E, RS, B>> {
        if self.backlight.is_some() {
            self.backlight
                .as_mut()
                .unwrap()
                .set_state(enable.into())
                .map_err(Parallel4BitsError::BacklightError)?;
        }
        Ok(())
    }
}
impl<D7, D6, D5, D4, E, RS, B, DELAY> Parallel4Bits<D7, D6, D5, D4, E, RS, B, DELAY, Blocking>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    E: OutputPin,
    RS: OutputPin,
    B: OutputPin,
    DELAY: embedded_hal::delay::DelayNs,
{
    #[inline]
    pub fn new(d7: D7, d6: D6, d5: D5, d4: D4, e: E, rs: RS, delay: DELAY) -> Self {
        Self {
            d7,
            d6,
            d5,
            d4,
            e,
            rs,
            delay,
            backlight: None,
            _mode: PhantomData,
        }
    }

    #[allow(clippy::complexity)]
    fn write_nibble(
        &mut self,
        data: u8,
    ) -> Result<(), Parallel4BitsError<D7, D6, D5, D4, E, RS, B>> {
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

impl<D7, D6, D5, D4, E, RS, B, DELAY> embedded_hal::delay::DelayNs
    for Parallel4Bits<D7, D6, D5, D4, E, RS, B, DELAY, Blocking>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    E: OutputPin,
    RS: OutputPin,
    B: OutputPin,
    DELAY: embedded_hal::delay::DelayNs,
{
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        self.delay.delay_ns(ns);
    }
}

impl<D7, D6, D5, D4, E, RS, B, DELAY> BlockingInterface
    for Parallel4Bits<D7, D6, D5, D4, E, RS, B, DELAY, Blocking>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    E: OutputPin,
    RS: OutputPin,
    B: OutputPin,
    DELAY: embedded_hal::delay::DelayNs,
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

    fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error> {
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

    #[inline]
    fn backlight(&mut self, enable: bool) -> Result<(), Self::Error> {
        self._backlight(enable)
    }
}

// -------------------------------------------------------------------------------------------------
// ASYNC INTERFACE
// -------------------------------------------------------------------------------------------------
impl<D7, D6, D5, D4, E, RS, B, DELAY> Parallel4Bits<D7, D6, D5, D4, E, RS, B, DELAY, Async>
where
    D7: AsyncOutputPin,
    D6: AsyncOutputPin,
    D5: AsyncOutputPin,
    D4: AsyncOutputPin,
    E: AsyncOutputPin,
    RS: AsyncOutputPin,
    B: AsyncOutputPin,
{
    async fn set_outputs(
        &mut self,
        data: u8,
    ) -> Result<(), Parallel4BitsError<D7, D6, D5, D4, E, RS, B>> {
        #[cfg(feature = "log")]
        log::trace!("Writing nibble {:#06b}", data & 0x0F);
        // Set the data bits
        match data & 0b1000 {
            0 => self.d7.set_low().await.map_err(Parallel4BitsError::D7Error),
            _ => self
                .d7
                .set_high()
                .await
                .map_err(Parallel4BitsError::D7Error),
        }?;
        match data & 0b0100 {
            0 => self.d6.set_low().await.map_err(Parallel4BitsError::D6Error),
            _ => self
                .d6
                .set_high()
                .await
                .map_err(Parallel4BitsError::D6Error),
        }?;
        match data & 0b0010 {
            0 => self.d5.set_low().await.map_err(Parallel4BitsError::D5Error),
            _ => self
                .d5
                .set_high()
                .await
                .map_err(Parallel4BitsError::D5Error),
        }?;
        match data & 0b0001 {
            0 => self.d4.set_low().await.map_err(Parallel4BitsError::D4Error),
            _ => self
                .d4
                .set_high()
                .await
                .map_err(Parallel4BitsError::D4Error),
        }
    }

    #[inline]
    async fn _backlight(
        &mut self,
        enable: bool,
    ) -> Result<(), Parallel4BitsError<D7, D6, D5, D4, E, RS, B>> {
        if self.backlight.is_some() {
            self.backlight
                .as_mut()
                .unwrap()
                .set_state(enable.into())
                .await
                .map_err(Parallel4BitsError::BacklightError)?;
        }
        Ok(())
    }
}

impl<D7, D6, D5, D4, E, RS, B, DELAY> Parallel4Bits<D7, D6, D5, D4, E, RS, B, DELAY, Async>
where
    D7: AsyncOutputPin,
    D6: AsyncOutputPin,
    D5: AsyncOutputPin,
    D4: AsyncOutputPin,
    E: AsyncOutputPin,
    RS: AsyncOutputPin,
    B: AsyncOutputPin,
    DELAY: embedded_hal_async::delay::DelayNs,
{
    #[inline]
    pub fn new_async(d7: D7, d6: D6, d5: D5, d4: D4, e: E, rs: RS, delay: DELAY) -> Self {
        Self {
            d7,
            d6,
            d5,
            d4,
            e,
            rs,
            delay,
            backlight: None,
            _mode: PhantomData,
        }
    }

    async fn write_nibble(
        &mut self,
        data: u8,
    ) -> Result<(), Parallel4BitsError<D7, D6, D5, D4, E, RS, B>> {
        // Set the output pin levels
        self.set_outputs(data).await?;
        // Open the latch
        self.e
            .set_high()
            .await
            .map_err(Parallel4BitsError::EError)?;
        // Wait for the controller to fetch the data
        self.delay.delay_ns(500).await;
        // Close the latch
        self.e.set_low().await.map_err(Parallel4BitsError::EError)?;
        // Wait until we can send the next data
        self.delay.delay_ns(500).await;
        Ok(())
    }
}

impl<D7, D6, D5, D4, E, RS, B, DELAY> embedded_hal_async::delay::DelayNs
    for Parallel4Bits<D7, D6, D5, D4, E, RS, B, DELAY, Async>
where
    D7: AsyncOutputPin,
    D6: AsyncOutputPin,
    D5: AsyncOutputPin,
    D4: AsyncOutputPin,
    E: AsyncOutputPin,
    RS: AsyncOutputPin,
    B: AsyncOutputPin,
    DELAY: embedded_hal_async::delay::DelayNs,
{
    #[inline]
    async fn delay_ns(&mut self, ns: u32) {
        self.delay.delay_ns(ns).await;
    }
}

impl<D7, D6, D5, D4, E, RS, B, DELAY> AsyncInterface
    for Parallel4Bits<D7, D6, D5, D4, E, RS, B, DELAY, Async>
where
    D7: AsyncOutputPin,
    D6: AsyncOutputPin,
    D5: AsyncOutputPin,
    D4: AsyncOutputPin,
    E: AsyncOutputPin,
    RS: AsyncOutputPin,
    B: AsyncOutputPin,
    DELAY: embedded_hal_async::delay::DelayNs,
{
    async fn initialize(&mut self, lines: Lines, font: Font) -> Result<(), Self::Error> {
        self.write_nibble(0b0011).await?;
        self.delay.delay_us(4500).await;
        self.write_nibble(0b0011).await?;
        self.delay.delay_us(150).await;
        self.write_nibble(0b0011).await?;
        self.write_nibble(0b0010).await?;

        let function_set = match font {
            Font::_5x10 => 0b0010_0100,
            Font::_5x8 => match lines {
                Lines::_1 => 0b0010_0000,
                Lines::_2 => 0b0010_1000,
            },
        };
        self.write(function_set, true).await
    }

    async fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error> {
        #[cfg(feature = "log")]
        log::debug!("Writing '{:#010b}' to LCD Display", data);
        // Enable data mode
        match command {
            true => self.rs.set_low().await.map_err(Parallel4BitsError::RSError),
            false => self
                .rs
                .set_high()
                .await
                .map_err(Parallel4BitsError::RSError),
        }?;
        // Wait for the address to settle
        self.delay.delay_ns(60).await;
        // Write the data in two nibbles in MSB first order.
        self.write_nibble(data >> 4).await?;
        self.write_nibble(data).await?;
        Ok(())
    }

    #[inline]
    async fn backlight(&mut self, enable: bool) -> Result<(), Self::Error> {
        self._backlight(enable).await
    }
}
