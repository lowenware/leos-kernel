//
// memory.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

mod heap;
mod map;
pub mod lock;

use arch::memory;
use crate::arch;
use crate::platform;
use heap::Heap;
use lock::Locked;
use map::Map;

pub use arch::memory::*;

pub const KERNEL_BASE: usize = 0xffffff80_00000000;
pub const PERIPHERALS_BASE: usize = 0xffffffff_c0000000;

const HEAP_SIZE: usize = 2 * 1024 * 1024;

// ------------------------------------------------------------------------------------------------

// second megabyte for heap
// const HEAP_BASE: usize = 0xfffffff0_00100000;
// const HEAP_PAGES: usize = 256;
// TODO: combine using MMU constants
// const HEAP_FLAGS: usize = 0x00600000_00000703;

// ------------------------------------------------------------------------------------------------

// Physical memory map
static MAP: Locked<Map> = Locked::new(Map::new());

// Kernel heap memory mapping
#[global_allocator]
static ALLOCATOR: Locked<Heap> = Locked::new(Heap::new());

pub fn init(kernel_base: usize, kernel_size: usize) {
    // initialize the heap
    let heap_base = kernel_base + kernel_size;
    let mut kernel_space = arch::memory::kernel();
    // map to virtual address
    let heap = kernel_space.map_L2(heap_base, memory::KERNEL_DATA) + KERNEL_BASE;

    unsafe {
        // initialize heap
        ALLOCATOR.lock().init(heap as usize, HEAP_SIZE);
    }
    // initialize physical memory map
    let mut memory_map = MAP.lock();
    memory_map.add(platform::MEMORY_BASE, platform::MEMORY_SIZE);

    // delete from the map memory used by the kernel
    memory_map.cut(kernel_base, kernel_size + HEAP_SIZE);
}

pub fn map(size: usize) -> Result<usize, ()> {
    MAP.lock().get(size)
}

pub fn cut(base: usize, size: usize) {
    MAP.lock().cut(base, size);
}

#[alloc_error_handler]
fn on_alloc_error(layout: alloc::alloc::Layout) -> ! {
    panic!("memory allocation failed: {:?}", layout)
}

#[no_mangle]
#[inline(never)]
pub unsafe fn zero_volatile(start: *mut usize, end: *mut usize) {
    let mut ptr = start;

    while ptr < end {
        core::ptr::write_volatile(ptr, 0);
        ptr = ptr.offset(1);
    }
}
