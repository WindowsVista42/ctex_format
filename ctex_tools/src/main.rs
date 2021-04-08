#![feature(seek_stream_len)]

use ctex::par_encode_all;
use std::io::{Write, Seek};

fn main() {
    let mut encodes = par_encode_all("input/*.png").unwrap();

    encodes.iter_mut().for_each(|(img, name)| {
        println!("{}", name);
        println!("{}", img.len());
        let file = std::fs::File::create(&*format!("output/{}.ctex", name)).unwrap();
        let mut encoder = lz4::EncoderBuilder::new().level(9).build(file).unwrap();
        encoder.write_all(img.as_slice()).unwrap();
        let meta = encoder.finish().0.stream_len().unwrap();//encoder.finish().1.unwrap();
        println!("{:?}", meta);

        let file = std::fs::File::open(&*format!("output/{}.ctex", name)).unwrap();
        let mut decoder = lz4::Decoder::new(file).unwrap();
        let mut buf: Vec<u8> = Vec::new();
        std::io::copy(&mut decoder, &mut buf).unwrap();

        println!("{}", buf.len());
        println!();
    });
}
