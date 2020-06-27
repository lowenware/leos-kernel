//
// pinebookpro.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//
//

use crate::aarch64::drivers::{gic::GIC};
use crate::drivers::{Driver};
use crate::drivers::soc::rk3399;
use crate::drivers::uart::uart_rk3399::RK3399Uart;
use crate::drivers::timer::rk3399_timer::RK3399Timer;
use crate::memory;

pub use crate::drivers::soc::rk3399::{MEMORY_BASE, MEMORY_SIZE};

const GICD_BASE: usize = 0;
const GICC_BASE: usize = 0;

pub static mut IC0: GIC = GIC::new(GICD_BASE, GICC_BASE);
pub static mut TIMER0: RK3399Timer = RK3399Timer::new();
pub static mut UART0: RK3399Uart = RK3399Uart::new(rk3399::UART0_BASE);

pub fn init() {
    // delete from the map memory used for peripherals (if in the middle of the region)
    memory::cut(rk3399::PERIPHERALS_REAL, rk3399::PERIPHERALS_SIZE);

    // map peripherals memory
    let mut p_addr = rk3399::PERIPHERALS_REAL;
    let mut v_addr = memory::PERIPHERALS_BASE;
    while p_addr < rk3399::PERIPHERALS_REAL + rk3399::PERIPHERALS_SIZE {
        memory::kernel().assign_L2(p_addr, v_addr, memory::PERIPHERALS_FLAGS);
        p_addr += memory::BLOCK_L2_SIZE;
        v_addr += memory::BLOCK_L2_SIZE;
    }

    // initialize UART
    unsafe {
        UART0.enable();
    }
}

pub fn get_interrupt_controller() -> &'static GIC {
    unsafe {
        &IC0
    }
}
