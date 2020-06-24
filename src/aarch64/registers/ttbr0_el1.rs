//
// ttbr0_el1.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#![allow(dead_code)]

#![allow(non_snake_case)]
pub mod TTBR0_EL1 {
    pub const CNP: u64 = 1;

    pub struct Register{}

    impl Register {
        aarch64_sysreg_read!(u64, "TTBR0_EL1");
        aarch64_sysreg_write!(u64, "TTBR0_EL1");
    }
}

pub static TTBR0_EL1: TTBR0_EL1::Register = TTBR0_EL1::Register{};
