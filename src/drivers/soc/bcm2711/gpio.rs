//
// gpio.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use crate::periphery::mmio;
use crate::arch::cpu;

pub const ALT_FUNC0: u32 = 0b100;

pub struct GPIO {
    GPFSEL1: mmio::Register<u32>,
    GPPUD: mmio::Register<u32>,
    GPPUDCLK0: mmio::Register<u32>,
}

impl GPIO {
    pub const fn new(base: usize) -> Self {
        Self {
            GPFSEL1: mmio::Register::new(base + 0x04),
            GPPUD: mmio::Register::new(base + 0x94),
            GPPUDCLK0: mmio::Register::new(base + 0x98),
        }
    }

    pub fn init(&mut self) {
        // TODO: implement common pinmux, init peripherals for board
        let value = self.GPFSEL1.read() & !(0b111111 << 12);
        self.GPFSEL1.write(value | (ALT_FUNC0 << 12) | (ALT_FUNC0 << 15));
        self.GPPUD.write(0);
        cpu::idle(150);
        self.GPPUDCLK0.write(self.GPPUDCLK0.read() | (0b11 << 14));
        cpu::idle(150);
        self.GPPUDCLK0.write(0);
    }
}
