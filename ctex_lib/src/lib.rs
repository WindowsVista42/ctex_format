#![feature(num_as_ne_bytes)]

pub mod decode;
pub mod encode;

use anyhow::*;

#[cfg(feature = "par_tools")]
pub fn par_encode_all(glob_str: &str) -> Result<Vec<(Vec<u8>, String)>> {
    use rayon::prelude::*;
    let mut paths = Vec::new();
    paths.extend(glob::glob(&*glob_str)?);

    paths
        .into_par_iter()
        .flatten()
        .map(encode::encode)
        .collect::<Vec<Result<(Vec<u8>, String)>>>()
        .into_iter()
        .collect::<Result<Vec<(Vec<u8>, String)>>>()
}

#[cfg(feature = "par_tools")]
pub fn par_decode_all(glob_str: &str) -> Result<Vec<Vec<u8>>> {
    use rayon::prelude::*;
    let mut paths = Vec::new();
    paths.extend(glob::glob(&*glob_str)?);

    paths
        .into_par_iter()
        .flatten()
        .map(decode::decode_path)
        .collect::<Vec<Result<Vec<u8>>>>()
        .into_iter()
        .collect::<Result<Vec<Vec<u8>>>>()
}