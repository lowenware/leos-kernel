//
// scheduler.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use crate::arch::{TaskContext};

pub struct Task {
    ctx: TaskContext,
    pid: u32,
}

static mut QUEUE: [Task; 3] = [
    Task {
        ctx: TaskContext{regs: [0; 12], sp: 0},
        pid: 0
    },
    Task {
        ctx: TaskContext{regs: [0; 12], sp: 0},
        pid: 0
    },
    Task {
        ctx: TaskContext{regs: [0; 12], sp: 0},
        pid: 0
    }
];

pub fn init() {
    unsafe {
        QUEUE[0].ctx.sp = 0;
        QUEUE[0].pid = 1;
        QUEUE[1].pid = 2;
        QUEUE[2].pid = 3;
    }
}

pub fn run() {

}
