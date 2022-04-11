#![no_std]
#![no_main]
#![feature(asm_const)]

use core::mem::size_of;

use log::{info, LevelFilter, debug};
use toyos::{task::{run_first_task, exit_current_and_run_next, run_next_task}, trap::context::{restore, TrapContext, all_trap}, config::APP_BASE_ADDRESS, loader::{KERNEL_STACK, USER_STACK}, link_app::APP_NUM};

#[no_mangle]
pub fn main() -> ! {
    toyos::clear_bss();
    toyos::logging::init(LevelFilter::Debug).unwrap();
    toyos::trap::init();
    toyos::loader::load_apps();
    debug!("restore {:?}",restore as *const u8);
    debug!("all_trap {:?}",all_trap as *const u8);

    for i in 0..APP_NUM {
        debug!("kernel stack start {i} {p:?}",p=KERNEL_STACK[i].data.as_ptr_range());
        debug!("user stack start {i} {p:?}",p=USER_STACK[i].data.as_ptr_range());
        
    }
    debug!("{}",size_of::<TrapContext>());
    //info!("Hello, kernel!");
    //info!("{:?}",USER_STACK.as_ptr());
    //let cx = TrapContext::init_app_context(APP_BASE_ADDRESS,USER_STACK[1].data.as_ptr() as usize);
    //unsafe {restore(&cx)};
    run_first_task();
    //run_next_task();
    //run_next_task();
    
    panic!("shutdown")
}
