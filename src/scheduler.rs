//
// scheduler.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

mod task;
mod queue;

use crate::{arch, irq};
use task::Task;
use queue::Queue;
use alloc::{vec::Vec};

static mut QUEUES: Option<Vec<Queue<Task>>> = None;
static mut PID: u32 = 0;

pub fn init() {
    // TODO: get number of CPU cores
    let num_of_queues = 1;
    let mut queues = Vec::new();
    for _i in 0..num_of_queues {
        queues.push(Queue::new());
    }

    unsafe {
        PID += 1;
        queues[0].push(Task::new(PID, 0, 0, 0, 0));
        QUEUES = Some(queues);
    }

    if let Some(current) = queue().head() {
        current.enable_preempt();
        log_debug!("core task preempt: {}", current.preempt());
    }
}

fn queue() -> &'static mut Queue<Task> {
    unsafe {
        &mut QUEUES.as_mut().unwrap()[0]
    }
}

#[no_mangle]
extern "C" fn start_task(argument: usize, callback: extern fn(usize)) {
    if let Some(current) = queue().head() {
        current.enable_preempt();
    }
    callback(argument);
    panic!("Task shutdown is not implemented");
}

// fn moo() {}
// println!("{:p}", moo as *const ());


// fn current() -> *mut Task {
//     let current = queue().head();
//      match current {
//         None => ptr::null_mut(),
//         Some(task) => &mut *task
//     }
// }

// fn next() -> *mut Task {
//     let next = queue().next();
//     match next {
//         None => ptr::null_mut(),
//         Some(task) => &mut *task
//     }
// }

fn switch_task(current: &mut Task) {
    if let Some(next) = queue().next() {
        if next.as_ptr() != current.as_ptr() {
            log_debug!("Switch context PID {} -> {}", current.pid(), next.pid());
            unsafe {
                arch::context::switch_context(current.as_ptr() as usize, next.as_ptr() as usize);
            }
        }
    }
}

pub fn run() {
    if let Some(current) = queue().head() {
        if current.preempt() == 0 {
            current.disable_preempt();
            // enable interrupts
            irq::enable_all();
            // switch tasks
            switch_task(current);
            // disable interrupts
            irq::disable_all();
            if let Some(next) = queue().head() {
                next.enable_preempt();
            }
        }
    }
}

pub fn add(argument: usize, callback: extern fn(usize), stack: usize, ttbr0_base: usize) {

    unsafe {
        PID += 1;
        queue().append(Task::new(PID, callback as usize, argument, stack, ttbr0_base));
    }
}
