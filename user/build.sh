#!/bin/bash
cargo build --release --target riscv64gc-unknown-none-elf
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/00hello_world -O binary target/riscv64gc-unknown-none-elf/release/00hello_world.bin
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/01store_fault -O binary target/riscv64gc-unknown-none-elf/release/01store_fault.bin
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/02power -O binary target/riscv64gc-unknown-none-elf/release/02power.bin
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/03priv_inst -O binary target/riscv64gc-unknown-none-elf/release/03priv_inst.bin
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/04priv_csr -O binary target/riscv64gc-unknown-none-elf/release/04priv_csr.bin