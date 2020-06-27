//
// memory.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use super::drivers::mmu;
use super::registers::{TTBR0_EL1, TTBR1_EL1};

pub use mmu::{BLOCK_L1_SIZE, BLOCK_L2_SIZE, PAGE_SIZE, PERIPHERALS_FLAGS};

// TODO: verify flags
pub const KERNEL_DATA: usize =
    mmu::UXN |
    mmu::PXN |
    mmu::ACCESS_FLAG |
    mmu::INNER_SHAREABLE |
    mmu::MEMORY_ATTR;



pub fn identity() -> mmu::AddressSpace {
    mmu::identity();
    mmu::AddressSpace::new(TTBR0_EL1.read() as usize)
}

pub fn kernel() -> mmu::AddressSpace {
    mmu::identity();
    mmu::AddressSpace::new(TTBR1_EL1.read() as usize)
}

