#![no_std]
#![no_main]
#![feature(alloc_layout_extra)]
#![feature(alloc_error_handler)]
#![feature(llvm_asm)]
#![feature(const_fn)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(const_raw_ptr_to_usize_cast)]

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

extern "C" fn task(arg: usize) {
    loop {
        log_state!("Task: {}", arg);

        unsafe {
            llvm_asm!("wfi");
        }
    }
}

#[no_mangle]
pub extern "C" fn kernel_main(kernel_base: usize, kernel_size: usize) {

    memory::init(kernel_base, kernel_size);
    timer::init();
    scheduler::init();

    irq::enable();

    log_state!("Starting LeOS kernel");
    for i in 2..5 {
        log_state!("Starting task {}", i);
        // allocate real memory for the stack
        if let Ok(mut stack) = memory::map(arch::memory::PAGE_SIZE) {
            log_state!(" - real stack @ 0x{:016x}", stack);
            // map stack memory page to a virtual memory
            stack = arch::memory::kernel().map_L3(stack, arch::memory::KERNEL_DATA);
            // get bottom of the stack
            stack += arch::memory::PAGE_SIZE;
            log_state!(" - virtual stack @ 0x{:016x}", stack);
            // create a new task
            scheduler::add(i, task, memory::KERNEL_BASE | stack, 0);
            log_state!(" - scheduled");
        }
    }

    loop {
        log_state!("Task: 1");
        unsafe {
            llvm_asm!("wfi");
        }
    }
}

// ldp    x8, x9, [x0]
