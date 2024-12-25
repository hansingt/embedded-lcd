#![no_std]
#![no_main]
#[allow(unused_imports)]
use esp_backtrace as _;

use embedded_lcd::Lines;
use esp_hal::i2c::master::I2c;
use esp_hal::{delay::Delay, prelude::*};
use esp_println::println;

#[entry]
fn main() -> ! {
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
    println!("Initialize LCD...");
    let mut lcd = embedded_lcd::Display::new(embedded_lcd::interfaces::I2c::new(&mut i2c, 0x27))
        .enabled(true)
        .with_cursor(true, false)
        .init(&mut delay)
        .unwrap();
    lcd.enable_backlight().unwrap();

    println!("Start writing...");
    loop {
        lcd.pos(Lines::One, 0, &mut delay).unwrap();
        lcd.write_str("Hello", &mut delay).unwrap();
        delay.delay(500.millis());
        lcd.pos(Lines::One, 0, &mut delay).unwrap();
        lcd.write_str("World!", &mut delay).unwrap();
        delay.delay(500.millis());
    }
}
