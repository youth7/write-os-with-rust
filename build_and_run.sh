#!/bin/bash
# 构建
cargo build --release --target riscv64gc-unknown-none-elf 

# 剪裁
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin 

# 加载并运行
qemu-system-riscv64 \
-machine virt   \
-nographic  \
-bios ./bootloader/rustsbi-qemu.bin \
-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000