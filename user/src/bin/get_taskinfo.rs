#![no_std]
#![no_main]

use user_lib::syscall::sys_get_taskinfo;
use core::str;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let mut v = [0u8;32];
    let taskid = sys_get_taskinfo(&mut v);
    let name = str::from_utf8(&v).unwrap();
    println!("task_name: {}",name);
    println!("task_id:   {}",taskid);
    
    0
}
