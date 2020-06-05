//
// sp_el1.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#![allow(non_snake_case)]
#![allow(dead_code)]
pub mod SP_EL1 {

    pub struct Register{}

    impl Register {
        aarch64_sysreg_read!(u64, "SP_EL1");
        aarch64_sysreg_write!(u64, "SP_EL1");
    }
}

pub static SP_EL1: SP_EL1::Register = SP_EL1::Register{};
