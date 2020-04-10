//
// timer.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use core::ptr;
use crate::scheduler;
use super::exceptions::ExceptionCtx;
use super::gic;

const TIMER_IRQ: u32 = 30; // TODO: board-dependent value

pub fn init() {
    unsafe {
        gic::init();
        gic::set_config(TIMER_IRQ, gic::ICFGR_EDGE);
        gic::set_priority(TIMER_IRQ, 0 << 4); // what is 4???
        gic::set_core(TIMER_IRQ, 0x01); // core0
        gic::clear(TIMER_IRQ);
        gic::enable(TIMER_IRQ);

        asm!("mrs x1, CNTFRQ_EL0");
        asm!("msr CNTP_TVAL_EL0, x1");
        asm!("mov x0, 1");
        asm!("msr CNTP_CTL_EL0, x0");
    }
}

#[no_mangle]
pub fn on_interrupt(_ctx: &mut ExceptionCtx) {
    const UART0: *mut u8 = 0xffff_ffe0_0900_0000 as *mut u8;
    unsafe {
        ptr::write_volatile(UART0, 0x2E);

        asm!("mrs x1, CNTFRQ_EL0");
        asm!("msr CNTP_TVAL_EL0, x1");

        gic::clear(TIMER_IRQ);
    }
    scheduler::run();
}

/*
pub fn get() -> u64 {
    return 0;
}
*/
