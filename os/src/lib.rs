#![feature(panic_info_message)]
#![feature(type_ascription)]
#![feature(asm_const)]
#![feature(fn_align)]
#![feature(naked_functions)]
#![no_std]

use core::arch::global_asm;

#[macro_use]
pub mod console;
pub mod config;
mod lang;
pub mod link_app;
pub mod loader;
pub mod logging;
pub mod sbi;
pub mod stack_trace;
pub mod sync;
pub mod syscall;
pub mod task;
pub mod timer;
pub mod trap;

global_asm!(
    "
    .section .text.entry
    .globl _start
    _start:
    la sp, boot_stack_top
    call main

    .section .bss.stack
    .globl boot_stack
    boot_stack:
    .space 1024 * 16
    .globl boot_stack_top
    boot_stack_top:
"
);

pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
