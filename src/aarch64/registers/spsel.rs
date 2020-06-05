//
// cntp_ctl_el0.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#![allow(non_snake_case)]
#![allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod SPSel {

    /// Use SP_ELx for Exception level ELx
    /// SP_EL0 if unset
    pub const SP_ELx: u64 = 1;

    pub struct Register{}

    impl Register {
        aarch64_sysreg_read!(u64, "SPSel");
        aarch64_sysreg_write!(u64, "SPSel");
        aarch64_sysreg_set!(u64, "SPSel");
        aarch64_sysreg_get!(u64, "SPSel");
        aarch64_sysreg_clear!(u64, "SPSel");
        aarch64_sysreg_is_set!(u64, "SPSel");
        aarch64_sysreg_has!(u64, "SPSel");
    }
}

#[allow(non_upper_case_globals)]
pub static SPSel: SPSel::Register = SPSel::Register{};
