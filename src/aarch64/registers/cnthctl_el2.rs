//
// cnthctl_el2.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#![allow(non_snake_case)]
#![allow(dead_code)]
pub mod CNTHCTL_EL2 {
    /// Traps EL0 and EL1 accesses to the EL1 physical counter register to EL2 when EL2 is enabled
    /// in the current Security state.
    ///
    /// 0b0 - Accesses to the CNTPCT_EL0 are trapped to EL2
    /// 0b1 - No instructions to be trapped
    pub const EL1PCTEN: u64 = 1;

    /// Traps EL0 and EL1 accesses to the EL1 physical timer registers to EL2 when EL2 is enabled
    /// in the current Security state
    ///
    /// 0b0 - Access to CNTP_CTL_EL0, CNTP_CVAL_EL0, and CNTP_TVAL_EL0 are trapped to EL2
    /// 0b1 - No instructions to be trapped
    pub const EL1PCEN: u64 = 1 << 1;

    pub struct Register{}

    impl Register {
        aarch64_sysreg_read!(u64, "CNTHCTL_EL2");
        aarch64_sysreg_write!(u64, "CNTHCTL_EL2");
        aarch64_sysreg_set!(u64, "CNTHCTL_EL2");
        aarch64_sysreg_get!(u64, "CNTHCTL_EL2");
        aarch64_sysreg_clear!(u64, "CNTHCTL_EL2");
        aarch64_sysreg_is_set!(u64, "CNTHCTL_EL2");
        aarch64_sysreg_has!(u64, "CNTHCTL_EL2");
    }
}

pub static CNTHCTL_EL2: CNTHCTL_EL2::Register = CNTHCTL_EL2::Register{};
