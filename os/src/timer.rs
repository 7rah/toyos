use core::time::Duration;

use riscv::register::time;

use crate::{sbi::set_timer, config::CLOCK_FREQ};

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