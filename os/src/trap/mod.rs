pub mod context;
use log::info;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap},
    stval, stvec,
};

use self::context::TrapContext;
use crate::{stack_trace::print_stack_trace, syscall::syscall, task::exit_current_and_run_next};

pub fn init() {
    unsafe {
        stvec::write(context::all_trap as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) {
    let scause = scause::read();
    let stval = stval::read();

    //info!("{cx:?} {stval:?}");

    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4; //move to next command
            cx.x10 = syscall(cx.x17, [cx.x10, cx.x11, cx.x12]) as usize;
            unsafe {
                context::restore_(cx);
            }
        }

        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            info!("PageFault in application, kernel killed it.");
            info!("sepc = {:#x}", cx.sepc);
            exit_current_and_run_next();
            unsafe { print_stack_trace(cx.x8 as *const usize) }
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            info!("IllegalInstruction in application, kernel killed it.");
            info!("sepc = {:#x}", cx.sepc);
            exit_current_and_run_next();
            unsafe { print_stack_trace(cx.x8 as *const usize) }
        }

        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
}
