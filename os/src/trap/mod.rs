pub mod context;

use core::time::Duration;

use log::{debug, info, trace};
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, sstatus, stval, stvec,
};

use self::context::TrapContext;
use crate::{
    stack_trace::print_stack_trace,
    syscall::{syscall, SyscallId},
    task::{exit_current_and_run_next, suspend_current_and_run_next, TASK_MANAGER},
    timer::{set_next_trigger, timer_now},
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
    let timestamp = TASK_MANAGER.get_timestamp();
    TASK_MANAGER.add_current_task_info_time(timer_now() - timestamp);
    let scause = scause::read();
    let stval = stval::read();
    let taskinfo = TASK_MANAGER.get_current_task_info();

    trace!(
        "sp: {:#x?} sepc: {:#x?} {:?} {}",
        cx.x2,
        cx.sepc,
        scause.cause(),
        sstatus::read().sie()
    );
    trace!("cx ptr {:#x?}", cx as *mut TrapContext);
    trace!("cx val {:#x?}", cx);
    trace!("{:?}", taskinfo);

    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4; //move to next command

            let start = timer_now();
            cx.x10 = syscall(cx.x17, [cx.x10, cx.x11, cx.x12]) as usize;
            let end = timer_now();
            TASK_MANAGER
                .add_current_task_info_call_times(SyscallId::from(cx.x17))
                .expect("Unreachable!");
            TASK_MANAGER.add_current_task_info_time(end - start);
        }

        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            info!("PageFault in application, kernel killed it.");
            info!("sepc = {:#x}, {:?}", cx.sepc, taskinfo);
            unsafe { print_stack_trace(cx.x8 as *const usize) }
            exit_current_and_run_next();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            info!("IllegalInstruction in application, kernel killed it.");
            info!("sepc = {:#x}, {:?}", cx.sepc, taskinfo);
            unsafe { print_stack_trace(cx.x8 as *const usize) }
            exit_current_and_run_next();
        }

        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            trace!("time is up, switch to next task");
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

    TASK_MANAGER.set_timestamp(timer_now());
    //panic!("trap_handler() leak!")
}
