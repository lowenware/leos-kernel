//
// cntvoff_el2.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#![allow(non_snake_case)]
#![allow(dead_code)]
pub mod CNTVOFF_EL2 {

    pub struct Register{}

    impl Register {
        aarch64_sysreg_read!(u64, "CNTVOFF_EL2");
        aarch64_sysreg_write!(u64, "CNTVOFF_EL2");
    }
}

pub static CNTVOFF_EL2: CNTVOFF_EL2::Register = CNTVOFF_EL2::Register{};
