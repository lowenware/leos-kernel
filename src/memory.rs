//
// memory.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

mod heap;
mod map;

use crate::{arch, board};
use crate::arch::mmu;
use map::Map;
use heap::{Heap, Locked};

// first megabyte for kernel
// second megabyte for heap
const HEAP_BASE: u64 = 0xfffffff0_00100000;
const HEAP_PAGES: usize = 256;
// todo: combine using MMU constants
const HEAP_FLAGS: u64 = 0x00600000_00000703;
const HEAP_SIZE: usize = HEAP_PAGES * (arch::PAGE_SIZE as usize);

static mut MAP: Map = Map::empty();

#[global_allocator]
static ALLOCATOR: Locked<Heap> = Locked::new(Heap::new());


fn address_to_map_index(address: u64) -> usize {
    let mut result: usize;
    if address >= board::MEMORY_BASE {
        result = (address - board::MEMORY_BASE) as usize;
        if result < board::MEMORY_SIZE {
            result = result >> arch::PAGE_SHIFT;
            return result;
        }
    }
    panic!("Memory: address out of range -> 0x{:016x}", address);
}

pub fn init(kernel_base: u64, kernel_size: usize, ttbr1_base: u64) {

    // lock kernel and heap pages in memory map
    let mut addr = kernel_base;
    let pages = (kernel_size >> arch::PAGE_SHIFT) + HEAP_PAGES;

    for _i in 0..pages {
        lock(addr);
        addr += arch::PAGE_SIZE as u64;
    }

    mmu::init(kernel_base, ttbr1_base);
    // TODO: full MMU implementation, just a workaround to map 1MB of heap for now
    mmu::translate(kernel_base + kernel_size as u64, HEAP_BASE, HEAP_PAGES, HEAP_FLAGS);
    log_state!("heap: {:#018x}: {}KB", HEAP_BASE, HEAP_SIZE >> 10);
    unsafe {
        ALLOCATOR.lock().init(HEAP_BASE as usize, HEAP_SIZE);
    }
}

pub fn lock(address: u64) {
    unsafe {
        MAP.set(address_to_map_index(address));
    }
}

pub fn get_page() -> u64 {
    let mut index: usize = 0;
    let res: bool;

    unsafe {
        res = MAP.find(&mut index);
    }

    if res {
        return board::MEMORY_BASE + (index << arch::PAGE_SHIFT) as u64;
    }
    // returning 0 is safe here, because zero page should not be ever allocated by the map
    return 0;
}

pub fn free_page(address: u64) {
    unsafe {
        MAP.clear(address_to_map_index(address));
    }
}

pub fn log_heap() {
    ALLOCATOR.lock().describe();
}

#[alloc_error_handler]
fn on_alloc_error(layout: alloc::alloc::Layout) -> ! {
    panic!("memory allocation failed: {:?}", layout)
}
