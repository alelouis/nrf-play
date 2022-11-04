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
    let device_address = 0b1100000;
    let write_dac = 0x40;
    let power_down_normal = 0b00;
    let data = 0x0fff;
    let send =     [
        write_dac as u8 + ((power_down_normal as u8) << 1),
        (data >> 4) as u8,
        (data & 0x000f << 4) as u8,
    ];

    let mut i2c = { twim::Twim::new(board.TWIM0, board.i2c_external.into(), FREQUENCY_A::K100) };
    i2c.write(device_address, &send).expect("TODO: panic message");

    //dac.set_dac(PowerDown::Normal, 0x0000).unwrap();
    rprintln!("Hello");
    loop {}
}