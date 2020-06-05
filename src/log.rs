//
// log.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use core::fmt;
use crate::drivers::{SystemTimer};

use crate::platform::{UART0, TIMER0};

struct Log;

impl Log {
    fn write_string(&mut self, msg: &str) {
        use crate::drivers::SerialDevice;
        for chr in msg.chars() {
            unsafe {
                UART0.write(chr);
            }
        }
    }
}

impl fmt::Write for Log {
    fn write_str(&mut self, msg: &str) -> fmt::Result {
        self.write_string(msg);
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

pub fn mark(symbol: &str) {
    // TODO: resolve drivers synchronization
    unsafe {
        let value = TIMER0.get_value();
        let freq = TIMER0.get_frequency();
        log_write!("{:03}.{:05} {} ", value / freq, ((value % freq) * 10000) / freq, symbol);
    }
}

#[macro_export]
macro_rules! log_state {
    () => ($crate::log::write_fmt!("\n"));
    ($($arg:tt)*) => ({
        $crate::log::mark("-");
        $crate::log::write_fmt(format_args_nl!($($arg)*));
    })
}

#[macro_export]
macro_rules! log_debug {
    () => ($crate::log::write_fmt!("\n"));
    ($($arg:tt)*) => ({
        $crate::log::mark("*");
        $crate::log::write_fmt(format_args_nl!($($arg)*));
    })
}

#[macro_export]
macro_rules! log_error {
    () => ($crate::log::write_fmt!("\n"));
    ($($arg:tt)*) => ({
        $crate::log::mark("!");
        $crate::log::write_fmt(format_args_nl!($($arg)*));
    })
}
