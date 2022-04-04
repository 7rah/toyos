#![no_std]
#![no_main]
#![feature(asm_const)]

use log::{info, LevelFilter};
use toyos::batch::run_next_app;

#[no_mangle]
pub fn main() -> ! {
    toyos::clear_bss();
    toyos::logging::init(LevelFilter::Debug).unwrap();
    info!("Hello, kernel!");

    toyos::trap::init();
    toyos::batch::init();

    run_next_app();
}
