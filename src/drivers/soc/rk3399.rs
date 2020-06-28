//
// rk3399.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

pub const PERIPHERALS_REAL: usize = 0xf800_0000;
pub const PERIPHERALS_SIZE: usize = 0x0800_0000;

pub const MEMORY_BASE: usize = 0x0000_0000;
pub const MEMORY_SIZE: usize = 0xf800_0000;

pub const GIC_BASE: usize = 0xfee0_0000;
pub const UART0_BASE: usize = 0xff18_0000;
pub const UART1_BASE: usize = 0xff19_0000;
pub const UART2_BASE: usize = 0xff1A_0000;
pub const UART3_BASE: usize = 0xff1B_0000;
pub const UART4_BASE: usize = 0xff37_0000;

pub const UART0_INTNUM: usize = 131;
pub const UART1_INTNUM: usize = 130;
pub const UART2_INTNUM: usize = 132;
pub const UART3_INTNUM: usize = 133;
pub const UART4_INTNUM: usize = 134;
