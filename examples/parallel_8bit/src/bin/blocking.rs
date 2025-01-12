#![no_std]
#![no_main]

#[allow(unused_imports)]
use esp_backtrace as _;

use embedded_hal::digital::OutputPin;
use embedded_lcd::interfaces::EightBitBus;
use embedded_lcd::{Blocking, Cursor, Font, Lines, Shift, ShiftDirection};
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output};
use esp_hal::prelude::*;

fn create_display<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B>(
    d0: D0,
    d1: D1,
    d2: D2,
    d3: D3,
    d4: D4,
    d5: D5,
    d6: D6,
    d7: D7,
    e: E,
    rs: RS,
    backlight: B,
) -> embedded_lcd::Display<
    embedded_lcd::interfaces::Parallel8Bits<
        D0,
        D1,
        D2,
        D3,
        D4,
        D5,
        D6,
        D7,
        E,
        RS,
        B,
        Delay,
        Blocking,
    >,
    EightBitBus,
    Blocking,
>
where
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
    E: OutputPin,
    RS: OutputPin,
    B: OutputPin,
{
    let interface = embedded_lcd::interfaces::Parallel8Bits::new(
        d0,
        d1,
        d2,
        d3,
        d4,
        d5,
        d6,
        d7,
        e,
        rs,
        Delay::new(),
    )
    .with_backlight(backlight);
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
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    esp_println::logger::init_logger_from_env();

    // Initialize the LCD
    delay.delay(50.millis());
    let mut lcd = create_display(
        Output::new(peripherals.GPIO22, Level::Low), // D0
        Output::new(peripherals.GPIO21, Level::Low), // D1
        Output::new(peripherals.GPIO19, Level::Low), // D2
        Output::new(peripherals.GPIO18, Level::Low), // D3
        Output::new(peripherals.GPIO5, Level::Low),  // D4
        Output::new(peripherals.GPIO4, Level::Low),  // D5
        Output::new(peripherals.GPIO0, Level::Low),  // D6
        Output::new(peripherals.GPIO2, Level::Low),  // D7
        Output::new(peripherals.GPIO23, Level::Low), // E
        Output::new(peripherals.GPIO32, Level::Low), // RS
        Output::new(peripherals.GPIO15, Level::Low), // Backlight
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
