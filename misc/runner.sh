#! /bin/bash

path=$1;
bin_path=$1".bin";


rust-objcopy --strip-all $path -O binary $bin_path
qemu-system-riscv64 -machine virt -bios misc/rustsbi-qemu-no-log.bin -nographic -device loader,file=$bin_path,addr=0x80200000 