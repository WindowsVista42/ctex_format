use crate::encode::encode_path;
use crate::flags::Flags;
use crate::CtexImage;
use std::io::Write;

use anyhow::Result;

pub fn write_ctex(input_path: &str, output_path: &str, flags: Flags) -> Result<()> {
    let CtexImage {
        flags,
        lut,
        offsets,
    } = encode_path(input_path, flags);

    let mut file = std::fs::File::create(output_path).unwrap();
    unsafe {
        file.write_all(flags.as_u64().as_ne_bytes())?;
        file.write_all(lut.as_slice().align_to::<u8>().1)?;
        file.write_all(offsets.as_slice())?;
    }

    Ok(())
}