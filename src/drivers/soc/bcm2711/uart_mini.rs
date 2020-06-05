//
// uart_mini.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use crate::periphery::mmio;

struct UartMini {
    IO: mmio::Register<u8>,         // In / Out data
    IER: mmio::Register<u8>,        // Interrupt Enable
    IIR: mmio::Register<u8>,        // Interrupt Identify
    LCR: mmio::Register<u8>,        // Line Control
    MCR: mmio::Register<u8>,        // Modem Control
    LSR: mmio::Register<u8>,        // Line Status
    MSR: mmio::Register<u8>,        // Modem Status
    SCRATCH: mmio::Register<u8>,    // Scratch
    CNTL: mmio::Register<u8>,       // Extra Control
    STAT: mmio::Register<u32>,      // Extra Status
    BAUD: mmio::Register<u16>,      // Baud Rate
}

impl UartMini {
    pub const fn new(base: usize) -> Self {
        UartMini {
            IO: mmio::Register::new(base + 0x40),
            IER: mmio::Register::new(base + 0x44),
            IIR: mmio::Register::new(base + 0x48),
            LCR: mmio::Register::new(base + 0x4C),
            MCR: mmio::Register::new(base + 0x50),
            LSR: mmio::Register::new(base + 0x54),
            MSR: mmio::Register::new(base + 0x58),
            SCRATCH: mmio::Register::new(base + 0x5C),
            CNTL: mmio::Register::new(base + 0x60),
            STAT: mmio::Register::new(base + 0x64),
            BAUD: mmio::Register::new(base + 0x68),
        }
    }
}
