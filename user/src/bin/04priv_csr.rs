#![no_std]
#![no_main]

use user::println;
use core::arch::asm;

#[no_mangle]
fn main() -> i32 {
    println!("Try to access privileged CSR in U Mode");
    println!("Kernel should kill this application!");
    unsafe {
        asm!("csrw sstatus, t0");
    }
    0
}
