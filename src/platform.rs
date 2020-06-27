//
// platform.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

// ------------------------------------------------------------------------------------------------

#[cfg(platform = "qemu")]
mod qemu;

#[cfg(platform = "qemu")]
pub use qemu::*;

// ------------------------------------------------------------------------------------------------

#[cfg(platform = "raspi4")]
mod raspi4;

#[cfg(platform = "raspi4")]
pub use raspi4::*;

// ------------------------------------------------------------------------------------------------

#[cfg(platform = "pinebookpro")]
mod pinebookpro;

#[cfg(platform = "pinebookpro")]
pub use pinebookpro::*;

// ------------------------------------------------------------------------------------------------
