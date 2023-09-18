#![feature(panic_info_message)]
#![no_main]
#![no_std]
use core::panic::PanicInfo;
use core::arch::global_asm;

mod console;
mod sbi;

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    //当panic的时候，打印堆栈信息然后关机
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    sbi::shutdown()
}

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("hello, power on !!");
    panic!("oh crash !! {}","-_-!")
}  


fn clear_bss() {
    extern "C" {
        fn sbss();//将链接脚本中sbss和ebss视为函数也不是不可以，因为后面不是实施真正的调用，只是利用了地址值
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}
   