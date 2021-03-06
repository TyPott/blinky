#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m::{interrupt::Mutex, peripheral::NVIC};
use embedded_hal::{
    blocking::delay::DelayMs,
    digital::v2::{OutputPin, StatefulOutputPin},
};
use panic_halt as _;
use tm4c129x_hal::{
    self as hal,
    delay::Delay,
    gpio::{gpiof::PF4, gpioj::PJ0, GpioExt, Input, InterruptMode, Output, PullUp, PushPull},
    interrupt::GPIOJ,
    sysctl::SysctlExt,
    tm4c129x::interrupt,
};

struct SharedPeripherals {
    led: PF4<Output<PushPull>>,
    button: PJ0<Input<PullUp>>,
}

static SHARED: Mutex<RefCell<Option<SharedPeripherals>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn GPIOJ() {
    cortex_m::interrupt::free(|cs| {
        let mut shared = SHARED.borrow(cs).borrow_mut();
        let shared = shared.as_mut().unwrap();
        if shared.led.is_set_high().unwrap() {
            shared.led.set_low().unwrap();
        } else {
            shared.led.set_high().unwrap();
        }

        shared.button.clear_interrupt();
    });
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let sc = p.SYSCTL.constrain();

    // Configure clock source for delay
    let clocks = sc.clock_setup.freeze();
    let mut delay = Delay::new(cp.SYST, &clocks);

    // Configure pins for button-controlled LED
    let port_f = p.GPIO_PORTF_AHB.split(&sc.power_control);
    let led3 = port_f.pf4.into_push_pull_output();

    let port_j = p.GPIO_PORTJ_AHB.split(&sc.power_control);
    let mut button = port_j.pj0.into_pull_up_input();
    button.set_interrupt_mode(InterruptMode::EdgeFalling);

    // Make peripherals accessible from interrupt handler
    cortex_m::interrupt::free(|cs| {
        SHARED
            .borrow(cs)
            .replace(Some(SharedPeripherals { led: led3, button }))
    });

    // Enable interrupts on GPIO Port J
    unsafe {
        NVIC::unmask(GPIOJ);
    }

    // Configure pins for flashing LEDs
    let port_n = p.GPIO_PORTN.split(&sc.power_control);
    let mut led1 = port_n.pn1.into_push_pull_output();
    let mut led2 = port_n.pn0.into_push_pull_output();

    loop {
        led1.set_high().unwrap();
        delay.delay_ms(1000u32);
        led1.set_low().unwrap();
        led2.set_high().unwrap();
        delay.delay_ms(1000u32);
        led2.set_low().unwrap();
    }
}
