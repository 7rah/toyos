//! The panic handler

use owo_colors::OwoColorize;

use crate::{
    sbi::shutdown,
    stack_trace::{get_fp, print_stack_trace},
};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "{} Panicked at {}:{} {}",
            "[K]".green(),
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("{} Panicked: {}", "[K]".green(), info.message().unwrap());
    }

    unsafe {
        print_stack_trace(get_fp());
    }
    shutdown()
}
