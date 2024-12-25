use crate::interfaces::{BlockingInterface, Interface, InterfaceWidth};
use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::{AddressMode, I2c as EMI2c};

pub struct I2c<'b, I, A> {
    i2c: &'b mut I,
    address: A,
    backlight: u8,
}

const ENABLE: u8 = 0b0000_0100;

impl<'b, A, I> I2c<'b, I, A>
where
    A: AddressMode + Clone,
    I: EMI2c<A>,
{
    #[inline]
    pub fn new(i2c: &'b mut I, address: A) -> Self {
        Self {
            i2c,
            address,
            backlight: 0,
        }
    }

    fn write_nibble(&mut self, data: u8, delay: &mut impl DelayNs) -> Result<(), I::Error> {
        self.i2c.write(
            self.address.clone(),
            &[data & 0b1111_0000 | self.backlight | ENABLE],
        )?;
        delay.delay_us(1);
        self.i2c.write(
            self.address.clone(),
            &[data & 0b1111_0000 | self.backlight & !ENABLE],
        )?;
        Ok(())
    }
}

impl<'b, A, I> Interface for I2c<'b, I, A>
where
    A: AddressMode,
    I: EMI2c<A>,
{
    type Error = I::Error;

    fn interface_width() -> InterfaceWidth {
        InterfaceWidth::FourBit
    }
}

impl<'b, A, I> BlockingInterface for I2c<'b, I, A>
where
    A: AddressMode + Clone,
    I: EMI2c<A>,
{
    fn write_command(&mut self, command: u8, delay: &mut impl DelayNs) -> Result<(), Self::Error> {
        self.write_nibble(command, delay)?;
        self.write_nibble(command << 4, delay)?;
        Ok(())
    }

    fn write_data(&mut self, data: u8, delay: &mut impl DelayNs) -> Result<(), Self::Error> {
        let data = 0b0000_0001 | (data << 4);
        self.write_nibble(data, delay)?;
        self.write_nibble(data << 4, delay)?;
        Ok(())
    }

    fn backlight(&mut self, enable: bool) -> Result<(), Self::Error> {
        self.backlight = if enable { 1 << 3 } else { 0 };
        self.i2c.write(self.address.clone(), &[self.backlight])?;
        Ok(())
    }
}
