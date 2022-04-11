#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;
use user_lib::syscall::{sys_write, sys_exit};
use wasmi::{
    ImportsBuilder, MemoryRef, ModuleInstance,
    RuntimeValue, Trap,
};

#[no_mangle]
pub extern fn fmod(a: f64, b: f64) -> f64{
    a % b
}

#[no_mangle]
pub extern fn fmodf(a: f32, b: f32) -> f32{
    a % b
}



use linked_list_allocator::LockedHeap;
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
const HEAP_SIZE: usize = 1024 * 1024 * 16;
static mut HEAP_SPACE: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

pub fn init_heap() {
    unsafe {
        ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, HEAP_SIZE);
    }
}


#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

#[macro_use]
extern crate user_lib;

use alloc::format;
use wasmi::{
    Error, Externals, FuncInstance, FuncRef, ModuleImportResolver, RuntimeArgs, Signature,
    ValueType,
};

struct HostExternals {
    memory: Option<MemoryRef>,
}

const ADD_FUNC_INDEX: usize = 0;
const FD_WRITE_INDEX: usize = 1;
const PROC_EXIT_INDEX: usize = 2;
const ENVIRON_GET_INDEX: usize = 3;
const ENVIRON_SIZES_GET_INDEX: usize = 4;

impl Externals for HostExternals {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        match index {
            ADD_FUNC_INDEX => {
                println!("{:?}", args);

                Ok(None)
            }
            PROC_EXIT_INDEX => {
                let status: i32 = args.nth_checked(0)?;
                println!("process exit with status: {}",status);
                if status < 126 {
                    Err(Trap::new(wasmi::TrapKind::TableAccessOutOfBounds))
                } else {
                    Err(Trap::new(wasmi::TrapKind::Unreachable))
                }
            }

            FD_WRITE_INDEX => {
                let _fd: i32 = args.nth_checked(0)?;
                let iovs: i32 = args.nth_checked(1)?;
                let _iovs_len: i32 = args.nth_checked(2)?;

                let nwritten: i32 = args.nth_checked(3)?;
                //println!("{fd} {iovs} {iovs_len} {nwritten}");

                let memory = self.memory.as_ref().unwrap();

                let _written = memory.with_direct_access_mut(|buf| {
                    let base = iovs as usize;
                    let nwritten = nwritten as usize;
                    let ptr = u32::from_le_bytes(buf[base..4 + base].try_into().unwrap()) as usize;
                    let len = u32::from_le_bytes(buf[4 + base..8 + base].try_into().unwrap());

                    buf.copy_within(4 + base..8 + base, nwritten);
                    let tmp = &buf[ptr..ptr + len as usize];

                    sys_write(1,tmp);
                    //println!("{nwritten}");
                });

                Ok(Some(RuntimeValue::I32(0)))
            }

            ENVIRON_GET_INDEX => {
                unimplemented!("environ_get")
            }
            ENVIRON_SIZES_GET_INDEX => {
                Ok(Some(RuntimeValue::I32(0)))
                //unimplemented!("environ_sizes_get")
            }
            _ => panic!("Unimplemented function at {}", index),
        }
    }
}

impl ModuleImportResolver for HostExternals {
    fn resolve_func(&self, field_name: &str, _signature: &Signature) -> Result<FuncRef, Error> {
        let f = |params, return_type, index| {
            Ok(FuncInstance::alloc_host(
                Signature::new(params, return_type),
                index,
            ))
        };

        match field_name {
            "add" => f(&[][..], None, ADD_FUNC_INDEX),
            "proc_exit" => f(&[ValueType::I32][..], None, PROC_EXIT_INDEX),
            "fd_write" => f(
                &[
                    ValueType::I32,
                    ValueType::I32,
                    ValueType::I32,
                    ValueType::I32,
                ][..],
                Some(ValueType::I32),
                FD_WRITE_INDEX,
            ),

            "environ_get" => f(
                &[ValueType::I32, ValueType::I32][..],
                Some(ValueType::I32),
                ENVIRON_GET_INDEX,
            ),
            "environ_sizes_get" => f(
                &[ValueType::I32, ValueType::I32][..],
                Some(ValueType::I32),
                ENVIRON_SIZES_GET_INDEX,
            ),
            _ => {
                return Err(Error::Instantiation(format!(
                    "Export {} not found",
                    field_name
                )))
            }
        }
    }
}

#[no_mangle]
fn main() -> i32 {
    init_heap();
    run();
    0
}


fn run() {
    let module = {
        let wasm_buf = include_bytes!("../../hello.wasm");
        wasmi::Module::from_buffer(&wasm_buf).unwrap()
    };

    
    let mut ext = HostExternals { memory: None };
    let i = ImportsBuilder::new().with_resolver("wasi_snapshot_preview1", &ext);

    let instance = ModuleInstance::new(&module, &i)
        .expect("failed to instantiate wasm module")
        .assert_no_start();

    use alloc::borrow::ToOwned;
    let memory = instance
        .export_by_name("memory")
        .expect("`memory` export not found")
        .as_memory()
        .expect("export name `memory` is not of memory type")
        .to_owned();

    ext.memory = Some(memory);

    
    let _i = instance
        .invoke_export("_start", &[], &mut ext);
}
