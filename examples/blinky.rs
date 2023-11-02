// LED 2 blink
// Based on https://github.com/rtic-rs/rtic/blob/master/examples/stm32f3_blinky/src/main.rs

#![no_main]
#![no_std]

use panic_halt as _; // panic handler
use rtic::app;

#[app(device = stm32f3xx_hal::pac, dispatchers = [SPI1])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use stm32f3xx_hal::gpio::{Output, PushPull, PA5};
    use systick_monotonic::*;
    use stm32f3xx_hal::flash::FlashExt;
    use stm32f3xx_hal::prelude::*;


    #[monotonic(binds = SysTick, default = true)]
    type MyMono = Systick<100>; // 100 Hz / 10 ms granularity

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PA5<Output<PushPull>>,
        state: bool,
        counter: u32,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut flash = cx.device.FLASH.constrain();
        let mut rcc = cx.device.RCC.constrain();

        let systick = cx.core.SYST;

        let mono = Systick::new(systick, 36_000_000);

        hprintln!("init");

        let _clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(36.MHz())
            .pclk1(36.MHz())
            .freeze(&mut flash.acr);

        // Setup LED
        let mut gpioa = cx.device.GPIOA.split(&mut rcc.ahb);
        let mut led = gpioa
            .pa5
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
        led.set_high().unwrap();

        // Schedule the blinking task
        blink::spawn().ok();

        (Shared {}, Local {led, state: false, counter: 0}, init::Monotonics(mono))
    }

    #[task(local = [led, state, counter])]
    fn blink(cx: blink::Context) {
        if *cx.local.state {
            cx.local.led.set_high().unwrap();
            *cx.local.state = false;

            hprintln!("blink no. {}", cx.local.counter);
        } else {
            cx.local.led.set_low().unwrap();
            *cx.local.state = true;

            hprintln!("fade no. {}", cx.local.counter);
            *cx.local.counter += 1;
        }


        blink::spawn_after(1.secs()).unwrap();
    }
}
