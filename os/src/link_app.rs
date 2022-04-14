pub static APP_BIN: &[&[u8]] = &[
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/00sleep.bin"),
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/01power_5.bin"),
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/02power_7.bin"),
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/04power_3.bin"),
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/0yield.bin"),
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/get_taskinfo.bin"),
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/hello_world.bin"),
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/power.bin"),
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/priv_csr.bin"),
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/priv_inst.bin"),
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/store_fault.bin"),
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/unsafe_syswrite.bin"),
];
pub static APP_NAME: &[&str] = &[
    "00sleep",
    "01power_5",
    "02power_7",
    "04power_3",
    "0yield",
    "get_taskinfo",
    "hello_world",
    "power",
    "priv_csr",
    "priv_inst",
    "store_fault",
    "unsafe_syswrite",
];
pub const APP_NUM: usize = 12;
