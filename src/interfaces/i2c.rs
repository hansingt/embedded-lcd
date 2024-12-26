use crate::interfaces::{BlockingInterface, Interface};
use crate::{Font, Lines};
use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::{AddressMode, I2c as EMI2c};

#[derive(Debug)]
pub struct I2c<'b, I, A, D> {
    i2c: &'b mut I,
    address: A,
    backlight: u8,
    delay: D,
}

const ENABLE: u8 = 0b0000_0100;

impl<'b, A, I, D> I2c<'b, I, A, D>
where
    A: AddressMode,
    I: EMI2c<A>,
{
    #[inline]
    pub fn new(i2c: &'b mut I, address: A, delay: D) -> Self {
        Self {
            i2c,
            address,
            backlight: 0,
            delay,
        }
    }
}

impl<'b, A, I, D> I2c<'b, I, A, D>
where
    A: AddressMode + Copy,
    I: EMI2c<A>,
    D: DelayNs,
{
    fn write_nibble(&mut self, data: u8, command: bool) -> Result<(), I::Error> {
        let nibble = data & 0xF0 | !command as u8;
        #[cfg(feature = "log")]
        log::trace!("Writing nibble {:#010b}", nibble >> 4);
        self.i2c
            .write(self.address, &[nibble | self.backlight | ENABLE])?;
        self.delay.delay_us(1);
        self.i2c
            .write(self.address, &[nibble | self.backlight & !ENABLE])?;
        Ok(())
    }
}

impl<'b, A, I, D> Interface for I2c<'b, I, A, D>
where
    A: AddressMode,
    I: EMI2c<A>,
{
    type Error = I::Error;
}

impl<'b, A, I, D> BlockingInterface for I2c<'b, I, A, D>
where
    A: AddressMode + Copy,
    I: EMI2c<A>,
    D: DelayNs,
{
    fn initialize(&mut self, lines: Lines, font: Font) -> Result<(), Self::Error> {
        // Initialize the display
        self.write_nibble(0b0011_0000, true)?;
        self.delay.delay_us(4500);
        self.write_nibble(0b0011_0000, true)?;
        self.delay.delay_us(150);
        self.write_nibble(0b0011_0000, true)?;
        // Set the interface to 4-Bit length
        self.write_nibble(0b0010_0000, true)?;
        // Configure the display
        self.write_command(0b0010_0000 | lines as u8 | font as u8)?;
        Ok(())
    }

    fn write_command(&mut self, command: u8) -> Result<(), Self::Error> {
        #[cfg(feature = "log")]
        log::debug!("Writing command {:#010b} to LCD Display", command);
        self.write_nibble(command, true)?;
        self.write_nibble(command << 4, true)?;
        Ok(())
    }

    fn write_data(&mut self, data: u8) -> Result<(), Self::Error> {
        #[cfg(feature = "log")]
        log::debug!("Writing data '{:#010b}' to LCD Display", data);
        self.write_nibble(data, false)?;
        self.write_nibble(data << 4, false)?;
        Ok(())
    }

    fn backlight(&mut self, enable: bool) -> Result<(), Self::Error> {
        self.backlight = if enable { 1 << 3 } else { 0 };
        self.i2c.write(self.address, &[self.backlight])?;
        Ok(())
    }
}
