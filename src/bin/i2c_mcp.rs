#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use mcp4725::*;

use microbit::hal::prelude::*;

#[cfg(feature = "v2")]
use microbit::{
    hal::twim,
    pac::twim0::frequency::FREQUENCY_A,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut i2c = { twim::Twim::new(board.TWIM0, board.i2c_external.into(), FREQUENCY_A::K100) };
    let mut dac = MCP4725::new(i2c, 0b000);

    dac.set_dac(PowerDown::Normal, 0x0000).unwrap();
    rprintln!("Hello");
    loop {}
}