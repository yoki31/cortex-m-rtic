//! examples/schedule.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965, dispatchers = [SSI0])]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use systick_monotonic::*;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = Systick<100>; // 100 Hz / 10 ms granularity

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let systick = cx.core.SYST;

        // Initialize the monotonic
        let mono = Systick::new(systick, 12_000_000);

        hprintln!("init").ok();

        // Schedule `foo` to run 1 second in the future
        foo::spawn_after(1.secs()).unwrap();

        (
            Shared {},
            Local {},
            init::Monotonics(mono), // Give the monotonic to RTIC
        )
    }

    #[task]
    fn foo(_: foo::Context) {
        hprintln!("foo").ok();

        // Schedule `bar` to run 2 seconds in the future (1 second after foo runs)
        bar::spawn_after(1.secs()).unwrap();
    }

    #[task]
    fn bar(_: bar::Context) {
        hprintln!("bar").ok();

        // Schedule `baz` to run 1 seconds from now, but with a specific time instant.
        baz::spawn_at(monotonics::now() + 1.secs()).unwrap();
    }

    #[task]
    fn baz(_: baz::Context) {
        hprintln!("baz").ok();
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }
}
