use core::{arch::asm, ptr};
use owo_colors::OwoColorize;

pub unsafe fn print_stack_trace(mut fp: *const usize) {
    println!("{}", "== Begin stack trace ==".green());
    while fp != ptr::null() {
        let saved_ra = *fp.sub(1);
        let saved_fp = *fp.sub(2);

        println!("ra = 0x{:016x}, fp = 0x{:016x}", saved_ra, saved_fp);

        fp = saved_fp as *const usize;
    }
    println!("{}", "== End stack trace ==".green());
}

pub unsafe fn get_fp() -> *const usize {
    let fp: *const usize;
    asm!("mv {}, fp", out(reg) fp);
    fp
}
