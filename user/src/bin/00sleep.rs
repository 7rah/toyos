#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{get_time, yield_};

#[no_mangle]
fn main() -> i32 {
    let current_timer = get_time();
    println!("current_time_ms {}", current_timer);
    let wait_for = current_timer + 3000;
    while get_time() < wait_for {
        yield_();
        //println!("time isn't up, sleeping");
        //println!("time isn't up, sleeping");
        //println!("time isn't up, sleeping");
    }
    println!("Test sleep OK!");
    0
}
