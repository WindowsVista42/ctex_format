#![feature(num_as_ne_bytes)]

pub mod decode;
pub mod encode;

use anyhow::*;
use rayon::prelude::*;

pub fn par_encode_all(rel_path: String) -> Result<Vec<(Vec<u8>, String)>> {
    let mut paths = Vec::new();
    paths.extend(glob::glob(&*rel_path)?);

    paths
        .into_par_iter()
        .flatten()
        .map(encode::encode)
        .collect::<Vec<Result<(Vec<u8>, String)>>>()
        .into_iter()
        .collect::<Result<Vec<(Vec<u8>, String)>>>()
}

pub fn par_decode_all(rel_path: String) -> Result<Vec<Vec<u8>>> {
    let mut paths = Vec::new();
    paths.extend(glob::glob(&*rel_path)?);

    paths
        .into_par_iter()
        .flatten()
        .map(decode::decode_path)
        .collect::<Vec<Result<Vec<u8>>>>()
        .into_iter()
        .collect::<Result<Vec<Vec<u8>>>>()
}

