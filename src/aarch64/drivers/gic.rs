//
// gic.rs - generic interrupt controller driver
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

mod gicd;
mod gicc;

use crate::drivers;

pub const ICFGR_EDGE: u32 = 2;


pub struct GIC {
    gicd: gicd::GICD,
    gicc: gicc::GICC,
}

impl GIC {
    pub const fn new(gicd_base: usize, gicc_base: usize) -> Self {
        Self {
            gicd: gicd::GICD::new(gicd_base),
            gicc: gicc::GICC::new(gicc_base),
        }
    }
}

impl drivers::Driver for GIC {

    fn name(&self) -> &str {
        "ARM Generic Interrupt Controller [GIC]"
    }

    fn enable(&mut self) -> Result<(), ()> {
        unsafe {
            self.gicd.enable();
            self.gicc.enable();
        }
        Ok(())
    }

    fn disable(&mut self) {
        unsafe {
            self.gicd.disable();
            self.gicc.disable();
        }
    }
}

impl drivers::InterruptController for GIC {

    fn set_interrupt(&self, intnum: u32) {
        unsafe {
            self.gicd.set_interrupt_config(intnum, gicd::ICFGR_EDGE);
            self.gicd.set_interrupt(intnum);
        }
    }

    fn unset_interrupt(&self, intnum: u32) {
        unsafe {
            self.gicd.unset_interrupt(intnum);
        }
    }

    fn clear_interrupt(&self, intnum: u32) {
        unsafe {
            self.gicd.clear_interrupt(intnum);
        }
    }

    fn is_interrupt(&self, intnum: u32) -> bool {
        unsafe {
            self.gicd.is_interrupt(intnum)
        }
    }

    fn set_interrupt_core(&self, intnum: u32, core: u32) {
        unsafe {
            self.gicd.set_interrupt_core(intnum, core);
        }
    }

    fn set_interrupt_priority(&self, intnum: u32, priority: u32){
        unsafe {
            self.gicd.set_interrupt_priority(intnum, priority);
        }
    }

    fn set_interrupt_config(&self, intnum: u32, config: u32) {
        unsafe {
            self.gicd.set_interrupt_config(intnum, config);
        }
    }
}

unsafe impl Sync for GIC {}
