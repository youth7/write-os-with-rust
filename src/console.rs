//! SBI console driver, for text output

use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// print string macro
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// println string macro
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println_with_color {
    ($color: expr, $fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[{}m", $fmt, "\x1b[0m", "\n"), $color, $($($arg)+)?));
    }
}

#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        println_with_color!(90, $fmt , $($($arg)+)?);
    }
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        println_with_color!(32, $fmt , $($($arg)+)?);
    }
}

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        println_with_color!(34, $fmt , $($($arg)+)?);
    }
}

#[macro_export]
macro_rules! WARN {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        println_with_color!(93, $fmt , $($($arg)+)?);
    }
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        println_with_color!(31, $fmt , $($($arg)+)?);
    }
}