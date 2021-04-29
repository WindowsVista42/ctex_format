#![feature(num_as_ne_bytes)]
#![feature(stdsimd)]
#![allow(clippy::identity_op)]

use crate::flags::Flags;

pub mod decode;

#[cfg(feature = "encode")]
pub mod encode;

#[cfg(feature = "encode")]
pub mod flags;

#[cfg(feature = "encode")]
pub mod util;

#[cfg(feature = "par_util")]
pub mod par_util;

mod tests;

pub(crate) const SECTOR_SIZE: usize = 0x40;

pub struct CtexImage {
    flags: Flags,
    lut: Vec<u32>,
    offsets: Vec<u8>,
}
