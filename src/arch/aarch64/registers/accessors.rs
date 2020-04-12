//
// accessors.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

/// Read value from system register
macro_rules! aarch64_sysreg_read {
    ($sysreg_size:ty, $sysreg_name:tt) => {
        #[inline]
        pub fn read(&self) -> $sysreg_size {
            let value;
            unsafe {
                asm!(concat!("mrs $0, ", $sysreg_name) : "=r"(value) ::: "volatile");
            }
            return value
        }
    };
}

/// Write value to a system register
macro_rules! aarch64_sysreg_write {
    ($sysreg_size:ty, $sysreg_name:tt) => {
        #[inline]
        pub fn write(&self, value: $sysreg_size) {
            unsafe {
                asm!(concat!("msr ", $sysreg_name, ", $0") :: "r"(value) ::: "volatile");
            }
        }
    };
}

/// Set bits in a system register
macro_rules! aarch64_sysreg_set {
    ($sysreg_size:ty, $sysreg_name:tt) => {
        #[inline]
        pub fn set(&self, fields: $sysreg_size) {
            self.write(self.read() | fields);
        }
    };
}

/// Get bits from a system register
macro_rules! aarch64_sysreg_get {
    ($sysreg_size:ty, $sysreg_name:tt) => {
        #[inline]
        pub fn get(&self, fields: $sysreg_size) -> $sysreg_size {
            return self.read() & fields;
        }
    };
}

/// Clear bits in a system register
macro_rules! aarch64_sysreg_clear {
    ($sysreg_size:ty, $sysreg_name:tt) => {
        #[inline]
        pub fn clear(&self, fields: $sysreg_size) {
            return self.write(self.read() & !fields);
        }
    };
}

/// Check if register specified bit(s) is set
macro_rules! aarch64_sysreg_is_set {
    ($sysreg_size:ty, $sysreg_name:tt) => {
        #[inline]
        pub fn is_set(&self, fields: $sysreg_size) -> bool {
            return (self.read() & fields) == fields;
        }
    };
}

/// Check if register has any of specified bits set
macro_rules! aarch64_sysreg_has {
    ($sysreg_size:ty, $sysreg_name:tt) => {
        #[inline]
        pub fn has(&self, fields: $sysreg_size) -> bool {
            return (self.read() & fields) != 0;
        }
    };
}

