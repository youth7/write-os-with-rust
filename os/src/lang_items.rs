//! The panic handler

use crate::sbi::shutdown;
use core::panic::PanicInfo;
use crate::{println_with_color, error};
// use crate::error;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        error!("[kernel] Panicked: {}", info.message().unwrap());
    }
    shutdown(true)
}
