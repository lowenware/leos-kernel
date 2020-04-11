#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]

mod arch;
mod log;
mod panic;
mod scheduler;

use arch::irq;
use arch::timer;

global_asm!(include_str!("arch/aarch64/start.s"));

#[no_mangle]
pub extern "C" fn kernel_main() {

    log_write!("Starting LeOS kernel");

    scheduler::init();
    timer::init();
    irq::enable();

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
