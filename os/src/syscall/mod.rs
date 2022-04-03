//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_GET_TASKINFO: usize = 233;

mod fs;
mod process;

use core::ops::Range;

use fs::*;
use log::info;
use process::*;

use crate::batch::{APP_MANAGER, USER_STACK};

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_GET_TASKINFO => sys_get_taskinfo(args[0] as *mut u8, args[1]),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}

fn check_buf(buf: *const u8, len: usize) -> bool {
    let user_stack_start = USER_STACK.data.as_ptr() as usize;
    let user_stack_range = user_stack_start..USER_STACK.data.len() + user_stack_start;
    let current_app_range = APP_MANAGER
        .exclusive_access()
        .get_current_app_data_section_range();
    let buf_range = (buf as usize)..(buf as usize + len);

    //debug!("{user_stack_range:?} {current_app_range:?} {buf_range:?}");
    let in_range = |a: &Range<usize>, b: &Range<usize>| (b.start <= a.start) & (b.end >= a.end);

    if !in_range(&buf_range, &user_stack_range) & !in_range(&buf_range, &current_app_range) {
        info!("App access out of bounds, excepted in user stack {:#x}..{:#x} or in data section {:#x}..{:#x}, but given {:#x}..{:#x}",user_stack_range.start,user_stack_range.end,current_app_range.start,current_app_range.end,buf_range.start,buf_range.end);
        false
    } else {
        true
    }
}
