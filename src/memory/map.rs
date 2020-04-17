//
// map.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

use crate::arch;
use crate::board;

const U64_SHIFT: u8 = 6; // 1 << U64_SHIFT == 64
const MAP_SIZE: usize = board::MEMORY_SIZE >> (arch::PAGE_SHIFT + U64_SHIFT);

pub struct Map {
    m: [u64; MAP_SIZE],  // map
    i: usize,            // index
    b: usize,            // bit
}

impl Map {
    pub const fn empty() -> Map {
        Map{m: [0; MAP_SIZE], i: 0, b: 0}
    }

    pub fn set(&mut self, index: usize) {
        let i = index / 64;
        let b = index % 64;
        if i < MAP_SIZE {
            self.m[i] |= 1 << b;
        }
    }

    pub fn find(&mut self, index: &mut usize) -> bool {
        let mut b = self.b;
        for i in self.i..MAP_SIZE {
            while b < 64 {
                if self.m[i] & (1 << b) == 0 {
                    self.m[i] |= 1 << b;
                    self.i = i;
                    self.b = b + 1;
                    *index = i * 64 + b;
                    return true;
                }
                b += 1;
            }
            b = 0;
        }
        return false;
    }

    pub fn clear(&mut self, index: usize) {
        let i = index / 64;
        let b = index % 64;
        if i < MAP_SIZE {
            self.m[i] &= !(1 << b);
            if i <= self.i {
                self.i = i;
                self.b = b;
            }
        }
    }
}
