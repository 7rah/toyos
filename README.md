## 安装必要的依赖

### Rust

```shell
# 首先确保你已经安装了 rust toolchain. https://www.rust-lang.org/tools/install
rustup toolchain add nightly
rustup component add llvm-tools-preview
rustup target add riscv64gc-unknown-none-elf
cargo install cargo-binutils
cargo install cargo-make
cargo install cargo-watch
```

### QEMU

确保系统中有 `qemu-system-riscv64` 即可

```shell
# Archlinux
sudo pacman -S qemu-arch-extra
# Ubuntu
sudo apt install qemu-system-misc
```

Windows / Mac 用户可以参考 [QEMU 下载页面](https://www.qemu.org/download) 进行安装



## 运行

```shell
makers qemu
```
