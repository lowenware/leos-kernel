//
// irq.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use crate::{arch, list};
use crate::platform;
use crate::drivers::{InterruptController};

pub trait IRQHandler {
    fn on_interrupt(&mut self, intnum: u32);
}

struct IRQNode {
    intnum: u32,
    handler: &'static mut (dyn IRQHandler),
}

static mut HANDLERS: list::List::<IRQNode> = list::List::<IRQNode>::new();

pub fn register_handler(intnum: u32, handler: &'static mut (dyn IRQHandler)) {
    let ic0 = platform::get_interrupt_controller();

    unsafe {
        HANDLERS.append(IRQNode::new(intnum, handler));
    }

    // ic0.set_config(TIMER_IRQ, gic::ICFGR_EDGE);
    ic0.set_interrupt_priority(intnum, 0);
    ic0.set_interrupt_core(intnum, 0x01); // core0
    ic0.clear_interrupt(intnum);
    ic0.set_interrupt(intnum);
}

pub fn enable_all() {
    arch::enable_irq();
}

pub fn disable_all() {
    arch::disable_irq();
}

/// IRQ Handler
///
/// # Safety
/// - Should not be called somewhere else than interrupt
///
#[no_mangle]
pub unsafe fn handler() {
    let ic0 = platform::get_interrupt_controller();

    let iter = HANDLERS.iter_mut();
    for node in iter {
        if ic0.is_interrupt(node.intnum) {
            node.handler.on_interrupt(node.intnum);
            ic0.clear_interrupt(node.intnum);
        }
    }
}

impl IRQNode {
    pub const fn new(intnum: u32, handler: &'static mut (dyn IRQHandler)) -> Self {
        Self {intnum, handler}
    }
}
