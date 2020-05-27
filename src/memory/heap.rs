//
// heap.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use alloc::alloc::{GlobalAlloc, Layout};
use core::{mem, ptr};
use super::lock::Locked;

struct HeapNode {
    next: Option<&'static mut HeapNode>,
    size: usize,
}

impl HeapNode {
    const fn empty() -> Self {
        HeapNode { next: None, size: 0 }
    }

    fn new(base: usize, size: usize, next: Option<&'static mut HeapNode>) -> &mut HeapNode {
        let p_node = base as *mut HeapNode;
        unsafe {
            (*p_node).size = size;
            (*p_node).next = next;
            &mut *p_node
        }
    }

    fn get_base(&self) -> usize {
        self as *const Self as usize
    }

    fn get_tail(&self) -> usize {
        self.get_base() + self.size
    }

    fn set_size(&mut self, size: usize) {
        self.size = size;
    }
}

pub struct Heap {
    head: HeapNode,
}

impl Heap {

    pub const fn new() -> Self {
        Self {
            head: HeapNode::empty()
        }
    }

    fn align_to(value: usize, align: usize) -> usize {
        let mask = align - 1;
        (value + mask) & !mask
    }

    pub unsafe fn init(&mut self, heap_base: usize, heap_size: usize) {
        assert!(Heap::align_to(heap_base, mem::align_of::<HeapNode>()) == heap_base);
        self.head.next = Some(HeapNode::new(heap_base, heap_size, None))
    }

    fn adjust_size(size: usize) -> usize {
        if size < mem::size_of::<HeapNode>() {
            mem::size_of::<HeapNode>()
        } else {
            Heap::align_to(size, mem::align_of::<HeapNode>())
        }
    }

    fn adjust_layout(layout: Layout) -> (usize, usize) {
        let align = layout
            .align_to(mem::align_of::<HeapNode>())
            .expect("could not adjust alignment")
            .pad_to_align()
            .align();

        let size = Heap::adjust_size(layout.size());

        (size, align)
    }

    fn try_allocate(node: &mut HeapNode, size: usize, align: usize) ->
        Result<usize, ()> {

        let node_base = node.get_base();
        let mut alloc_base = Heap::align_to(node_base, align);

        if alloc_base != node_base {
            alloc_base = Heap::align_to(node_base + mem::size_of::<HeapNode>(), align);
        }
        let alloc_tail = alloc_base + size;

        if alloc_tail > node.get_tail() {
            return Err(())
        }

        let remains = node.get_tail() - alloc_tail;

        if remains > 0 {
            if remains < mem::size_of::<HeapNode>() {
                return Err(())
            }
            // insert new heap node after current one
            node.next = Some(HeapNode::new(alloc_tail, remains, node.next.take()));
        }

        if alloc_base != node.get_base() {
            // current node stays, but with smaller size
            node.set_size(alloc_base - node_base);
        } else {
            // there is no node here anymore
            node.size = 0;
        }

        Ok(alloc_base)
    }

    fn pick(&mut self, size: usize, align: usize) -> Result <usize, ()> {
       let mut current = &mut self.head;

        while let Some(ref mut node) = current.next {
            if let Ok(alloc_base) = Self::try_allocate(node, size, align) {
                if node.size == 0 {
                    current.next = node.next.take();
                }
                return Ok(alloc_base)
            }

            current = current.next.as_mut().unwrap();
        }

        Err(())
    }

    unsafe fn put(&mut self, node_base: usize, size: usize) {
        assert!(Heap::align_to(node_base, mem::align_of::<HeapNode>()) == node_base);
        assert!(size >= mem::size_of::<HeapNode>());

        let mut current = &mut self.head;
        let node_tail = node_base + size;

        while let Some(ref mut next_node) = current.next {
            let next_base = next_node.get_base();
            if node_tail == next_base {
                current.next = Some(
                    HeapNode::new(node_base, size + next_node.size, next_node.next.take())
                );
                return
            } else if node_base < next_base {
                break;
            }
            current = current.next.as_mut().unwrap();
        }
        current.next = Some(HeapNode::new(node_base, size, current.next.take()));
    }

//    pub fn describe(&mut self) {
//        let mut current = &mut self.head;
//        while let Some(ref mut node) = current.next {
//            log_debug!("HeapNode {:#018x}:{}B", node.get_base(), node.size);
//            current = current.next.as_mut().unwrap();
//        }
//    }
}

// ------------------------------------------------------------------------------------------------

unsafe impl GlobalAlloc for Locked<Heap> {

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        log_debug!("heap: allocate {:?}", layout);
        let (size, align) = Heap::adjust_layout(layout);
        let mut heap = self.lock();

        if let Ok(alloc_base) = heap.pick(size, align) {
            return alloc_base as *mut u8
        }

        ptr::null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        log_debug!("heap: deallocate {:?}", layout);
        let size = Heap::adjust_size(layout.size());
        self.lock().put(ptr as usize, size);
    }
}


