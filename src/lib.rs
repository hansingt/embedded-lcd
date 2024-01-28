#![no_std]

use core::fmt::{Error, Write};

mod interfaces;

pub use interfaces::*;

pub enum ShiftDirection {
    Right = 0b0000_0110,
    Left = 0b0000_0100,
}


#[derive(Debug)]
pub struct LCD1602<I: Interface> {
    interface: I,
}

impl<I: Interface> LCD1602<I> {
    pub fn new(interface: I, lines: Lines, font: Font) -> Result<Self, I::Error> {
        let mut lcd = Self { interface };
        lcd.interface.initialize(lines, font)?;
        Ok(lcd)
    }

    pub fn configure(&mut self, enable_display: bool, enable_cursor: bool, cursor_blink: bool, shift_display: bool, shift_direction: ShiftDirection) -> Result<(), I::Error> {
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
        self.interface.write(0b0000_0001, true)
    }

    #[inline]
    pub fn home(&mut self) -> Result<(), I::Error> {
        self.interface.write(0b0000_0010, true)
    }

    pub fn shift(&mut self, shift_display: bool, shift_direction: ShiftDirection) -> Result<(), I::Error> {
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

    #[inline]
    pub fn write_byte(&mut self, data: u8) -> Result<(), I::Error> {
        self.interface.write(data, false)
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

impl<I: Interface> Write for LCD1602<I> {
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