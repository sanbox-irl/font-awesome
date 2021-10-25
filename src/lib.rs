#![no_std]
#![doc = include_str!("../README.md")]

/// All const char points.
pub mod chars;
/// All consts char points re-exported as
/// single character string literals.
pub mod strs;

pub use chars::*;

pub const MINIMUM_CODEPOINT: usize = 0xe005;
pub const MAXIMUM_CODEPOINT: usize = 0xf8ff;
