#![no_std]
#![no_main]

pub mod hal;
use panic_halt as _;
use cortex_m_rt::entry;
use crate::hal::Pac;
use crate::hal::gpio::{pwrp::UnlockRegisters, Output::OutputPin};

#[entry]
fn main() -> ! {
    UnlockRegisters();
    let dp = Pac::Prephipals::take();

    let led = OutputPin::new(dp.pins.D13);

    loop {
        led.set_high();
    }
}