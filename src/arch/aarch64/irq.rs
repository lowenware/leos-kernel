//
// irq.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use super::exceptions::ExceptionContext;
use super::{gic, timer};

#[no_mangle]
pub fn handler(ctx: &mut ExceptionContext) {
    if gic::is_pending(timer::TIMER_IRQ) {
        timer::on_interrupt(ctx);
    }
}

pub fn enable() {
    unsafe {
        llvm_asm!("msr daifclr, #2");
    }
}

pub fn disable() {
    unsafe {
        llvm_asm!("msr daifset, #2");
    }
}

