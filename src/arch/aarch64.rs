//
// aarch64.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

mod exceptions;
mod gic;
mod registers;

mod asm {
    global_asm!(include_str!("aarch64/start.s"));
    global_asm!(include_str!("aarch64/context.s"));

    #[no_mangle]
    extern "C" {
        pub fn aarch64_switch_context(current_addr: usize, next_addr: usize);
        pub fn aarch64_start_task();
    }
}

pub mod irq;
pub mod timer;
pub mod memory;
pub mod mmu; // TODO: remove, use only inside memory
pub use asm::aarch64_switch_context as switch_context;


#[repr(C)]
pub struct TaskContext {
    /// generic purpose registers x19-x29
    pub regs: [u64; 11],
    /// link register
    pub lr: u64,
    /// stack pointer
    pub sp: u64,
    /// Address space
    pub ttbr0: mmu::AddressSpace,
}

impl TaskContext {
    pub const fn new(entry: usize, arg: usize, stack: usize, ttbr0_base: usize) -> Self {
        unsafe {
            Self {
                regs: [arg as u64, entry as u64, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                lr: (asm::aarch64_start_task as *const()) as u64,
                sp: stack as u64,
                ttbr0: mmu::AddressSpace::new(ttbr0_base)
            }
        }
    }
}
