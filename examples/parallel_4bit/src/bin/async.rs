#![no_std]
#![no_main]

#[allow(unused_imports)]
use esp_backtrace as _;

use embassy_executor::Spawner;
use embassy_time::{Delay, Duration, Timer};
use embedded_lcd::interfaces::FourBitBus;
use embedded_lcd::{Async, AsyncOutputPin, Cursor, Font, Lines, Shift, ShiftDirection};
use esp_hal::gpio::{Level, Output};
use esp_hal::timer::timg::TimerGroup;

async fn create_display<D7, D6, D5, D4, EN, RS, B>(
    d7: D7,
    d6: D6,
    d5: D5,
    d4: D4,
    en: EN,
    rs: RS,
    backlight: B,
) -> embedded_lcd::Display<
    embedded_lcd::interfaces::Parallel4Bits<D7, D6, D5, D4, EN, RS, B, Delay, Async>,
    FourBitBus,
    Async,
>
where
    D7: AsyncOutputPin,
    D6: AsyncOutputPin,
    D5: AsyncOutputPin,
    D4: AsyncOutputPin,
    EN: AsyncOutputPin,
    RS: AsyncOutputPin,
    B: AsyncOutputPin,
{
    let interface =
        embedded_lcd::interfaces::Parallel4Bits::new_async(d7, d6, d5, d4, en, rs, Delay)
            .with_backlight(backlight);
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

#[esp_hal_embassy::main]
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
    let mut lcd = create_display(
        Output::new(peripherals.GPIO2, Level::Low),
        Output::new(peripherals.GPIO0, Level::Low),
        Output::new(peripherals.GPIO4, Level::Low),
        Output::new(peripherals.GPIO5, Level::Low),
        Output::new(peripherals.GPIO18, Level::Low),
        Output::new(peripherals.GPIO19, Level::Low),
        Output::new(peripherals.GPIO15, Level::Low),
    )
    .await;

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
