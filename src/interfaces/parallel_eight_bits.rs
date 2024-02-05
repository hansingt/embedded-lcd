use core::fmt::Debug;

use embedded_hal::{blocking::delay::DelayUs, digital::v2::OutputPin};

use super::{Font, Interface, Lines};

#[derive(Debug)]
pub enum Parallel8BitsError<
    D0Error,
    D1Error,
    D2Error,
    D3Error,
    D4Error,
    D5Error,
    D6Error,
    D7Error,
    ENError,
    RSError,
> {
    ENError(ENError),
    RSError(RSError),
    D0Error(D0Error),
    D1Error(D1Error),
    D2Error(D2Error),
    D3Error(D3Error),
    D4Error(D4Error),
    D5Error(D5Error),
    D6Error(D6Error),
    D7Error(D7Error),
}

pub struct Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, EN, RS, Delay>
where
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    EN: OutputPin,
    RS: OutputPin,
    Delay: DelayUs<u16>,
{
    d0: D0,
    d1: D1,
    d2: D2,
    d3: D3,
    d4: D4,
    d5: D5,
    d6: D6,
    d7: D7,
    en: EN,
    rs: RS,
    delay: Delay,
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, EN, RS, Delay>
    Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, EN, RS, Delay>
where
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    EN: OutputPin,
    RS: OutputPin,
    Delay: DelayUs<u16>,
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
        en: EN,
        rs: RS,
        delay: Delay,
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
            en,
            rs,
            delay,
        }
    }

    #[allow(clippy::complexity)]
    fn write_byte(
        &mut self,
        data: u8,
    ) -> Result<
        (),
        Parallel8BitsError<
            D0::Error,
            D1::Error,
            D2::Error,
            D3::Error,
            D4::Error,
            D5::Error,
            D6::Error,
            D7::Error,
            EN::Error,
            RS::Error,
        >,
    > {
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
        }?;

        // Open the latch
        self.en.set_high().map_err(Parallel8BitsError::ENError)?;

        self.delay.delay_us(1);
        // Close the latch
        self.en.set_low().map_err(Parallel8BitsError::ENError)?;
        Ok(())
    }
}

impl<D0, D1, D2, D3, D4, D5, D6, D7, EN, RS, Delay> Interface
    for Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, EN, RS, Delay>
where
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    EN: OutputPin,
    RS: OutputPin,
    Delay: DelayUs<u16>,
{
    type Error = Parallel8BitsError<
        D0::Error,
        D1::Error,
        D2::Error,
        D3::Error,
        D4::Error,
        D5::Error,
        D6::Error,
        D7::Error,
        EN::Error,
        RS::Error,
    >;

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
        // We want to write data
        self.write_byte(data)
    }
}
