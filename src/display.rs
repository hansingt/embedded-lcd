use crate::interfaces::{AsyncInterface, BlockingInterface, BusWidth};
use crate::{Async, Blocking, Cursor, Font, Lines, Mode, Shift, ShiftDirection};
use core::fmt;
use core::marker::PhantomData;

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
pub struct Display<I, W: BusWidth, DM: Mode> {
    interface: I,
    lines: Lines,
    font: Font,
    display_control: u8,
    entry_mode: u8,
    _mode: PhantomData<DM>,
    _width: PhantomData<W>,
}

impl<I, W: BusWidth, DM: Mode> Display<I, W, DM> {
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

    fn character_as_byte(c: char) -> u8 {
        match c.is_ascii() {
            true => c as u8,
            false => match c {
                'ä' | 'Ä' => 0b1110_0001,
                'ß' => 0b1110_0010,
                'ö' | 'Ö' => 0b1110_1111,
                'ü' | 'Ü' => 0b1111_0101,
                '°' => 0b1101_1111,
                _ => 0b0011_1111, // == ?
            },
        }
    }
}

// -------------------------------------------------------------------------------------------------
// BLOCKING INTERFACE
// -------------------------------------------------------------------------------------------------
impl<I, W> Display<I, W, Blocking>
where
    W: BusWidth,
    I: BlockingInterface<W>,
{
    #[inline(always)]
    pub fn new(interface: I) -> Self {
        Self {
            interface,
            lines: Lines::default(),
            font: Font::default(),
            display_control: Commands::DisplayControl as u8,
            entry_mode: Commands::EntryModeSet as u8,
            _mode: PhantomData,
            _width: PhantomData,
        }
    }

    pub fn init(mut self) -> Result<Self, I::Error> {
        #[cfg(feature = "log")]
        log::info!("Initializing LCD");
        self.interface.initialize()?;
        // Configure font and lines
        let function_set = match self.font {
            Font::_5x10 => W::WIDTH | 0b0000_0100,
            Font::_5x8 => match self.lines {
                Lines::_1 => W::WIDTH,
                Lines::_2 => W::WIDTH | 0b000_1000,
            },
        };
        self.interface.write(function_set, true)?;
        // Configure the display
        self.interface.write(self.display_control, true)?;
        self.interface.write(self.entry_mode, true)?;
        self.clear()?;
        Ok(self)
    }

    pub fn clear(&mut self) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Clearing display");
        self.interface.write(Commands::Clear as u8, true)?;
        self.interface.delay_us(50);
        Ok(())
    }

    pub fn home(&mut self) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Moving cursor home");
        self.interface.write(Commands::Home as u8, true)?;
        self.interface.delay_us(50);
        Ok(())
    }

    pub fn shift(&mut self, shift: Shift, shift_direction: ShiftDirection) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Shifting the {} to the {}", shift, shift_direction);
        self.interface.write(
            Commands::Shift as u8 + ((shift as u8 + shift_direction as u8) << 2),
            true,
        )
    }

    pub fn pos(&mut self, line: Lines, position: u8) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Moving cursor to position {} on line {}", position, line);
        let cmd = Commands::SetDisplayDataAddress as u8
            | match line {
                Lines::_1 => position,
                Lines::_2 => 0x40 + position,
            };
        self.interface.write(cmd, true)?;
        Ok(())
    }

    #[inline]
    pub fn write_byte(&mut self, data: u8) -> Result<(), I::Error> {
        self.interface.write(data, false)
    }

    pub fn write_bytes(&mut self, data: &[u8]) -> Result<(), I::Error> {
        for b in data {
            self.write_byte(*b)?;
        }
        Ok(())
    }

    #[inline]
    pub fn write_character(&mut self, c: char) -> Result<(), I::Error> {
        self.write_byte(Self::character_as_byte(c))
    }

    pub fn write_string<S: AsRef<str>>(&mut self, s: S) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Writing string '{}' to LCD", s.as_ref());
        for c in s.as_ref().chars() {
            self.write_character(c)?;
        }
        Ok(())
    }

    pub fn enable_backlight(&mut self) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Enable backlight");
        self.interface.backlight(true)
    }

    pub fn disable_backlight(&mut self) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Disable backlight");
        self.interface.backlight(false)
    }
}

impl<I, W> fmt::Write for Display<I, W, Blocking>
where
    W: BusWidth,
    I: BlockingInterface<W>,
{
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s).map_err(|_| fmt::Error)
    }

    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.write_character(c).map_err(|_| fmt::Error)
    }
}

// -------------------------------------------------------------------------------------------------
// ASYNC INTERFACE
// -------------------------------------------------------------------------------------------------
impl<I, W> Display<I, W, Async>
where
    W: BusWidth,
    I: AsyncInterface<W>,
{
    #[inline(always)]
    pub fn new_async(interface: I) -> Self {
        Self {
            interface,
            lines: Lines::default(),
            font: Font::default(),
            display_control: Commands::DisplayControl as u8,
            entry_mode: Commands::EntryModeSet as u8,
            _mode: PhantomData,
            _width: PhantomData,
        }
    }

    pub async fn init(mut self) -> Result<Self, I::Error> {
        #[cfg(feature = "log")]
        log::info!("Initializing LCD");
        self.interface.initialize().await?;
        // Configure font and lines
        let function_set = match self.font {
            Font::_5x10 => 0b0010_0100,
            Font::_5x8 => match self.lines {
                Lines::_1 => 0b0010_0000,
                Lines::_2 => 0b0010_1000,
            },
        };
        self.interface.write(function_set, true).await?;
        // Configure the display
        self.interface.write(self.display_control, true).await?;
        self.interface.write(self.entry_mode, true).await?;
        self.clear().await?;
        Ok(self)
    }

    pub async fn clear(&mut self) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Clearing display");
        self.interface.write(Commands::Clear as u8, true).await?;
        self.interface.delay_us(50).await;
        Ok(())
    }

    pub async fn home(&mut self) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Moving cursor home");
        self.interface.write(Commands::Home as u8, true).await
    }

    pub async fn shift(
        &mut self,
        shift: Shift,
        shift_direction: ShiftDirection,
    ) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Shifting the {} to the {}", shift, shift_direction);
        self.interface
            .write(
                Commands::Shift as u8 + ((shift as u8 + shift_direction as u8) << 2),
                true,
            )
            .await
    }

    pub async fn pos(&mut self, line: Lines, position: u8) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Moving cursor to position {} on line {}", position, line);
        let cmd = Commands::SetDisplayDataAddress as u8
            | match line {
                Lines::_1 => position,
                Lines::_2 => 0x40 + position,
            };
        self.interface.write(cmd, true).await
    }

    #[inline]
    pub async fn write_byte(&mut self, data: u8) -> Result<(), I::Error> {
        self.interface.write(data, false).await
    }

    pub async fn write_bytes(&mut self, data: &[u8]) -> Result<(), I::Error> {
        for b in data {
            self.write_byte(*b).await?;
        }
        Ok(())
    }

    #[inline]
    pub async fn write_character(&mut self, c: char) -> Result<(), I::Error> {
        self.write_byte(Self::character_as_byte(c)).await
    }

    pub async fn write_string<S: AsRef<str>>(&mut self, s: S) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Writing string '{}' to LCD", s.as_ref());
        for c in s.as_ref().chars() {
            self.write_character(c).await?;
        }
        Ok(())
    }

    pub async fn enable_backlight(&mut self) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Enable backlight");
        self.interface.backlight(true).await
    }

    pub async fn disable_backlight(&mut self) -> Result<(), I::Error> {
        #[cfg(feature = "log")]
        log::info!("Disable backlight");
        self.interface.backlight(false).await
    }
}
