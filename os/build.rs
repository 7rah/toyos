use std::{
    env,
    fs::{read_dir, File},
    io::{Result, Write},
    process::Command,
};

fn main() {
    if env::var("CARGO_FEATURE_CHECK_ONLY").is_err() {
        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-changed=../user/src/");
        println!("cargo:rerun-if-changed=misc/linker64.ld");
        println!("cargo:rerun-if-changed={}", TARGET_PATH);
        build_apps();
        insert_app_data().unwrap();
    }
}

fn build_apps() {
    Command::new("makers")
        .env_clear()
        .env("PATH", env!("PATH"))
        .current_dir("../user")
        .arg("strip-all")
        .status()
        .expect("failed to build apps")
        .success()
        .then(|| 0)
        .expect("failed to build apps");
}

static TARGET_PATH: &str = "../../user/target/riscv64gc-unknown-none-elf/release/";

fn insert_app_data() -> Result<()> {
    let mut f = File::create("src/link_app.rs").unwrap();
    let mut apps: Vec<_> = read_dir("../user/src/bin")
        .unwrap()
        .into_iter()
        .map(|dir_entry| {
            let mut name_with_ext = dir_entry.unwrap().file_name().into_string().unwrap();
            name_with_ext.drain(name_with_ext.find('.').unwrap()..name_with_ext.len());
            name_with_ext
        })
        .collect();
    apps.sort();

    writeln!(f, "pub static APP_BIN: &[&[u8]] = &[").unwrap();
    for app in &apps {
        writeln!(f, "    include_bytes!(\"{TARGET_PATH}{app}.bin\"),").unwrap();
    }
    writeln!(f, "];").unwrap();

    writeln!(f, "pub static APP_NAME: &[&str] = &[").unwrap();
    for name in &apps {
        writeln!(f, "    \"{name}\",").unwrap();
    }
    writeln!(f, "];").unwrap();

    writeln!(f, "pub const APP_NUM: usize = {};", apps.len()).unwrap();

    Ok(())
}
