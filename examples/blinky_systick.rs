//! Blinks an LED with a SysTick as a delay timer.

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

use cortex_m;
use cortex_m_rt as rt;
use panic_halt as _;
use stm32wb_hal as hal;

use embedded_hal::digital::v2::OutputPin;

use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use crate::rt::entry;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    // Use default clock frequency of 4 MHz running from MSI
    let mut rcc = dp.RCC.constrain();

    // On STM32WB55-NUCLEO a green LED is connected to the pin PB0
    let mut gpiob = dp.GPIOB.split(&mut rcc);
    let mut led = gpiob
        .pb0
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut timer = Delay::new(cp.SYST, hal::rcc::Clocks::default());
    loop {
        timer.delay_ms(500 as u32);
        let _ = led.set_high();
        timer.delay_ms(500 as u32);
        let _ = led.set_low();
    }
}
