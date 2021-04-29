#![feature(num_as_ne_bytes)]
#![feature(stdsimd)]

use crate::flag::Flags;

pub mod decode;
pub mod encode;

mod flag;
mod tests;

const SECTOR_SIZE: usize = 64;

pub struct CtexImage {
    flags: Flags,
    lut: Vec<u32>,
    offsets: Vec<u8>,
}
