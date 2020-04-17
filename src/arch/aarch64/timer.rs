//
// timer.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use crate::{scheduler};
use super::exceptions::ExceptionContext;
use super::gic;
use super::registers::{CNTPCT_EL0, CNTP_CTL_EL0, CNTFRQ_EL0, CNTP_TVAL_EL0};

pub const TIMER_IRQ: u32 = 30; // TODO: board-dependent value

pub fn init() {
    gic::init();
    gic::set_config(TIMER_IRQ, gic::ICFGR_EDGE);
    gic::set_priority(TIMER_IRQ, 0);
    gic::set_core(TIMER_IRQ, 0x01); // core0
    gic::clear(TIMER_IRQ);
    gic::enable(TIMER_IRQ);

    CNTP_TVAL_EL0.write(CNTFRQ_EL0.read());
    CNTP_CTL_EL0.write(CNTP_CTL_EL0::ENABLE);
}

#[no_mangle]
pub fn on_interrupt(_ctx: &mut ExceptionContext) {
    log_debug!("{}", "TICK");

    CNTP_TVAL_EL0.write(CNTFRQ_EL0.read());

    gic::clear(TIMER_IRQ);
    scheduler::run();
}


pub fn get() -> u64 {
    CNTPCT_EL0.read()
}

pub fn get_frequency() -> u64 {
    CNTFRQ_EL0.read()
}

