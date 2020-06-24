//
// uart_pl011.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use crate::{drivers, irq};
use crate::drivers::mmio;

#[allow(non_snake_case)]
mod CR {
    pub const UARTEN: u16 = 1;
    pub const TXE: u16 = 1 << 8;
    pub const RXE: u16 = 1 << 9;
}

#[allow(non_snake_case)]
mod LCRH {
    pub const WLEN_MASK: u16 = 0b11 << 5;
    pub const WLEN_8BITS: u16 = 0b11 << 5;
    pub const WLEN_7BITS: u16 = 0b10 << 5;
    pub const WLEN_6BITS: u16 = 0b01 << 5;
    pub const WLEN_5BITS: u16 = 0b00 << 5;

    pub const PEN: u16 = 1 << 1;
    pub const EPS: u16 = 1 << 2;

//    pub const FEN: u16 = 1 << 4;
}

#[allow(non_snake_case)]
mod IMSC {
    /// RX Interrupt Mask
    pub const RXIM: u16 = 1 << 4;
}

#[allow(non_snake_case)]
mod ICR {
    /// RX Interrupt Clear bit
    pub const RXIC: u16 = 1 << 4;
}

// #[allow(non_snake_case)]
// mod FR {
//    pub const TXFF: u16 = 1 << 5;
// }

#[allow(non_snake_case)]
pub struct UARTPL011 {
    DR: mmio::Register::<u16>,
//    RSRECR: mmio::Register::<u8>,
//    FR: mmio::Register::<u16>,
//    ILPR: mmio::Register::<u8>,
    IBRD: mmio::Register::<u16>,
    FBRD: mmio::Register::<u16>,
    LCRH: mmio::Register::<u16>,
    CR: mmio::Register::<u16>,
//    IFLS: mmio::Register::<u16>,
    IMSC: mmio::Register::<u16>,
//    RIS: mmio::Register::<u16>,
//    MIS: mmio::Register::<u16>,
    ICR: mmio::Register::<u16>,
//    DMACR: mmio::Register::<u16>,
    clock_frequency: u32,
    baudrate: drivers::serial::Baudrate,
}

impl UARTPL011 {
    pub const fn new(base: usize, clock_frequency: u32) -> Self {
        Self {
            DR: mmio::Register::new(base),
//            RSRECR: mmio::Register::new(base + 0x04),
//            FR: mmio::Register::new(base + 0x18),
//            ILPR: mmio::Register::new(base + 0x20),
            IBRD: mmio::Register::new(base + 0x24),
            FBRD: mmio::Register::new(base + 0x28),
            LCRH: mmio::Register::new(base + 0x2C),
            CR: mmio::Register::new(base + 0x30),
//            IFLS: mmio::Register::new(base + 0x34),
            IMSC: mmio::Register::new(base + 0x38),
//            RIS: mmio::Register::new(base + 0x3C),
//            MIS: mmio::Register::new(base + 0x40),
            ICR: mmio::Register::new(base + 0x44),
//            DMACR: mmio::Register::new(base + 0x48),
            clock_frequency,
            baudrate: 115200,
        }
    }
}

impl drivers::Driver for UARTPL011 {

    fn enable(&mut self) -> Result<(), ()> {
        use crate::drivers::SerialDevice;
        self.CR.write(0);
        self.set_parity(drivers::serial::Parity::None);
        self.set_data_bits(drivers::serial::DataBits::Eight);
        self.set_baudrate(self.baudrate);
        self.CR.write(CR::UARTEN | CR::RXE | CR::TXE);
        // enable RX Interrupt
        self.IMSC.write(IMSC::RXIM);
        Ok(())
    }

    fn disable(&mut self) {
        self.CR.write(0);
    }

    fn name(&self) -> &str {
        "ARM PL011 UART"
    }
}

impl drivers::SerialDevice for UARTPL011 {

    fn write(&mut self, chr: char) {
        self.DR.write(chr as u16);
    }

    fn read(&mut self) -> char {
        (self.DR.read() & 0xFF) as u8 as char
    }

    fn set_baudrate(&mut self, baudrate: drivers::serial::Baudrate) {
        let bauddiv = self.clock_frequency * 100 / 16 / baudrate;
        let ibrd = bauddiv / 100;
        let mut fbrd = bauddiv - (ibrd * 100);
        if fbrd > 31 {
            fbrd = 31;
        }
        self.IBRD.write((ibrd & 0xffff) as u16);
        self.FBRD.write((fbrd & 0x1f) as u16);
        self.baudrate = baudrate;
    }

    fn set_parity(&mut self, parity: drivers::serial::Parity) {
        let reg = self.LCRH.read();
        match parity {
            drivers::serial::Parity::None => self.LCRH.write(reg & !(LCRH::PEN | LCRH::EPS)),
            drivers::serial::Parity::Even => self.LCRH.write(reg | LCRH::PEN | LCRH::EPS),
            drivers::serial::Parity::Odd => self.LCRH.write(reg & (!LCRH::EPS) | LCRH::PEN),
        }
    }

    fn set_data_bits(&mut self, data_bits: drivers::serial::DataBits) {
        let reg = self.LCRH.read() & !(LCRH::WLEN_MASK);
        match data_bits {
            drivers::serial::DataBits::Eight => self.LCRH.write(reg | LCRH::WLEN_8BITS),
            drivers::serial::DataBits::Seven => self.LCRH.write(reg | LCRH::WLEN_7BITS),
            drivers::serial::DataBits::Six => self.LCRH.write(reg | LCRH::WLEN_6BITS),
            drivers::serial::DataBits::Five => self.LCRH.write(reg | LCRH::WLEN_5BITS),
        }
    }

    fn baudrate(&self) -> drivers::serial::Baudrate {
        self.baudrate
    }

    fn parity(&self) -> drivers::serial::Parity {
        let state = self.LCRH.read() & (LCRH::PEN | LCRH::EPS);
        const PARITY_EVEN: u16 = LCRH::PEN | LCRH::EPS;
        match state {
            PARITY_EVEN => drivers::serial::Parity::Even,
            LCRH::PEN => drivers::serial::Parity::Odd,
            _ => drivers::serial::Parity::None,
        }
    }

    fn data_bits(&self) -> drivers::serial::DataBits {
        let state = self.LCRH.read() & LCRH::WLEN_8BITS;
        match state {
            LCRH::WLEN_8BITS => drivers::serial::DataBits::Eight,
            LCRH::WLEN_7BITS => drivers::serial::DataBits::Seven,
            LCRH::WLEN_6BITS => drivers::serial::DataBits::Six,
            _ => drivers::serial::DataBits::Five,
        }
    }
}

impl irq::IRQHandler for UARTPL011 {
    fn on_interrupt(&mut self, _intnum: u32) {
        use crate::drivers::SerialDevice;
        let chr = self.read();
        self.write(chr);
        self.ICR.write(ICR::RXIC);
    }
}

unsafe impl Sync for UARTPL011 {
}
