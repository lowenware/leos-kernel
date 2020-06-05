//
// current.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#![allow(non_snake_case)]
#![allow(dead_code)]
pub mod CurrentEL {

    pub const EL3_VALUE: u64 = 0b11 << 2;
    pub const EL2_VALUE: u64 = 0b10 << 2;
    pub const EL1_VALUE: u64 = 0b01 << 2;
    pub const EL0_VALUE: u64 = 0b00 << 2;
    pub const EL_MASK: u64 = 0b11 << 2;

    pub struct Register{}

    impl Register {
        aarch64_sysreg_read!(u64, "CurrentEL");
    }
}

#[allow(non_upper_case_globals)]
pub static CurrentEL: CurrentEL::Register = CurrentEL::Register{};
