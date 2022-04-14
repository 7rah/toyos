pub mod context;
use core::arch::asm;

use log::{debug, info};
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, sstatus, stval, stvec,
};

use self::context::TrapContext;
use crate::{
    stack_trace::print_stack_trace,
    syscall::syscall,
    task::{exit_current_and_run_next, suspend_current_and_run_next},
    timer::set_next_trigger,
};

pub fn init() {
    unsafe {
        stvec::write(context::all_trap as usize, TrapMode::Direct);
    }
}

/// timer interrupt enabled
pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) {
    let scause = scause::read();
    let stval = stval::read();

    debug!(
        "sp: {:#x?} sepc: {:#x?} {:?} {}",
        cx.x2,
        cx.sepc,
        scause.cause(),
        sstatus::read().sie()
    );
    debug!("cx ptr {:#x?}", cx as *mut TrapContext);
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4; //move to next command
            cx.x10 = syscall(cx.x17, [cx.x10, cx.x11, cx.x12]) as usize;
        }

        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            info!("PageFault in application, kernel killed it.");
            info!("sepc = {:#x}", cx.sepc);
            debug!("{:#X?}", cx);
            exit_current_and_run_next();
            unsafe { print_stack_trace(cx.x8 as *const usize) }
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            info!("IllegalInstruction in application, kernel killed it.");
            info!("sepc = {:#x}", cx.sepc);
            debug!("{:#x?}", cx);
            exit_current_and_run_next();
            unsafe { print_stack_trace(cx.x8 as *const usize) }
        }

        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            debug!("{:#X?}", cx);
            debug!("time is up, switch to next task");
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    panic!("trap_handler() leak!")
}
