use core::arch::asm;

use log::info;
use repr_offset::ReprOffset;
use riscv::register::{
    scause::{self, Exception, Trap},
    sstatus::{self, Sstatus, SPP},
    stval,
};
use seq_macro::seq;

use crate::{batch::run_next_app, stack_trace::print_stack_trace, syscall::syscall};

seq!(N in 5..=31 {
#[repr(C)]
#[derive(Debug, ReprOffset)]
#[roff(usize_offsets)]
pub struct TrapContext {
    x1: usize,
    x2: usize,
    x3: usize,
    #(
        x~N:usize, //x5~x31
    )*
    sstatus: Sstatus,
    sepc: usize,
}});

seq!(N in 5..=31 {

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
            sd x3, {off_x3}(sp)",
        #(
            concat!("sd x",N,", {off_x",N,"}(sp)"), //equal to sd x5~31, {off_x5~31}(sp)
        )*
        "
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
        #(
            off_x~N = const { TrapContext::OFFSET_X~N }, //x5~x31
        )*
        off_sstatus = const { TrapContext::OFFSET_SSTATUS },
        off_sepc = const { TrapContext::OFFSET_SEPC },
        options(noreturn)
    );
}
});

seq!(N in 5..=31 {
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
            ld x3, {off_x3}(sp)",
            #(
                concat!("ld x",N,", {off_x",N,"}(sp)"), //equal to sd x5~31, {off_x5~31}(sp)
            )*
            "
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
        #(
            off_x~N = const { TrapContext::OFFSET_X~N }, //x5~x31
        )*
        off_sstatus = const { TrapContext::OFFSET_SSTATUS },
        off_sepc = const { TrapContext::OFFSET_SEPC },
        options(noreturn)
    );
}
});

impl TrapContext {
    pub fn init_app_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);

        seq!(N in 5..=31 {

        TrapContext {
                x1: 0,
                x2: sp,
                x3: 0,
                #(
                    x~N:0, //x5~x31
                )*
                sstatus,
                sepc: entry,
        }

        })
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
            info!("sepc = {:#x}", cx.sepc);
            unsafe { print_stack_trace(cx.x8 as *const usize) }
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            info!("IllegalInstruction in application, kernel killed it.");
            info!("sepc = {:#x}", cx.sepc);
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
