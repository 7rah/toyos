//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

//#![deny(missing_docs)]
//#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(type_ascription)]

use core::arch::global_asm;
use log::{info, LevelFilter};
use owo_colors::OwoColorize;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

//entry.asm
global_asm!(
    "
.section .text.entry
.globl _start
_start:
la sp, boot_stack_top
call rust_main

.section .bss.stack
.globl boot_stack
boot_stack:
.space 4096 * 16
.globl boot_stack_top
boot_stack_top:
"
);

/// clear BSS segment
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn boot_stack(); // stack bottom
        fn boot_stack_top(); // stack top
    }
    clear_bss();

    logging::init(LevelFilter::Debug).unwrap();

    info!("Hello, kernel!");
    info!("My number is {}!", "4ffff4".bright_green());

    println!(".text\t[{:#x}, {:#x})", stext as usize, etext as usize);
    println!(
        ".rodata\t[{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    println!(".data\t[{:#x}, {:#x})", sdata as usize, edata as usize);
    println!(
        "boot_stack\t[{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    println!(".bss\t[{:#x}, {:#x})", sbss as usize, ebss as usize);

    let res: u128 = (0..91121221413330000333333333223333333122).sum();
    println!("res:{}", res);

    panic!("Shutdown machine!");
}
