//
// irq.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use super::exceptions::ExceptionCtx;
use super::{gic, timer};

#[no_mangle]
pub fn handler(ctx: &mut ExceptionCtx) {
    if gic::is_pending(timer::TIMER_IRQ) {
        timer::on_interrupt(ctx);
    }
}

pub fn enable() {
    unsafe {
        asm!("msr daifclr, #2");
    }
}

/*
pub fn disable() {
    unsafe {
        asm!("msr daifset, #2");
    }
}
*/

