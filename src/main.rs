#![no_std]
#![no_main]
#![feature(global_asm)]

use core::ptr;

mod panic;

global_asm!(include_str!("aarch64/start.s"));

#[no_mangle]
pub extern "C" fn kernel_main() {
    const UART0: *mut u8 = 0xffff_ffe0_0900_0000 as *mut u8;
    let out_str = b"Starting LeOS kernel";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
}
