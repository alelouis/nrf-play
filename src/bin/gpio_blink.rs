#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;

use microbit::board::Board;
use microbit::hal::gpio::Level;
use microbit::hal::prelude::OutputPin;
use microbit::hal::Timer;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let pins = board.pins;
    let mut p8 = pins.p0_10.into_push_pull_output(Level::Low);
    p8.set_high().unwrap();
    loop {
        timer.delay_ms(500u16);
        rprintln!("High");
        p8.set_high().unwrap();
        timer.delay_ms(500u16);
        rprintln!("Low");
        p8.set_low().unwrap();
    }
}
