//
// mmu.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

static mut TTB_L1_BASE: u64 = 0;
static mut TTB_L2_BASE: u64 = 0;
static mut TTB_L3_BASE: u64 = 0;

use core::ptr;
use super::{PAGE_SIZE, PAGE_SHIFT};

const PAGE_4K_MASK: u64 = 0x01FF;
const PAGE_4K_L3_MASK: u64 = PAGE_4K_MASK << PAGE_SHIFT;

pub fn init(_kernel_base: u64, ttbr1_base: u64) {
    unsafe {
        TTB_L1_BASE = ttbr1_base;
        TTB_L2_BASE = TTB_L1_BASE + PAGE_SIZE as u64;
        TTB_L3_BASE = TTB_L2_BASE + PAGE_SIZE as u64;
        log_debug!("TTB_L1_BASE: 0x{:016x}", TTB_L1_BASE);
        log_debug!("TTB_L2_BASE: 0x{:016x}", TTB_L2_BASE);
        log_debug!("TTB_L3_BASE: 0x{:016x}", TTB_L3_BASE);
    }
}

pub fn translate(p_addr: u64, v_addr: u64, count: usize, flags: u64) {
    unsafe {
        let mut entry_addr: *mut u64 = (TTB_L3_BASE | ((v_addr & PAGE_4K_L3_MASK) >> PAGE_SHIFT)) as *mut u64;
        let mut entry_value: u64 = p_addr;

        for _i in 0..count {
            ptr::write_volatile(entry_addr, entry_value | flags);
            entry_addr = entry_addr.add(1);
            entry_value += PAGE_SIZE as u64;
        }
    }
}
