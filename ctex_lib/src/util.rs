use crate::flags::Flags;
use crate::{Image, Compression, decode_raw, __encode, encode};
use std::io::{Write, Read};

use anyhow::Result;
use lz4_flex::decompress;

#[cfg(feature = "encode_util")]
use image::GenericImageView;

#[cfg(feature = "encode_util")]
pub fn write_ctex(input_path: &str, output_path: &str, flags: Flags) -> Result<()> {
    let Image {
        flags,
        lut,
        offsets,
    } = __encode_path(input_path, flags);

    let mut file = std::fs::File::create(output_path).unwrap();
    unsafe {
        file.write_all(flags.as_u64().as_ne_bytes())?;
        file.write_all(lut.as_slice().align_to::<u8>().1)?;
        file.write_all(offsets.as_slice())?;
    }

    Ok(())
}

pub fn decode_path(path: &str) -> Result<Vec<u8>> {
    let mut file = std::fs::File::open(path)?;
    let mut buff = Vec::new();
    file.read_to_end(&mut buff)?;

    let flags: Flags = unsafe { buff.get(0..8).unwrap().align_to::<u64>().1[0].into() };
    let lut_off = 8 + flags.lut_len() * 4;

    let lut = unsafe { buff.get(8..lut_off).unwrap().align_to::<u32>().1 };
    let offsets = buff.get(lut_off..).unwrap();

    let data = match flags.compression() {
        Compression::None => offsets.to_vec(),
        Compression::Lz4 => decompress(offsets, flags.offsets_len()).unwrap(),
    };

    let data = decode_raw(lut, &*data);
    unsafe {
        let mut data = std::mem::transmute::<Vec<u32>, Vec<u8>>(data);
        data.set_len(data.len() * 4);
        Ok(data)
    }
}

#[cfg(feature = "encode_util")]
pub(crate) fn __encode_path(path: &str, mut flags: Flags) -> Image {
    let img = image::open(path).unwrap();
    assert_eq!(img.width(), img.height(), "Input image must be square!");

    let img = img
        .to_rgba8()
        .pixels()
        .map(|p| unsafe { p.0.align_to::<u32>().1[0] })
        .collect::<Vec<u32>>();

    let (lut, offsets, flags) = __encode(&*img, &mut flags);

    Image {
        flags,
        lut,
        offsets,
    }
}

#[cfg(feature = "encode_util")]
pub fn encode_path(path: &str, flags: Flags) -> Result<Vec<u8>> {
    let img = image::open(path).unwrap();
    assert_eq!(img.width(), img.height(), "Input image must be square!");

    let img = img
        .to_rgba8()
        .pixels()
        .map(|p| unsafe { p.0.align_to::<u32>().1[0] })
        .collect::<Vec<u32>>();

    unsafe {
        encode(img.align_to::<u8>().1, flags)
    }
}
