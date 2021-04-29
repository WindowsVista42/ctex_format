use crate::flags::{Compression, Flags};
use crate::{CtexImage, SECTOR_SIZE};
use image::GenericImageView;
use num_integer::Roots;
use std::io::Write;

fn search(color: u32, lut: &[u32]) -> Option<usize> {
    for (i, x) in lut.iter().enumerate() {
        if *x == color {
            return Some(i);
        }
    }
    None
}

fn add(color: u32, lut: &mut Vec<u32>, offsets: &mut Vec<u8>) {
    if let Some(idx) = search(color, &*lut) {
        offsets.push(idx as u8)
    } else {
        lut.push(color);
        offsets.push(lut.len() as u8 - 1);
    }
}

fn encode(img: &[u32], flags: &mut Flags) -> (Vec<u32>, Vec<u8>, Flags) {
    let w = img.len();
    assert!(
        w.is_power_of_two(),
        "Input image must be square and a power of two!"
    );

    let mut lut = Vec::new();
    let mut offsets = Vec::with_capacity(w);

    for i in 0..(w / SECTOR_SIZE) {
        for n in 0..(SECTOR_SIZE / 4) {
            let px_0 = img[SECTOR_SIZE * i + n + 00];
            let px_1 = img[SECTOR_SIZE * i + n + 16];
            let px_2 = img[SECTOR_SIZE * i + n + 32];
            let px_3 = img[SECTOR_SIZE * i + n + 48];

            add(px_0, &mut lut, &mut offsets);
            add(px_1, &mut lut, &mut offsets);
            add(px_2, &mut lut, &mut offsets);
            add(px_3, &mut lut, &mut offsets);
        }
    }
    assert!(
        lut.len() <= 256,
        "Input image must have 256 or fewer unique colors!",
    );

    flags.lutw_0 = lut.len() as u8;
    flags.offw_0 = w.sqrt() as u16;

    (lut, offsets, *flags)
}

pub fn encode_raw(img: &[u32], mut flags: Flags) -> (Vec<u32>, Vec<u8>, Flags) {
    encode(img, &mut flags)
}

pub fn encode_path(path: &str, mut flags: Flags) -> CtexImage {
    let img = image::open(path).unwrap();
    assert_eq!(img.width(), img.height(), "Input image must be square!");

    let img = img
        .to_rgba8()
        .pixels()
        .map(|p| unsafe { p.0.align_to::<u32>().1[0] })
        .collect::<Vec<u32>>();

    let (lut, offsets, flags) = encode(&*img, &mut flags);

    CtexImage {
        flags,
        lut,
        offsets,
    }
}
