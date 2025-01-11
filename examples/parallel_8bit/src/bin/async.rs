#![no_std]
#![no_main]

#[allow(unused_imports)]
use esp_backtrace as _;

use embassy_executor::Spawner;
use embassy_time::{Delay, Duration, Timer};
use embedded_hal::digital::OutputPin;
use embedded_lcd::{Async, AsyncOutputPin, Cursor, Font, Lines, Shift, ShiftDirection};
use esp_hal::gpio::{Level, Output};
use esp_hal::timer::timg::TimerGroup;

async fn create_display<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B>(
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
    embedded_lcd::interfaces::Parallel8Bits<D0, D1, D2, D3, D4, D5, D6, D7, E, RS, B, Delay, Async>,
    Async,
>
where
    D0: AsyncOutputPin,
    D1: AsyncOutputPin,
    D2: AsyncOutputPin,
    D3: AsyncOutputPin,
    D4: AsyncOutputPin,
    D5: AsyncOutputPin,
    D6: AsyncOutputPin,
    D7: AsyncOutputPin,
    E: AsyncOutputPin,
    RS: AsyncOutputPin,
    B: AsyncOutputPin,
{
    let interface = embedded_lcd::interfaces::Parallel8Bits::new_async(
        d0, d1, d2, d3, d4, d5, d6, d7, e, rs, Delay,
    )
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
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let timg0 = TimerGroup::new(peripherals.TIMG0);

    esp_println::logger::init_logger_from_env();

    // Initialize the embassy runtime
    esp_hal_embassy::init(timg0.timer0);

    // Initialize the LCD
    Timer::after(Duration::from_millis(50)).await;
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
