use crate::flags::{Compression, Flags};
use crate::{SECTOR_SIZE};
use num_integer::Roots;
use lz4_flex::compress;
use anyhow::Result;

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

pub fn encode(buff: &[u8], mut flags: Flags) -> Result<Vec<u8>> {
    let img = unsafe {
        buff.align_to::<u32>().1
    };
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

    match flags.compression() {
        Compression::None => {},
        Compression::Lz4 => offsets = compress(&*offsets),
    }

    let mut outp = Vec::new();
    unsafe {
        outp.extend(&flags.as_u64().to_le_bytes());
        outp.extend(lut.align_to::<u8>().1);
        outp.extend(&*offsets);
        Ok(outp)
    }
}

pub(crate) fn __encode(img: &[u32], flags: &mut Flags) -> (Vec<u32>, Vec<u8>, Flags) {
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

    match flags.compression() {
        Compression::None => {},
        Compression::Lz4 => offsets = compress(&*offsets),
    }

    (lut, offsets, *flags)
}
