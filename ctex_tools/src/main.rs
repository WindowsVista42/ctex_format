#![feature(seek_stream_len)]

use ctex::{par_decode_all_fifo, par_encode_all};
use std::io::{Seek, Write};

use glob::glob;
use std::path::PathBuf;

fn main() {
    let mut encodes = par_encode_all("input/*.png").unwrap();

    let mut paths = Vec::new();
    paths.extend(glob("output/*.ctex").unwrap());
    println!("{:?}", paths);
    println!();
    let paths = paths.iter().flatten().collect::<Vec<&PathBuf>>();
    let decodes = par_decode_all_fifo(&paths)
        .unwrap();
    decodes.iter().for_each(|img| {
        println!("{}", img.len());
    });

    /*
    encodes.iter_mut().for_each(|(img, name)| {
        println!("{}", name);
        println!("{}", img.len());
        let file = std::fs::File::create(&*format!("output/{}.ctex", name)).unwrap();
        let mut encoder = lz4::EncoderBuilder::new().level(9).build(file).unwrap();
        encoder.write_all(img.as_slice()).unwrap();
        let meta = encoder.finish().0.stream_len().unwrap();
        println!("{:?}", meta);

        let file = std::fs::File::open(&*format!("output/{}.ctex", name)).unwrap();
        let mut decoder = lz4::Decoder::new(file).unwrap();
        let mut buf: Vec<u8> = Vec::new();
        std::io::copy(&mut decoder, &mut buf).unwrap();

        println!("{}", buf.len());
        println!();
    });
     */
}
