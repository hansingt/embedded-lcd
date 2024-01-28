use core::fmt::Debug;

use embedded_hal::{blocking::delay::DelayUs, digital::v2::OutputPin};

use super::{Interface, Lines, Font};

#[derive(Debug)]
pub enum Parallel8BitsError<
    D0Error: Debug,
    D1Error: Debug,
    D2Error: Debug,
    D3Error: Debug,
    D4Error: Debug,
    D5Error: Debug,
    D6Error: Debug,
    D7Error: Debug,
    ENError: Debug,
    RSError: Debug,
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

pub struct Parallel8Bits<
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    EN,
    RS,
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
    Delay,
> where
    D0: OutputPin<Error = D0Error>,
    D1: OutputPin<Error = D1Error>,
    D2: OutputPin<Error = D2Error>,
    D3: OutputPin<Error = D3Error>,
    D4: OutputPin<Error = D4Error>,
    D5: OutputPin<Error = D5Error>,
    D6: OutputPin<Error = D6Error>,
    D7: OutputPin<Error = D7Error>,
    EN: OutputPin<Error = ENError>,
    RS: OutputPin<Error = RSError>,
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

impl<
        D0,
        D1,
        D2,
        D3,
        D4,
        D5,
        D6,
        D7,
        EN,
        RS,
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
        Delay,
    >
    Parallel8Bits<
        D0,
        D1,
        D2,
        D3,
        D4,
        D5,
        D6,
        D7,
        EN,
        RS,
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
        Delay,
    >
where
    D0: OutputPin<Error = D0Error>,
    D1: OutputPin<Error = D1Error>,
    D2: OutputPin<Error = D2Error>,
    D3: OutputPin<Error = D3Error>,
    D4: OutputPin<Error = D4Error>,
    D5: OutputPin<Error = D5Error>,
    D6: OutputPin<Error = D6Error>,
    D7: OutputPin<Error = D7Error>,
    EN: OutputPin<Error = ENError>,
    RS: OutputPin<Error = RSError>,
    D0Error: Debug,
    D1Error: Debug,
    D2Error: Debug,
    D3Error: Debug,
    D4Error: Debug,
    D5Error: Debug,
    D6Error: Debug,
    D7Error: Debug,
    ENError: Debug,
    RSError: Debug,
    Delay: DelayUs<u16>,
{
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

    fn write_byte(
        &mut self,
        data: u8,
    ) -> Result<
        (),
        Parallel8BitsError<
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
        >,
    > {
        // Close the latch
        self.en
            .set_low()
            .or_else(|err| Err(Parallel8BitsError::ENError(err)))?;

        // Set the data bits
        if data & 0b0000_0001 == 1 {
            self.d0
                .set_high()
                .or_else(|err| Err(Parallel8BitsError::D0Error(err)))?;
        } else {
            self.d0
                .set_low()
                .or_else(|err| Err(Parallel8BitsError::D0Error(err)))?;
        }
        if data & 0b0000_0010 == 1 {
            self.d1
                .set_high()
                .or_else(|err| Err(Parallel8BitsError::D1Error(err)))?;
        } else {
            self.d1
                .set_low()
                .or_else(|err| Err(Parallel8BitsError::D1Error(err)))?;
        }
        if data & 0b0000_0100 == 1 {
            self.d2
                .set_high()
                .or_else(|err| Err(Parallel8BitsError::D2Error(err)))?;
        } else {
            self.d2
                .set_low()
                .or_else(|err| Err(Parallel8BitsError::D2Error(err)))?;
        }
        if data & 0b0000_1000 == 1 {
            self.d3
                .set_high()
                .or_else(|err| Err(Parallel8BitsError::D3Error(err)))?;
        } else {
            self.d3
                .set_low()
                .or_else(|err| Err(Parallel8BitsError::D3Error(err)))?;
        }
        if data & 0b0001_0000 == 1 {
            self.d4
                .set_high()
                .or_else(|err| Err(Parallel8BitsError::D4Error(err)))?;
        } else {
            self.d4
                .set_low()
                .or_else(|err| Err(Parallel8BitsError::D4Error(err)))?;
        }
        if data & 0b0010_0000 == 1 {
            self.d5
                .set_high()
                .or_else(|err| Err(Parallel8BitsError::D5Error(err)))?;
        } else {
            self.d5
                .set_low()
                .or_else(|err| Err(Parallel8BitsError::D5Error(err)))?;
        }
        if data & 0b0100_0000 == 1 {
            self.d6
                .set_high()
                .or_else(|err| Err(Parallel8BitsError::D6Error(err)))?;
        } else {
            self.d6
                .set_low()
                .or_else(|err| Err(Parallel8BitsError::D6Error(err)))?;
        }
        if data & 0b1000_0000 == 1 {
            self.d7
                .set_high()
                .or_else(|err| Err(Parallel8BitsError::D7Error(err)))?;
        } else {
            self.d7
                .set_low()
                .or_else(|err| Err(Parallel8BitsError::D7Error(err)))?;
        }
        // Open the latch
        self.en
            .set_high()
            .or_else(|err| Err(Parallel8BitsError::ENError(err)))?;

        self.delay.delay_us(1);
       
        // Close the latch
        self.en
            .set_low()
            .or_else(|err| Err(Parallel8BitsError::ENError(err)))?;
        self.delay.delay_us(40);
        Ok(()) 
    }
}

impl<
        D0,
        D1,
        D2,
        D3,
        D4,
        D5,
        D6,
        D7,
        EN,
        RS,
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
        Delay,
    > Interface
    for Parallel8Bits<
        D0,
        D1,
        D2,
        D3,
        D4,
        D5,
        D6,
        D7,
        EN,
        RS,
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
        Delay,
    >
where
    D0: OutputPin<Error = D0Error>,
    D1: OutputPin<Error = D1Error>,
    D2: OutputPin<Error = D2Error>,
    D3: OutputPin<Error = D3Error>,
    D4: OutputPin<Error = D4Error>,
    D5: OutputPin<Error = D5Error>,
    D6: OutputPin<Error = D6Error>,
    D7: OutputPin<Error = D7Error>,
    EN: OutputPin<Error = ENError>,
    RS: OutputPin<Error = RSError>,
    D0Error: Debug,
    D1Error: Debug,
    D2Error: Debug,
    D3Error: Debug,
    D4Error: Debug,
    D5Error: Debug,
    D6Error: Debug,
    D7Error: Debug,
    ENError: Debug,
    RSError: Debug,
    Delay: DelayUs<u16>,
{
    type Error = Parallel8BitsError<
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
            true => self.rs
            .set_low()
            .or_else(|err| Err(Parallel8BitsError::RSError(err))),
            false => self.rs
            .set_high()
            .or_else(|err| Err(Parallel8BitsError::RSError(err))),
        }?;
        // We want to write data
        self.write_byte(data)
    }
}
