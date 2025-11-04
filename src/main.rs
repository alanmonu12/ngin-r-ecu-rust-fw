#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use rtic::app;
use rtic_monotonics::systick::prelude::*;
use rtt_target::{rprintln, rtt_init_print};
// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to c    atch panics
extern crate cortex_m_rt;
extern crate cortex_m;
//extern crate stm32l4;

use stm32h7xx_hal::{
    pac, prelude::*, gpio::PE3, gpio::Output, gpio::PushPull
};

//use cortex_m::asm;
//use cortex_m_rt::entry;

systick_monotonic!(Mono, 1000);

#[app(device = pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PE3<Output<PushPull>>,
        state: bool,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        // Setup clocks
        let mut _flash = cx.device.FLASH;

        // Initialize the systick interrupt & obtain the token to prove that we did
        Mono::start(cx.core.SYST, 100_000_000); // default STM32F303 clock-rate is 36MHz

        rtt_init_print!();
        rprintln!("init");

        let pwr = cx.device.PWR.constrain();
        let pwrcfg = pwr.freeze();

        let rcc = cx.device.RCC.constrain();
        let ccdr = rcc
        .use_hse(25.MHz())
        .sys_ck(100.MHz())
        .freeze(pwrcfg, &cx.device.SYSCFG);

        // Setup LED
        let gpioe = cx.device.GPIOE.split(ccdr.peripheral.GPIOE);

        // Configure PE1 as output.
        let led = gpioe.pe3.into_push_pull_output();

        // Schedule the blinking task
        blink::spawn().ok();

        (Shared {}, Local { led, state: false })
    }

    #[task(local = [led, state])]
    async fn blink(cx: blink::Context) {
        loop {
            rprintln!("blink");
            if *cx.local.state {
                cx.local.led.set_high();
                *cx.local.state = false;
            } else {
                cx.local.led.set_low();
                *cx.local.state = true;
            }
            Mono::delay(500.millis()).await;
        }
    }
}