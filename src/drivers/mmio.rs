//
// mmio.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use core::ptr;

pub struct Register<T> {
    base: *mut T,
}

impl<T> Register<T> {
    pub const fn new(base: usize) -> Self {
        Self { base: base as *mut T }
    }

    pub fn read(&self) -> T {
        unsafe {
            ptr::read_volatile(self.base)
        }
    }

    pub fn write(&self, value: T) {
        unsafe {
            ptr::write_volatile(self.base, value);
        }
    }

}
