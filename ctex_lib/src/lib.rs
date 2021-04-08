#![feature(num_as_ne_bytes)]
#![feature(box_into_inner)]

pub mod decode;
pub mod encode;

#[cfg(feature = "par_tools")]
pub mod par_tools;