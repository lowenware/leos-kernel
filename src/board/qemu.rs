//
// qemu.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//
//

use crate::memory;
use crate::arch;

// pub const PERIPHERALS_REAL: u64 = 0x00000000;
pub const PERIPHERALS_BASE: u64 = 0xffffffe0_00000000;
pub const GIC_BASE: u64 = PERIPHERALS_BASE + 0x08000000;
pub const UART_BASE: u64 = PERIPHERALS_BASE + 0x09000000;

pub const MEMORY_BASE: u64 = 0x4000_0000;
pub const MEMORY_SIZE: usize = 1024 * 1024 * 1024;

pub fn init() {
    let mut addr = 0x40000000;
    // lock 128 pages from 0x40000000..40080000
    for _i in 0..128 {
        memory::lock(addr);
        addr += arch::PAGE_SIZE as u64;
    }
}
