//
// qemu.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//
//

use crate::{irq, memory};
use crate::aarch64::drivers::{gic::GIC, timer::Timer};
use crate::drivers::{Driver};
use crate::drivers::uart::uart_pl011::UARTPL011;

pub const PERIPHERALS_BASE: usize = memory::PERIPHERALS_BASE;

const PERIPHERALS_REAL: usize = 0x0000_0000;
const PERIPHERALS_SIZE: usize = 0x4000_0000;

pub const MEMORY_BASE: usize = 0x4000_0000;
pub const MEMORY_SIZE: usize = 0x1_00000000;

pub const GICD_BASE: usize = PERIPHERALS_BASE + 0x0800_0000;
pub const GICC_BASE: usize = PERIPHERALS_BASE + 0x0801_0000;
pub const UART_BASE: usize = PERIPHERALS_BASE + 0x0900_0000;
pub const MMIO_BASE: usize = PERIPHERALS_BASE + 0x0a00_0000;
pub const PCIE_MMIO_BASE: usize = PERIPHERALS_BASE + 0x3eff_0000;

const TIMER0_IRQNUM: u32 = 30;
const UART0_IRQNUM: u32 = 33;

const UART_CLOCK_FREQ: u32 = 48000000;

pub static mut IC0: GIC = GIC::new(GICD_BASE, GICC_BASE);
pub static mut TIMER0: Timer = Timer::new();
pub static mut UART0: UARTPL011 = UARTPL011::new(UART_BASE, UART_CLOCK_FREQ);

pub fn init() {
    // delete from the map memory used for peripherals (if in the middle of the region)
    memory::cut(PERIPHERALS_REAL, PERIPHERALS_SIZE);
    // map peripherals memory
    memory::kernel().assign_L1(PERIPHERALS_REAL, PERIPHERALS_BASE, memory::PERIPHERALS_FLAGS);

    unsafe {
        // initalize GIC
        if let Err(_e) = IC0.enable() {
            panic!("Failed to initialize {}", IC0.name());
        }

        // initialize timer
        if let Err(_e) = TIMER0.enable() {
            panic!("Failed to initialize {}", TIMER0.name());
        }
        irq::register_handler(TIMER0_IRQNUM, &mut TIMER0);

        // initialize uart
        if let Err(_e) = UART0.enable() {
            panic!("Failed to initialize {}", UART0.name());
        }
        irq::register_handler(UART0_IRQNUM, &mut UART0);
    }
}

pub fn get_interrupt_controller() -> &'static GIC {
    unsafe {
        &IC0
    }
}
