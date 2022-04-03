//! App management syscalls
use super::check_buf;
use crate::batch::{run_next_app, APP_MANAGER};

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    run_next_app()
}

pub fn sys_get_taskinfo(buf: *mut u8, len: usize) -> isize {
    if !check_buf(buf, len) {
        return -1;
    }
    let app_manager = APP_MANAGER.exclusive_access();
    let app_id = app_manager.current_app;

    let dst = unsafe { core::slice::from_raw_parts_mut(buf, len) };
    let name = app_manager.app_name[app_id];
    let src = name.as_bytes();

    if src.len() > dst.len() {
        return -1;
    }

    dst[0..src.len()].copy_from_slice(src);

    app_id as isize
}
