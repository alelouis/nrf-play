#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;

use microbit::hal::pac::Peripherals;
use microbit::hal::{Saadc, saadc::SaadcConfig, gpio};
use microbit::hal::prelude::*;

/*
Script used with potentiometer voltage read by ADC.
*/

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Get ownership of peripherals
    let board = Peripherals::take().unwrap();

    // Get GPIOS P0
    let gpios = gpio::p0::Parts::new(board.P0);

    // Initialize ADC on analog P0 (p0_02)
    let mut saadc = Saadc::new(board.SAADC, SaadcConfig::default());
    let mut saadc_pin = gpios.p0_02;

    // First ADC read
    let _saadc_result = saadc.read(&mut saadc_pin);

    loop {
        // Read ADC value
        let read_value = saadc.read(&mut saadc_pin).unwrap();

        // RTT debug print of sampled value
        rprintln!("{}", read_value);
    }
}
