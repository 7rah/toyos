#![no_std]
#![no_main]
#![feature(asm_const)]

use log::LevelFilter;
use toyos::task::run_first_task;

#[no_mangle]
pub fn main() -> ! {
    toyos::clear_bss();
    toyos::logging::init(LevelFilter::Debug).unwrap();
    toyos::trap::init();
    toyos::loader::load_apps();

    toyos::trap::enable_timer_interrupt();
    toyos::timer::set_next_trigger();

    run_first_task();

    panic!("shutdown")
}
