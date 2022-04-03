use core::arch::asm;

use log::info;
use repr_offset::ReprOffset;
use riscv::register::{
    scause::{self, Exception, Trap},
    sstatus::{self, Sstatus, SPP},
    stval,
};

use crate::{batch::run_next_app, stack_trace::print_stack_trace, syscall::syscall};

#[repr(C)]
#[derive(Debug, ReprOffset)]
#[roff(usize_offsets)]
pub struct TrapContext {
    x1: usize,
    x2: usize,
    x3: usize,
    x5: usize,
    x6: usize,
    x7: usize,
    x8: usize,
    x9: usize,
    x10: usize,
    x11: usize,
    x12: usize,
    x13: usize,
    x14: usize,
    x15: usize,
    x16: usize,
    x17: usize,
    x18: usize,
    x19: usize,
    x20: usize,
    x21: usize,
    x22: usize,
    x23: usize,
    x24: usize,
    x25: usize,
    x26: usize,
    x27: usize,
    x28: usize,
    x29: usize,
    x30: usize,
    x31: usize,
    sstatus: Sstatus,
    sepc: usize,
}

#[naked]
#[no_mangle]
#[repr(align(4))]
pub unsafe extern "C" fn all_trap() {
    asm!(
        "
            # sp->kernel stack, sscratch->user stack
            csrrw sp, sscratch, sp
            addi sp, sp, -{total_size}

            # save x1 x3 x5-x31
            sd x1, {off_x1}(sp)
            sd x3, {off_x3}(sp)
            sd x5, {off_x5}(sp)
			sd x6, {off_x6}(sp)
			sd x7, {off_x7}(sp)
			sd x8, {off_x8}(sp)
			sd x9, {off_x9}(sp)
			sd x10, {off_x10}(sp)
			sd x11, {off_x11}(sp)
			sd x12, {off_x12}(sp)
			sd x13, {off_x13}(sp)
			sd x14, {off_x14}(sp)
			sd x15, {off_x15}(sp)
			sd x16, {off_x16}(sp)
			sd x17, {off_x17}(sp)
			sd x18, {off_x18}(sp)
			sd x19, {off_x19}(sp)
			sd x20, {off_x20}(sp)
			sd x21, {off_x21}(sp)
			sd x22, {off_x22}(sp)
			sd x23, {off_x23}(sp)
			sd x24, {off_x24}(sp)
			sd x25, {off_x25}(sp)
			sd x26, {off_x26}(sp)
			sd x27, {off_x27}(sp)
			sd x28, {off_x28}(sp)
			sd x29, {off_x29}(sp)
			sd x30, {off_x30}(sp)
			sd x31, {off_x31}(sp)



            # save sstatus and spec
            # due to we have saved t0,t1 to kernel stack, we can use them to save sstatus and spec
            csrr t0, sstatus
            csrr t1, sepc
            sd t0, {off_sstatus}(sp)
            sd t1, {off_sepc}(sp)

            # save user stack to x2
            # t2 is the alias of x2
            csrr t2, sscratch
            sd t2, {off_x2}(sp)

            mv a0, sp
            call trap_handler

        ",
        total_size = const { core::mem::size_of::<TrapContext>() },
        off_x1 = const { TrapContext::OFFSET_X1 },
        off_x2 = const { TrapContext::OFFSET_X2 },
        off_x3 = const { TrapContext::OFFSET_X3 },
        off_x5 = const { TrapContext::OFFSET_X5 },
        off_x6 = const { TrapContext::OFFSET_X6 },
        off_x7 = const { TrapContext::OFFSET_X7 },
        off_x8 = const { TrapContext::OFFSET_X8 },
        off_x9 = const { TrapContext::OFFSET_X9 },
        off_x10 = const { TrapContext::OFFSET_X10 },
        off_x11 = const { TrapContext::OFFSET_X11 },
        off_x12 = const { TrapContext::OFFSET_X12 },
        off_x13 = const { TrapContext::OFFSET_X13 },
        off_x14 = const { TrapContext::OFFSET_X14 },
        off_x15 = const { TrapContext::OFFSET_X15 },
        off_x16 = const { TrapContext::OFFSET_X16 },
        off_x17 = const { TrapContext::OFFSET_X17 },
        off_x18 = const { TrapContext::OFFSET_X18 },
        off_x19 = const { TrapContext::OFFSET_X19 },
        off_x20 = const { TrapContext::OFFSET_X20 },
        off_x21 = const { TrapContext::OFFSET_X21 },
        off_x22 = const { TrapContext::OFFSET_X22 },
        off_x23 = const { TrapContext::OFFSET_X23 },
        off_x24 = const { TrapContext::OFFSET_X24 },
        off_x25 = const { TrapContext::OFFSET_X25 },
        off_x26 = const { TrapContext::OFFSET_X26 },
        off_x27 = const { TrapContext::OFFSET_X27 },
        off_x28 = const { TrapContext::OFFSET_X28 },
        off_x29 = const { TrapContext::OFFSET_X29 },
        off_x30 = const { TrapContext::OFFSET_X30 },
        off_x31 = const { TrapContext::OFFSET_X31 },
        off_sstatus = const { TrapContext::OFFSET_SSTATUS },
        off_sepc = const { TrapContext::OFFSET_SEPC },
        options(noreturn)
    );
}

#[naked]
#[no_mangle]
#[repr(align(4))]
pub unsafe extern "C" fn restore(cx: &TrapContext) {
    asm!(
        "
            # case1: start running app by __restore
            # case2: back to U after handling trap
            

            # restoe sstatus,spec
            mv sp, a0

            ld t0, {off_sstatus}(sp)
            ld t1, {off_sepc}(sp)
            ld t2, {off_x2}(sp)

            csrw sstatus, t0
            csrw sepc, t1
            csrw sscratch, t2 
        
            # restore x1 x3 x5~x31
            ld x1, {off_x1}(sp)
            ld x3, {off_x3}(sp)
            ld x5, {off_x5}(sp)
			ld x6, {off_x6}(sp)
			ld x7, {off_x7}(sp)
			ld x8, {off_x8}(sp)
			ld x9, {off_x9}(sp)
			ld x10, {off_x10}(sp)
			ld x11, {off_x11}(sp)
			ld x12, {off_x12}(sp)
			ld x13, {off_x13}(sp)
			ld x14, {off_x14}(sp)
			ld x15, {off_x15}(sp)
			ld x16, {off_x16}(sp)
			ld x17, {off_x17}(sp)
			ld x18, {off_x18}(sp)
			ld x19, {off_x19}(sp)
			ld x20, {off_x20}(sp)
			ld x21, {off_x21}(sp)
			ld x22, {off_x22}(sp)
			ld x23, {off_x23}(sp)
			ld x24, {off_x24}(sp)
			ld x25, {off_x25}(sp)
			ld x26, {off_x26}(sp)
			ld x27, {off_x27}(sp)
			ld x28, {off_x28}(sp)
			ld x29, {off_x29}(sp)
			ld x30, {off_x30}(sp)
			ld x31, {off_x31}(sp)

            # release TrapContext on kernel stack
            addi sp, sp, {total_size}

            # now sp->kernel stack, sscratch->user stack
            csrrw sp, sscratch, sp

            sret

        ",
        total_size = const { core::mem::size_of::<TrapContext>() },
        off_x1 = const { TrapContext::OFFSET_X1 },
        off_x2 = const { TrapContext::OFFSET_X2 },
        off_x3 = const { TrapContext::OFFSET_X3 },
        off_x5 = const { TrapContext::OFFSET_X5 },
        off_x6 = const { TrapContext::OFFSET_X6 },
        off_x7 = const { TrapContext::OFFSET_X7 },
        off_x8 = const { TrapContext::OFFSET_X8 },
        off_x9 = const { TrapContext::OFFSET_X9 },
        off_x10 = const { TrapContext::OFFSET_X10 },
        off_x11 = const { TrapContext::OFFSET_X11 },
        off_x12 = const { TrapContext::OFFSET_X12 },
        off_x13 = const { TrapContext::OFFSET_X13 },
        off_x14 = const { TrapContext::OFFSET_X14 },
        off_x15 = const { TrapContext::OFFSET_X15 },
        off_x16 = const { TrapContext::OFFSET_X16 },
        off_x17 = const { TrapContext::OFFSET_X17 },
        off_x18 = const { TrapContext::OFFSET_X18 },
        off_x19 = const { TrapContext::OFFSET_X19 },
        off_x20 = const { TrapContext::OFFSET_X20 },
        off_x21 = const { TrapContext::OFFSET_X21 },
        off_x22 = const { TrapContext::OFFSET_X22 },
        off_x23 = const { TrapContext::OFFSET_X23 },
        off_x24 = const { TrapContext::OFFSET_X24 },
        off_x25 = const { TrapContext::OFFSET_X25 },
        off_x26 = const { TrapContext::OFFSET_X26 },
        off_x27 = const { TrapContext::OFFSET_X27 },
        off_x28 = const { TrapContext::OFFSET_X28 },
        off_x29 = const { TrapContext::OFFSET_X29 },
        off_x30 = const { TrapContext::OFFSET_X30 },
        off_x31 = const { TrapContext::OFFSET_X31 },
        off_sstatus = const { TrapContext::OFFSET_SSTATUS },
        off_sepc = const { TrapContext::OFFSET_SEPC },
        options(noreturn)
    );
}

impl TrapContext {
    pub fn init_app_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);

        TrapContext {
            x1: 0,
            x2: sp,
            x3: 0,
            x5: 0,
            x6: 0,
            x7: 0,
            x8: 0,
            x9: 0,
            x10: 0,
            x11: 0,
            x12: 0,
            x13: 0,
            x14: 0,
            x15: 0,
            x16: 0,
            x17: 0,
            x18: 0,
            x19: 0,
            x20: 0,
            x21: 0,
            x22: 0,
            x23: 0,
            x24: 0,
            x25: 0,
            x26: 0,
            x27: 0,
            x28: 0,
            x29: 0,
            x30: 0,
            x31: 0,
            sstatus,
            sepc: entry,
        }
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) {
    let scause = scause::read();
    let stval = stval::read();

    //info!("{cx:?} {stval:?}");

    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4; //move to next
            cx.x10 = syscall(cx.x17, [cx.x10, cx.x11, cx.x12]) as usize;
            unsafe {
                restore(cx);
            }
        }

        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            info!("PageFault in application, kernel killed it.");
            info!("sepc = {:#x}",cx.sepc);
            unsafe { print_stack_trace(cx.x8 as *const usize) }
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            info!("IllegalInstruction in application, kernel killed it.");
            info!("sepc = {:#x}",cx.sepc);
            unsafe { print_stack_trace(cx.x8 as *const usize) }
            run_next_app();
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
