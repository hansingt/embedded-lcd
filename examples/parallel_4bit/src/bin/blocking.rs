#![no_std]
#![no_main]

#[allow(unused_imports)]
use {defmt_rtt as _, esp_backtrace as _};

use embedded_hal::digital::OutputPin;
use embedded_lcd::{Blocking, Cursor, Font, Lines, Shift, ShiftDirection};
use esp_hal::gpio::{Level, Output};
use esp_hal::{delay::Delay, prelude::*};

fn create_display<D7, D6, D5, D4, EN, RS, B>(
    d7: D7,
    d6: D6,
    d5: D5,
    d4: D4,
    en: EN,
    rs: RS,
    backlight: B,
) -> embedded_lcd::Display<
    embedded_lcd::interfaces::Parallel4Bits<D7, D6, D5, D4, EN, RS, Delay>,
    B,
    Blocking,
>
where
    D7: OutputPin,
    D6: OutputPin,
    D5: OutputPin,
    D4: OutputPin,
    EN: OutputPin,
    RS: OutputPin,
    B: OutputPin,
{
    let interface =
        embedded_lcd::interfaces::Parallel4Bits::new(d7, d6, d5, d4, en, rs, Delay::new());
    let mut lcd = embedded_lcd::Display::new(interface)
        .with_lines(Lines::_2)
        .with_font(Font::_5x8)
        .with_cursor(Cursor::Disabled)
        .with_shift(Shift::Cursor, ShiftDirection::Right)
        .enabled(true)
        .with_backlight(backlight)
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
    let mut lcd = create_display(
        Output::new(peripherals.GPIO2, Level::Low),
        Output::new(peripherals.GPIO0, Level::Low),
        Output::new(peripherals.GPIO4, Level::Low),
        Output::new(peripherals.GPIO5, Level::Low),
        Output::new(peripherals.GPIO18, Level::Low),
        Output::new(peripherals.GPIO19, Level::Low),
        Output::new(peripherals.GPIO15, Level::Low),
    );

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
