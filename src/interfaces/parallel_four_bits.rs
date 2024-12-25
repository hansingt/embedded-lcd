use core::fmt::Debug;
use embedded_hal::digital::OutputPin;


#[derive(Debug)]
pub enum Parallel4BitsError<D7Error, D6Error, D5Error, D4Error, ENError, RSError> {
    ENError(ENError),
    RSError(RSError),
    D7Error(D7Error),
    D6Error(D6Error),
    D5Error(D5Error),
    D4Error(D4Error),
}

pub struct Parallel4Bits<D7, D6, D5, D4, EN, RS, M>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    EN: OutputPin,
    RS: OutputPin,
    M: Mode,
{
    d7: D7,
    d6: D6,
    d5: D5,
    d4: D4,
    en: EN,
    rs: RS,
    _mode: PhantomData<M>,
}

impl<D7, D6, D5, D4, EN, RS, M> Parallel4Bits<D7, D6, D5, D4, EN, RS, M>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    EN: OutputPin,
    RS: OutputPin,
    M: Mode,
{
    pub fn new(d7: D7, d6: D6, d5: D5, d4: D4, en: EN, rs: RS) -> Self {
        Parallel4Bits {
            d7,
            d6,
            d5,
            d4,
            en,
            rs,
            _mode: PhantomData,
        }
    }
}

impl<D7, D6, D5, D4, EN, RS> Parallel4Bits<D7, D6, D5, D4, EN, RS, Blocking>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    EN: OutputPin,
    RS: OutputPin, 
{
    #[allow(clippy::complexity)]
    fn write_nibble<D: DelayNs>(
        &mut self,
        data: u8,
        delay: &mut D,
    ) -> Result<
        (),
        Parallel4BitsError<D7::Error, D6::Error, D5::Error, D4::Error, EN::Error, RS::Error>,
    > {
        // Set the data bits
        match data & 0b1000 {
            0 => self.d7.set_low().map_err(Parallel4BitsError::D7Error),
            _ => self.d7.set_high().map_err(Parallel4BitsError::D7Error),
        }?;
        match data & 0b0100 {
            0 => self.d6.set_low().map_err(Parallel4BitsError::D6Error),
            _ => self.d6.set_high().map_err(Parallel4BitsError::D6Error),
        }?;
        match data & 0b0010 {
            0 => self.d5.set_low().map_err(Parallel4BitsError::D5Error),
            _ => self.d5.set_high().map_err(Parallel4BitsError::D5Error),
        }?;
        match data & 0b0001 {
            0 => self.d4.set_low().map_err(Parallel4BitsError::D4Error),
            _ => self.d4.set_high().map_err(Parallel4BitsError::D4Error),
        }?;
        // Open the latch
        self.en.set_high().map_err(Parallel4BitsError::ENError)?;
        delay.delay_us(1);
        // Close the latch
        self.en.set_low().map_err(Parallel4BitsError::ENError)?;
        Ok(())
    }
}

impl<D7, D6, D5, D4, EN, RS> Interface for Parallel4Bits<D7, D6, D5, D4, EN, RS, Blocking>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    EN: OutputPin,
    RS: OutputPin,
{
    type Error =
    Parallel4BitsError<D7::Error, D6::Error, D5::Error, D4::Error, EN::Error, RS::Error>;
}

impl<D7, D6, D5, D4, EN, RS> BlockingInterface for Parallel4Bits<D7, D6, D5, D4, EN, RS, Blocking>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    EN: OutputPin,
    RS: OutputPin,
{
    fn initialize(&mut self, lines: Lines, font: Font, delay: &mut impl DelayNs) -> Result<(), Self::Error> {
        self.write_nibble(0b0011, delay)?;
        delay.delay_us(4500);
        self.write_nibble(0b0011, delay)?;
        delay.delay_us(150);
        self.write_nibble(0b0011, delay)?;
        self.write_nibble(0b0010, delay)?;

        let function_set = match font {
            Font::FiveTimesTenDots => 0b0010_0100,
            Font::FiveTimesEightDots => match lines {
                Lines::One => 0b0010_0000,
                Lines::Two => 0b0010_1000,
            },
        };
        self.write(function_set, true, delay)
    }

    fn write(&mut self, data: u8, command: bool, delay: &mut impl DelayNs) -> Result<(), Self::Error> {
        match command {
            true => self.rs.set_low().map_err(Parallel4BitsError::RSError),
            false => self.rs.set_high().map_err(Parallel4BitsError::RSError),
        }?;

        // Write the upper word
        self.write_nibble(data >> 4, delay)?;

        // Then write the lower word
        self.write_nibble(data & 0b0000_1111, delay)?;
        Ok(())
    }
}
