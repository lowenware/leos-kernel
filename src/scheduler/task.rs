//
// task.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use core::fmt;
use crate::arch::{TaskContext};

type PID = u32;

#[repr(C)]
pub struct Task {
    ctx: TaskContext,
    pid: PID,
    preempt: u32,
}

impl Task {

    pub const fn new(pid: PID, entry: usize, arg: usize, stack: usize, ttbr0_base: usize) -> Self {
        Task {
            ctx: TaskContext::new(entry, arg, stack, ttbr0_base),
            pid,
            preempt: 1,
        }
    }

    pub fn pid(&self) -> PID {
        self.pid
    }

    pub fn preempt(&self) -> u32 {
        self.preempt
    }

    pub fn enable_preempt(&mut self) {
        self.preempt -= 1;
        log_debug!("task {}: enable_preempt -> {}", self.pid, self.preempt);
    }

    pub fn disable_preempt(&mut self) {
        self.preempt += 1;
        log_debug!("task {}: disable_preempt -> {}", self.pid, self.preempt);
    }

    pub fn as_ptr(&mut self) -> *mut Self {
        &mut *self
    }

}

impl fmt::Debug for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Task")
         .field("pid", &self.pid)
         .finish()
    }
}
