use crate::interfaces::{
    Async4BitBus, AsyncInterface, Blocking4BitBus, BlockingInterface, ErrorType, FourBitBus,
};
use crate::{Async, Blocking, Mode};
use core::fmt::Debug;
use core::marker::PhantomData;
use embedded_hal::i2c::AddressMode;

const ENABLE: u8 = 0b0000_0100;
const DATA: u8 = 0b0000_0001;
const BACKGROUND: u8 = 0b0000_1000;

#[derive(Debug)]
pub struct I2c<'a, I2C, A, DELAY, M: Mode> {
    i2c: &'a mut I2C,
    address: A,
    delay: DELAY,
    config: u8,
    _mode: PhantomData<M>,
}

impl<I2C, A, DELAY, M: Mode> ErrorType for I2c<'_, I2C, A, DELAY, M>
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
    A: AddressMode,
    I2C: embedded_hal::i2c::I2c<A>,
    DELAY: embedded_hal::delay::DelayNs,
{
    #[inline]
    pub fn new(i2c: &'a mut I2C, address: A, delay: DELAY) -> Self {
        Self {
            i2c,
            address,
            delay,
            config: 0,
            _mode: PhantomData,
        }
    }
}

impl<I2C, A, DELAY> embedded_hal::delay::DelayNs for I2c<'_, I2C, A, DELAY, Blocking>
where
    DELAY: embedded_hal::delay::DelayNs,
{
    #[inline]
    fn delay_ns(&mut self, ns: u32) {
        self.delay.delay_ns(ns);
    }
}

impl<I2C, A, DELAY> Blocking4BitBus for I2c<'_, I2C, A, DELAY, Blocking>
where
    A: AddressMode + Clone,
    DELAY: embedded_hal::delay::DelayNs,
    I2C: embedded_hal::i2c::I2c<A>,
{
    fn write_nibble(&mut self, nibble: u8) -> Result<(), Self::Error> {
        let data = nibble << 4 | self.config;
        // Write the data and open the latch
        self.i2c.write(self.address.clone(), &[data | ENABLE])?;
        // Wait for the controller to fetch the data
        self.delay.delay_ns(500);
        // Close the latch again
        self.i2c.write(self.address.clone(), &[data & !ENABLE])?;
        self.delay.delay_ns(500);
        Ok(())
    }

    #[inline]
    fn set_command_mode(&mut self, command: bool) -> Result<(), Self::Error> {
        match command {
            true => self.config &= !DATA,
            false => self.config |= DATA,
        }
        self.i2c.write(self.address.clone(), &[self.config])
    }
}

impl<I2C, A, DELAY> BlockingInterface<FourBitBus> for I2c<'_, I2C, A, DELAY, Blocking>
where
    A: AddressMode + Clone,
    I2C: embedded_hal::i2c::I2c<A>,
    DELAY: embedded_hal::delay::DelayNs,
{
    fn backlight(&mut self, enable: bool) -> Result<(), Self::Error> {
        match enable {
            true => self.config |= BACKGROUND,
            false => self.config &= !BACKGROUND,
        }
        self.i2c.write(self.address.clone(), &[self.config])
    }
}

// -------------------------------------------------------------------------------------------------
// ASYNC INTERFACE
// -------------------------------------------------------------------------------------------------
impl<'a, I2C, A, DELAY> I2c<'a, I2C, A, DELAY, Async>
where
    A: AddressMode,
    I2C: embedded_hal_async::i2c::I2c<A>,
    DELAY: embedded_hal_async::delay::DelayNs,
{
    #[inline]
    pub fn new_async(i2c: &'a mut I2C, address: A, delay: DELAY) -> Self {
        Self {
            i2c,
            address,
            config: 0,
            delay,
            _mode: PhantomData,
        }
    }
}

impl<I2C, A, DELAY> embedded_hal_async::delay::DelayNs for I2c<'_, I2C, A, DELAY, Async>
where
    DELAY: embedded_hal_async::delay::DelayNs,
{
    #[inline]
    async fn delay_ns(&mut self, ns: u32) {
        self.delay.delay_ns(ns).await;
    }
}

impl<I2C, A, DELAY> Async4BitBus for I2c<'_, I2C, A, DELAY, Async>
where
    A: AddressMode + Clone,
    DELAY: embedded_hal_async::delay::DelayNs,
    I2C: embedded_hal_async::i2c::I2c<A>,
{
    async fn write_nibble(&mut self, nibble: u8) -> Result<(), Self::Error> {
        let data = nibble << 4 | self.config;
        // Write the data and open the latch
        self.i2c
            .write(self.address.clone(), &[data | ENABLE])
            .await?;
        // Wait for the controller to fetch the data
        self.delay.delay_ns(500).await;
        // Close the latch again
        self.i2c
            .write(self.address.clone(), &[data & !ENABLE])
            .await?;
        self.delay.delay_ns(500).await;
        Ok(())
    }

    #[inline]
    async fn set_command_mode(&mut self, command: bool) -> Result<(), Self::Error> {
        match command {
            true => self.config &= !DATA,
            false => self.config |= DATA,
        }
        self.i2c.write(self.address.clone(), &[self.config]).await
    }
}

impl<I2C, A, DELAY> AsyncInterface<FourBitBus> for I2c<'_, I2C, A, DELAY, Async>
where
    A: AddressMode + Clone,
    I2C: embedded_hal_async::i2c::I2c<A>,
    DELAY: embedded_hal_async::delay::DelayNs,
{
    async fn backlight(&mut self, enable: bool) -> Result<(), Self::Error> {
        match enable {
            true => self.config |= BACKGROUND,
            false => self.config &= !BACKGROUND,
        }
        self.i2c.write(self.address.clone(), &[self.config]).await
    }
}
