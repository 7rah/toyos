use core::{arch::asm, ops::Range};

use log::info;

use crate::{
    link_app::{APP_BIN, APP_NAME},
    sync::UPSafeCell,
    trap::context::{restore, TrapContext},
};

const USER_STACK_SIZE: usize = 4096 * 2; //1024 * 1024 * 16; //16MB  For using wasm_hello app
const KERNEL_STACK_SIZE: usize = 4096 * 2; //8kB
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x200000;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
pub struct UserStack {
    pub data: [u8; USER_STACK_SIZE],
}

// we use customed kernel stack to avoid stack overflow
// if we use default stack, we can't strink kernel stack
static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};
pub static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

impl KernelStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
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
}

pub struct AppManager {
    num_app: usize,
    next_app: usize,
    pub current_app: usize,
    app_bin: &'static [&'static [u8]],
    pub app_name: &'static [&'static str],
}

lazy_static::lazy_static! {
    pub static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {UPSafeCell::new({
        let app_bin = APP_BIN;
        let app_name = APP_NAME;
        AppManager { num_app:app_bin.len(),next_app:0,app_bin,current_app:0,app_name}
    })};
}

impl AppManager {
    pub fn get_current_app_data_section_range(&self) -> Range<*const u8> {
        let app_id = self.current_app;
        let len = self.app_bin[app_id].len();
        (APP_BASE_ADDRESS as *const u8)..((APP_BASE_ADDRESS + len) as *const u8)
    }

    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            panic!("All applications completed!");
        }

        let app_name = self.app_name[app_id];
        info!("Loading app_{app_id} {app_name}");
        //clear icache
        asm!("fence.i");

        //copy app data section
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
        let app_src = self.app_bin[app_id];
        if app_src.len() > APP_SIZE_LIMIT {
            panic!("current app_{app_id} {app_name} size is bigger than APP_SIZE_LIMIT {APP_SIZE_LIMIT:#x}");
        }
        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
        app_dst.copy_from_slice(app_src);
    }

    fn move_to_next_app(&mut self) {
        self.next_app += 1
    }
}

#[allow(dead_code)]
fn print_current_sp() {
    let sp: usize;
    unsafe {
        asm!("mv {sp}, sp", sp = out(reg) sp);
    }
    info!("sp {sp}");
}

pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    unsafe {
        app_manager.load_app(app_manager.next_app);
    }
    app_manager.current_app = app_manager.next_app;
    app_manager.move_to_next_app();

    // release stack
    //print_current_sp();
    drop(app_manager);
    //print_current_sp();
    //drop(len);

    unsafe {
        restore(KERNEL_STACK.push_context(TrapContext::init_app_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp(),
        )));

        //restore(&TrapContext::init_app_context(
        //    APP_BASE_ADDRESS,
        //    USER_STACK.get_sp(),
        //));
    };
    panic!("Unreachable in batch::run_current_app!");
}

pub fn init() {
    info!("There are {} apps", APP_MANAGER.exclusive_access().num_app);
}
