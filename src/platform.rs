//
// board.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//

// ------------------------------------------------------------------------------------------------

#[cfg(board = "qemu")]
mod qemu;

#[cfg(board = "qemu")]
pub use qemu::*;

// ------------------------------------------------------------------------------------------------

#[cfg(board = "raspi4")]
mod raspi4;

#[cfg(board = "raspi4")]
pub use raspi4::*;

// ------------------------------------------------------------------------------------------------

#[cfg(board = "pinebookpro")]
mod pinebookpro;

#[cfg(board = "pinebookpro")]
pub use pinebookpro::*;

// ------------------------------------------------------------------------------------------------
