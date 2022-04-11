use core::arch::asm;

use log::{info, debug};
use repr_offset::ReprOffset;
use seq_macro::seq;

use crate::trap::context::{restore, TrapContext};

#[repr(C)]
#[derive(Debug, ReprOffset, Copy, Clone)]
#[roff(usize_offsets)]
pub struct TaskContext {
    /// return address ( e.g. restore ) of switch ASM function
    ra: usize,
    /// kernel stack pointer of app
    sp: usize,
    /// callee saved registers:  s 0..11
    s: [usize; 12],
}

pub unsafe fn switch(current_task: *mut TaskContext, next_task: *const TaskContext) {
    // we use compiler to help us save registers which should be saved by caller
    __switch(current_task, next_task)
}

seq! {N in 0..=11 {
#[naked]
#[no_mangle]
#[repr(align(4))]
pub unsafe extern "C" fn __switch(current_task: *mut TaskContext, next_task: *const TaskContext) {
    asm!(
        "
            # save current sp, ra, s0~s11
            sd sp, {off_sp}(a0)
            sd ra, {off_ra}(a0)",
            #(
                concat!("sd s",N,", {off_s",N,"}(a0)"), //equal to sd s0~11, {off_x0~11}(a0)
            )*

            // restore next sp, ra, s0~s11
            #(
                concat!("ld s",N,", {off_s",N,"}(a1)"), //equal to ld s0~11, {off_x0~11}(a0)
            )*
        "
            ld sp, {off_sp}(a1)
            ld ra, {off_ra}(a1)
            ret",
    off_sp = const {TaskContext::OFFSET_SP},
    off_ra = const {TaskContext::OFFSET_RA},
    #(
        off_s~N  = const {TaskContext::OFFSET_S + core::mem::size_of::<usize>() * N}, //off_s0~12
    )*
    options(noreturn))
}
}}

impl TaskContext {
    pub fn goto_restore(kstack_ptr: &TrapContext) -> TaskContext {
        debug!("kstack_ptr {:?}",kstack_ptr as *const TrapContext);
        TaskContext {
            ra: restore as usize,
            sp: kstack_ptr as *const TrapContext as usize,
            s: [0; 12],
        }
    }
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
}
