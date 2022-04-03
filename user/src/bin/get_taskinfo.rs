#![no_std]
#![no_main]

use core::str;

use user_lib::syscall::sys_get_taskinfo;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let mut v = [0u8; 32];
    let taskid = sys_get_taskinfo(&mut v);
    let name = str::from_utf8(&v).unwrap();
    println!("task_name: {}", name);
    println!("task_id:   {}", taskid);

    0
}
