use core::fmt::Debug;

use embedded_hal::{blocking::delay::DelayUs, digital::v2::OutputPin};

use super::{Font, Interface, Lines};

#[derive(Debug)]
pub enum Parallel4BitsError<D7Error, D6Error, D5Error, D4Error, ENError, RSError>
where
    D7Error: Debug,
    D6Error: Debug,
    D5Error: Debug,
    D4Error: Debug,
    ENError: Debug,
    RSError: Debug,
{
    ENError(ENError),
    RSError(RSError),
    D7Error(D7Error),
    D6Error(D6Error),
    D5Error(D5Error),
    D4Error(D4Error),
}

pub struct Parallel4Bits<
    D7,
    D6,
    D5,
    D4,
    EN,
    RS,
    D7Error,
    D6Error,
    D5Error,
    D4Error,
    ENError,
    RSError,
    Delay,
> where
    D7: OutputPin<Error = D7Error>,
    D6: OutputPin<Error = D6Error>,
    D5: OutputPin<Error = D5Error>,
    D4: OutputPin<Error = D4Error>,
    EN: OutputPin<Error = ENError>,
    RS: OutputPin<Error = RSError>,
    Delay: DelayUs<u16>,
{
    d7: D7,
    d6: D6,
    d5: D5,
    d4: D4,
    en: EN,
    rs: RS,
    delay: Delay,
}

impl<D7, D6, D5, D4, EN, RS, D7Error, D6Error, D5Error, D4Error, ENError, RSError, Delay>
    Parallel4Bits<
        D7,
        D6,
        D5,
        D4,
        EN,
        RS,
        D7Error,
        D6Error,
        D5Error,
        D4Error,
        ENError,
        RSError,
        Delay,
    >
where
    D7: OutputPin<Error = D7Error>,
    D6: OutputPin<Error = D6Error>,
    D5: OutputPin<Error = D5Error>,
    D4: OutputPin<Error = D4Error>,
    EN: OutputPin<Error = ENError>,
    RS: OutputPin<Error = RSError>,
    D7Error: Debug,
    D6Error: Debug,
    D5Error: Debug,
    D4Error: Debug,
    ENError: Debug,
    RSError: Debug,
    Delay: DelayUs<u16>,
{
    pub fn new(d7: D7, d6: D6, d5: D5, d4: D4, en: EN, rs: RS, delay: Delay) -> Self {
        Parallel4Bits {
            d7,
            d6,
            d5,
            d4,
            en,
            rs,
            delay,
        }
    }

    fn write_nibble(
        &mut self,
        data: u8,
    ) -> Result<(), Parallel4BitsError<D7Error, D6Error, D5Error, D4Error, ENError, RSError>> {
        // Set the data bits
        match data & 0b1000 {
            0 => self
                .d7
                .set_low()
                .or_else(|err| Err(Parallel4BitsError::D7Error(err))),
            _ => self
                .d7
                .set_high()
                .or_else(|err| Err(Parallel4BitsError::D7Error(err))),
        }?;
        match data & 0b0100 {
            0 => self
                .d6
                .set_low()
                .or_else(|err| Err(Parallel4BitsError::D6Error(err))),
            _ => self
                .d6
                .set_high()
                .or_else(|err| Err(Parallel4BitsError::D6Error(err))),
        }?;
        match data & 0b0010 {
            0 => self
                .d5
                .set_low()
                .or_else(|err| Err(Parallel4BitsError::D5Error(err))),
            _ => self
                .d5
                .set_high()
                .or_else(|err| Err(Parallel4BitsError::D5Error(err))),
        }?;
        match data & 0b0001 {
            0 => self
                .d4
                .set_low()
                .or_else(|err| Err(Parallel4BitsError::D4Error(err))),
            _ => self
                .d4
                .set_high()
                .or_else(|err| Err(Parallel4BitsError::D4Error(err))),
        }?;
        // Open the latch
        self.en
            .set_high()
            .or_else(|err| Err(Parallel4BitsError::ENError(err)))?;
        self.delay.delay_us(1);
        // Close the latch
        self.en
            .set_low()
            .or_else(|err| Err(Parallel4BitsError::ENError(err)))?;
        Ok(())
    }
}

impl<D7, D6, D5, D4, EN, RS, D7Error, D6Error, D5Error, D4Error, ENError, RSError, Delay> Interface
    for Parallel4Bits<
        D7,
        D6,
        D5,
        D4,
        EN,
        RS,
        D7Error,
        D6Error,
        D5Error,
        D4Error,
        ENError,
        RSError,
        Delay,
    >
where
    D7: OutputPin<Error = D7Error>,
    D6: OutputPin<Error = D6Error>,
    D5: OutputPin<Error = D5Error>,
    D4: OutputPin<Error = D4Error>,
    EN: OutputPin<Error = ENError>,
    RS: OutputPin<Error = RSError>,
    D7Error: Debug,
    D6Error: Debug,
    D5Error: Debug,
    D4Error: Debug,
    ENError: Debug,
    RSError: Debug,
    Delay: DelayUs<u16>,
{
    type Error = Parallel4BitsError<D7Error, D6Error, D5Error, D4Error, ENError, RSError>;

    fn initialize(&mut self, lines: Lines, font: Font) -> Result<(), Self::Error> {
        self.write_nibble(0b0011)?;
        self.delay.delay_us(4500);
        self.write_nibble(0b0011)?;
        self.delay.delay_us(150);
        self.write_nibble(0b0011)?;

        self.write_nibble(0b0010)?;

        let function_set = match font {
            Font::FiveTimesTenDots => 0b0010_0100,
            Font::FiveTimesEightDots => match lines {
                Lines::One => 0b0010_0000,
                Lines::Two => 0b0010_1000,
            },
        };
        self.write(function_set, true)
    }

    fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error> {
        match command {
            true => self
                .rs
                .set_low()
                .or_else(|err| Err(Parallel4BitsError::RSError(err))),
            false => self
                .rs
                .set_high()
                .or_else(|err| Err(Parallel4BitsError::RSError(err))),
        }?;

        // Write the upper word
        self.write_nibble(data >> 4)?;

        // Then write the lower word
        self.write_nibble(data & 0b0000_1111)?;
        Ok(())
    }
}
