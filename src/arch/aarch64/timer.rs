//
// timer.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use crate::{scheduler, log_write};
use super::exceptions::ExceptionCtx;
use super::gic;

const TIMER_IRQ: u32 = 30; // TODO: board-dependent value

// TODO: make safe timer initialization and remove inline attribute
#[inline(never)]
pub fn init() {
    unsafe {
        gic::init();
        gic::set_config(TIMER_IRQ, gic::ICFGR_EDGE);
        gic::set_priority(TIMER_IRQ, 0);
        gic::set_core(TIMER_IRQ, 0x01); // core0
        gic::clear(TIMER_IRQ);
        gic::enable(TIMER_IRQ);

        asm!("mrs x1, CNTFRQ_EL0");
        asm!("msr CNTP_TVAL_EL0, x1");
        asm!("mov x0, 1");
        asm!("msr CNTP_CTL_EL0, x0");
    }
}

// TODO: make safe timer interrupt handling and remove inline attribute
#[no_mangle]
#[inline(never)]
pub fn on_interrupt(_ctx: &mut ExceptionCtx) {
    unsafe {
        log_write!(".");

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
