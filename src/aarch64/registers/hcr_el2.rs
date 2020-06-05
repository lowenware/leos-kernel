//
// hcr_el2.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#![allow(non_snake_case)]
#![allow(dead_code)]
pub mod HCR_EL2 {

    /// Execution state control for lower Exception levels
    ///
    /// 0b0 - Lower levels are all AArch32
    /// 0b1 - The Execution state for EL1 is AArch64
    pub const RW: u64 = 1 << 31;

    /// Set/Way Invalidation Override
    ///
    /// 0b0 - No effect
    /// 0b1 - Data cache invalidate by set/way instructions
    pub const SWIO: u64 = 1 << 1;

    pub struct Register{}

    impl Register {
        aarch64_sysreg_read!(u64, "HCR_EL2");
        aarch64_sysreg_write!(u64, "HCR_EL2");
        aarch64_sysreg_set!(u64, "HCR_EL2");
        aarch64_sysreg_get!(u64, "HCR_EL2");
        aarch64_sysreg_clear!(u64, "HCR_EL2");
        aarch64_sysreg_is_set!(u64, "HCR_EL2");
        aarch64_sysreg_has!(u64, "HCR_EL2");
    }
}

pub static HCR_EL2: HCR_EL2::Register = HCR_EL2::Register{};
