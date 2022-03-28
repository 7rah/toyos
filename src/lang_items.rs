//! The panic handler

use owo_colors::OwoColorize;

use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "{} Panicked at {}:{} {}",
            "[kernel]".yellow(),
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!(
            "{} Panicked: {}",
            "[kernel]".yellow(),
            info.message().unwrap()
        );
    }
    shutdown()
}
