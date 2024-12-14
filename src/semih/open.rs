#![allow(missing_docs)]

/// Values for the mode parameter of the OPEN syscall.

/// Mode corresponding to fopen "r" mode.
pub const R: usize = 0;
/// Mode corresponding to fopen "rb" mode.
pub const R_BINARY: usize = 1;
/// Mode corresponding to fopen "r+" mode.
pub const RW: usize = 2;
/// Mode corresponding to fopen "r+b" mode.
pub const RW_BINARY: usize = 3;
/// Mode corresponding to fopen "w" mode.
pub const W_TRUNC: usize = 4;
/// Mode corresponding to fopen "wb" mode.
pub const W_TRUNC_BINARY: usize = 5;
/// Mode corresponding to fopen "w+" mode.
pub const RW_TRUNC: usize = 6;
/// Mode corresponding to fopen "w+b" mode.
pub const RW_TRUNC_BINARY: usize = 7;
/// Mode corresponding to fopen "a" mode.
pub const W_APPEND: usize = 8;
/// Mode corresponding to fopen "ab" mode.
pub const W_APPEND_BINARY: usize = 9;
/// Mode corresponding to fopen "a+" mode.
pub const RW_APPEND: usize = 10;
/// Mode corresponding to fopen "a+b" mode.
pub const RW_APPEND_BINARY: usize = 11;
