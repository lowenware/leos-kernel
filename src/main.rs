#![no_std]
#![no_main]
#![feature(alloc_layout_extra)]
#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(core_intrinsics)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(naked_functions)]
#![feature(or_patterns)]
#![feature(panic_info_message)]

extern crate alloc;

/// Console output module
#[macro_use]
mod log;
/// Architecture abstraction
pub mod aarch64;
pub use aarch64 as arch;
/// Platform abstraction
pub mod platform;
/// Device, Networking, FS and other Drivers
pub mod drivers;
/// Interrupts management
pub mod irq;
/// Linked lists
pub mod list;
/// Memory management
mod memory;
/// Kernel Panic handler
mod panic;
/// Tasks Scheduler
mod scheduler;

extern "C" fn task(arg: usize) {
    loop {
        log_state!("Task: {}", arg);

        unsafe {
            llvm_asm!("wfi");
        }
    }
}

#[no_mangle]
pub extern "C" fn kernel_main(kernel_base: usize, kernel_size: usize) -> ! {

    // TODO
    // [x] Rewrite boot procedure with Rust
    // [x] Extend memory::init with MMU initialization
    // [x] Add boards initialization
    // [ ] Add device manager

    /*
    let mut gpio = soc::gpio::GPIO::new(soc::GPIO_BASE);
    gpio.init();

    let mut uart = soc::uart::Uart::new(soc::UART0_BASE);
    uart.init();
    uart.write('L');
    uart.write('e');
    uart.write('O');
    uart.write('S');
    uart.write('\n');
    loop {}
    */

    memory::init(kernel_base, kernel_size);
    platform::init();
    log_state!("Starting LeOS kernel");

    scheduler::init();

    irq::enable_all();

    /*
     loop {
        log_debug!("CNTP {:016x}", aarch64::registers::CNTP_CTL_EL0.read());
        }
    unsafe {
        let addr = (board::PCIE_MMIO_BASE + 0x4) as *mut u32;
        log_state!("read {:p} {:08x} -> ", addr, *addr)
    }

    for i in (0..0x4000).step_by(0x200) {
        virtio::MMIO::new(board::MMIO_BASE + i).log();
    }
    */

    for i in 2..5 {
        log_state!("Starting task {}", i);
        // allocate real memory for the stack
        if let Ok(mut stack) = memory::map(arch::memory::PAGE_SIZE) {
            // log_state!(" - real stack @ 0x{:016x}", stack);
            // map stack memory page to a virtual memory
            stack = arch::memory::kernel().map_L3(stack, arch::memory::KERNEL_DATA);
            // get bottom of the stack
            stack += arch::memory::PAGE_SIZE;
            // log_state!(" - virtual stack @ 0x{:016x}", stack);
            // create a new task
            scheduler::add(i, task, memory::KERNEL_BASE | stack, 0);
            // log_state!(" - scheduled");
        }
    }

    loop {
        unsafe {
            log_state!("Task: 1");
            llvm_asm!("wfi");
        }
    }
}

