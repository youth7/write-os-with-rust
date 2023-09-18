#![allow(unused)]
use core::arch::asm;

#[inline(always)]
fn sbi_call(eid: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {    // 用asm!而不是像之前那样include_str!的原因是需要给汇编传参
    let mut ret;
    unsafe {
        asm!(
            "ecall",//调用riscv的ecall指令
            inlateout("x10") arg0 => ret,//x10寄存器的值从由变量arg0指定。并且，指令调用结束后的返回值写到ret变量中
            in("x11") arg1,//传参，a1寄存器的值从由变量arg1指定
            in("x12") arg2,//传参，a2寄存器的值从由变量arg2指定
            in("x16") fid,//a6存function id
            in("x17") eid,//a7存extension id
        );
    }
    ret
}



// legacy extensions: ignore fid
const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;

// system reset extension
const SRST_EXTENSION: usize = 0x53525354;
const SBI_SHUTDOWN: usize = 0;

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, 0, c, 0, 0);
}

pub fn shutdown() -> ! {
    crate::println!("shutdown now ...");//这个宏是在后面console.rs中导出的
    sbi_call(SRST_EXTENSION, SBI_SHUTDOWN, 0, 0, 0);
    unreachable!("impossible to be here");
}