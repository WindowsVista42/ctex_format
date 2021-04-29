#![feature(num_as_ne_bytes)]
#![feature(stdsimd)]
#![allow(clippy::identity_op)]

use crate::flags::Flags;

pub mod decode;
pub mod encode;
pub mod flags;
pub mod util;

mod tests;

pub const SECTOR_SIZE: usize = 0x40;

pub struct CtexImage {
    flags: Flags,
    lut: Vec<u32>,
    offsets: Vec<u8>,
}
