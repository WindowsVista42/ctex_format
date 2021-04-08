#![feature(seek_stream_len)]

use std::io::{Seek, Write};

use glob::glob;
use std::path::PathBuf;
use ctex::par_tools::par_encode_all;

fn main() {
    let mut encodes = par_encode_all("input/*.png").unwrap();

    encodes.iter_mut().for_each(|(img, name)| {
        let file = std::fs::File::create(&*format!("output/{}.ctex", name)).unwrap();
        let mut encoder = lz4::EncoderBuilder::new().level(9).build(file).unwrap();
        encoder.write_all(img.as_slice()).unwrap();
    });
}
