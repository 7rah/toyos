//! File and filesystem-related syscalls

use super::check_buf;
use crate::sbi::console_putchar;

const FD_STDOUT: usize = 1;

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if !check_buf(buf, len) {
        return 0;
    }

    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };

            for &i in slice {
                console_putchar(i as usize);
            }

            len as isize
        }
        _ => {
            panic!("Unsupported fd {fd} in sys_write!");
        }
    }
}
