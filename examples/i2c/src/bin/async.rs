#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Delay, Duration, Timer};
use embedded_hal::i2c::SevenBitAddress;
#[allow(unused_imports)]
use esp_backtrace as _;

use embedded_lcd::interfaces::FourBitBus;
use embedded_lcd::{Async, Cursor, Font, Lines, Shift, ShiftDirection};
use esp_hal::i2c::master::I2c;
use esp_hal::prelude::*;
use esp_hal::timer::timg::TimerGroup;

async fn create_display<I2C>(
    i2c: &mut I2C,
) -> embedded_lcd::Display<
    embedded_lcd::interfaces::I2c<I2C, SevenBitAddress, Delay, Async>,
    FourBitBus,
    Async,
>
where
    I2C: embedded_hal_async::i2c::I2c,
{
    let interface = embedded_lcd::interfaces::I2c::new_async(i2c, 0x27, Delay);
    let mut lcd = embedded_lcd::Display::new_async(interface)
        .with_lines(Lines::_2)
        .with_font(Font::_5x8)
        .with_cursor(Cursor::Disabled)
        .with_shift(Shift::Cursor, ShiftDirection::Right)
        .enabled(true)
        .init()
        .await
        .unwrap();
    lcd.enable_backlight().await.unwrap();
    lcd
}

#[main]
async fn main(_s: Spawner) -> ! {
    esp_println::logger::init_logger_from_env();
    // This line is for Wokwi only so that the console output is formatted correctly
    esp_println::print!("\x1b[20h");
    
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let timg0 = TimerGroup::new(peripherals.TIMG0);

    // Initialize the embassy runtime
    esp_hal_embassy::init(timg0.timer0);

    // Initialize the LCD
    Timer::after(Duration::from_millis(50)).await;
    let mut i2c = I2c::new(peripherals.I2C0, esp_hal::i2c::master::Config::default())
        .with_scl(peripherals.GPIO32)
        .with_sda(peripherals.GPIO33)
        .into_async();
    let mut lcd = create_display(&mut i2c).await;

    loop {
        lcd.clear().await.unwrap();
        lcd.pos(Lines::_1, 0).await.unwrap();
        lcd.write_string("     Hello      ").await.unwrap();
        Timer::after(Duration::from_secs(1)).await;
        lcd.clear().await.unwrap();
        lcd.pos(Lines::_2, 0).await.unwrap();
        lcd.write_string("     World!     ").await.unwrap();
        Timer::after(Duration::from_secs(1)).await;
    }
}
