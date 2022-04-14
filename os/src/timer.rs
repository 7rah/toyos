use core::time::Duration;

use riscv::register::time;

use crate::{config::CLOCK_FREQ, sbi::set_timer};

pub fn get_cycle() -> u64 {
    time::read() as u64
}

pub fn timer_now() -> Duration {
    let time = get_cycle();
    Duration::from_nanos(time * 100)
}

const TICKS_PER_SEC: usize = 100;

/// read the `mtime` register
pub fn get_time() -> usize {
    time::read()
}

pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

const MSEC_PER_SEC: usize = 1000;
pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}
