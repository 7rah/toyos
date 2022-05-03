pub const USER_STACK_SIZE: usize = 4096 * 2; //8kB
pub const KERNEL_STACK_SIZE: usize = 4096 * 2; //8kB
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x200000;
pub const CLOCK_FREQ: usize = 12500000;
