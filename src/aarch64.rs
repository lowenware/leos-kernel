//
// aarch64.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

mod boot;
mod cpu;
mod exceptions;
mod registers;

pub mod context;
pub mod drivers;
pub mod memory;

pub fn enable_irq() {
    exceptions::enable_all();
}

pub fn disable_irq() {
    exceptions::disable_all();
}
