//! App management syscalls

use super::check_buf;
use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next, TASK_MANAGER},
    timer::get_time_ms,
};

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get time in milliseconds
pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

pub fn sys_get_taskinfo(buf: *mut u8, len: usize) -> isize {
    if !check_buf(buf, len) {
        return -1;
    }
    let task_id = TASK_MANAGER.get_current_task();
    let name = TASK_MANAGER.get_current_task_name();

    let dst = unsafe { core::slice::from_raw_parts_mut(buf, len) };
    let src = name.as_bytes();

    if src.len() > dst.len() {
        return -1;
    }

    dst[0..src.len()].copy_from_slice(src);

    task_id as isize
}
