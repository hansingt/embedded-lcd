use crate::interfaces::{BlockingInterface, Interface};
use crate::{Cursor, Font, Lines, Shift, ShiftDirection};
use embedded_hal::delay::DelayNs;

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Commands {
    Clear = 1,
    Home = 2,
    EntryModeSet = 4,
    DisplayControl = 8,
    Shift = 16,
    //SetCharacterGeneratorAddress = 64,
    SetDisplayDataAddress = 128,
}

#[derive(Debug)]
pub struct Display<I> {
    interface: I,
    lines: Lines,
    font: Font,
    display_control: u8,
    entry_mode: u8,
}

impl<I> Display<I>
where
    I: Interface,
{
    #[inline]
    pub fn new(interface: I) -> Self {
        Self {
            interface,
            lines: Lines::default(),
            font: Font::default(),
            display_control: Commands::DisplayControl as u8,
            entry_mode: Commands::EntryModeSet as u8,
        }
    }

    #[inline]
    pub fn with_lines(mut self, lines: Lines) -> Self {
        self.lines = lines;
        self
    }

    #[inline]
    pub fn with_font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    #[inline]
    pub fn with_shift(mut self, shift: Shift, shift_direction: ShiftDirection) -> Self {
        self.entry_mode += shift_direction as u8 + shift as u8;
        self
    }

    #[inline]
    pub fn with_cursor(mut self, cursor: Cursor) -> Self {
        self.display_control += cursor as u8;
        self
    }

    #[inline]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.display_control |= (enabled as u8) << 2;
        self
    }
}

impl<I: BlockingInterface> Display<I> {
    pub fn init(mut self, delay: &mut impl DelayNs) -> Result<Self, I::Error> {
        log::info!("Initializing LCD");
        self.interface.initialize(self.lines, self.font, delay)?;
        // Configure the display
        self.interface.write_command(self.display_control, delay)?;
        self.interface.write_command(self.entry_mode, delay)?;
        self.clear(delay)?;
        Ok(self)
    }

    #[inline]
    pub fn enable_backlight(&mut self) -> Result<(), I::Error> {
        log::info!("Enable backlight");
        self.interface.backlight(true)
    }

    #[inline]
    pub fn disable_backlight(&mut self) -> Result<(), I::Error> {
        log::info!("Disable backlight");
        self.interface.backlight(false)
    }

    #[inline]
    pub fn clear(&mut self, delay: &mut impl DelayNs) -> Result<(), I::Error> {
        log::info!("Clearing display");
        self.interface.write_command(Commands::Clear as u8, delay)?;
        delay.delay_us(50);
        Ok(())
    }

    #[inline]
    pub fn home(&mut self, delay: &mut impl DelayNs) -> Result<(), I::Error> {
        log::info!("Moving cursor home");
        self.interface.write_command(Commands::Home as u8, delay)?;
        Ok(())
    }

    #[inline]
    pub fn shift(
        &mut self,
        shift: Shift,
        shift_direction: ShiftDirection,
        delay: &mut impl DelayNs,
    ) -> Result<(), I::Error> {
        log::info!("Shifting the {} to the {}", shift, shift_direction);
        self.interface.write_command(
            Commands::Shift as u8 + ((shift as u8 + shift_direction as u8) << 2),
            delay,
        )
    }

    pub fn pos(
        &mut self,
        line: Lines,
        position: u8,
        delay: &mut impl DelayNs,
    ) -> Result<(), I::Error> {
        log::info!("Moving cursor to position {} on line {}", position, line);
        let cmd = Commands::SetDisplayDataAddress as u8
            | match line {
                Lines::One => position,
                Lines::Two => 0x40 + position,
            };
        self.interface.write_command(cmd, delay)?;
        Ok(())
    }

    #[inline]
    pub fn write_byte(&mut self, data: u8, delay: &mut impl DelayNs) -> Result<(), I::Error> {
        self.interface.write_data(data, delay)
    }

    pub fn write_bytes(&mut self, data: &[u8], delay: &mut impl DelayNs) -> Result<(), I::Error> {
        for b in data {
            self.write_byte(*b, delay)?;
        }
        Ok(())
    }

    pub fn write_str<S: AsRef<str>>(
        &mut self,
        s: S,
        delay: &mut impl DelayNs,
    ) -> Result<(), I::Error> {
        log::info!("Writing string '{}' to LCD", s.as_ref());
        for c in s.as_ref().chars() {
            match c.is_ascii() {
                true => self.write_byte(c as u8, delay)?,
                false => self.write_byte(255, delay)?,
            }
        }
        Ok(())
    }
}
