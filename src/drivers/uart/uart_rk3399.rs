//
// uart_rk3399.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use crate::{drivers, irq};
use crate::drivers::mmio;

#[allow(non_snake_case)]
pub struct RK3399Uart {
    base: usize
}

impl RK3399Uart {
    pub const fn new(base: usize) -> Self {
        Self {
            base
        }
    }
}

impl drivers::Driver for RK3399Uart {

    fn enable(&mut self) -> Result<(), ()> {
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
    }

    fn read(&mut self) -> char {
        '0'
    }

    fn set_baudrate(&mut self, baudrate: drivers::serial::Baudrate) {
    }

    fn set_parity(&mut self, parity: drivers::serial::Parity) {
    }

    fn set_data_bits(&mut self, data_bits: drivers::serial::DataBits) {
    }

    fn baudrate(&self) -> drivers::serial::Baudrate {
        115200
    }

    fn parity(&self) -> drivers::serial::Parity {
        drivers::serial::Parity::None
    }

    fn data_bits(&self) -> drivers::serial::DataBits {
        drivers::serial::DataBits::Eight
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
