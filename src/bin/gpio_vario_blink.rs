#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;

use microbit::hal::pac::Peripherals;
use microbit::hal::{Saadc, Timer, saadc::SaadcConfig, gpio, gpio::Level};
use microbit::hal::prelude::*;

/*
Script used with potentiometer voltage read by ADC
and adjusting LED blinking speed.
*/

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Get ownership of peripherals
    let board = Peripherals::take().unwrap();

    // Declare TIMER0
    let mut timer = Timer::new(board.TIMER0);

    // Get GPIOS P0
    let gpios = gpio::p0::Parts::new(board.P0);

    // Initialize ADC on analog P0 (p0_02)
    let mut saadc = Saadc::new(board.SAADC, SaadcConfig::default());
    let mut saadc_pin = gpios.p0_02;

    // Configure GPIO P8 as digital output
    let mut p8 = gpios.p0_10.into_push_pull_output(Level::Low);

    // First ADC read
    let _saadc_result = saadc.read(&mut saadc_pin);

    loop {
        // Read value from ADC
        let read_value = saadc.read(&mut saadc_pin).unwrap();

        // Adjust value
        let delay = read_value as u32 / 10;

        // Make it blink
        timer.delay_ms(delay);
        p8.set_high().unwrap();
        timer.delay_ms(delay);
        p8.set_low().unwrap();
    }
}
