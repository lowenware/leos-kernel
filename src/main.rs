#![no_std]
#![no_main]
#![feature(alloc_layout_extra)]
#![feature(alloc_error_handler)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]

extern crate alloc;

#[macro_use]
mod log;

mod arch;
mod board;
mod memory;
mod panic;
mod scheduler;

use arch::irq;
use arch::timer;

use alloc::{boxed::Box};

global_asm!(include_str!("arch/aarch64/start.s"));

// pub const KERNEL_BASE: u64 = 0xffff_fff0_0000_0000;
//
#[inline(never)]
fn test_heap() {
    memory::log_heap();
    // test heap
    let x = Box::new(41);
    log_debug!("allocated from heap {:p}", x);
    memory::log_heap();

    let x = Box::new([1, 2, 3]);
    log_debug!("allocated from heap {:p}", x);
    memory::log_heap();
}

#[no_mangle]
pub extern "C" fn kernel_main(kernel_base: u64, kernel_size: usize, stack_pointer: u64) {

    log_state!("Starting LeOS kernel");
    log_state!("kernel base: 0x{:08x}", kernel_base);
    log_state!("kernel size: {:x}", kernel_size);
    timer::init();
    memory::init(kernel_base, kernel_size, stack_pointer);
    board::init();
    scheduler::init();
    irq::enable();

    test_heap();
    memory::log_heap();

    // test memory_map
    let p = memory::get_page();
    log_debug!("allocated 4k page: {:#018x}", p);
    memory::free_page(p);
    log_debug!("cleared page: {:#018x}", p);
    let p = memory::get_page();
    log_debug!("same 4k page must be allocated: {:#018x}", p);
    memory::free_page(p);
    log_debug!("cleared page: {:#018x}", p);

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
