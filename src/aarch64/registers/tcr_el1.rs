//
// tcr_el1.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#![allow(non_snake_case)]
#![allow(dead_code)]
pub mod TCR_EL1 {
    pub const IPS_36BIT: u64 = 0b001 << 32;

    pub const TG1_16KB: u64 = 0b01 << 30;
    pub const TG1_4KB: u64 = 0b10 << 30;
    pub const TG1_64KB: u64 = 0b11 << 30;
    pub const SH1_INNER_SHAREABLE: u64 = 0b11 << 28;
    pub const ORGN1_NORMAL_OUTER: u64 = 0b01 << 26;
    pub const IRGN1_NORMAL_INNER: u64 = 0b01 << 24;

    #[inline(always)]
    pub fn T1SZ(value: u64) -> u64 {
        (value & 0x3F) << 16
    }

    pub const TG0_4KB: u64 = 0b00;
    pub const TG0_64KB: u64 = 0b01;
    pub const TG0_16KB: u64 = 0b10;
    pub const SH0_INNER_SHAREABLE: u64 = 0b11 << 12;
    pub const ORGN0_NORMAL_OUTER: u64 = 0b01 << 10;
    pub const IRGN0_NORMAL_INNER: u64 = 0b01 << 8;

    #[inline(always)]
    pub fn T0SZ(value: u64) -> u64 {
        value & 0x3F
    }

    pub struct Register{}

    impl Register {
        aarch64_sysreg_read!(u64, "TCR_EL1");
        aarch64_sysreg_write!(u64, "TCR_EL1");
        aarch64_sysreg_set!(u64, "TCR_EL1");
        aarch64_sysreg_get!(u64, "TCR_EL1");
        aarch64_sysreg_clear!(u64, "TCR_EL1");
        aarch64_sysreg_is_set!(u64, "TCR_EL1");
        aarch64_sysreg_has!(u64, "TCR_EL1");
    }
}

pub static TCR_EL1: TCR_EL1::Register = TCR_EL1::Register{};
