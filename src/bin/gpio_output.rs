#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;

use microbit::board::Board;
use microbit::hal::gpio::Level;
use microbit::hal::prelude::OutputPin;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let pins = board.pins;
    let mut p8 = pins.p0_10.into_push_pull_output(Level::Low);
    p8.set_high().unwrap();
    loop {}
}
