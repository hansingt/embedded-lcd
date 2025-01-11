use crate::async_output_pin::AsyncOutputPin;
use crate::interfaces::{
    AsyncInterface, BlockingInterface, Interface, Parallel4Bits, Parallel4BitsError,
};
use crate::{Async, Blocking, Font, Lines, Mode};
use core::cell::RefCell;
use core::fmt::Debug;
use core::marker::PhantomData;
use embedded_hal::digital::PinState;
use embedded_hal::i2c::AddressMode;

#[derive(Debug)]
struct Driver<'a, I2C, A, M: Mode> {
    i2c: &'a mut I2C,
    address: A,
    state: u8,
    _mode: PhantomData<M>,
}

impl<'a, I2C, A> Driver<'a, I2C, A, Blocking>
where
    A: AddressMode + Clone,
    I2C: embedded_hal::i2c::I2c<A>,
{
    fn set_bit(&mut self, bit: u8, state: PinState) -> Result<(), I2C::Error> {
        let new_state = match state {
            PinState::High => self.state | (1 << bit),
            PinState::Low => self.state & !(1 << bit),
        };
        self.i2c
            .write(self.address.clone(), &[self.state])
            .map(|_| {
                self.state = new_state;
            })
    }
}

impl<'a, I2C, A> Driver<'a, I2C, A, Async>
where
    A: AddressMode + Clone,
    I2C: embedded_hal_async::i2c::I2c<A>,
{
    async fn set_bit(&mut self, bit: u8, state: PinState) -> Result<(), I2C::Error> {
        let new_state = match state {
            PinState::High => self.state | (1 << bit),
            PinState::Low => self.state & !(1 << bit),
        };
        self.i2c
            .write(self.address.clone(), &[self.state])
            .await
            .map(|_| {
                self.state = new_state;
            })
    }
}

#[derive(Debug)]
struct Pin<'a, 'b, I2C, A, const PIN: u8, M: Mode>(&'b RefCell<Driver<'a, I2C, A, M>>);

#[derive(Debug)]
struct PinError<E, const PIN: u8>(E);

impl<E: Debug, const PIN: u8> embedded_hal::digital::Error for PinError<E, PIN> {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        embedded_hal::digital::ErrorKind::Other
    }
}

impl<'a, 'b, I2C, A, M, const PIN: u8> embedded_hal::digital::ErrorType
    for Pin<'a, 'b, I2C, A, PIN, M>
where
    M: Mode,
    I2C: embedded_hal::i2c::ErrorType,
{
    type Error = PinError<I2C::Error, PIN>;
}

impl<'a, 'b, I2C, A, const PIN: u8> embedded_hal::digital::OutputPin
    for Pin<'a, 'b, I2C, A, PIN, Blocking>
where
    A: AddressMode + Clone,
    I2C: embedded_hal::i2c::I2c<A>,
{
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.0
            .borrow_mut()
            .set_bit(PIN, PinState::Low)
            .map_err(PinError)
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.0
            .borrow_mut()
            .set_bit(PIN, PinState::High)
            .map_err(PinError)
    }

    fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
        self.0.borrow_mut().set_bit(PIN, state).map_err(PinError)
    }
}

impl<'a, 'b, I2C, A, const PIN: u8> AsyncOutputPin for Pin<'a, 'b, I2C, A, PIN, Async>
where
    A: AddressMode + Clone,
    I2C: embedded_hal_async::i2c::I2c<A>,
{
    async fn set_low(&mut self) -> Result<(), Self::Error> {
        self.0
            .borrow_mut()
            .set_bit(PIN, PinState::Low)
            .await
            .map_err(PinError)
    }

    async fn set_high(&mut self) -> Result<(), Self::Error> {
        self.0
            .borrow_mut()
            .set_bit(PIN, PinState::High)
            .await
            .map_err(PinError)
    }

    async fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
        self.0
            .borrow_mut()
            .set_bit(PIN, state)
            .await
            .map_err(PinError)
    }
}

#[derive(Debug)]
pub struct I2c<'a, I2C, A, DELAY, M: Mode> {
    driver: RefCell<Driver<'a, I2C, A, M>>,
    delay: DELAY,
}

type I2cInterface<'a, 'b, I2C, A, DELAY, M> = Parallel4Bits<
    Pin<'a, 'b, I2C, A, 7, M>,
    Pin<'a, 'b, I2C, A, 6, M>,
    Pin<'a, 'b, I2C, A, 5, M>,
    Pin<'a, 'b, I2C, A, 4, M>,
    Pin<'a, 'b, I2C, A, 2, M>,
    Pin<'a, 'b, I2C, A, 0, M>,
    Pin<'a, 'b, I2C, A, 3, M>,
    DELAY,
    M,
>;

type InterfaceError<'a, 'b, I2C, A, M> = Parallel4BitsError<
    Pin<'a, 'b, I2C, A, 7, M>,
    Pin<'a, 'b, I2C, A, 6, M>,
    Pin<'a, 'b, I2C, A, 5, M>,
    Pin<'a, 'b, I2C, A, 4, M>,
    Pin<'a, 'b, I2C, A, 2, M>,
    Pin<'a, 'b, I2C, A, 0, M>,
    Pin<'a, 'b, I2C, A, 3, M>,
>;

impl<'a, 'b, I2C, A, M> InterfaceError<'a, 'b, I2C, A, M>
where
    M: Mode,
    I2C: embedded_hal::i2c::ErrorType,
{
    fn into_error(self) -> I2C::Error {
        match self {
            Parallel4BitsError::EError(e) => e.0,
            Parallel4BitsError::RSError(e) => e.0,
            Parallel4BitsError::D7Error(e) => e.0,
            Parallel4BitsError::D6Error(e) => e.0,
            Parallel4BitsError::D5Error(e) => e.0,
            Parallel4BitsError::D4Error(e) => e.0,
            Parallel4BitsError::BacklightError(e) => e.0,
        }
    }
}

impl<'a, I2C, A, DELAY, M: Mode> Interface for I2c<'a, I2C, A, DELAY, M>
where
    I2C: embedded_hal::i2c::ErrorType,
{
    type Error = I2C::Error;
}

// -------------------------------------------------------------------------------------------------
// BLOCKING INTERFACE
// -------------------------------------------------------------------------------------------------
impl<'a, I2C, A, DELAY> I2c<'a, I2C, A, DELAY, Blocking>
where
    A: AddressMode + Clone,
    I2C: embedded_hal::i2c::I2c<A>,
    DELAY: embedded_hal::delay::DelayNs + Clone,
{
    #[inline]
    pub fn new(i2c: &'a mut I2C, address: A, delay: DELAY) -> Self {
        Self {
            driver: RefCell::new(Driver {
                i2c,
                address,
                state: 0,
                _mode: PhantomData,
            }),
            delay,
        }
    }

    #[inline]
    fn interface<'b>(&'b mut self) -> I2cInterface<'a, 'b, I2C, A, DELAY, Blocking> {
        Parallel4Bits::new(
            Pin(&self.driver),
            Pin(&self.driver),
            Pin(&self.driver),
            Pin(&self.driver),
            Pin(&self.driver),
            Pin(&self.driver),
            self.delay.clone(),
        )
        .with_backlight(Pin(&self.driver))
    }
}

impl<'a, I2C, A, DELAY> embedded_hal::delay::DelayNs for I2c<'a, I2C, A, DELAY, Blocking>
where
    DELAY: embedded_hal::delay::DelayNs + Clone,
{
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        self.delay.delay_ns(ns);
    }
}

impl<'a, I2C, A, DELAY> BlockingInterface for I2c<'a, I2C, A, DELAY, Blocking>
where
    A: AddressMode + Clone,
    I2C: embedded_hal::i2c::I2c<A>,
    DELAY: embedded_hal::delay::DelayNs + Clone,
{
    fn initialize(&mut self, lines: Lines, font: Font) -> Result<(), Self::Error> {
        self.interface()
            .initialize(lines, font)
            .map_err(|e| e.into_error())
    }

    fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error> {
        self.interface()
            .write(data, command)
            .map_err(|e| e.into_error())
    }

    fn backlight(&mut self, enable: bool) -> Result<(), Self::Error> {
        self.interface()
            .backlight(enable)
            .map_err(|e| e.into_error())
    }
}

// -------------------------------------------------------------------------------------------------
// ASYNC INTERFACE
// -------------------------------------------------------------------------------------------------
impl<'a, I2C, A, DELAY> I2c<'a, I2C, A, DELAY, Async>
where
    A: AddressMode + Clone,
    I2C: embedded_hal_async::i2c::I2c<A>,
    DELAY: embedded_hal_async::delay::DelayNs + Clone,
{
    #[inline]
    pub fn new_async(i2c: &'a mut I2C, address: A, delay: DELAY) -> Self {
        Self {
            driver: RefCell::new(Driver {
                i2c,
                address,
                state: 0,
                _mode: PhantomData,
            }),
            delay,
        }
    }

    #[inline]
    fn interface<'b>(&'b mut self) -> I2cInterface<'a, 'b, I2C, A, DELAY, Async> {
        Parallel4Bits::new_async(
            Pin(&self.driver),
            Pin(&self.driver),
            Pin(&self.driver),
            Pin(&self.driver),
            Pin(&self.driver),
            Pin(&self.driver),
            self.delay.clone(),
        )
        .with_backlight(Pin(&self.driver))
    }
}

impl<'a, I2C, A, DELAY> embedded_hal_async::delay::DelayNs for I2c<'a, I2C, A, DELAY, Async>
where
    DELAY: embedded_hal_async::delay::DelayNs + Clone,
{
    #[inline]
    async fn delay_ns(&mut self, ns: u32) {
        self.delay.delay_ns(ns).await;
    }
}

impl<'a, I2C, A, DELAY> AsyncInterface for I2c<'a, I2C, A, DELAY, Async>
where
    A: AddressMode + Clone,
    I2C: embedded_hal_async::i2c::I2c<A>,
    DELAY: embedded_hal_async::delay::DelayNs + Clone,
{
    async fn initialize(&mut self, lines: Lines, font: Font) -> Result<(), Self::Error> {
        self.interface()
            .initialize(lines, font)
            .await
            .map_err(|e| e.into_error())
    }

    async fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error> {
        self.interface()
            .write(data, command)
            .await
            .map_err(|e| e.into_error())
    }

    async fn backlight(&mut self, enable: bool) -> Result<(), Self::Error> {
        self.interface()
            .backlight(enable)
            .await
            .map_err(|e| e.into_error())
    }
}
