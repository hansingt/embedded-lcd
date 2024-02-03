#![no_std]

use core::fmt::{Error, Write};

mod interfaces;

use embedded_hal::blocking::delay::DelayUs;
pub use interfaces::*;

pub enum ShiftDirection {
    Right = 0b0000_0110,
    Left = 0b0000_0100,
}

#[derive(Debug)]
pub struct LCD1602<I: Interface, Delay: DelayUs<u16>> {
    interface: I,
    delay: Delay,
}

impl<I: Interface, Delay: DelayUs<u16>> LCD1602<I, Delay> {
    pub fn new(interface: I, lines: Lines, font: Font, delay: Delay) -> Result<Self, I::Error> {
        let mut lcd = Self { interface, delay };
        lcd.interface.initialize(lines, font)?;
        lcd.clear()?;
        Ok(lcd)
    }

    pub fn configure(
        &mut self,
        enable_display: bool,
        enable_cursor: bool,
        cursor_blink: bool,
        shift_display: bool,
        shift_direction: ShiftDirection,
    ) -> Result<(), I::Error> {
        let display_control = match enable_display {
            true => 0b0000_1100,
            false => 0b0000_1000,
        } | match enable_cursor {
            true => 0b0000_1010,
            false => 0b0000_1000,
        } | match cursor_blink {
            true => 0b0000_1001,
            false => 0b0000_1000,
        };
        self.interface.write(display_control, true)?;

        let entry_mode = match shift_display {
            true => shift_direction as u8 | 0x01,
            false => shift_direction as u8,
        };
        self.interface.write(entry_mode, true)
    }

    #[inline]
    pub fn clear(&mut self) -> Result<(), I::Error> {
        self.interface.write(0b0000_0001, true)?;
        self.delay.delay_us(50);
        Ok(())
    }

    #[inline]
    pub fn home(&mut self) -> Result<(), I::Error> {
        self.interface.write(0b0000_0010, true)?;
        Ok(())
    }

    pub fn shift(
        &mut self,
        shift_display: bool,
        shift_direction: ShiftDirection,
    ) -> Result<(), I::Error> {
        let mut cmd = match shift_display {
            true => 0b0001_1000,
            false => 0b0001_0000,
        };
        cmd = match shift_direction {
            ShiftDirection::Right => cmd | 0b000_0100,
            ShiftDirection::Left => cmd,
        };
        self.interface.write(cmd, true)
    }

    pub fn pos(&mut self, line: Lines, position: u8) -> Result<(), I::Error> {
        let cmd = match line {
            Lines::One => position | 0b1000_0000,
            Lines::Two => (0x40 + position) | 0b1000_0000,
        };
        self.interface.write(cmd, true)
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

    pub fn write_str(&mut self, s: &str) -> Result<(), I::Error> {
        for c in s.chars() {
            match c.is_ascii() {
                true => self.write_byte(c as u8)?,
                false => self.write_byte(255)?,
            }
        }
        Ok(())
    }
}

impl<I: Interface, D: DelayUs<u16>> Write for LCD1602<I, D> {
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        match c.is_ascii() {
            true => match self.write_byte(c as u8) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error {}),
            },
            false => Err(Error {}),
        }
    }

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self.write_str(s) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error {}),
        }
    }
}
