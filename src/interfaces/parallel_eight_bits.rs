use super::{
    Async8BitBus, AsyncInterface, Blocking8BitBus, BlockingInterface, EightBitBus, ErrorType,
};
use crate::async_output_pin::AsyncOutputPin;
use crate::{Async, Blocking, Mode};
use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;
use embedded_hal::digital::OutputPin;

pub enum Parallel8BitsError<
    D0: embedded_hal::digital::ErrorType,
    D1: embedded_hal::digital::ErrorType,
    D2: embedded_hal::digital::ErrorType,
    D3: embedded_hal::digital::ErrorType,
    D4: embedded_hal::digital::ErrorType,
    D5: embedded_hal::digital::ErrorType,
    D6: embedded_hal::digital::ErrorType,
    D7: embedded_hal::digital::ErrorType,
    E: embedded_hal::digital::ErrorType,
    RS: embedded_hal::digital::ErrorType,
    B: embedded_hal::digital::ErrorType,
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
    BacklightError(B::Error),
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B> Debug
    for Parallel8BitsError<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B>
where
    D0: embedded_hal::digital::ErrorType,
    D1: embedded_hal::digital::ErrorType,
    D2: embedded_hal::digital::ErrorType,
    D3: embedded_hal::digital::ErrorType,
    D4: embedded_hal::digital::ErrorType,
    D5: embedded_hal::digital::ErrorType,
    D6: embedded_hal::digital::ErrorType,
    D7: embedded_hal::digital::ErrorType,
    E: embedded_hal::digital::ErrorType,
    RS: embedded_hal::digital::ErrorType,
    B: embedded_hal::digital::ErrorType,
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
            Parallel8BitsError::BacklightError(e) => write!(f, "{:?}", e),
        }
    }
}

#[derive(Debug)]
pub struct Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY, M: Mode> {
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
    backlight: Option<B>,
    _mode: PhantomData<M>,
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY, M: Mode>
    Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY, M>
{
    pub fn with_backlight(mut self, backlight: B) -> Self {
        self.backlight = Some(backlight);
        self
    }
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY, M: Mode> ErrorType
    for Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY, M>
where
    D0: embedded_hal::digital::ErrorType,
    D1: embedded_hal::digital::ErrorType,
    D2: embedded_hal::digital::ErrorType,
    D3: embedded_hal::digital::ErrorType,
    D4: embedded_hal::digital::ErrorType,
    D5: embedded_hal::digital::ErrorType,
    D6: embedded_hal::digital::ErrorType,
    D7: embedded_hal::digital::ErrorType,
    E: embedded_hal::digital::ErrorType,
    RS: embedded_hal::digital::ErrorType,
    B: embedded_hal::digital::ErrorType,
{
    type Error = Parallel8BitsError<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B>;
}

// -------------------------------------------------------------------------------------------------
// BLOCKING INTERFACE
// -------------------------------------------------------------------------------------------------
impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY>
    Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY, Blocking>
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
    B: OutputPin,
{
    fn set_outputs(&mut self, data: u8) -> Result<(), <Self as ErrorType>::Error> {
        // Set the data bits
        self.d7
            .set_state((data & (1 << 7) != 0).into())
            .map_err(Parallel8BitsError::D7Error)?;
        self.d6
            .set_state((data & (1 << 6) != 0).into())
            .map_err(Parallel8BitsError::D6Error)?;
        self.d5
            .set_state((data & (1 << 5) != 0).into())
            .map_err(Parallel8BitsError::D5Error)?;
        self.d4
            .set_state((data & (1 << 4) != 0).into())
            .map_err(Parallel8BitsError::D4Error)?;
        self.d3
            .set_state((data & (1 << 3) != 0).into())
            .map_err(Parallel8BitsError::D3Error)?;
        self.d2
            .set_state((data & (1 << 2) != 0).into())
            .map_err(Parallel8BitsError::D2Error)?;
        self.d1
            .set_state((data & (1 << 1) != 0).into())
            .map_err(Parallel8BitsError::D1Error)?;
        self.d0
            .set_state((data & (1 << 0) != 0).into())
            .map_err(Parallel8BitsError::D0Error)
    }

    fn _backlight(&mut self, enable: bool) -> Result<(), <Self as ErrorType>::Error> {
        if self.backlight.is_some() {
            self.backlight
                .as_mut()
                .unwrap()
                .set_state(enable.into())
                .map_err(Parallel8BitsError::BacklightError)?;
        }
        Ok(())
    }
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY>
    Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY, Blocking>
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
    B: OutputPin,
    DELAY: embedded_hal::delay::DelayNs,
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
            backlight: None,
            _mode: PhantomData,
        }
    }
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY, M: Mode> embedded_hal::delay::DelayNs
    for Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY, M>
where
    DELAY: embedded_hal::delay::DelayNs,
{
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        self.delay.delay_ns(ns);
    }
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY> Blocking8BitBus
    for Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY, Blocking>
where
    B: OutputPin,
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    DELAY: embedded_hal::delay::DelayNs,
    E: OutputPin,
    RS: OutputPin,
{
    fn write_byte(&mut self, data: u8) -> Result<(), Self::Error> {
        // Set the output pin levels
        self.set_outputs(data)?;
        // Open the latch
        self.e.set_high().map_err(Parallel8BitsError::EError)?;
        // Wait for the controller to fetch the data
        self.delay.delay_ns(500);
        // Close the latch
        self.e.set_low().map_err(Parallel8BitsError::EError)?;
        self.delay.delay_ns(500);
        Ok(())
    }

    #[inline]
    fn set_command_mode(&mut self, command: bool) -> Result<(), Self::Error> {
        self.rs
            .set_state((!command).into())
            .map_err(Parallel8BitsError::RSError)
    }
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY> BlockingInterface<EightBitBus>
    for Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY, Blocking>
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
impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY>
    Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY, Async>
where
    D0: AsyncOutputPin,
    D1: AsyncOutputPin,
    D2: AsyncOutputPin,
    D3: AsyncOutputPin,
    D4: AsyncOutputPin,
    D5: AsyncOutputPin,
    D6: AsyncOutputPin,
    D7: AsyncOutputPin,
    E: AsyncOutputPin,
    RS: AsyncOutputPin,
    B: AsyncOutputPin,
{
    async fn set_outputs(&mut self, data: u8) -> Result<(), <Self as ErrorType>::Error> {
        // Set the data bits
        self.d7
            .set_state((data & (1 << 7) != 0).into())
            .await
            .map_err(Parallel8BitsError::D7Error)?;
        self.d6
            .set_state((data & (1 << 6) != 0).into())
            .await
            .map_err(Parallel8BitsError::D6Error)?;
        self.d5
            .set_state((data & (1 << 5) != 0).into())
            .await
            .map_err(Parallel8BitsError::D5Error)?;
        self.d4
            .set_state((data & (1 << 4) != 0).into())
            .await
            .map_err(Parallel8BitsError::D4Error)?;
        self.d3
            .set_state((data & (1 << 3) != 0).into())
            .await
            .map_err(Parallel8BitsError::D3Error)?;
        self.d2
            .set_state((data & (1 << 2) != 0).into())
            .await
            .map_err(Parallel8BitsError::D2Error)?;
        self.d1
            .set_state((data & (1 << 1) != 0).into())
            .await
            .map_err(Parallel8BitsError::D1Error)?;
        self.d0
            .set_state((data & (1 << 0) != 0).into())
            .await
            .map_err(Parallel8BitsError::D0Error)
    }

    async fn _backlight(
        &mut self,
        enable: bool,
    ) -> Result<(), Parallel8BitsError<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B>> {
        if self.backlight.is_some() {
            self.backlight
                .as_mut()
                .unwrap()
                .set_state(enable.into())
                .await
                .map_err(Parallel8BitsError::BacklightError)?;
        }
        Ok(())
    }
}
impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY>
    Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY, Async>
where
    D0: AsyncOutputPin,
    D1: AsyncOutputPin,
    D2: AsyncOutputPin,
    D3: AsyncOutputPin,
    D4: AsyncOutputPin,
    D5: AsyncOutputPin,
    D6: AsyncOutputPin,
    D7: AsyncOutputPin,
    E: AsyncOutputPin,
    RS: AsyncOutputPin,
    B: AsyncOutputPin,
    DELAY: embedded_hal_async::delay::DelayNs,
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
            backlight: None,
            _mode: PhantomData,
        }
    }
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY, M: Mode> embedded_hal_async::delay::DelayNs
    for Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY, M>
where
    DELAY: embedded_hal_async::delay::DelayNs,
{
    #[inline]
    async fn delay_ns(&mut self, ns: u32) {
        self.delay.delay_ns(ns).await;
    }
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY> Async8BitBus
    for Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY, Async>
where
    B: AsyncOutputPin,
    D0: AsyncOutputPin,
    D1: AsyncOutputPin,
    D2: AsyncOutputPin,
    D3: AsyncOutputPin,
    D4: AsyncOutputPin,
    D5: AsyncOutputPin,
    D6: AsyncOutputPin,
    D7: AsyncOutputPin,
    DELAY: embedded_hal_async::delay::DelayNs,
    E: AsyncOutputPin,
    RS: AsyncOutputPin,
{
    async fn write_byte(&mut self, data: u8) -> Result<(), Self::Error> {
        // Set the output pin levels
        self.set_outputs(data).await?;
        // Open the latch
        self.e
            .set_high()
            .await
            .map_err(Parallel8BitsError::EError)?;
        // Wait for the controller to fetch the data
        self.delay.delay_ns(500).await;
        // Close the latch
        self.e.set_low().await.map_err(Parallel8BitsError::EError)?;
        // Wait until we can send the next data
        self.delay.delay_ns(500).await;
        Ok(())
    }

    #[inline]
    async fn set_command_mode(&mut self, command: bool) -> Result<(), Self::Error> {
        self.rs
            .set_state((!command).into())
            .await
            .map_err(Parallel8BitsError::RSError)
    }
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY> AsyncInterface<EightBitBus>
    for Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, DELAY, Async>
where
    D0: AsyncOutputPin,
    D1: AsyncOutputPin,
    D2: AsyncOutputPin,
    D3: AsyncOutputPin,
    D4: AsyncOutputPin,
    D5: AsyncOutputPin,
    D6: AsyncOutputPin,
    D7: AsyncOutputPin,
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
