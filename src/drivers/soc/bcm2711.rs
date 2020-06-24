//
// bcm2711.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

pub mod aux;
pub mod uart;
pub mod gpio;

// pub const UART0_BASE: usize = 0x3F20_1000;
pub const UART0_BASE: usize = 0xFE20_1000;
pub const UART2_BASE: usize = 0xFE20_1400;
pub const UART3_BASE: usize = 0xFE20_1600;
pub const UART4_BASE: usize = 0xFE20_1800;
pub const UART5_BASE: usize = 0xFE20_1A00;

pub const AUX_BASE: usize = 0xFE21_5000;
pub const GPIO_BASE: usize = 0xFE20_0000;
