use anyhow::*;
use image::Rgba;
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

pub fn encode(path: PathBuf) -> Result<(Vec<u8>, String)> {
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

    let mut bytes = Vec::new();
    {
        bytes.extend(width.as_ne_bytes());
        bytes.extend((colors.len() as u32).as_ne_bytes());
        bytes.extend(unsafe { colors.align_to::<u8>().1 });
        bytes.extend(data.as_slice());
    }
    //let bytes = encoder.finish().unwrap();

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
