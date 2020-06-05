//
// uart.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//
//

use crate::periphery::mmio;

pub const ICR_CLEAR: u32 = 0;
pub const LCRH_8BIT: u32 = 0b11 << 5;
pub const LCRH_FEN: u32 = 1 << 4;

pub const FR_TXFF: u32 = 1 << 5;

pub const CR_UARTEN: u32 = 1;
pub const CR_TXE: u32 = 1 << 8;
pub const CR_RXE: u32 = 1 << 9;

pub struct Uart {
    DR: mmio::Register<u32>,        // Data Register
    RSRECR: mmio::Register<u32>,
    FR: mmio::Register<u32>,        // Flag Register
    ILPR: mmio::Register<u32>,      // Not in use
    IBRD: mmio::Register<u32>,      // Ineger Baud Rate divisor
    FBRD: mmio::Register<u32>,      // Fractional Baud rate divisor
    LCRH: mmio::Register<u32>,      // Line Control register
    CR: mmio::Register<u32>,        // Control register
    IFLS: mmio::Register<u32>,      // Interrupt FIFO Level Select Register
    IMSC: mmio::Register<u32>,      // Interrupt Mask Set Clear Register
    RIS: mmio::Register<u32>,       // Raw Interrupt Status register
    MIS: mmio::Register<u32>,       // Masked Interrupt Status Register
    ICR: mmio::Register<u32>,       // Interrupt clear register
    DMACR: mmio::Register<u32>,     // DMA Control Register
    ITCR: mmio::Register<u32>,      // Test Control Register
    ITIP: mmio::Register<u32>,      // Integration test input register
    ITOP: mmio::Register<u32>,      // Integration test output register
    TDR: mmio::Register<u32>,       // Test Data register
}

impl Uart {
    pub const fn new(base: usize) -> Self {
        Uart {
            DR: mmio::Register::new(base),
            RSRECR: mmio::Register::new(base + 0x04),
            FR: mmio::Register::new(base + 0x18),
            ILPR: mmio::Register::new(base + 0x20),
            IBRD: mmio::Register::new(base + 0x24),
            FBRD: mmio::Register::new(base + 0x28),
            LCRH: mmio::Register::new(base + 0x2C),
            CR: mmio::Register::new(base + 0x30),
            IFLS: mmio::Register::new(base + 0x34),
            IMSC: mmio::Register::new(base + 0x38),
            RIS: mmio::Register::new(base + 0x3C),
            MIS: mmio::Register::new(base + 0x40),
            ICR: mmio::Register::new(base + 0x44),
            DMACR: mmio::Register::new(base + 0x48),
            ITCR: mmio::Register::new(base + 0x80),
            ITIP: mmio::Register::new(base + 0x84),
            ITOP: mmio::Register::new(base + 0x88),
            TDR: mmio::Register::new(base + 0x8C),
        }
    }

    #[no_mangle]
    #[inline(always)]
    pub fn init(&mut self) {
        self.CR.write(0);

        // self.ICR.write(ICR_CLEAR);
        // set baudrate 230400
        self.IBRD.write(13);
        self.FBRD.write(2);
        // Enable 8n1 and FIFO
        self.LCRH.write(LCRH_8BIT | LCRH_FEN);
        self.CR.write(CR_UARTEN | CR_RXE | CR_TXE);
    }

    pub fn write(&mut self, c: char) {

        /*
        while self.FR.read() & FR_TXFF != 0 {
            unsafe {
                llvm_asm!("nop");
            }
        }*/

        self.DR.write(c as u32);
    }
}
