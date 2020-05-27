//
// memory.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use super::mmu;
use super::registers::{TTBR0_EL1, TTBR1_EL1};

pub use mmu::PAGE_SIZE;

// TODO: verify flags
pub const KERNEL_DATA: usize =
    mmu::UXN |
    mmu::PXN |
    mmu::ACCESS_FLAG |
    mmu::INNER_SHAREABLE |
    mmu::DATA;

// for identity paging
static mut TTBR0_IDENTITY: u64 = 0;

pub fn init() {
    unsafe {
        TTBR0_IDENTITY = TTBR0_EL1.read();
    }
}

pub fn identity_on() {
    unsafe {
        // enable identity
        TTBR0_EL1.write(TTBR0_IDENTITY);
    }
}

pub fn identity() -> mmu::AddressSpace {
    identity_on();
    mmu::AddressSpace::new(TTBR0_EL1.read() as usize)
}

pub fn kernel() -> mmu::AddressSpace {
    identity_on();
    mmu::AddressSpace::new(TTBR1_EL1.read() as usize)
}

// pub fn user() -> mmu::AddressSpace {
//     identity_on();
//     mmu::AddressSpace::new(TTBR0_EL0.read() as usize)
// }

// pub fn task(ttb_base: usize) -> mmu::AddressSpace {
//     identity_on();
//     mmu::AddressSpace::new(ttb_base)
// }

