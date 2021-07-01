#![feature(stdsimd)]
#![allow(clippy::identity_op)]

mod flags;
mod decoding;
mod encoding;
pub mod util;

pub use crate::flags::*;
pub use crate::decoding::*;
pub use crate::encoding::*;

//mod par_util;

mod tests;

#[cfg(feature = "rayon")]
pub mod par_util;

pub(crate) const SECTOR_SIZE: usize = 0x40;

#[allow(dead_code)]
pub(crate) struct Image {
    flags: Flags,
    lut: Vec<u32>,
    offsets: Vec<u8>,
}
