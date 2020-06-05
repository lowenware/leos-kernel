//
// gicd.rs - GIC Distributor
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//
#![allow(dead_code)]

use core::ptr;


// Distributor Registers
const CTLR: usize = 0;
const ISENABLER: usize = 0x0100;
const ICENABLER: usize = 0x0180;
const ICPENDR: usize = 0x0280;
const ITARGETSR: usize = 0x0800;
const IPRIORITYR: usize = 0x0400;
const ICFGR: usize = 0x0c00;

// Registers VALUES
const CTLR_ENABLE: u32 = 1;
const CTLR_DISABLE: u32 = 0;

const ICENABLER_SHIFT: u32 = 5; // 32 bits -> x / 32 == x >> 5
const ICENABLER_MASK: u32 = 0b11111;

const ISENABLER_SHIFT: u32 = 5; // 32 bits -> x / 32 == x >> 5
const ISENABLER_MASK: u32 = 0b11111;

const ICPENDR_SHIFT: u32 = 5; // 32 bits -> x / 32 == x >> 5
const ICPENDR_MASK: u32 = 0b11111;

// 8 bits per 4 interrupts in register
const ITARGETSR_INTERRUPT_MASK: u32 = 0b11; // interrupt reminder mask
const ITARGETSR_INTERRUPT_SHIFT: u32 = 2; // interrupt divider shift (division by 4)
const ITARGETSR_VALUE_SHIFT: u32 = 3; // core number shift (multiplication by 8)
const ITARGETSR_VALUE_MASK: u32 = 0b111111111; // core number 1 byte mask

const IPRIORITYR_INTERRUPT_MASK: u32 = 0b11; // interrupt reminder mask
const IPRIORITYR_INTERRUPT_SHIFT: u32 = 2; // interrupt divider shift (division by 4)
const IPRIORITYR_VALUE_SHIFT: u32 = 3; // priority number shift (multiplication by 8)
const IPRIORITYR_VALUE_MASK: u32 = 0b111111111; // priority number 1 byte mask

const ICFGR_INTERRUPT_MASK: u32 = 0b1111; // interrupt reminder mask
const ICFGR_INTERRUPT_SHIFT: u32 = 4; // interrupt divider shift (division by 4)
const ICFGR_VALUE_SHIFT: u32 = 1; // config value shift (multiplication by 8)
const ICFGR_VALUE_MASK: u32 = 0b11; // config value 2 bits mask

const IPRIORITY_SIZE: u32 = 4;
const IPRIORITY_BITS: u32 = 8;
const ICFGR_SIZE: u32 = 16;
const ICFGR_BITS: u32 = 2;

pub const ICFGR_EDGE: u32 = 2;

pub struct GICD {
    base: *mut u32,
}

impl GICD {
    pub const fn new(gicd_base: usize) -> Self {
        Self {
            base: gicd_base as *mut u32
        }
    }

    #[inline(always)]
    unsafe fn write(&self, register: usize, offset: u32, value: u32) {
        ptr::write_volatile(self.base.add((register >> 2) + offset as usize), value);
    }

    #[inline(always)]
    unsafe fn read(&self, register: usize, offset: u32) -> u32 {
        ptr::read_volatile(self.base.add((register >> 2) + offset as usize))
    }

    /// enable GIC Distributor
    pub unsafe fn enable(&self) {
        self.write(CTLR, 0, CTLR_ENABLE);
    }

    /// Disable GIC Distributor
    pub unsafe fn disable(&self) {
        self.write(CTLR, 0, CTLR_DISABLE);
    }

    /// Set interrupt enabled
    pub unsafe fn set_interrupt(&self, intnum: u32) {
        self.write(ISENABLER
            , intnum >> ISENABLER_SHIFT
            , 1 << (intnum & ISENABLER_MASK));
    }

    /// Unset enabled interrupt
    pub unsafe fn unset_interrupt(&self, intnum: u32) {
        self.write(ISENABLER
            , intnum >> ICENABLER_SHIFT
            , 1 << (intnum & ICENABLER_MASK));
    }

    /// Check if interrupt is pending
    pub unsafe fn is_interrupt(&self, intnum: u32) -> bool {

        let value = self.read(ICPENDR, intnum >> ICPENDR_SHIFT);
        value & (1 << (intnum & ICPENDR_MASK)) != 0
    }

    /// Clear pending interrupt
    pub unsafe fn clear_interrupt(&self, intnum: u32) {
        self.write(ICPENDR
            , intnum >> ICPENDR_SHIFT
            , 1 << (intnum & ICPENDR_MASK));
    }

    pub unsafe fn set_interrupt_core(&self, intnum: u32, core: u32) {
        let shift = (intnum & ITARGETSR_INTERRUPT_MASK) << ITARGETSR_VALUE_SHIFT;
        let offset = intnum >> ITARGETSR_INTERRUPT_SHIFT;
        let value = self.read(ITARGETSR, offset) & !(ITARGETSR_VALUE_MASK << shift);
        self.write(ITARGETSR, offset, value | core << shift);
    }

    pub unsafe fn set_interrupt_priority(&self, intnum: u32, priority: u32) {
        let shift = (intnum & IPRIORITYR_INTERRUPT_MASK) << IPRIORITYR_VALUE_SHIFT;
        let offset = intnum >> IPRIORITYR_INTERRUPT_SHIFT;
        let value = self.read(IPRIORITYR, offset) & !(IPRIORITYR_VALUE_MASK << shift);
        self.write(IPRIORITYR, offset, value | priority << shift);
    }

    pub unsafe fn set_interrupt_config(&self, intnum: u32, config: u32) {
        let shift = (intnum & ICFGR_INTERRUPT_MASK) << ICFGR_VALUE_SHIFT;
        let offset = intnum >> ICFGR_INTERRUPT_SHIFT;
        let value = self.read(ICFGR, offset) & !(ICFGR_VALUE_MASK << shift);
        self.write(ICFGR, offset, value | config << shift);
    }
}
