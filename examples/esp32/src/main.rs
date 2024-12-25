#![no_std]
#![no_main]
#[allow(unused_imports)]
use esp_backtrace as _;

use embedded_lcd::{Cursor, Font, Lines, Shift, ShiftDirection};
use esp_hal::i2c::master::I2c;
use esp_hal::{delay::Delay, prelude::*};

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut delay = Delay::new();

    // Initialize the I²C Bus
    let mut i2c = I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config {
            frequency: 400.kHz(),
            ..Default::default()
        },
    )
    .with_scl(peripherals.GPIO32)
    .with_sda(peripherals.GPIO33);

    // Initialize the LCD
    delay.delay(50.millis());
    let mut lcd = embedded_lcd::Display::new(embedded_lcd::interfaces::I2c::new(&mut i2c, 0x27))
        .with_lines(Lines::One)
        .with_font(Font::Font5x10)
        .with_cursor(Cursor::Disabled)
        .with_shift(Shift::Cursor, ShiftDirection::Right)
        .enabled(true)
        .init(&mut delay)
        .unwrap();
    lcd.enable_backlight().unwrap();

    loop {
        lcd.clear(&mut delay).unwrap();
        lcd.write_str("Hello", &mut delay).unwrap();
        delay.delay(1000.millis());
        lcd.home(&mut delay).unwrap();
        lcd.write_str("World!", &mut delay).unwrap();
        delay.delay(1000.millis());
    }
}
