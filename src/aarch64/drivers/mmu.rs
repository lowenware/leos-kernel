//
// mmu.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use crate::{memory, platform};
use crate::arch::cpu;
use crate::arch::registers::{TTBR0_EL1, TTBR1_EL1, TCR_EL1, SCTLR_EL1, MAIR_EL1};

// Memory Options
pub const UXN: usize = 1 << 54;
pub const PXN: usize = 1 << 53;
pub const ACCESS_FLAG: usize = 1 << 10;
pub const OUTER_SHAREABLE: usize = 2 << 8;
pub const INNER_SHAREABLE: usize = 3 << 8;
pub const READ_ONLY: usize = 1 << 9;
// pub const USER_SPACE: usize = 1 << 8;

// entry types
pub const BLOCK: usize = 1;
pub const TABLE: usize = 3;

pub const ADDRESS_MASK: usize = 0x0000FFFF_FFFFF000;
pub const VADDRESS_MASK: usize = 0x0000007F_FFFFF000;

pub const TRANSLATION_LEVELS: usize = 3;

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

pub const ADDRESS_BITS: usize = PAGE_BITS + TRANSLATION_LEVELS * INDEX_BITS;

pub const KERNEL_BASE: usize = !(VADDRESS_MASK | PAGE_MASK);

pub const STACK_PAGES: usize = 4;

// MAIR indices
pub const DEVICE_ATTR: usize = 1;
pub const MEMORY_ATTR: usize = 0;


static mut TTBR0_IDENTITY: u64 = 0;

// pub const INDEX_MASK: usize = INDEX_SIZE - 1;

// pub const INDEX_MASK_L3: usize = INDEX_MASK << PAGE_BITS;
// pub const INDEX_MASK_L2: usize = INDEX_MASK_L3 << INDEX_BITS;
// pub const INDEX_MASK_L1: usize = INDEX_MASK_L2 << INDEX_BITS;

const IDENTITY_FLAGS: usize = UXN | ACCESS_FLAG | INNER_SHAREABLE | MEMORY_ATTR | BLOCK;

pub const PERIPHERALS_FLAGS: usize = UXN | PXN | ACCESS_FLAG | OUTER_SHAREABLE | DEVICE_ATTR;

const KERNEL_CODE: usize = UXN | ACCESS_FLAG | INNER_SHAREABLE | READ_ONLY | MEMORY_ATTR;

const KERNEL_RODATA: usize = UXN | PXN | ACCESS_FLAG | INNER_SHAREABLE | READ_ONLY | MEMORY_ATTR;

const KERNEL_DATA: usize = UXN | PXN | ACCESS_FLAG | INNER_SHAREABLE | MEMORY_ATTR;

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

/// Initialiaze MMU
///
/// # Safety
/// - User must ensure that proper HW is selected
///
#[naked]
#[inline(never)]
pub unsafe fn init() -> (usize, usize) {
    // set up MMU:
    let kernel_base: usize;
    let data_base: usize;
    let rodata_base: usize;
    let kernel_end: usize;
    let mut address = platform::MEMORY_BASE;
    let mut start = index_L1(address);
    let mut end = index_L1(platform::MEMORY_BASE + platform::MEMORY_SIZE);

    // setup translation controls
    TCR_EL1.write(
        TCR_EL1::IPS_36BIT
        | TCR_EL1::TG1_4KB
        | TCR_EL1::SH1_INNER_SHAREABLE
        | TCR_EL1::ORGN1_NORMAL_OUTER
        | TCR_EL1::IRGN1_NORMAL_INNER
        | TCR_EL1::T1SZ(64 - ADDRESS_BITS as u64)
        | TCR_EL1::TG0_4KB
        | TCR_EL1::SH0_INNER_SHAREABLE
        | TCR_EL1::ORGN0_NORMAL_OUTER
        | TCR_EL1::IRGN0_NORMAL_INNER
        | TCR_EL1::T0SZ(64 - ADDRESS_BITS as u64)
    );

    // setup memory attributes in MAIR
    MAIR_EL1.write(
        MAIR_EL1::attr(DEVICE_ATTR, MAIR_EL1::device::nGnRE)
        | MAIR_EL1::attr(MEMORY_ATTR, MAIR_EL1::normal::inner::WriteBackNonTransient
            | MAIR_EL1::normal::inner::ReadAllocate |  MAIR_EL1::normal::inner::WriteAllocate
            | MAIR_EL1::normal::outer::WriteBackNonTransient
            | MAIR_EL1::normal::outer::ReadAllocate |  MAIR_EL1::normal::outer::WriteAllocate
        ),
    );

    llvm_asm!("adr $0, __kernel_base" : "=r"(kernel_base) ::: "volatile");
    llvm_asm!("adr $0, __rodata_base" : "=r"(rodata_base) ::: "volatile");
    llvm_asm!("adr $0, __data_base" : "=r"(data_base) ::: "volatile");
    llvm_asm!("adr $0, __kernel_end" : "=r"(kernel_end) ::: "volatile");

    // save identity paging translation table address
    TTBR0_IDENTITY = kernel_end as u64;

    // set up identity mapping
    TTBR0_EL1.write(kernel_end as u64 | TTBR0_EL1::CNP);

    // prepare identity map
    let mut table = &mut *(kernel_end as *mut TranslationTable);
    for i in start..end {
        table.set(i, address, IDENTITY_FLAGS);
        address += BLOCK_L1_SIZE;
    }

    // prepare L1 for higher addresses
    address = kernel_end + PAGE_SIZE;

    TTBR1_EL1.write(address as u64 | TTBR0_EL1::CNP);

    table = &mut *(address as *mut TranslationTable);
    address += PAGE_SIZE;
    table.set(0, address, KERNEL_DATA | TABLE);

    // prepare L2 for higher addresses
    table = &mut *(address as *mut TranslationTable);
    address += PAGE_SIZE;
    table.set(0, address, KERNEL_DATA | TABLE);

    // prepare L3 for higher addresses
    table = &mut *(address as *mut TranslationTable);
    end = (kernel_end - kernel_base) >> PAGE_BITS;
    if end > 512 {
        cpu::hang();
    }
    address = kernel_base;

    let mut flags = KERNEL_CODE;

    for i in 0..end {
        if address >= data_base {
            flags = KERNEL_DATA;
        } else if address >= rodata_base {
            flags = KERNEL_RODATA;
        }
        table.set(i, address, flags | TABLE);
        address += PAGE_SIZE;
    }


    // skip translation tables
    address += 4 * PAGE_SIZE;

    // as soon as data section is the lowest one, flags can stay unchanged
    // flags = KERNEL_DATA;
    // map one empty page between data and stack to trigger an exception on stack overflow
    table.set(end, 0xBADC0FFEE, flags);
    start = end + 1;
    end = start + STACK_PAGES;
    for i in start..end {
        table.set(i, address, flags | TABLE);
        address += PAGE_SIZE;
    }

    // set up stack
    cpu::sp::write(KERNEL_BASE | (end << PAGE_BITS));

    // set stack pointer to virtual address
    cpu::lr::write(cpu::lr::read() - kernel_base + KERNEL_BASE);

    // enable MMU
    SCTLR_EL1.set(SCTLR_EL1::M);

    cpu::isb();

    (kernel_base, address - kernel_base)
}

pub fn identity() {
    unsafe {
        // enable identity
        TTBR0_EL1.write(TTBR0_IDENTITY);
    }
}
