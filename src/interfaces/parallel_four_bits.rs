use crate::async_output_pin::AsyncOutputPin;
use crate::interfaces::{
    Async4BitBus, AsyncInterface, Blocking4BitBus, BlockingInterface, ErrorType, FourBitBus,
};
use crate::{Async, Blocking, Mode};
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use embedded_hal::digital::OutputPin;

pub enum Parallel4BitsError<
    D7: embedded_hal::digital::ErrorType,
    D6: embedded_hal::digital::ErrorType,
    D5: embedded_hal::digital::ErrorType,
    D4: embedded_hal::digital::ErrorType,
    E: embedded_hal::digital::ErrorType,
    RS: embedded_hal::digital::ErrorType,
    B: embedded_hal::digital::ErrorType,
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
    D7: embedded_hal::digital::ErrorType,
    D6: embedded_hal::digital::ErrorType,
    D5: embedded_hal::digital::ErrorType,
    D4: embedded_hal::digital::ErrorType,
    E: embedded_hal::digital::ErrorType,
    RS: embedded_hal::digital::ErrorType,
    B: embedded_hal::digital::ErrorType,
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

impl<D7, D6, D5, D4, E, RS, B, DELAY, M: Mode> ErrorType
    for Parallel4Bits<D7, D6, D5, D4, E, RS, B, DELAY, M>
where
    D7: embedded_hal::digital::ErrorType,
    D6: embedded_hal::digital::ErrorType,
    D5: embedded_hal::digital::ErrorType,
    D4: embedded_hal::digital::ErrorType,
    E: embedded_hal::digital::ErrorType,
    RS: embedded_hal::digital::ErrorType,
    B: embedded_hal::digital::ErrorType,
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
    fn set_outputs(&mut self, data: u8) -> Result<(), <Self as ErrorType>::Error> {
        // Set the data bits
        self.d7
            .set_state((data & (1 << 3) != 0).into())
            .map_err(Parallel4BitsError::D7Error)?;
        self.d6
            .set_state((data & (1 << 2) != 0).into())
            .map_err(Parallel4BitsError::D6Error)?;
        self.d5
            .set_state((data & (1 << 1) != 0).into())
            .map_err(Parallel4BitsError::D5Error)?;
        self.d4
            .set_state((data & (1 << 0) != 0).into())
            .map_err(Parallel4BitsError::D4Error)
    }

    #[inline]
    fn _backlight(&mut self, enable: bool) -> Result<(), <Self as ErrorType>::Error> {
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

impl<D7, D6, D5, D4, E, RS, B, DELAY> Blocking4BitBus
    for Parallel4Bits<D7, D6, D5, D4, E, RS, B, DELAY, Blocking>
where
    B: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    DELAY: embedded_hal::delay::DelayNs,
    E: OutputPin,
    RS: OutputPin,
{
    fn write_nibble(&mut self, data: u8) -> Result<(), Self::Error> {
        // Set the output pin levels
        self.set_outputs(data)?;
        // Open the latch
        self.e.set_high().map_err(Parallel4BitsError::EError)?;
        self.delay.delay_ns(500);
        // Close the latch
        self.e.set_low().map_err(Parallel4BitsError::EError)?;
        self.delay.delay_ns(500);
        Ok(())
    }

    #[inline]
    fn set_command_mode(&mut self, command: bool) -> Result<(), Self::Error> {
        self.rs
            .set_state((!command).into())
            .map_err(Parallel4BitsError::RSError)
    }
}

impl<D7, D6, D5, D4, E, RS, B, DELAY> BlockingInterface<FourBitBus>
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
        // Set the data bits
        self.d7
            .set_state((data & (1 << 3) != 0).into())
            .await
            .map_err(Parallel4BitsError::D7Error)?;
        self.d6
            .set_state((data & (1 << 2) != 0).into())
            .await
            .map_err(Parallel4BitsError::D6Error)?;
        self.d5
            .set_state((data & (1 << 1) != 0).into())
            .await
            .map_err(Parallel4BitsError::D5Error)?;
        self.d4
            .set_state((data & (1 << 0) != 0).into())
            .await
            .map_err(Parallel4BitsError::D4Error)
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

impl<D7, D6, D5, D4, E, RS, B, DELAY> Async4BitBus
    for Parallel4Bits<D7, D6, D5, D4, E, RS, B, DELAY, Async>
where
    B: AsyncOutputPin,
    D4: AsyncOutputPin,
    D5: AsyncOutputPin,
    D6: AsyncOutputPin,
    D7: AsyncOutputPin,
    DELAY: embedded_hal_async::delay::DelayNs,
    E: AsyncOutputPin,
    RS: AsyncOutputPin,
{
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
        self.delay.delay_ns(500).await;
        Ok(())
    }

    #[inline]
    async fn set_command_mode(&mut self, command: bool) -> Result<(), Self::Error> {
        self.rs
            .set_state((!command).into())
            .await
            .map_err(Parallel4BitsError::RSError)
    }
}

impl<D7, D6, D5, D4, E, RS, B, DELAY> AsyncInterface<FourBitBus>
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
    async fn backlight(&mut self, enable: bool) -> Result<(), Self::Error> {
        self._backlight(enable).await
    }
}
