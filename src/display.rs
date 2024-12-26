use crate::interfaces::{BlockingInterface, Interface};
use crate::{Cursor, Font, Lines, Shift, ShiftDirection};

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
    pub fn init(mut self) -> Result<Self, I::Error> {
        #[cfg(feature = "log")]
        log::info!("Initializing LCD");
        self.interface.initialize(self.lines, self.font)?;
        // Configure the display
        self.interface.write_command(self.display_control)?;
        self.interface.write_command(self.entry_mode)?;
        self.clear()?;
        Ok(self)
    }

    #[inline]
    pub fn enable_backlight(&mut self) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Enable backlight");
        self.interface.backlight(true)
    }

    #[inline]
    pub fn disable_backlight(&mut self) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Disable backlight");
        self.interface.backlight(false)
    }

    #[inline]
    pub fn clear(&mut self) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Clearing display");
        self.interface.write_command(Commands::Clear as u8)?;
        Ok(())
    }

    #[inline]
    pub fn home(&mut self) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Moving cursor home");
        self.interface.write_command(Commands::Home as u8)?;
        Ok(())
    }

    #[inline]
    pub fn shift(&mut self, shift: Shift, shift_direction: ShiftDirection) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Shifting the {} to the {}", shift, shift_direction);
        self.interface
            .write_command(Commands::Shift as u8 + ((shift as u8 + shift_direction as u8) << 2))
    }

    pub fn pos(&mut self, line: Lines, position: u8) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Moving cursor to position {} on line {}", position, line);
        let cmd = Commands::SetDisplayDataAddress as u8
            | match line {
                Lines::One => position,
                Lines::Two => 0x40 + position,
            };
        self.interface.write_command(cmd)?;
        Ok(())
    }

    #[inline]
    pub fn write_byte(&mut self, data: u8) -> Result<(), I::Error> {
        self.interface.write_data(data)
    }

    pub fn write_bytes(&mut self, data: &[u8]) -> Result<(), I::Error> {
        for b in data {
            self.write_byte(*b)?;
        }
        Ok(())
    }

    pub fn write_character(&mut self, c: char) -> Result<(), I::Error> {
        match c.is_ascii() {
            true => self.write_byte(c as u8)?,
            false => match c {
                'ä' | 'Ä' => self.write_byte(0b1110_0001)?,
                'ß' => self.write_byte(0b1110_0010)?,
                'ö' | 'Ö' => self.write_byte(0b1110_1111)?,
                'ü' | 'Ü' => self.write_byte(0b1111_0101)?,
                '°' => self.write_byte(0b1101_1111)?,
                _ => self.write_byte(0b0011_1111)?, // == ?
            },
        }
        Ok(())
    }

    pub fn write_string<S: AsRef<str>>(&mut self, s: S) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Writing string '{}' to LCD", s.as_ref());
        for c in s.as_ref().chars() {
            self.write_character(c)?;
        }
        Ok(())
    }
}

impl<I> core::fmt::Write for Display<I>
where
    I: BlockingInterface,
{
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s).map_err(|_| core::fmt::Error)
    }

    #[inline]
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.write_character(c).map_err(|_| core::fmt::Error)
    }
}
