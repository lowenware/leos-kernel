//
// uart_rk3399.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use crate::{drivers, irq};
use crate::drivers::mmio;

mod LCR {
    const DIV_LAT_ACCESS: u32 = 1 << 7;
    const BREAK_CTRL: u32 = 1 << 6;
    const EVEN_PARITY_SEL: u32 = 1 << 4;
    const PARITY_EN: u32 = 1 << 3;
    const STOP_BITS_NUM_1BIT: u32 = 0;
    const STOP_BITS_NUM_2BITS: u32 = 1 << 2;
    const DATA_LENGTH_SEL_5BITS = 0b00;
    const DATA_LENGTH_SEL_6BITS = 0b01;
    const DATA_LENGTH_SEL_7BITS = 0b10;
    const DATA_LENGTH_SEL_8BITS = 0b11;
}

mod LSR {
    const RECEIVER_FIFO_ERROR: u32 = 1 << 7;
    const TRANS_EMPTY: u32 = 1 << 6;
    const TRANS_HOLD_REG_EMPTY: u32 = 1 << 5;
    const BREAK_INT: u32 = 1 << 4;
    const FRAMING_ERROR: u32 = 1 << 3;
    const PARITY_ERROR: u32 = 1 << 2;
    const OVERRUN_ERROR: u32 = 1 << 1;
    const DATA_READY: u32 = 1;
}

#[allow(non_snake_case)]
pub struct RK3399Uart {
    // TODO:try anonymous unions
    union {
        rbr: mmio::Register<u32>,
        thr: mmio::Register<u32>,
        dll: mmio::Register<u32>,
    },
    rbr_thr_dll: mmio::Register<u32>,
    dlh_ier: mmio::Register<u32>,
    iir_fcr: mmio::Register<u32>,
    lcr: mmio::Register<u32>,
    mcr: mmio::Register<u32>,
    lsr: mmio::Register<u32>,
    msr: mmio::Register<u32>,
    scr: mmio::Register<u32>,
    srbr: mmio::Register<u32>,
    sthr: mmio::Register<u32>,
    far: mmio::Register<u32>,
    tfr: mmio::Register<u32>,
    rfw: mmio::Register<u32>,
    usr: mmio::Register<u32>,
    tfl: mmio::Register<u32>,
    rfl: mmio::Register<u32>,
    srr: mmio::Register<u32>,
    srts: mmio::Register<u32>,
    sbcr: mmio::Register<u32>,
    sdmam: mmio::Register<u32>,
    sfe: mmio::Register<u32>,
    srt: mmio::Register<u32>,
    stet: mmio::Register<u32>,
    htx: mmio::Register<u32>,
    dmasa: mmio::Register<u32>,
    cpr: mmio::Register<u32>,
    ucv: mmio::Register<u32>,
    ctr: mmio::Register<u32>,
}

impl RK3399Uart {
    pub const fn new(base: usize) -> Self {
        Self {
            rbr_thr_dll: mmio::Register::new(base),
            dlh_ier: mmio::Register::new(base + 0x04),
            iir_fcr: mmio::Register::new(base + 0x08),
            lcr: mmio::Register::new(base + 0x0c),
            mcr: mmio::Register::new(base + 0x10),
            lsr: mmio::Register::new(base + 0x14),
            msr: mmio::Register::new(base + 0x18),
            scr: mmio::Register::new(base + 0x1c),
            srbr: mmio::Register::new(base + 0x30),
            sthr: mmio::Register::new(base + 0x6c),
            far: mmio::Register::new(base + 0x70),
            tfr: mmio::Register::new(base + 0x74),
            rfw: mmio::Register::new(base + 0x78),
            usr: mmio::Register::new(base + 0x7c),
            tfl: mmio::Register::new(base + 0x80),
            rfl: mmio::Register::new(base + 0x84),
            srr: mmio::Register::new(base + 0x88),
            srts: mmio::Register::new(base + 0x8c),
            sbcr: mmio::Register::new(base + 0x90),
            sdmam: mmio::Register::new(base + 0x94),
            sfe: mmio::Register::new(base + 0x98),
            srt: mmio::Register::new(base + 0x9c),
            stet: mmio::Register::new(base + 0xa0),
            htx: mmio::Register::new(base + 0xa4),
            dmasa: mmio::Register::new(base + 0xa8),
            cpr: mmio::Register::new(base + 0xf4),
            ucv: mmio::Register::new(base + 0xf8),
            ctr: mmio::Register::new(base + 0xfc),
        }
    }

    /// set baudrate
    ///
    /// # safety
    ///
    /// Function assumes that LCR::DIV_LAT_ACCESS is set
    unsafe fn set_baudrate_unsafe(&mut self, baudrate: drivers::serial::Baudrate) {
        let bauddiv = self.clock_frequency / 16 / baudrate;
        self.dlh_ier.write(((bauddiv >> 8) & 0xff) as u32);
        self.rbr_thr_dll.write((bauddiv & 0xff) as u32);
    }

    fn flush_tx(&mut self) {
        while (self.lsr.read() & LSR::TEMT == 0) {
        }
    }
}

impl drivers::Driver for RK3399Uart {

    fn enable(&mut self) -> Result<(), ()> {
        self.flush_tx();
        // Disable interrupts
        self.dlh_ier.write(0);
        // Set DTR and RTS to high
        // self.mcr.write(MCR::DTR | MCR::RTS)
        let lcr = DATA_LENGTH_SEL_8BITS; // 8n1
        self.lcr.write(lcr | DIV_LAT_ACCESS);
        unsafe {
            self.set_baudrate_unsafe(self.baudrate);
        }
        self.lcr.write(lcr);
        self.fcr.write(FCR::CLEAR_RCVR | FCR::CLEAR_XMIT);
        Ok(())
    }

    fn disable(&mut self) {
    }

    fn name(&self) -> &str {
        "RK3399 UART"
    }
}

impl drivers::SerialDevice for RK3399Uart {

    fn write(&mut self, chr: char) {
        self.rbr_thr_dll.write(chr as u32);
    }

    fn read(&mut self) -> char {
        self.rbr_thr_dll.read() as char
    }

    fn set_baudrate(&mut self, baudrate: drivers::serial::Baudrate) {
        let const lcr = self.lcr.read();
        self.lcr.write(lcr | LCR::DIV_LAT_ACCESS);
        unsafe {
            self.set_baudrate_unsafe(baudrate);
        }
        self.baudrate = baudrate;
        self.lcr.write(lcr);
    }

    fn set_parity(&mut self, parity: drivers::serial::Parity) {
        let reg = self.lcr.read();
        match parity {
            drivers::serial::Parity::None =>
                self.lcr.write(reg & !(LCR::PARITY_EN | LCR::EVEN_PARITY_SEL)),
            drivers::serial::Parity::Even =>
                self.lcr.write(reg | LCR::PARITY_EN | LCR::EVEN_PARITY_SEL),
            drivers::serial::Parity::Odd =>
                self.lcr.write(reg & (!LCR::EVEN_PARITY_SEL) | LCRH::PARITY_EN),
        }
    }

    fn set_data_bits(&mut self, data_bits: drivers::serial::DataBits) {
        let reg = self.lcr.read() & !(LCR::DATA_LENGTH_SEL_8BITS);
        match data_bits {
            drivers::serial::DataBits::Eight => self.lcr.write(reg | LCR::DATA_LENGTH_SEL_8BITS),
            drivers::serial::DataBits::Seven => self.lcr.write(reg | LCR::DATA_LENGTH_SEL_7BITS),
            drivers::serial::DataBits::Six => self.lcr.write(reg | LCR::DATA_LENGTH_SEL_6BITS),
            drivers::serial::DataBits::Five => self.lcr.write(reg | LCR::DATA_LENGTH_SEL_5BITS),
        }
    }

    fn baudrate(&self) -> drivers::serial::Baudrate {
        self.baudrate
    }

    fn parity(&self) -> drivers::serial::Parity {
        const PARITY_EVEN: u16 = LCR::PARITY_EN | LCR::EVEN_PARITY_SEL;
        let state = self.lcr.read() & (PARITY_EVEN);
        match state {
            PARITY_EVEN => drivers::serial::Parity::Even,
            LCR::PARITY_EN => drivers::serial::Parity::Odd,
            _ => drivers::serial::Parity::None,
        }
    }

    fn data_bits(&self) -> drivers::serial::DataBits {
        let state = self.lcr.read() & LCR::DATA_LENGTH_SEL_8BITS;
        match state {
            LCR::DATA_LENGTH_SEL_8BITS => drivers::serial::DataBits::Eight,
            LCR::DATA_LENGTH_SEL_7BITS => drivers::serial::DataBits::Seven,
            LCR::DATA_LENGTH_SEL_6BITS => drivers::serial::DataBits::Six,
            _ => drivers::serial::DataBits::Five,
        }
    }
}

impl irq::IRQHandler for RK3399Uart {
    fn on_interrupt(&mut self, _intnum: u32) {
        use crate::drivers::SerialDevice;
        let chr = self.read();
        self.write(chr);
    }
}

unsafe impl Sync for RK3399Uart {
}
