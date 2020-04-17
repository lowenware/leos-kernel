//
// ttbr1_el1.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#![allow(dead_code)]

#![allow(non_snake_case)]
mod TTBR1_EL1 {
    pub struct Register{}

    impl Register {
        aarch64_sysreg_read!(u64, "TTBR1_EL1");
        aarch64_sysreg_write!(u64, "TTBR1_EL1");
    }
}

pub static TTBR1_EL1: TTBR1_EL1::Register = TTBR1_EL1::Register{};
