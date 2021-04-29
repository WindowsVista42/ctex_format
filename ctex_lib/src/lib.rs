#![feature(num_as_ne_bytes)]
#![feature(stdsimd)]
#![allow(clippy::identity_op)]

use crate::flags::Flags;

pub mod decode;
pub mod encode;
pub mod util;
pub mod flags;

mod tests;

pub const SECTOR_SIZE: usize = 64;

pub struct CtexImage {
    flags: Flags,
    lut: Vec<u32>,
    offsets: Vec<u8>,
}
