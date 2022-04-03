#![no_std]
#![no_main]

use user_lib::syscall::sys_write;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Hellol, world!");

    let v = unsafe { core::slice::from_raw_parts(11 as *const u8, 3) };
    sys_write(1, v);
    0
}
