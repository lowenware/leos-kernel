//
// queue.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use core::ptr;
use alloc::{boxed::Box};

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
}

pub struct Queue<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

impl<T> Queue<T> {

    pub const fn new() -> Self {
        Self { head: None, tail: ptr::null_mut() }
    }

    pub fn push(&mut self, item: T) {
        let mut item = Box::new(
            Node { item, next: self.head.take() }
        );

        if self.tail.is_null() {
            self.tail = &mut *item;
        }

        self.head = Some(item);
    }

//     pub fn pop(&mut self) -> Option<T> {
//         self.head.take().map(|head| {
//             let head = *head;
//             self.head = head.next;
//
//             if self.head.is_none() {
//                 self.tail = ptr::null_mut();
//             }
//
//             head.item
//         })
//     }

    pub fn append(&mut self, item: T) {
        let mut item = Box::new(
            Node { item, next: None }
        );

        let new_tail: *mut _ = &mut *item;

        if self.tail.is_null() {
            self.head = Some(item);
        } else {
            unsafe {
                (*self.tail).next = Some(item);
            }
        }

        self.tail = new_tail;
    }

    pub fn head(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.item
        })
    }

    pub fn next(&mut self) -> Option<&mut T> {
        if let Some(mut head) = self.head.take() {
            self.head = head.next.take();
            if self.head.is_none() {
                self.head = Some(head);
            } else {
                let p_tail = self.tail;
                self.tail = &mut *head;
                unsafe {
                    (*p_tail).next = Some(head);
                }
            }
        }

        self.head.as_mut().map(|head| {
            &mut head.item
        })
    }

}
