//
// map.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use alloc::{boxed::Box};
use core::cmp::Ordering;

const HEAP_HEAD_ADDRESS: usize = 0xfeed; // fake not aligned to page address

type Link = Option<Box<Node>>;

struct Node {
    address: usize,
    size: usize,
    next: Link,
}

pub struct Map {
    head: Node,     // Map.head.size contains free memory
    total: usize,
}

impl Node {

    const fn new(address: usize, size: usize, next: Link) -> Self {
        Self { address, size, next }
    }
}

impl Map {

    pub const fn new() -> Self {
        Self { head: Node::new(HEAP_HEAD_ADDRESS, 0, None), total: 0 }
    }

    pub fn get(&mut self, size: usize) -> Result<usize, ()> {
        let mut current = &mut self.head;

        while let Some(ref mut node) = current.next {

            let address = node.address;

            match node.size.cmp(&size) {
                Ordering::Equal => {
                    current.next = node.next.take();
                    self.head.size -= size;
                    return Ok(address)
                },
                Ordering::Greater => {
                    node.address += size;
                    node.size -= size;
                    self.head.size -= size;
                    return Ok(address)
                },
                Ordering::Less => current = current.next.as_mut().unwrap(),
            }
        }
        Err(())
    }

    pub fn add(&mut self, address: usize, size: usize) {
        /*
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(0, 0, None)));
        }
        */
        self.put(address, size);
        self.total += size;
    }

    pub fn put(&mut self, address: usize, size: usize) {
        // TODO: optimize branching here
        let mut current = &mut self.head;

        while let Some(ref mut node) = current.next {
            if address > node.address {
                current = current.next.as_mut().unwrap();
                continue;
            }

            if current.address + current.size == address {
                current.size += size;
                if current.address + current.size == node.address {
                    // merge with next node
                    current.size += node.size;
                    current.next = node.next.take();
                    break;
                }
            } else {
                if address + size == node.address {
                    // extend next node to the left
                    node.address = address;
                    node.size += size;
                    break;
                }
                current.next = Some(Box::new(Node::new(address, size, current.next.take())));
            }

            break;
        }

        if current.next.is_none() {
            if current.address + current.size == address {
                // extend current
                current.size += size;
            } else {
                current.next = Some(Box::new(Node::new(address, size, current.next.take())));
            }
        }

        self.head.size += size;
    }

    pub fn cut(&mut self, address: usize, size: usize) {
        let mut current = &mut self.head;

        while let Some(ref mut node) = current.next {
            if address == node.address {
                if node.size == size {
                    current.next = node.next.take();
                } else {
                    node.address = address + size;
                    node.size -= size;
                }
            } else if address + size == node.address + node.size {
                node.size -= size;
            } else if address > node.address && address < node.address + node.size {
                let r_address = address + size;
                let r_size = node.address + node.size - r_address;
                node.next = Some(Box::new(Node::new(r_address, r_size, node.next.take())));
                node.size = address - node.address;
            } else {
                current = current.next.as_mut().unwrap();
                continue;
            }
            break;
        }
    }

    pub fn log(&mut self) {
        let mut current = &mut self.head;

        while let Some(ref mut node) = current.next {
            log_debug!("memory.map.node = 0x{:016x}, 0x{:016x}", node.address, node.size);
            current = current.next.as_mut().unwrap();
        }
    }
}

