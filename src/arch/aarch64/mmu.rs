//
// mmu.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use crate::memory;

// Memory Options
pub const UXN: usize = 1 << 54;
pub const PXN: usize = 1 << 53;
pub const ACCESS_FLAG: usize = 1 << 10;
pub const OUTER_SHAREABLE: usize = 2 << 8;
pub const INNER_SHAREABLE: usize = 3 << 8;
// pub const READ_ONLY: usize = 1 << 9;
// pub const USER_SPACE: usize = 1 << 8;

// MAIR attributes
pub const DATA: usize = 0 << 2;
// pub const CODE: usize = 0 << 2;
pub const DEVICE: usize = 1 << 2;

// entry types
pub const BLOCK: usize = 1;
pub const TABLE: usize = 3;

pub const ADDRESS_MASK: usize = 0x0000FFFF_FFFFF000;
pub const VADDRESS_MASK: usize = 0x0000007F_FFFFF000;

// granularity aftermath
pub const PAGE_BITS: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_BITS;
pub const PAGE_MASK: usize = PAGE_SIZE - 1;

pub const BLOCK_L1_BITS: usize = 30;
pub const BLOCK_L1_SIZE: usize = 1 << BLOCK_L1_BITS;
pub const BLOCK_L1_MASK: usize = BLOCK_L1_SIZE - 1;

pub const BLOCK_L2_BITS: usize = 21;
pub const BLOCK_L2_SIZE: usize = 1 << BLOCK_L2_BITS;
pub const BLOCK_L2_MASK: usize = BLOCK_L2_SIZE - 1;

pub const INDEX_BITS: usize = 9;
pub const INDEX_SIZE: usize = 1 << INDEX_BITS;
// pub const INDEX_MASK: usize = INDEX_SIZE - 1;

// pub const INDEX_MASK_L3: usize = INDEX_MASK << PAGE_BITS;
// pub const INDEX_MASK_L2: usize = INDEX_MASK_L3 << INDEX_BITS;
// pub const INDEX_MASK_L1: usize = INDEX_MASK_L2 << INDEX_BITS;

const IDENTITY_FLAGS: usize =
    UXN |
    ACCESS_FLAG |
    INNER_SHAREABLE |
    DATA |
    BLOCK;

const PERIPHERALS_FLAGS: usize =
    UXN |
    PXN |
    ACCESS_FLAG |
    OUTER_SHAREABLE |
    DEVICE;


#[allow(non_snake_case)]
fn index_L1(v_addr: usize) -> usize {
    (v_addr & VADDRESS_MASK) >> (2 * INDEX_BITS + PAGE_BITS)
}

#[allow(non_snake_case)]
fn index_L2(v_addr: usize) -> usize {
    (v_addr & VADDRESS_MASK) >> (INDEX_BITS + PAGE_BITS)
}

#[allow(non_snake_case)]
fn index_L3(v_addr: usize) -> usize {
    (v_addr & VADDRESS_MASK) >> PAGE_BITS
}


pub struct TranslationTable {
    entry: [usize; INDEX_SIZE],
}

impl TranslationTable {
    pub fn set(&mut self, index: usize, address: usize, flags: usize) {
        self.entry[index] = address | flags;
    }

//    pub fn get(&self, index: usize) -> usize {
//        self.entry[index]
//    }

    pub fn entry(&self, index: usize) -> *mut TranslationTable {
        (self.entry[index] & ADDRESS_MASK) as *mut TranslationTable
    }

    pub fn translate(&mut self, index: usize, flags: usize) -> *mut TranslationTable {
        let mut entry = self.entry[index];
        if entry == 0 {
            if let Ok(address) = memory::map(PAGE_SIZE) {
                let mut table = address as *mut TranslationTable;
                for i in 0..INDEX_SIZE {
                    unsafe {
                        (*table).entry[i] = 0;
                    }
                }
                entry = address | ACCESS_FLAG | flags;
            } else {
                panic!("Not enough memory to allocate translation table");
            }
            self.entry[index] = entry;
        }
        (entry & ADDRESS_MASK) as *mut TranslationTable
    }

    pub fn last(&self, offset: usize) -> usize {
        // TODO: use OS-specific flag to mark entries as completely occupied
        let mut result = 0;
        for i in 0..INDEX_SIZE {
            let entry = self.entry[i];
            if entry != 0 {
                if entry & TABLE == TABLE {
                    result = i;
                }
            } else {
                break;
            }
        }
        log_debug!("last: {}, 0x{:016x}", result, result << offset);
        result << offset
    }
}

pub struct AddressSpace {
    base: *mut TranslationTable,
}

impl AddressSpace {

    pub const fn new(ttbr: usize) -> Self {
        Self { base: (ttbr & ADDRESS_MASK) as *mut TranslationTable }
    }

//    pub fn base(&self) -> usize {
//        self.base as usize
//    }

    #[allow(non_snake_case)]
    fn table_L1(&mut self) -> &mut TranslationTable {
        unsafe {
            &mut *self.base
        }
    }

    #[allow(non_snake_case)]
    fn table_L2(&mut self, v_addr: usize, flags: usize) -> &mut TranslationTable {
        unsafe {
            &mut *(self.table_L1().translate(index_L1(v_addr), flags))
        }
    }

    #[allow(non_snake_case)]
    fn table_L3(&mut self, v_addr: usize, _flags: usize) -> &mut TranslationTable {
        unsafe {
            &mut *(self.table_L2(v_addr, TABLE).entry(index_L2(v_addr)))
        }
    }

    #[inline(never)]
    #[allow(non_snake_case)]
    pub fn assign_L1(&mut self, p_addr: usize, v_addr: usize, flags: usize) {
        self.table_L1().set(index_L1(v_addr), p_addr, flags);
    }

    #[allow(non_snake_case)]
    pub fn assign_L2(&mut self, p_addr: usize, v_addr: usize, flags: usize) {
        self.table_L2(v_addr, TABLE).set(index_L2(v_addr), p_addr, flags);
    }

    #[allow(non_snake_case)]
    pub fn assign_L3(&mut self, p_addr: usize, v_addr: usize, flags: usize) {
        self.table_L3(v_addr, TABLE).set(index_L3(v_addr), p_addr, flags);
    }

//    #[allow(non_snake_case)]
//    pub fn next_L1(&mut self) -> usize {
//        self.table_L1().last(2 * INDEX_BITS + PAGE_BITS) + BLOCK_L1_SIZE
//    }

    #[inline(never)]
    #[allow(non_snake_case)]
    pub fn next_L2(&mut self) -> usize{
        let v_addr = self.table_L1().last(2 * INDEX_BITS + PAGE_BITS);
        v_addr + self.table_L2(v_addr, TABLE).last(INDEX_BITS + PAGE_BITS) + BLOCK_L2_SIZE
    }

    #[allow(non_snake_case)]
    pub fn next_L3(&mut self) -> usize {
        let mut v_addr = self.table_L1().last(2 * INDEX_BITS + PAGE_BITS);
        v_addr += self.table_L2(v_addr, TABLE).last(INDEX_BITS + PAGE_BITS);
        v_addr + self.table_L3(v_addr, TABLE).last(PAGE_BITS) + PAGE_SIZE
    }

//    #[allow(non_snake_case)]
//    pub fn map_L1(&mut self, p_addr: usize, flags: usize) -> usize {
//        let v_addr = self.next_L1();
//        self.assign_L1(p_addr, v_addr, flags | BLOCK);
//        v_addr
//    }

    #[inline(never)]
    #[allow(non_snake_case)]
    pub fn map_L2(&mut self, p_addr: usize, flags: usize) -> usize {
        let v_addr = self.next_L2();
        self.assign_L2(p_addr, v_addr, flags | BLOCK);
        v_addr
    }

    #[allow(non_snake_case)]
    pub fn map_L3(&mut self, p_addr: usize, flags: usize) -> usize {
        let v_addr = self.next_L3();
        log_debug!("- map to 0x{:016x}", v_addr);
        self.assign_L3(p_addr, v_addr, flags | TABLE);
        v_addr
    }

    #[inline(never)]
    pub fn identity(&mut self, base: usize, size: usize) {
        if size % BLOCK_L1_SIZE == 0 && base & BLOCK_L1_MASK == 0 {
            let mut addr = base;
            while addr != size {
                self.assign_L1(addr, addr, IDENTITY_FLAGS);
                addr += BLOCK_L1_SIZE;
            }
            return
        }

        panic!("Don't know, how to map identity @{} : {}", base, size);
    }

    #[inline(never)]
    pub fn peripherals(&mut self, p_addr: usize, v_addr: usize, size: usize) {
        let mut pa = p_addr;
        let mut va = v_addr;
        let end = p_addr + size;
        if size % BLOCK_L1_SIZE == 0 && p_addr & BLOCK_L1_MASK == 0 {
            while pa != end {
                self.assign_L1(pa, va, PERIPHERALS_FLAGS | BLOCK);
                pa += BLOCK_L1_SIZE;
                va += BLOCK_L1_SIZE;
            }
            return
        }

        if size % BLOCK_L2_SIZE == 0 && p_addr & BLOCK_L2_MASK == 0 {
            while pa != end {
                self.assign_L2(pa, va, PERIPHERALS_FLAGS | BLOCK);
                pa += BLOCK_L2_SIZE;
                va += BLOCK_L2_SIZE;
            }
            return
        }

        if size % PAGE_SIZE == 0 && p_addr & PAGE_MASK == 0 {
            while pa != end {
                self.assign_L3(pa, va, PERIPHERALS_FLAGS | TABLE);
                pa += PAGE_SIZE;
                va += PAGE_SIZE;
            }
            return
        }

        panic!("Don't know, how to map peripherals @{} : {}", p_addr, size);
    }
}

