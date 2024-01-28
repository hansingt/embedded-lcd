use core::fmt::Debug;

use embedded_hal::{blocking::delay::DelayUs, digital::v2::OutputPin};

use super::{Interface, Font, Lines};

use esp_println::println;

#[derive(Debug)]
pub enum Parallel4BitsError<D0Error, D1Error, D2Error, D3Error, ENError, RSError>
where
    D0Error: Debug,
    D1Error: Debug,
    D2Error: Debug,
    D3Error: Debug,
    ENError: Debug,
    RSError: Debug,
{
    ENError(ENError),
    RSError(RSError),
    D0Error(D0Error),
    D1Error(D1Error),
    D2Error(D2Error),
    D3Error(D3Error),
}

pub struct Parallel4Bits<
    D0,
    D1,
    D2,
    D3,
    EN,
    RS,
    D0Error,
    D1Error,
    D2Error,
    D3Error,
    ENError,
    RSError,
    Delay,
> where
    D0: OutputPin<Error = D0Error>,
    D1: OutputPin<Error = D1Error>,
    D2: OutputPin<Error = D2Error>,
    D3: OutputPin<Error = D3Error>,
    EN: OutputPin<Error = ENError>,
    RS: OutputPin<Error = RSError>,
    Delay: DelayUs<u16>,
{
    d0: D0,
    d1: D1,
    d2: D2,
    d3: D3,
    en: EN,
    rs: RS,
    delay: Delay,
}

impl<D0, D1, D2, D3, EN, RS, D0Error, D1Error, D2Error, D3Error, ENError, RSError, Delay>
    Parallel4Bits<D0, D1, D2, D3, EN, RS, D0Error, D1Error, D2Error, D3Error, ENError, RSError, Delay>
where
    D0: OutputPin<Error = D0Error>,
    D1: OutputPin<Error = D1Error>,
    D2: OutputPin<Error = D2Error>,
    D3: OutputPin<Error = D3Error>,
    EN: OutputPin<Error = ENError>,
    RS: OutputPin<Error = RSError>,
    D0Error: Debug,
    D1Error: Debug,
    D2Error: Debug,
    D3Error: Debug,
    ENError: Debug,
    RSError: Debug,
    Delay: DelayUs<u16>,
{
    pub fn new(d0: D0, d1: D1, d2: D2, d3: D3, en: EN, rs: RS, delay: Delay) -> Self {
        Parallel4Bits {
            d0,
            d1,
            d2,
            d3,
            en,
            rs,
            delay,
        }
    }

    fn write_word(
        &mut self,
        data: u8,
    ) -> Result<(), Parallel4BitsError<D0Error, D1Error, D2Error, D3Error, ENError, RSError>> { 
        println!("WRITE WORD: {data:04b}");
        // Set the data bits
        match data & 0b0001 {
            1 => self.d0.set_high().or_else(|err| Err(Parallel4BitsError::D0Error(err))),
            _ => self.d0.set_low().or_else(|err| Err(Parallel4BitsError::D0Error(err))),
        }?;
        match data & 0b0010 {
            1 => self.d1.set_high().or_else(|err| Err(Parallel4BitsError::D1Error(err))),
            _ => self.d1.set_low().or_else(|err| Err(Parallel4BitsError::D1Error(err))),
        }?;
        match data & 0b0100 {
            1 => self.d2.set_high().or_else(|err| Err(Parallel4BitsError::D2Error(err))),
            _ => self.d2.set_low().or_else(|err| Err(Parallel4BitsError::D2Error(err))),
        }?;
        match data & 0b1000 {
            1 => self.d3.set_high().or_else(|err| Err(Parallel4BitsError::D3Error(err))),
            _ => self.d3.set_low().or_else(|err| Err(Parallel4BitsError::D3Error(err))),
        }?;
        // Open the latch
        self.en.set_high().or_else(|err| Err(Parallel4BitsError::ENError(err)))?;
        self.delay.delay_us(1);
       
        // Close the latch
        self.en.set_low().or_else(|err| Err(Parallel4BitsError::ENError(err)))?;
        self.delay.delay_us(40);
        Ok(())

    }
}

impl<D0, D1, D2, D3, EN, RS, D0Error, D1Error, D2Error, D3Error, ENError, RSError, Delay> Interface
    for Parallel4Bits<D0, D1, D2, D3, EN, RS, D0Error, D1Error, D2Error, D3Error, ENError, RSError, Delay>
where
    D0: OutputPin<Error = D0Error>,
    D1: OutputPin<Error = D1Error>,
    D2: OutputPin<Error = D2Error>,
    D3: OutputPin<Error = D3Error>,
    EN: OutputPin<Error = ENError>,
    RS: OutputPin<Error = RSError>,
    D0Error: Debug,
    D1Error: Debug,
    D2Error: Debug,
    D3Error: Debug,
    ENError: Debug,
    RSError: Debug,
    Delay: DelayUs<u16>,
{
    type Error = Parallel4BitsError<D0Error, D1Error, D2Error, D3Error, ENError, RSError>;


    fn initialize(&mut self, lines: Lines, font: Font) -> Result<(), Self::Error> {
        self.write_word(0b0011)?;
        println!("WAIT 45ms");
        self.delay.delay_us(4500);
        self.write_word(0b0011)?;
        println!("WAIT 150us");
        self.delay.delay_us(150);
        self.write_word(0b0011)?;

        self.write_word(0b0010)?;
        
        let function_set = match font {
            Font::FiveTimesTenDots => 0b0010_0100,
            Font::FiveTimesEightDots => match lines {
                Lines::One => 0b0010_0000,
                Lines::Two => 0b0010_1000,
            }
        };
        println!("");
        self.write(function_set, true)
    }

    fn write(&mut self, data: u8, command: bool) -> Result<(), Self::Error> {
        match command {
            true => self.rs.set_low().or_else(|err| Err(Parallel4BitsError::RSError(err))),
            false => self.rs.set_high().or_else(|err| Err(Parallel4BitsError::RSError(err))),
        }?;

        // Write the upper word
        self.write_word(data >> 4)?;

        // Then write the lower word
        self.write_word(data & 0b0000_1111)?;
        println!("");
        Ok(())
    }
}
