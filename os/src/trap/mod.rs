pub mod context;
use riscv::register::{mtvec::TrapMode, stvec};

pub fn init() {
    unsafe {
        stvec::write(context::all_trap as usize, TrapMode::Direct);
    }
}
