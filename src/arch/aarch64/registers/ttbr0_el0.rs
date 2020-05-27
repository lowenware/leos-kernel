//
// ttbr0_el0.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#![allow(dead_code)]

#![allow(non_snake_case)]
mod TTBR0_EL0 {
    pub struct Register{}

    impl Register {
        aarch64_sysreg_read!(u64, "TTBR0_EL0");
        aarch64_sysreg_write!(u64, "TTBR0_EL0");
    }
}

pub static TTBR0_EL0: TTBR0_EL0::Register = TTBR0_EL0::Register{};
