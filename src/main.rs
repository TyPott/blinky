#![no_std]
#![no_main]

use panic_halt as _;
use tm4c129x_hal::{self as hal, delay::Delay, prelude::*};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let sc = p.SYSCTL.constrain();
    let clocks = sc.clock_setup.freeze();
    let mut delay = Delay::new(cp.SYST, &clocks);

    let port_n = p.GPIO_PORTN.split(&sc.power_control);
    let mut led1 = port_n.pn1.into_push_pull_output();
    let mut led2 = port_n.pn0.into_push_pull_output();

    loop {
        led1.set_high();
        delay.delay_ms(1000u32);
        led2.set_high();
        led1.set_low();
        delay.delay_ms(1000u32);
        led2.set_low();
    }
}
