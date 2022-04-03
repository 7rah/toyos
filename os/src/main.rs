#![no_std]
#![no_main]
#![feature(asm_const)]

use log::{info, LevelFilter};
use toyos::{batch::run_next_app, timer::timer_now};

#[no_mangle]
pub fn main() -> ! {
    toyos::clear_bss();
    toyos::logging::init(LevelFilter::Debug).unwrap();
    info!("Hello, kernel!");
    info!("{:.6}", timer_now().as_secs_f64());
    info!("{:?}", timer_now());
    info!("{:?}", timer_now());
    info!("{:?}", timer_now());

    toyos::trap::init();
    toyos::batch::init();

    run_next_app();
}
