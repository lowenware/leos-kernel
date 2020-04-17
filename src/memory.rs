//
// memory.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

mod heap;
mod map;
pub mod lock;

use crate::arch::memory;
use crate::board;
use heap::Heap;
use lock::Locked;
use map::Map;

pub const PERIPHERALS_BASE: usize = 0xffffffff_c0000000;

pub const KERNEL_BASE: usize = 0xffffff80_00000000;
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
    // unused memory REAL base
    let heap_real = board::heap(kernel_base, kernel_size);
    // initialize memory mapping
    memory::init();
    // map identity paging
    memory::identity().identity(board::MEMORY_BASE, board::MEMORY_SIZE);

    let mut kernel_space = memory::kernel();
    // map peripherals memory
    kernel_space.peripherals(board::PERIPHERALS_REAL, PERIPHERALS_BASE, board::PERIPHERALS_SIZE);

    log_debug!("memory.heap.real = 0x{:016x}", heap_real);
    // map area for heap
    let heap_virt = kernel_space.map_L2(heap_real, memory::KERNEL_DATA) + KERNEL_BASE;

    log_debug!("memory.heap.virtual = 0x{:016x}", heap_virt);
    log_debug!("memory.heap.size = 0x{:016x}", HEAP_SIZE);

    unsafe {
        // initialize heap
        ALLOCATOR.lock().init(heap_virt as usize, HEAP_SIZE);
    }
    // initialize physical memory map
    let mut memory_map = MAP.lock();
    log_debug!("memory.map = 0x{:016x}, 0x{:016x}", board::MEMORY_BASE, board::MEMORY_SIZE);
    memory_map.add(board::MEMORY_BASE, board::MEMORY_SIZE);
    log_debug!("complete memory map");
    memory_map.log();

    // delete from the map memory used by the kernel
    log_debug!("cut kernel space: 0x{:016x}, 0x{:016x}", kernel_base, kernel_size);
    memory_map.cut(kernel_base, kernel_size);
    memory_map.log();
    // delete from the map memory used by the heap
    log_debug!("cut heap: 0x{:016x}, 0x{:016x}", heap_real, HEAP_SIZE);
    memory_map.cut(heap_real, HEAP_SIZE);
    memory_map.log();
    // delete from the map memory used for peripherals (if in the middle of the region)
    log_debug!("cut peripherals space");
    memory_map.cut(board::PERIPHERALS_REAL, board::PERIPHERALS_SIZE);

    memory_map.log();
}

pub fn map(size: usize) -> Result<usize, ()> {
    MAP.lock().get(size)
}

#[alloc_error_handler]
fn on_alloc_error(layout: alloc::alloc::Layout) -> ! {
    panic!("memory allocation failed: {:?}", layout)
}

