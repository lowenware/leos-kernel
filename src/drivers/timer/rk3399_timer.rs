//
// rk3399_timer.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use crate::{irq, scheduler, drivers};

pub struct RK3399Timer {
}

impl RK3399Timer {
    pub const fn new() -> Self {
        RK3399Timer {}
    }
}

impl drivers::Driver for RK3399Timer {
    fn name(&self) -> &str {
        "RK3399 Timer"
    }

    fn enable(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn disable(&mut self) {
    }
}

impl drivers::SystemTimer for RK3399Timer {
    fn get_value(&self) -> u64 {
        0
    }

    fn get_frequency(&self) -> u64 {
        1
    }
}

impl irq::IRQHandler for RK3399Timer {
    fn on_interrupt(&mut self, _intnum: u32) {
        scheduler::run();
    }
}

