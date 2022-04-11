use core::arch::asm;

use log::{debug, info};

use crate::{
    config::{APP_BASE_ADDRESS, APP_SIZE_LIMIT, KERNEL_STACK_SIZE, USER_STACK_SIZE},
    link_app::{APP_BIN, APP_NUM},
    trap::context::TrapContext,
};

#[repr(align(4096))]
#[derive(Copy, Clone)]
pub struct KernelStack {
    pub data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
#[derive(Copy, Clone)]
pub struct UserStack {
    pub data: [u8; USER_STACK_SIZE],
}

// we use customed kernel stack to avoid stack overflow
// if we use default stack, we can't strink kernel stack
pub static KERNEL_STACK: [KernelStack; APP_NUM] = [KernelStack {
    data: [0; KERNEL_STACK_SIZE],
}; APP_NUM];

pub static USER_STACK: [UserStack; APP_NUM] = [UserStack {
    data: [0; USER_STACK_SIZE],
}; APP_NUM];

impl KernelStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn get_ptr(&self) -> usize {
        self.data.as_ptr() as usize
    }
    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn get_stack(&self) -> &[u8] {
        &self.data
    }
}

// load app to specific address
pub fn load_apps() {
    let srcs = APP_BIN;
    let dsts = unsafe {
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut [u8; APP_SIZE_LIMIT], APP_NUM)
    };

    let mut i = 0;
    for (src, dst) in srcs.iter().zip(dsts.iter_mut()) {
        dst[0..src.len()].copy_from_slice(src);
        dst[src.len()..].fill(0);
        debug!("app {i} at {:?}", dst.as_ptr_range());
        i += 1;
    }
}

pub fn init_app_cx(app_id: usize) -> &'static mut TrapContext {
    let get_base_i = |app_id| APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT;

    let t = KERNEL_STACK[app_id].push_context(TrapContext::init_app_context(
        get_base_i(app_id),
        USER_STACK[app_id].get_sp(),
    ));
    //debug!("{app_id} {t:?}");
    t
}
