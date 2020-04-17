//
// aarch64.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

mod exceptions;
mod gic;
mod registers;

pub mod irq;
pub mod timer;
pub mod mmu;

pub const PAGE_SHIFT: u8 = 12; // 4Kb Page Size
pub const PAGE_SIZE: u32 = 1 << PAGE_SHIFT;

pub struct TaskContext {
    /// generic purpose registers x19-x30
    pub regs: [u64; 12],
    /// stack pointer
    pub sp: u64,
}
