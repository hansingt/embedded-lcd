use core::future::Future;
use embedded_hal::digital::{ErrorType, OutputPin, PinState};

pub trait AsyncOutputPin: ErrorType {
    fn set_low(&mut self) -> impl Future<Output = Result<(), Self::Error>>;
    fn set_high(&mut self) -> impl Future<Output = Result<(), Self::Error>>;

    #[allow(async_fn_in_trait)]
    #[inline]
    async fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
        match state {
            PinState::High => self.set_high().await,
            PinState::Low => self.set_low().await,
        }
    }
}

impl<T: OutputPin + ?Sized> AsyncOutputPin for T {
    #[inline]
    async fn set_low(&mut self) -> Result<(), Self::Error> {
        T::set_low(self)
    }

    #[inline]
    async fn set_high(&mut self) -> Result<(), Self::Error> {
        T::set_high(self)
    }

    #[inline]
    async fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
        T::set_state(self, state)
    }
}
