#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

mod panic;
mod arch;
mod scheduler;

use core::ptr;
use arch::irq;
use arch::timer;

global_asm!(include_str!("arch/aarch64/start.s"));

#[no_mangle]
pub extern "C" fn kernel_main() {

    scheduler::init();
    timer::init();
    irq::enable();

    const UART0: *mut u8 = 0xffff_ffe0_0900_0000 as *mut u8;
    let out_str = b"Starting LeOS kernel";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    loop {
        unsafe {
            // TODO: remove timer debug register reads
            asm!("mrs x0, CNTPCT_EL0");
            asm!("mrs x0, CNTP_CTL_EL0");
            asm!("mrs x0, CNTP_TVAL_EL0");
            asm!("mrs x0, CNTP_CVAL_EL0");
            asm!("wfi");
        }
    }
}
