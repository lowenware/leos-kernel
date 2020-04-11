//
// log.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use core::ptr;
use core::fmt;

struct Log;

const UART0: *mut u8 = 0xffff_ffe0_0900_0000 as *mut u8; // TODO: board dependent code

impl fmt::Write for Log {
    fn write_str(&mut self, msg: &str) -> fmt::Result {
        for chr in msg.chars() {
            unsafe {
                ptr::write_volatile(UART0, chr as u8);
            }
        }
        Ok(())
    }
}

fn log() -> impl fmt::Write {
    Log {}
}

pub fn write_fmt(args: fmt::Arguments) {
    use core::fmt::Write;
    log().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! log_write {
    ($($arg:tt)*) => ($crate::log::write_fmt(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! log {
    () => ($crate::log::write_fmt!("\n"));
    ($($arg:tt)*) => ({
        $crate::log::write_fmt(format_args_nl!($($arg)*));
    })
}
