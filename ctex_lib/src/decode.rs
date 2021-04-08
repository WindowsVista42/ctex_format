use anyhow::*;
use std::path::PathBuf;

pub fn decode(lz4_buf: &Vec<u8>) -> Result<Vec<u8>> {
    let buf = lz4_buf;

    let width = unsafe {
        buf.get(0..4)
            .expect("Incorrect input format")
            .align_to::<u32>()
            .1[0]
    };
    let num_colors = unsafe {
        buf.get(4..8)
            .expect("Incorrect input format")
            .align_to::<u32>()
            .1[0]
    };

    let colors_end = 8 + 4 * num_colors as usize;
    let data_end = colors_end + width as usize * width as usize;

    let palette = unsafe {
        buf.get(8..colors_end)
            .expect("Incorrect input format")
            .align_to::<[u8; 4]>()
            .1
    };
    let data = buf
        .get(colors_end..data_end)
        .expect("Incorrect input format");

    let mut out: Vec<[u8; 4]> = Vec::with_capacity((width * width) as usize);

    for i in 0..(width * width) {
        out.push(palette[data[i as usize] as usize]);
    }

    Ok(unsafe { out.align_to::<u8>().1.to_vec() })
}

pub fn decode_path(path: &PathBuf) -> Result<Vec<u8>> {
    let file = std::fs::File::open(path)?;
    let mut decoder = lz4::Decoder::new(file)?;
    let mut buf: Vec<u8> = Vec::new();
    std::io::copy(&mut decoder, &mut buf)?;

    let width = unsafe {
        buf.get(0..4)
            .expect("Incorrect input format")
            .align_to::<u32>()
            .1[0]
    };
    let num_colors = unsafe {
        buf.get(4..8)
            .expect("Incorrect input format")
            .align_to::<u32>()
            .1[0]
    };

    let colors_end = 8 + 4 * num_colors as usize;
    let data_end = colors_end + width as usize * width as usize;

    let palette = unsafe {
        buf.get(8..colors_end)
            .expect("Incorrect input format")
            .align_to::<[u8; 4]>()
            .1
    };
    let data = buf
        .get(colors_end..data_end)
        .expect("Incorrect input format");

    let mut out: Vec<[u8; 4]> = Vec::with_capacity((width * width) as usize);

    // TODO: This can be optimized
    for i in 0..(width * width) {
        out.push(palette[data[i as usize] as usize]);
    }

    Ok(unsafe { out.align_to::<u8>().1.to_vec() })
}
