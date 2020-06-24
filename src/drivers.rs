//
// drivers.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

pub mod mmio;
pub mod soc;
pub mod uart;

pub trait Driver {
    /// Callback to enable the device
    fn enable(&mut self) -> Result<(), ()> {
        Ok(())
    }

    /// Callback to disable the device
    fn disable(&mut self);

    /// Get driver name
    fn name(&self) -> &str;
}

pub trait InterruptController {
    /// Set interrupt enabled
    fn set_interrupt(&self, intnum: u32);
    /// Unset enabled interrupt
    fn unset_interrupt(&self, intnum: u32);
    /// Check if interrupt is pending
    fn is_interrupt(&self, intnum: u32) -> bool;
    /// Clear pending interrupt
    fn clear_interrupt(&self, intnum: u32);
    /// set interrupt core
    fn set_interrupt_core(&self, intnum: u32, core: u32);
    /// set interrupt priority
    fn set_interrupt_priority(&self, intnum: u32, priority: u32);
    /// set interrupt config
    fn set_interrupt_config(&self, intnum: u32, config: u32);
}

pub trait SystemTimer {
    /// Get timer value
    fn get_value(&self) -> u64;
    /// Get timer frequency
    fn get_frequency(&self) -> u64;
}

pub mod serial {
    pub type Baudrate = u32;

    pub enum Parity {
        None,
        Even,
        Odd,
    }

    pub enum DataBits {
        Five,
        Six,
        Seven,
        Eight,
    }
}

pub trait SerialDevice {
    fn write(&mut self, chr: char);
    fn read(&mut self) -> char;
    fn set_baudrate(&mut self, baudrate: serial::Baudrate);
    fn set_parity(&mut self, parity: serial::Parity);
    fn set_data_bits(&mut self, data_bits: serial::DataBits);
    fn baudrate(&self) -> serial::Baudrate;
    fn parity(&self) -> serial::Parity;
    fn data_bits(&self) -> serial::DataBits;
}

