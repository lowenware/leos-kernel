//
// aux.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use crate::periphery::mmio;

// pub const UART_IRQ: u8 = 1;
// pub const SPI1_IRQ: u8 = 2;
// pub const SPI2_IRQ: u8 = 4;

pub const UART: u8 = 1;
// pub const SPI1: u8 = 2;
// pub const SPI2: u8 = 4;

struct Aux {
    IRQ: mmio::Register<u8>,
    ENABLES: mmio::Register<u8>,
}

impl Aux {
    pub const fn new(base: usize) -> Self {
        Aux {
            IRQ: mmio::Register::new(base),
            ENABLES: mmio::Register::new(base + 0x04),
        }
    }

    pub fn irq(&mut self, func: u8, enabled: bool) {
        let mut value = self.IRQ.read();
        if enabled {
            value |= func;
        } else {
            value &= !func;
        }
        self.IRQ.write(value);
    }

    pub fn set(&mut self, func: u8, enabled: bool) {
        let mut value = self.ENABLES.read();
        if enabled {
            value |= func;
        } else {
            value &= !func;
        }
        self.ENABLES.write(value);
    }
}
