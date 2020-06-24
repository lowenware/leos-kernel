//
// gicc.rs - Generic Interrupt Controller CPU
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use core::ptr;


// const BASE: usize = GIC_BASE + 0x10000;
const CTLR: usize = 0x0000;
const PMR: usize = 0x0004;
const BPR: usize = 0x0008;

const CTLR_ENABLE: u32 = 1;
const CTLR_DISABLE: u32 = 0;

const PMR_PRIO_LOW: u32 = 0xff;
// const PMR_PRIO_HIGH: u32 = 0x00;

const BPR_NO_GROUP: u32 = 0x00;

pub struct GICC {
    base: *mut u32,
}

impl GICC {
    #[inline(always)]
    unsafe fn write(&self, register: usize, value: u32) {
        ptr::write_volatile(self.base.add(register >> 2), value);
    }

    pub const fn new(gicc_base: usize) -> Self {
        Self {
            base: gicc_base as *mut u32
        }
    }

    pub unsafe fn enable(&self) {
        self.write(CTLR, CTLR_ENABLE);
        self.write(PMR, PMR_PRIO_LOW);
        self.write(BPR, BPR_NO_GROUP);
    }

    pub unsafe fn disable(&self) {
        self.write(CTLR, CTLR_DISABLE);
    }
}
