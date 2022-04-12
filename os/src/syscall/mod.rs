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

use crate::task::TASK_MANAGER;

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
    let user_stack_range = TASK_MANAGER
        .get_current_task_stack()
        .get_stack()
        .as_ptr_range();
    let task_range = TASK_MANAGER.get_current_task_data_section().as_ptr_range();
    let buf_range = (buf as *const u8)..((buf as usize + len) as *const u8);

    //debug!("{user_stack_range:?} {current_app_range:?} {buf_range:?}");
    let in_range =
        |a: &Range<*const u8>, b: &Range<*const u8>| (b.start <= a.start) & (b.end >= a.end);

    if !in_range(&buf_range, &user_stack_range) & !in_range(&buf_range, &task_range) {
        info!("Task access out of bounds, excepted in user stack {user_stack_range:?} or in data section {task_range:?}, but given {buf_range:?}");
        false
    } else {
        true
    }
}
