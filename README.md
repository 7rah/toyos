## 安装必要的依赖

### Rust

```shell
# 无论选择哪种安装方式，请先安装好 Rust toolchain
# 自动安装
cargo install cargo-make
makers install-dependencies

# 手动安装
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
