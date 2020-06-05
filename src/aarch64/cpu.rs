//
// cpu.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//
#![allow(dead_code)]

use super::registers::MPIDR_EL1;

#[inline(always)]
pub fn core_id() -> u8
{
    (MPIDR_EL1.read() & 0b11) as u8
}

#[inline(always)]
pub fn hang() -> ! {
    loop {
        wfe()
    }
}

#[inline(always)]
pub fn wfe() {
    unsafe {
        llvm_asm!("wfe" :::: "volatile");
    }
}

#[inline(always)]
pub fn wfi() {
    unsafe {
        llvm_asm!("wfi" :::: "volatile");
    }
}

#[inline(always)]
pub fn isb() {
    unsafe {
        llvm_asm!("isb" :::: "volatile");
    }
}

#[inline(always)]
pub fn idle(cycles: usize) {
    for _ in 0..cycles {
        unsafe {
            llvm_asm!("nop" :::: "volatile");
        }
    }
}

#[inline(always)]
pub fn eret() -> ! {
    unsafe {
        llvm_asm!("eret" :::: "volatile");
        core::intrinsics::unreachable()
    }
}

#[inline(always)]
pub fn address() -> usize {
    let result : usize;
    unsafe {
        llvm_asm!("adr $0, ." : "=r"(result) ::: "volatile");
    }
    result
}

pub mod sp {
    #[inline(always)]
    pub fn write(value: usize) {
        unsafe {
            llvm_asm!("mov sp, $0" :: "r"(value) ::: "volatile");
        }
    }
}

pub mod lr {
    #[inline(always)]
    pub fn read() -> usize {
        let value: usize;
        unsafe {
            llvm_asm!("mov $0, lr" : "=r"(value) ::: "volatile");
        }
        value
    }

    #[inline(always)]
    pub fn write(value: usize) {
        unsafe {
            llvm_asm!("mov lr, $0" :: "r"(value) ::: "volatile");
        }
    }
}
