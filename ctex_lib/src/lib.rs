#![feature(num_as_ne_bytes)]
#![feature(box_into_inner)]

pub mod decode;
pub mod encode;

use anyhow::*;
use std::path;
use std::sync::mpsc::channel;

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
pub fn par_decode_all_fifo(paths: &Vec<glob::GlobResult>) -> Result<Vec<Vec<u8>>> {
    rayon::scope_fifo(|s| {
        let (sen, rec) = channel();
        paths
            .iter()
            .flatten()
            .map(|p| {
                let sender = sen.clone();
                s.spawn_fifo(move |_| sender.send(decode::decode_path(p)).unwrap());
            })
            .map(|_| {
                rec.recv().unwrap()
            })
            .collect::<Vec<Result<Vec<u8>>>>()
            .into_iter()
            .collect::<Result<Vec<Vec<u8>>>>()
    })
}

        /*
    paths
        .into_par_iter()
        .flatten()
        .map(decode::decode_path)
        .collect::<Vec<Result<Vec<u8>>>>() // out of order
        .into_iter()
        .collect::<Result<Vec<Vec<u8>>>>()

         */