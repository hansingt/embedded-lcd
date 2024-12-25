use crate::interfaces::{BlockingInterface, Interface, InterfaceWidth};
use crate::{Font, Lines, ShiftDirection};
use embedded_hal::delay::DelayNs;

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
            display_control: 0b0000_1000,
            entry_mode: 0b0000_0100,
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
    pub fn with_shift(mut self, shift_display: bool, shift_direction: ShiftDirection) -> Self {
        self.entry_mode |= (shift_direction as u8) << 1 | (shift_display as u8);
        self
    }

    #[inline]
    pub fn with_cursor(mut self, enabled: bool, blink: bool) -> Self {
        self.display_control |= (enabled as u8) << 1 | blink as u8;
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
        // Initialize the display
        self.interface.write_command(0b0011_0000, delay)?;
        delay.delay_us(4500);
        self.interface.write_command(0b0011_0000, delay)?;
        delay.delay_us(150);
        self.interface.write_command(0b0011_0000, delay)?;
        if I::interface_width() == InterfaceWidth::FourBit {
            // Set the interface to 4-Bit length
            self.interface.write_command(0b0010_0000, delay)?;
        }
        // configure the number of lines and the font size to use
        if I::interface_width() == InterfaceWidth::FourBit {
            self.interface
                .write_command(0b0010_0000 | self.lines as u8 | self.font as u8, delay)?;
        } else {
            self.interface
                .write_command(0b0011_0000 | self.lines as u8 | self.font as u8, delay)?;
        }
        // Configure the display
        self.interface.write_command(self.display_control, delay)?;
        self.interface.write_command(self.entry_mode, delay)?;
        self.clear(delay)?;
        Ok(self)
    }

    pub fn enable_display(&mut self, delay: &mut impl DelayNs) -> Result<(), I::Error> {
        self.display_control |= 0b0000_0100;
        self.interface.write_command(self.display_control, delay)?;
        Ok(())
    }

    pub fn disable_display(&mut self, delay: &mut impl DelayNs) -> Result<(), I::Error> {
        self.display_control &= !0b0000_0100;
        self.interface.write_command(self.display_control, delay)?;
        Ok(())
    }

    pub fn enable_cursor(&mut self, delay: &mut impl DelayNs) -> Result<(), I::Error> {
        self.display_control |= 0b0000_0010;
        self.interface.write_command(self.display_control, delay)?;
        Ok(())
    }

    pub fn disable_cursor(&mut self, delay: &mut impl DelayNs) -> Result<(), I::Error> {
        self.display_control &= !0b0000_0010;
        self.interface.write_command(self.display_control, delay)?;
        Ok(())
    }

    pub fn enable_blink(&mut self, delay: &mut impl DelayNs) -> Result<(), I::Error> {
        self.display_control |= 0b0000_0001;
        self.interface.write_command(self.display_control, delay)?;
        Ok(())
    }

    pub fn disable_blink(&mut self, delay: &mut impl DelayNs) -> Result<(), I::Error> {
        self.display_control &= !0b0000_0001;
        self.interface.write_command(self.display_control, delay)?;
        Ok(())
    }

    #[inline]
    pub fn enable_backlight(&mut self) -> Result<(), I::Error> {
        self.interface.backlight(true)
    }

    #[inline]
    pub fn disable_backlight(&mut self) -> Result<(), I::Error> {
        self.interface.backlight(false)
    }

    #[inline]
    pub fn clear(&mut self, delay: &mut impl DelayNs) -> Result<(), I::Error> {
        self.interface.write_command(0b0000_0001, delay)?;
        delay.delay_us(50);
        Ok(())
    }

    #[inline]
    pub fn home(&mut self, delay: &mut impl DelayNs) -> Result<(), I::Error> {
        self.interface.write_command(0b0000_0010, delay)?;
        Ok(())
    }

    pub fn pos(
        &mut self,
        line: Lines,
        position: u8,
        delay: &mut impl DelayNs,
    ) -> Result<(), I::Error> {
        let cmd = match line {
            Lines::One => position | 0b1000_0000,
            Lines::Two => (0x40 + position) | 0b1000_0000,
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
        for c in s.as_ref().chars() {
            match c.is_ascii() {
                true => self.write_byte(c as u8, delay)?,
                false => self.write_byte(255, delay)?,
            }
        }
        Ok(())
    }
}
