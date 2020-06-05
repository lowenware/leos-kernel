//
// boot.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//


use crate::{kernel_main, memory::zero_volatile};
use super::cpu;
use super::drivers::mmu;
use super::registers::{
    CNTHCTL_EL2,
    CNTVOFF_EL2,
    CurrentEL,
    HCR_EL2,
    SPSR_EL2,
    ELR_EL2,
    SPSel,
    VBAR_EL1
};

#[naked]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {

    if cpu::core_id() == 0 {
        let current_el = CurrentEL.read() & CurrentEL::EL_MASK;
        if current_el == CurrentEL::EL2_VALUE {
            // Do not trap access to EL1 physical timer registers to EL2
            CNTHCTL_EL2.set(CNTHCTL_EL2::EL1PCEN | CNTHCTL_EL2::EL1PCTEN);

            // No offset for counter reading
            CNTVOFF_EL2.write(0);

            // Set EL1 execution state to AArch64.
            HCR_EL2.set(HCR_EL2::RW | HCR_EL2::SWIO);

            // Simulate an exception
            SPSR_EL2.write(SPSR_EL2::D | SPSR_EL2::A | SPSR_EL2::I | SPSR_EL2::F | SPSR_EL2::EL1h);

            // Second, let the link register point to runtime_init().
            ELR_EL2.write(boot as *const () as u64);

            // Use `eret` to "return" to EL1. This results in execution of runtime_init() in EL1.
            cpu::eret()
        }
    }

    cpu::hang()
}

#[naked]
#[no_mangle]
unsafe fn boot() -> ! {

    // Each EL has own stack
    SPSel.write(SPSel::SP_ELx);

    // zero BSS
    let bss_base: *mut usize;
    let bss_end: *mut usize;
    llvm_asm!("adr $0, __bss_base" : "=r"(bss_base) ::: "volatile");
    llvm_asm!("adr $0, __bss_end" : "=r"(bss_end) ::: "volatile");
    zero_volatile(bss_base, bss_end);

    // init MMU
    let (kernel_base, kernel_size) = mmu::init();

    // Enable interrupts vector table
    extern "C" {
        fn exception_vector_table();
    }
    VBAR_EL1.write(exception_vector_table as usize as u64);

    // start kernel
    kernel_main(kernel_base, kernel_size);
}
