//
// timer.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use crate::aarch64::registers::{CNTPCT_EL0, CNTP_CTL_EL0, CNTFRQ_EL0, CNTP_TVAL_EL0};
use crate::{irq, scheduler, drivers};

pub struct Timer {
}

impl Timer {
    pub const fn new() -> Self {
        Timer {}
    }
}

impl drivers::Driver for Timer {
    fn name(&self) -> &str {
        "ARM Generic Timer"
    }

    fn enable(&mut self) -> Result<(), ()> {
        CNTP_TVAL_EL0.write(CNTFRQ_EL0.read());
        CNTP_CTL_EL0.write(CNTP_CTL_EL0::ENABLE);
        Ok(())
    }

    fn disable(&mut self) {
        CNTP_CTL_EL0.clear(CNTP_CTL_EL0::ENABLE);
    }
}

impl drivers::SystemTimer for Timer {
    fn get_value(&self) -> u64 {
        CNTPCT_EL0.read()
    }

    fn get_frequency(&self) -> u64 {
        CNTFRQ_EL0.read()
    }
}

impl irq::IRQHandler for Timer {
    fn on_interrupt(&mut self, _intnum: u32) {
        CNTP_TVAL_EL0.write(CNTFRQ_EL0.read());
        scheduler::run();
    }
}

