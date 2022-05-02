#![no_std]
#![no_main]

//use user_lib::yield_;

use user_lib::yield_;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let x = 123.0 / 0.0;
    println!("float {}",x);
    0
}
