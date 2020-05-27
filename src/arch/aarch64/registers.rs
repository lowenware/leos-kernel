//
// registers.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

#[macro_use]
mod accessors;

mod cntfrq_el0;
mod cntp_ctl_el0;
mod cntp_tval_el0;
mod cntpct_el0;
mod ttbr0_el0;
mod ttbr0_el1;
mod ttbr1_el1;

pub use self::cntfrq_el0::CNTFRQ_EL0;
pub use self::cntp_ctl_el0::CNTP_CTL_EL0;
pub use self::cntp_tval_el0::CNTP_TVAL_EL0;
pub use self::cntpct_el0::CNTPCT_EL0;
pub use self::ttbr0_el0::TTBR0_EL0;
pub use self::ttbr0_el1::TTBR0_EL1;
pub use self::ttbr1_el1::TTBR1_EL1;
