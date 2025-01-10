use crate::interfaces::{AsyncInterface, BlockingInterface, Interface};
use crate::{Async, Blocking, Font, Lines, Mode};
use core::marker::PhantomData;
use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::{AddressMode, ErrorType, I2c as EhI2c, SevenBitAddress};
use embedded_hal_async::delay::DelayNs as AsyncDelayNs;
use embedded_hal_async::i2c::I2c as EhAsyncI2c;

#[derive(Debug)]
pub struct I2c<'b, I, A: AddressMode = SevenBitAddress, M: Mode = Blocking> {
    i2c: &'b mut I,
    address: A,
    backlight: u8,
    _mode: PhantomData<M>,
}

const ENABLE: u8 = 0b0000_0100;

impl<'b, I, A> I2c<'b, I, A, Blocking>
where
    A: AddressMode + Copy,
    I: EhI2c<A>,
{
    #[inline(always)]
    pub fn new(i2c: &'b mut I, address: A) -> Self {
        I2c {
            i2c,
            address,
            backlight: 0,
            _mode: PhantomData,
        }
    }

    fn write_nibble(
        &mut self,
        data: u8,
        command: bool,
        delay: &mut impl DelayNs,
    ) -> Result<(), I::Error> {
        let nibble = data & 0xF0 | !command as u8;
        #[cfg(feature = "log")]
        log::trace!("Writing nibble {:#010b}", nibble >> 4);
        self.i2c
            .write(self.address, &[nibble | self.backlight | ENABLE])?;
        delay.delay_us(1);
        self.i2c
            .write(self.address, &[nibble | self.backlight & !ENABLE])
    }
}

impl<'b, I, A> I2c<'b, I, A, Async>
where
    A: AddressMode + Copy,
    I: EhAsyncI2c<A>,
{
    #[inline(always)]
    pub fn new_async(i2c: &'b mut I, address: A) -> Self {
        Self {
            i2c,
            address,
            backlight: 0,
            _mode: PhantomData,
        }
    }

    async fn write_nibble_async(
        &mut self,
        data: u8,
        command: bool,
        delay: &mut impl AsyncDelayNs,
    ) -> Result<(), I::Error> {
        let nibble = data & 0xF0 | !command as u8;
        #[cfg(feature = "log")]
        log::trace!("Writing nibble {:#010b}", nibble >> 4);
        self.i2c
            .write(self.address, &[nibble | self.backlight | ENABLE])
            .await?;
        delay.delay_us(1).await;
        self.i2c
            .write(self.address, &[nibble | self.backlight & !ENABLE])
            .await
    }
}

impl<A, I, M> Interface for I2c<'_, I, A, M>
where
    A: AddressMode,
    I: ErrorType,
    M: Mode,
{
    type Error = I::Error;
}

impl<A, I> BlockingInterface for I2c<'_, I, A, Blocking>
where
    A: AddressMode + Copy,
    I: EhI2c<A>,
{
    fn initialize(
        &mut self,
        lines: Lines,
        font: Font,
        delay: &mut impl DelayNs,
    ) -> Result<(), Self::Error> {
        // Initialize the display
        self.write_nibble(0b0011_0000, true, delay)?;
        delay.delay_us(4500);
        self.write_nibble(0b0011_0000, true, delay)?;
        delay.delay_us(150);
        self.write_nibble(0b0011_0000, true, delay)?;
        // Set the interface to 4-Bit length
        self.write_nibble(0b0010_0000, true, delay)?;
        // Configure the display
        self.write_command(0b0010_0000 | lines as u8 | font as u8, delay)
    }

    fn write_command(&mut self, command: u8, delay: &mut impl DelayNs) -> Result<(), Self::Error> {
        #[cfg(feature = "log")]
        log::debug!("Writing command {:#010b} to LCD Display", command);
        self.write_nibble(command, true, delay)?;
        self.write_nibble(command << 4, true, delay)
    }

    fn write_data(&mut self, data: u8, delay: &mut impl DelayNs) -> Result<(), Self::Error> {
        #[cfg(feature = "log")]
        log::debug!("Writing data '{:#010b}' to LCD Display", data);
        self.write_nibble(data, false, delay)?;
        self.write_nibble(data << 4, false, delay)
    }

    fn backlight(&mut self, enable: bool) -> Result<(), Self::Error> {
        self.backlight = (enable as u8) << 3;
        self.i2c.write(self.address, &[self.backlight])
    }
}

impl<A, I> AsyncInterface for I2c<'_, I, A, Async>
where
    A: AddressMode + Copy,
    I: EhAsyncI2c<A>,
{
    async fn initialize(
        &mut self,
        lines: Lines,
        font: Font,
        delay: &mut impl AsyncDelayNs,
    ) -> Result<(), Self::Error> {
        // Initialize the display
        self.write_nibble_async(0b0011_0000, true, delay).await?;
        delay.delay_us(4500).await;
        self.write_nibble_async(0b0011_0000, true, delay).await?;
        delay.delay_us(150).await;
        self.write_nibble_async(0b0011_0000, true, delay).await?;
        // Set the interface to 4-Bit length
        self.write_nibble_async(0b0010_0000, true, delay).await?;
        // Configure the display
        self.write_command(0b0010_0000 | lines as u8 | font as u8, delay)
            .await
    }

    async fn write_command(
        &mut self,
        command: u8,
        delay: &mut impl AsyncDelayNs,
    ) -> Result<(), Self::Error> {
        #[cfg(feature = "log")]
        log::debug!("Writing command {:#010b} to LCD Display", command);
        self.write_nibble_async(command, true, delay).await?;
        self.write_nibble_async(command << 4, true, delay).await
    }

    async fn write_data(
        &mut self,
        data: u8,
        delay: &mut impl AsyncDelayNs,
    ) -> Result<(), Self::Error> {
        #[cfg(feature = "log")]
        log::debug!("Writing data '{:#010b}' to LCD Display", data);
        self.write_nibble_async(data, false, delay).await?;
        self.write_nibble_async(data << 4, false, delay).await
    }

    async fn backlight(&mut self, enable: bool) -> Result<(), Self::Error> {
        self.backlight = (enable as u8) << 3;
        self.i2c.write(self.address, &[self.backlight]).await
    }
}
