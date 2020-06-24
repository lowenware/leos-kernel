//
// spsr_el2.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#![allow(non_snake_case)]
#![allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod SPSR_EL2 {

    /// Debug exception mask
    pub const D: u64 = 1 << 9;

    /// SError exception mask
    pub const A: u64 = 1 << 8;

    /// IRQ exception mask
    pub const I: u64 = 1 << 7;

    /// FIQ exception mask
    pub const F: u64 = 1 << 6;

    pub const EL1h: u64 = 0b0101;

    pub struct Register{}

    impl Register {
        aarch64_sysreg_read!(u64, "SPSR_EL2");
        aarch64_sysreg_write!(u64, "SPSR_EL2");
        aarch64_sysreg_set!(u64, "SPSR_EL2");
        aarch64_sysreg_get!(u64, "SPSR_EL2");
        aarch64_sysreg_clear!(u64, "SPSR_EL2");
        aarch64_sysreg_is_set!(u64, "SPSR_EL2");
        aarch64_sysreg_has!(u64, "SPSR_EL2");
    }
}

pub static SPSR_EL2: SPSR_EL2::Register = SPSR_EL2::Register{};
