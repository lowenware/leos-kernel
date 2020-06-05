//
// mair_el1.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#![allow(non_snake_case)]
#![allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod MAIR_EL1 {

    pub mod device {
        pub const nGnRnE: u64 = 0b00 << 2;
        pub const nGnRE: u64 = 0b01 << 2;
        pub const nGRE: u64 = 0b10 << 2;
        pub const GRE: u64 = 0b11 << 2;
    }

    pub mod normal {
        pub mod inner {
            pub const WriteThroughTransient: u64 = 0b0000; // + Read/Write
            pub const NonCacheable: u64 = 0b0100; // No Read/Write
            pub const WriteBackTransient: u64 = 0b0100; // + Read/Write
            pub const WriteThroughNonTransient: u64 = 0b1000;
            pub const WriteBackNonTransient: u64 = 0b1100;
            pub const ReadAllocate: u64 = 0b10;
            pub const WriteAllocate: u64 = 0b01;
        }

        pub mod outer {
            pub const WriteThroughTransient: u64 = 0b0000 << 4; // + Read/Write
            pub const NonCacheable: u64 = 0b0100 << 4; // No Read/Write
            pub const WriteBackTransient: u64 = 0b0100 << 4; // + Read/Write
            pub const WriteThroughNonTransient: u64 = 0b1000 << 4;
            pub const WriteBackNonTransient: u64 = 0b1100 << 4;
            pub const ReadAllocate: u64 = 0b10 << 4;
            pub const WriteAllocate: u64 = 0b01 << 4;
        }
    }

    #[inline(always)]
    pub fn attr(index: usize, value: u64) -> u64 {
        value << (index * 8)
    }

    pub struct Register{}

    impl Register {
        aarch64_sysreg_read!(u64, "MAIR_EL1");
        aarch64_sysreg_write!(u64, "MAIR_EL1");
    }
}

pub static MAIR_EL1: MAIR_EL1::Register = MAIR_EL1::Register{};
