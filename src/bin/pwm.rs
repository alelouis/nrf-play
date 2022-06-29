#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;

use microbit::hal::{pac::Peripherals, Timer, gpio, pwm, gpio::Level};
use microbit::hal::prelude::*;
use microbit::hal::time::Hertz;


#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Get ownership of peripherals
    let board = Peripherals::take().expect("Couldn't initialize board.");

    // Configuring output pin
    let gpio = gpio::p0::Parts::new(board.P0);
    let pwm_pin = gpio.p0_02.into_push_pull_output(Level::Low).degrade();

    // Configuring output pin
    let pwm_motor = pwm::Pwm::new(board.PWM0);
    pwm_motor.set_output_pin(pwm::Channel::C0, pwm_pin);

    // Using a timer for delayed commands
    let mut timer = Timer::new(board.TIMER0);
    
    pwm_motor.set_period(Hertz(50));
    pwm_motor.enable_channel(pwm::Channel::C0);
    pwm_motor.loop_inf();

    let mut duty = 0_u16;
    loop {
        rprintln!("{}", duty);
        timer.delay_ms(1000_u32);
        duty += 1000;
        duty = duty % 30000;
        pwm_motor.set_duty_on(pwm::Channel::C0, duty);
    }
}