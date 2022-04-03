use core::time::Duration;
use riscv::register::time;

pub fn get_cycle() -> u64 {
    time::read() as u64
}


pub fn timer_now() -> Duration {
    let time = get_cycle();
    Duration::from_nanos(time * 100)
}