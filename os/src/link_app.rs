<<<<<<< HEAD
pub static APP_BIN: &[&[u8]] = &[include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/get_taskinfo.bin"),include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/hello_world.bin"),include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/power.bin"),include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/priv_csr.bin"),include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/priv_inst.bin"),include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/store_fault.bin"),include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/unsafe_syswrite.bin"),];
pub static APP_NAME: &[&str] = &["get_taskinfo", "hello_world", "power", "priv_csr", "priv_inst", "store_fault", "unsafe_syswrite"];
=======
pub static APP_BIN: &[&[u8]] = &[
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/get_taskinfo.bin"),
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/hello_world.bin"),
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/power.bin"),
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/priv_csr.bin"),
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/priv_inst.bin"),
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/store_fault.bin"),
    include_bytes!("../../user/target/riscv64gc-unknown-none-elf/release/unsafe_syswrite.bin"),
];
pub static APP_NAME: &[&str] = &[
    "get_taskinfo",
    "hello_world",
    "power",
    "priv_csr",
    "priv_inst",
    "store_fault",
    "unsafe_syswrite",
];
pub const APP_NUM: usize = 7;
>>>>>>> tmp