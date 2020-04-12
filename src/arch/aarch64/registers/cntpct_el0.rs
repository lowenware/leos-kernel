//
// cntpct_el0.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#![allow(non_snake_case)]
mod CNTPCT_EL0 {
    pub struct Register{}

    impl Register {
        aarch64_sysreg_read!(u64, "CNTPCT_EL0");
    }
}

pub static CNTPCT_EL0: CNTPCT_EL0::Register = CNTPCT_EL0::Register{};
