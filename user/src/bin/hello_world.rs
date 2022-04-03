#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let i: i32 = (0..111111).sum();
    println!("Hello, world! {}", i);

    0
}
