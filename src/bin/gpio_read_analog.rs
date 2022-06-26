#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;

use microbit::hal::{Saadc, Timer};
use microbit::hal::prelude::*;
use microbit::hal::saadc::SaadcConfig;
use microbit::hal::pac::Peripherals;
use microbit::hal::gpio;
use microbit::hal::gpio::Level;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Peripherals::take().unwrap();
    let gpios = gpio::p0::Parts::new(board.P0);
    let mut saadc = Saadc::new(board.SAADC, SaadcConfig::default());
    let mut saadc_pin = gpios.p0_02; // the pin your analog device is connected to
    let _saadc_result = saadc.read(&mut saadc_pin);

    loop {
        let read_value = saadc.read(&mut saadc_pin).unwrap();
        rprintln!("{}", read_value);
    }
}
