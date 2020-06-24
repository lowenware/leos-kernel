//
// cntfrq_el0.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#![allow(non_snake_case)]
mod CNTFRQ_EL0 {
    pub struct Register{}

    impl Register {
        aarch64_sysreg_read!(u64, "CNTFRQ_EL0");
    }
}

pub static CNTFRQ_EL0: CNTFRQ_EL0::Register = CNTFRQ_EL0::Register{};
