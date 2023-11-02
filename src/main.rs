// Minimal binary

#![no_main]
#![no_std]

use panic_halt as _; // panic handler
use rtic::app;
use stm32f3xx_hal::prelude::*;

#[app(device = stm32f3xx_hal::pac)]
mod app {
    use cortex_m_semihosting::{debug, hprintln};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        hprintln!("Hello world!");
        (Shared {}, Local {}, init::Monotonics())
    }
}
