//
// mpidr_el1.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#![allow(non_snake_case)]
mod MPIDR_EL1 {
    pub struct Register{}

    impl Register {
        aarch64_sysreg_read!(u64, "MPIDR_EL1");
    }
}

pub static MPIDR_EL1: MPIDR_EL1::Register = MPIDR_EL1::Register{};
