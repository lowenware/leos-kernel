//
// cntp_ctl_el0.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#![allow(non_snake_case)]
#![allow(dead_code)]
pub mod CNTP_CTL_EL0 {
    /// The status of the timer, indicating if timer condition is met
    ///
    /// 0b0 - Timer condition is not met,
    /// 0b1 - TImer condition is met;
    pub const ISTATUS: u64 = 1 << 2;

    /// Timer interrupt mask bit
    ///
    /// 0b0 - Timer interrupt is not masked
    /// 0b1 - Timer interrupt is masked
    pub const IMASK: u64 = 1 << 1;

    /// Enables the Timer
    ///
    /// 0b0 - Timer is enabled
    /// 0b1 - Timer is disabled
    pub const ENABLE: u64 = 1 << 0;

    pub struct Register{}

    impl Register {
        aarch64_sysreg_read!(u64, "CNTP_CTL_EL0");
        aarch64_sysreg_write!(u64, "CNTP_CTL_EL0");
        aarch64_sysreg_set!(u64, "CNTP_CTL_EL0");
        aarch64_sysreg_get!(u64, "CNTP_CTL_EL0");
        aarch64_sysreg_clear!(u64, "CNTP_CTL_EL0");
        aarch64_sysreg_is_set!(u64, "CNTP_CTL_EL0");
        aarch64_sysreg_has!(u64, "CNTP_CTL_EL0");
    }
}

pub static CNTP_CTL_EL0: CNTP_CTL_EL0::Register = CNTP_CTL_EL0::Register{};
