//
// sctlr_el1.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#![allow(non_snake_case)]
#![allow(dead_code)]
pub mod SCTLR_EL1 {
    /// Enables the MMU
    ///
    /// 0b0 - MMU EL1 and EL0 is disabled
    /// 0b1 - MMU EL1 and EL0 is enabled
    pub const M: u64 = 1;

    pub struct Register{}

    impl Register {
        aarch64_sysreg_read!(u64, "SCTLR_EL1");
        aarch64_sysreg_write!(u64, "SCTLR_EL1");
        aarch64_sysreg_set!(u64, "SCTLR_EL1");
        aarch64_sysreg_get!(u64, "SCTLR_EL1");
        aarch64_sysreg_clear!(u64, "SCTLR_EL1");
        aarch64_sysreg_is_set!(u64, "SCTLR_EL1");
        aarch64_sysreg_has!(u64, "SCTLR_EL1");
    }
}

pub static SCTLR_EL1: SCTLR_EL1::Register = SCTLR_EL1::Register{};
