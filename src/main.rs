#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, timer::Timer, twim},
};
use panic_halt as _;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut led_display = Display::new(board.display_pins);

    // Show a smiley to indicate program started
    let smiley = [
        [0, 1, 0, 1, 0],
        [0, 1, 0, 1, 0],
        [0, 0, 0, 0, 0],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ];
    led_display.show(&mut timer, smiley, 1000);
    led_display.clear();

    // Use the external I2C bus (pins 19/20 on edge connector)
    let i2c = twim::Twim::new(
        board.TWIM0,
        board.i2c_external.into(),
        twim::Frequency::K100,
    );

    // Set up OLED display at address 0x3C
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    // Try to initialize the display
    if display.init().is_err() {
        // Show X on LED matrix if init fails
        let x_pattern = [
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 0, 1, 0],
            [1, 0, 0, 0, 1],
        ];
        loop {
            led_display.show(&mut timer, x_pattern, 1000);
            led_display.clear();
            timer.delay_ms(500u32);
        }
    }

    // Show checkmark on LED matrix - init succeeded!
    let check = [
        [0, 0, 0, 0, 1],
        [0, 0, 0, 1, 0],
        [1, 0, 1, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    led_display.show(&mut timer, check, 1000);
    led_display.clear();

    // Draw "Hello World!" on OLED
    display.clear(BinaryColor::Off).ok();
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new("Hello Tony of Time!", Point::new(0, 10), text_style)
        .draw(&mut display)
        .ok();
    display.flush().ok();

    // Show heart on LED matrix - display updated!
    let heart = [
        [0, 1, 0, 1, 0],
        [1, 0, 1, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
    ];
    led_display.show(&mut timer, heart, 2000);
    led_display.clear();

    loop {
        timer.delay_ms(1000u32);
    }
}
