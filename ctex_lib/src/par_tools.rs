    use anyhow::*;
    use rayon::prelude::*;
    use std::sync::mpsc::channel;
    use std::path::PathBuf;
    use crate::{encode, decode};

    pub fn par_encode_all(glob_str: &str) -> Result<Vec<(Vec<u8>, String)>> {
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

    pub fn par_decode_all_fifo(paths: &Vec<PathBuf>) -> Result<Vec<Vec<u8>>> {
        rayon::scope_fifo(|s| {
            let (sen, rec) = channel();
            paths
                .iter()
                //.flatten()
                .map(|p| {
                    let sender = sen.clone();
                    s.spawn_fifo(move |_| sender.send(decode::decode_path(&p)).unwrap());
                })
                .map(|_| {
                    rec.recv().unwrap()
                })
                .collect::<Vec<Result<Vec<u8>>>>()
                .into_iter()
                .collect::<Result<Vec<Vec<u8>>>>()
        })
    }