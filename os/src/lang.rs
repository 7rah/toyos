//! The panic handler

use core::panic::PanicInfo;

use owo_colors::OwoColorize;

use crate::{
    sbi::shutdown,
    stack_trace::{get_fp, print_stack_trace},
};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "{} Panicked at {}:{} {}",
            "[K]".green(),
            location.file().cyan(),
            location.line().cyan(),
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
