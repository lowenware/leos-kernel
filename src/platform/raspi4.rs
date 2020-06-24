//
// qemu.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//
//

use crate::memory::{PERIPHERALS_BASE};

pub const PERIPHERALS_REAL: usize = 0x0000_0000;
pub const PERIPHERALS_SIZE: usize = 0x4000_0000;
pub const MEMORY_BASE: usize = 0x0000_0000;
pub const MEMORY_SIZE: usize = 0x3b40_0000;

pub const GIC_BASE: usize = PERIPHERALS_BASE + 0x0800_0000;
pub const UART_BASE: usize = PERIPHERALS_BASE + 0x0900_0000;
pub const MMIO_BASE: usize = PERIPHERALS_BASE + 0x0a00_0000;
pub const PCIE_MMIO_BASE: usize = PERIPHERALS_BASE + 0x3eff_0000;

pub fn heap(kernel_base: usize, kernel_size: usize) -> usize {
    kernel_base + kernel_size
}

pub fn init() {

}
