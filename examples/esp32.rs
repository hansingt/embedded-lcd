#![no_std]
#![no_main]
use esp32_hal::{clock::ClockControl, i2c::I2C, peripherals::Peripherals, prelude::*, Delay, IO};
use esp_backtrace as _;
use lcd1602::{Font, Lines, Parallel4Bits, ShiftDirection, LCD1602};
use pcf857x::{Pcf8574, SlaveAddr};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio13,
        io.pins.gpio14,
        100u32.kHz(),
        &clocks,
    );
    let expander = Pcf8574::new(i2c, SlaveAddr::Alternative(true, true, true));
    let mut expander_pins = expander.split();
    let lcd_interface = Parallel4Bits::new(
        expander_pins.p7,
        expander_pins.p6,
        expander_pins.p5,
        expander_pins.p4,
        expander_pins.p2,
        expander_pins.p0,
        Delay::new(&clocks),
    );
    // We do only write to the LCD. Set P1 to low
    expander_pins.p1.set_low().unwrap();
    // Enable the backlight. Set P3 to high
    expander_pins.p3.set_high().unwrap();

    // Configure the LCD
    delay.delay_ms(50u32);
    let mut lcd = LCD1602::new(
        lcd_interface,
        Lines::Two,
        Font::FiveTimesEightDots,
        Delay::new(&clocks),
    )
    .unwrap();
    lcd.configure(true, false, false, false, ShiftDirection::Right)
        .unwrap();
    loop {
        lcd.write_str("  Hello ESP32!").unwrap();
        delay.delay_ms(1000u32);
        lcd.clear().unwrap();
        delay.delay_ms(1000u32);
    }
}
