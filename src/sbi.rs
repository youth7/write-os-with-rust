#![allow(unused)]
use core::arch::asm;

#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    // 用asm!而不是像之前那样include_str!的原因是需要给汇编传参
    let mut ret;
    unsafe {
        asm!(
            "ecall",//调用riscv的ecall指令
            inlateout("x10") arg0 => ret,//x10寄存器的值从由变量arg0指定，指令调用结束后的返回值写到ret变量中
            in("x11") arg1,//x11寄存器的值从由变量arg1指定
            in("x12") arg2,//x12寄存器的值从由变量arg2指定
            in("x17") which,//x17寄存器的值从由变量which指定

        );
    }
    ret
}



const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    loop{}
}