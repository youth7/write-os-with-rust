#![no_main]
#![no_std]
use core::panic::PanicInfo;
use core::arch::global_asm;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

global_asm!(include_str!("entry.asm"));