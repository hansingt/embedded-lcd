#![no_std]
#![no_main]

use embedded_hal::i2c::SevenBitAddress;
#[allow(unused_imports)]
use esp_backtrace as _;

use embedded_lcd::{Blocking, Cursor, Font, Lines, Shift, ShiftDirection};
use esp_hal::i2c::master::I2c;
use esp_hal::{delay::Delay, prelude::*};

fn create_display<I2C>(
    i2c: &mut I2C,
) -> embedded_lcd::Display<
    embedded_lcd::interfaces::I2c<I2C, SevenBitAddress, Delay, Blocking>,
    Blocking,
>
where
    I2C: embedded_hal::i2c::I2c,
{
    let interface = embedded_lcd::interfaces::I2c::new(i2c, 0x27, Delay::new());
    let mut lcd = embedded_lcd::Display::new(interface)
        .with_lines(Lines::_2)
        .with_font(Font::_5x8)
        .with_cursor(Cursor::Disabled)
        .with_shift(Shift::Cursor, ShiftDirection::Right)
        .enabled(true)
        .init()
        .unwrap();
    lcd.enable_backlight().unwrap();
    lcd
}

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Initialize the LCD
    delay.delay(50.millis());
    let mut i2c = I2c::new(peripherals.I2C0, esp_hal::i2c::master::Config::default())
        .with_scl(peripherals.GPIO32)
        .with_sda(peripherals.GPIO33);
    let mut lcd = create_display(&mut i2c);

    loop {
        lcd.clear().unwrap();
        lcd.pos(Lines::_1, 0).unwrap();
        lcd.write_string("     Hello      ").unwrap();
        delay.delay(1000.millis());
        lcd.clear().unwrap();
        lcd.pos(Lines::_2, 0).unwrap();
        lcd.write_string("     World!     ").unwrap();
        delay.delay(1000.millis());
    }
}
