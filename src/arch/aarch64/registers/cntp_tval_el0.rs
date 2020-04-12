//
// cntp_tval_el0.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#![allow(non_snake_case)]
#![allow(dead_code)]
pub mod CNTP_TVAL_EL0 {
    pub struct Register{}

    impl Register {
        aarch64_sysreg_read!(u64, "CNTP_TVAL_EL0");
        aarch64_sysreg_write!(u64, "CNTP_TVAL_EL0");
    }
}

pub static CNTP_TVAL_EL0: CNTP_TVAL_EL0::Register = CNTP_TVAL_EL0::Register{};
