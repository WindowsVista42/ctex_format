#![feature(num_as_ne_bytes)]

use anyhow::*;
use image::Rgba;
use rayon::prelude::*;
use std::io::Write;
use std::path::PathBuf;

fn search(data: &[[u8; 4]], val: &[u8; 4]) -> Option<usize> {
    let val = unsafe { val.align_to::<u32>().1[0] };
    for (i, x) in data.iter().enumerate() {
        let x = unsafe { x.align_to::<u32>().1[0] };
        if x == val {
            return Some(i);
        }
    }
    return None;
}

fn load_and_compress(path: PathBuf) -> Result<(Vec<u8>, String)> {
    let img = image::open(path.clone()).unwrap();
    let img = img.to_rgba8();
    assert_eq!(
        img.width(),
        img.height(),
        "Image height and width not equal"
    );
    assert_eq!(
        img.width() % 64,
        0,
        "Image dimensions must be a multiple of 64"
    );

    // image data
    let width = img.width();
    let mut colors: Vec<[u8; 4]> = Vec::new();
    let mut data: Vec<u8> = Vec::new();

    // Build colors data
    let pixels = img.pixels().map(|p| *p).collect::<Vec<Rgba<u8>>>();
    for px in pixels.iter() {
        if let Some(idx) = search(&colors.as_slice(), &px.0) {
            // found
            data.push(idx as u8);
        } else {
            // not found add to list
            colors.push(px.0);
            data.push((colors.len() - 1) as u8);
        }
    }
    assert!(
        colors.len() < 256,
        "Image cannot contain more than 256 colors"
    );
    println!("{:?}: {}", path.clone(), colors.len());

    use flate2::write::GzEncoder;
    use flate2::Compression;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    {
        encoder.write_all(width.as_ne_bytes()).unwrap();
        encoder
            .write_all((colors.len() as u32).as_ne_bytes())
            .unwrap();
        encoder
            .write_all(unsafe { colors.as_slice().align_to::<u32>().1.align_to::<u8>().1 })
            .unwrap();
        encoder.write_all(data.as_slice()).unwrap();
    }
    let bytes = encoder.finish().unwrap();

    Ok((
        bytes,
        String::from(
            path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .trim_end_matches(".png"),
        ),
    ))
}

fn main() -> Result<()> {
    let now = std::time::Instant::now();
    let mut paths = Vec::new();
    paths.extend(glob::glob("input/*.png")?);

    let images = paths
        .into_par_iter()
        .flatten()
        .map(load_and_compress)
        .collect::<Vec<Result<(Vec<u8>, String)>>>()
        .into_iter()
        .collect::<Result<Vec<(Vec<u8>, String)>>>()?;

    for (img, name) in images {
        std::fs::File::create(&*format!("output/{}.tex.gz", name))
            .unwrap()
            .write_all(img.as_slice())
            .unwrap();
    }

    println!("elapsed: {}ms", now.elapsed().as_millis());

    Ok(())
}
