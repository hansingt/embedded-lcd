#![no_std]
#![no_main]

#[allow(unused_imports)]
use esp_backtrace as _;

use embedded_lcd::{Cursor, Font, Lines, Shift, ShiftDirection};
use esp_hal::i2c::master::I2c;
use esp_hal::{delay::Delay, prelude::*};

fn create_display<I, A>(
    i2c: &mut I,
    address: A,
) -> embedded_lcd::Display<embedded_lcd::interfaces::I2c<I, A, Delay>>
where
    A: embedded_hal::i2c::AddressMode + Copy,
    I: embedded_hal::i2c::I2c<A>,
{
    let interface = embedded_lcd::interfaces::I2c::new(i2c, address, Delay::new());
    let mut lcd = embedded_lcd::Display::new(interface)
        .with_lines(Lines::One)
        .with_font(Font::Font5x10)
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
    let mut lcd = create_display(&mut i2c, 0x27);
    loop {
        lcd.clear().unwrap();
        lcd.write_string("Hello").unwrap();
        delay.delay(1000.millis());
        lcd.home().unwrap();
        lcd.write_string("World!").unwrap();
        delay.delay(1000.millis());
    }
}
