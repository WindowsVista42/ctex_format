use crate::flags::{Flags};
use crate::SECTOR_SIZE;
use std::arch::x86_64::*;
use std::io::Read;
use anyhow::Result;

#[allow(dead_code)]
union Avx512fBucket {
    meal: __m512i,
    snacks: [i32; 16],
}

#[allow(dead_code)]
union Avx2Bucket {
    meal: __m256i,
    snacks: [i32; 8],
}

union Sse2Bucket {
    meal: __m128i,
    snacks: [i32; 4],
}

pub fn decode_path(path: &str) -> Result<Vec<u32>> {
    let mut file = std::fs::File::open(path)?;
    let mut buff = Vec::new();
    file.read_to_end(&mut buff)?;

    let flags: Flags = unsafe { buff.get(0..8).unwrap().align_to::<u64>().1[0].into() };
    let lut_off = 8 + flags.lut_len() * 4;

    let lut = unsafe { buff.get(8..lut_off).unwrap().align_to::<u32>().1 };
    let offsets = buff.get(lut_off..).unwrap();

    let data = decode_raw(lut, offsets, flags);
    Ok(data)
}

pub fn decode_raw(lut: &[u32], offsets: &[u8], _flags: Flags) -> Vec<u32> {
    if is_x86_feature_detected!("avx512f") {
        return avx512f_decode(lut, offsets);
    } else if is_x86_feature_detected!("avx2") {
        return avx2_decode(lut, offsets);
    } else if is_x86_feature_detected!("sse2") {
        return sse2_decode(lut, offsets);
    } else {
        todo!()
        // Todo: return default_decode(lut, offsets);
    }
}

#[inline]
unsafe fn _mm_gather_epi32(ptr: *const i32, reg: __m128i, size: i32) -> __m128i {
    let buck = Sse2Bucket { meal: reg };
    _mm_set_epi32(
        *((ptr as isize + (buck.snacks[3] * size) as isize) as *const i32),
        *((ptr as isize + (buck.snacks[2] * size) as isize) as *const i32),
        *((ptr as isize + (buck.snacks[1] * size) as isize) as *const i32),
        *((ptr as isize + (buck.snacks[0] * size) as isize) as *const i32),
    )
}

#[rustfmt::skip]
pub(crate) fn sse2_decode(lut: &[u32], offsets: &[u8]) -> Vec<u32> {
    let w = offsets.len();
    assert!(w >= SECTOR_SIZE, "Input CTEX data must have a width greater than {}!", SECTOR_SIZE - 1);
    assert_eq!(w % SECTOR_SIZE, 0, "Input CTEX data must have a width divisible by {}!", SECTOR_SIZE);
    assert_ne!(lut.len(), 0, "Input lut must not have a length of zero!");

    let mut out = Vec::with_capacity(4 * w);

    let off_ptr = offsets.as_ptr() as *const __m128i;
    let out_ptr = out.as_mut_ptr() as *mut __m128i;
    let lut_ptr = lut.as_ptr() as *const i32;

    unsafe {
        out.set_len(w);

        for i in 0..(w / SECTOR_SIZE) {
            let sect_0 = _mm_loadu_si128(off_ptr.add((i * 4) + 0));
            let sect_1 = _mm_loadu_si128(off_ptr.add((i * 4) + 1));
            let sect_2 = _mm_loadu_si128(off_ptr.add((i * 4) + 2));
            let sect_3 = _mm_loadu_si128(off_ptr.add((i * 4) + 3));

            let     shfl_00 = _mm_and_si128(sect_0, _mm_set1_epi32(0x00_00_00_FF));
            let mut shfl_01 = _mm_and_si128(sect_0, _mm_set1_epi32(0x00_00_FF_00));
            let mut shfl_02 = _mm_and_si128(sect_0, _mm_set1_epi32(0x00_FF_00_00));
            let     shfl_03;

            let     shfl_04 = _mm_and_si128(sect_1, _mm_set1_epi32(0x00_00_00_FF));
            let mut shfl_05 = _mm_and_si128(sect_1, _mm_set1_epi32(0x00_00_FF_00));
            let mut shfl_06 = _mm_and_si128(sect_1, _mm_set1_epi32(0x00_FF_00_00));
            let     shfl_07;

            let     shfl_08 = _mm_and_si128(sect_2, _mm_set1_epi32(0x00_00_00_FF));
            let mut shfl_09 = _mm_and_si128(sect_2, _mm_set1_epi32(0x00_00_FF_00));
            let mut shfl_10 = _mm_and_si128(sect_2, _mm_set1_epi32(0x00_FF_00_00));
            let     shfl_11;

            let     shfl_12 = _mm_and_si128(sect_3, _mm_set1_epi32(0x00_00_00_FF));
            let mut shfl_13 = _mm_and_si128(sect_3, _mm_set1_epi32(0x00_00_FF_00));
            let mut shfl_14 = _mm_and_si128(sect_3, _mm_set1_epi32(0x00_FF_00_00));
            let     shfl_15;

            shfl_01 = _mm_srli_epi32(shfl_01,08);
            shfl_02 = _mm_srli_epi32(shfl_02,16);
            shfl_03 = _mm_srli_epi32(sect_0, 24);

            shfl_05 = _mm_srli_epi32(shfl_05,08);
            shfl_06 = _mm_srli_epi32(shfl_06,16);
            shfl_07 = _mm_srli_epi32(sect_1, 24);

            shfl_09 = _mm_srli_epi32(shfl_09,08);
            shfl_10 = _mm_srli_epi32(shfl_10,16);
            shfl_11 = _mm_srli_epi32(sect_2, 24);

            shfl_13 = _mm_srli_epi32(shfl_13,08);
            shfl_14 = _mm_srli_epi32(shfl_14,16);
            shfl_15 = _mm_srli_epi32(sect_3, 24);

            let gath_00 = _mm_gather_epi32(lut_ptr, shfl_00, 4);
            let gath_01 = _mm_gather_epi32(lut_ptr, shfl_01, 4);
            let gath_02 = _mm_gather_epi32(lut_ptr, shfl_02, 4);
            let gath_03 = _mm_gather_epi32(lut_ptr, shfl_03, 4);

            let gath_04 = _mm_gather_epi32(lut_ptr, shfl_04, 4);
            let gath_05 = _mm_gather_epi32(lut_ptr, shfl_05, 4);
            let gath_06 = _mm_gather_epi32(lut_ptr, shfl_06, 4);
            let gath_07 = _mm_gather_epi32(lut_ptr, shfl_07, 4);

            let gath_08 = _mm_gather_epi32(lut_ptr, shfl_08, 4);
            let gath_09 = _mm_gather_epi32(lut_ptr, shfl_09, 4);
            let gath_10 = _mm_gather_epi32(lut_ptr, shfl_10, 4);
            let gath_11 = _mm_gather_epi32(lut_ptr, shfl_11, 4);

            let gath_12 = _mm_gather_epi32(lut_ptr, shfl_12, 4);
            let gath_13 = _mm_gather_epi32(lut_ptr, shfl_13, 4);
            let gath_14 = _mm_gather_epi32(lut_ptr, shfl_14, 4);
            let gath_15 = _mm_gather_epi32(lut_ptr, shfl_15, 4);

            _mm_storeu_si128(out_ptr.add(16 * i + 00), gath_00);
            _mm_storeu_si128(out_ptr.add(16 * i + 01), gath_04);
            _mm_storeu_si128(out_ptr.add(16 * i + 02), gath_08);
            _mm_storeu_si128(out_ptr.add(16 * i + 03), gath_12);

            _mm_storeu_si128(out_ptr.add(16 * i + 04), gath_01);
            _mm_storeu_si128(out_ptr.add(16 * i + 05), gath_05);
            _mm_storeu_si128(out_ptr.add(16 * i + 06), gath_09);
            _mm_storeu_si128(out_ptr.add(16 * i + 07), gath_13);

            _mm_storeu_si128(out_ptr.add(16 * i + 08), gath_02);
            _mm_storeu_si128(out_ptr.add(16 * i + 09), gath_06);
            _mm_storeu_si128(out_ptr.add(16 * i + 10), gath_10);
            _mm_storeu_si128(out_ptr.add(16 * i + 11), gath_14);

            _mm_storeu_si128(out_ptr.add(16 * i + 12), gath_03);
            _mm_storeu_si128(out_ptr.add(16 * i + 13), gath_07);
            _mm_storeu_si128(out_ptr.add(16 * i + 14), gath_11);
            _mm_storeu_si128(out_ptr.add(16 * i + 15), gath_15);
        }
    }

    out
}

#[rustfmt::skip]
pub(crate) fn avx2_decode(lut: &[u32], offsets: &[u8]) -> Vec<u32> {
    let w = offsets.len();
    assert!(w >= SECTOR_SIZE, "Input CTEX data must have a width greater than {}!", SECTOR_SIZE - 1);
    assert_eq!(w % SECTOR_SIZE, 0, "Input CTEX data must have a width divisible by {}!", SECTOR_SIZE);
    assert_ne!(lut.len(), 0, "Input lut must not have a length of zero!");

    let mut out = Vec::with_capacity(4 * w);

    let off_ptr = offsets.as_ptr() as *const __m256i;
    let out_ptr = out.as_mut_ptr() as *mut __m256i;
    let lut_ptr = lut.as_ptr() as *const i32;

    unsafe {
        out.set_len(w);

        for i in 0..(w / SECTOR_SIZE) {
            let sect_0 = _mm256_lddqu_si256(off_ptr.add(2 * i + 0));
            let sect_1 = _mm256_lddqu_si256(off_ptr.add(2 * i + 1));

            let     shfl_0 = _mm256_and_si256(sect_0, _mm256_set1_epi32(0x00_00_00_FF));
            let mut shfl_1 = _mm256_and_si256(sect_0, _mm256_set1_epi32(0x00_00_FF_00));
            let mut shfl_2 = _mm256_and_si256(sect_0, _mm256_set1_epi32(0x00_FF_00_00));
            let     shfl_3;

            let     shfl_4 = _mm256_and_si256(sect_1, _mm256_set1_epi32(0x00_00_00_FF));
            let mut shfl_5 = _mm256_and_si256(sect_1, _mm256_set1_epi32(0x00_00_FF_00));
            let mut shfl_6 = _mm256_and_si256(sect_1, _mm256_set1_epi32(0x00_FF_00_00));
            let     shfl_7;

            shfl_1 = _mm256_srli_epi32(shfl_1, 08);
            shfl_2 = _mm256_srli_epi32(shfl_2, 16);
            shfl_3 = _mm256_srli_epi32(sect_0, 24);

            shfl_5 = _mm256_srli_epi32(shfl_5, 08);
            shfl_6 = _mm256_srli_epi32(shfl_6, 16);
            shfl_7 = _mm256_srli_epi32(sect_1, 24);

            let gath_0 = _mm256_i32gather_epi32(lut_ptr, shfl_0, 4);
            let gath_1 = _mm256_i32gather_epi32(lut_ptr, shfl_1, 4);
            let gath_2 = _mm256_i32gather_epi32(lut_ptr, shfl_2, 4);
            let gath_3 = _mm256_i32gather_epi32(lut_ptr, shfl_3, 4);

            let gath_4 = _mm256_i32gather_epi32(lut_ptr, shfl_4, 4);
            let gath_5 = _mm256_i32gather_epi32(lut_ptr, shfl_5, 4);
            let gath_6 = _mm256_i32gather_epi32(lut_ptr, shfl_6, 4);
            let gath_7 = _mm256_i32gather_epi32(lut_ptr, shfl_7, 4);

            _mm256_storeu_si256(out_ptr.add(8 * i + 0), gath_0);
            _mm256_storeu_si256(out_ptr.add(8 * i + 1), gath_4);

            _mm256_storeu_si256(out_ptr.add(8 * i + 2), gath_1);
            _mm256_storeu_si256(out_ptr.add(8 * i + 3), gath_5);

            _mm256_storeu_si256(out_ptr.add(8 * i + 4), gath_2);
            _mm256_storeu_si256(out_ptr.add(8 * i + 5), gath_6);

            _mm256_storeu_si256(out_ptr.add(8 * i + 6), gath_3);
            _mm256_storeu_si256(out_ptr.add(8 * i + 7), gath_7);
        }
    }

    out
}

#[rustfmt::skip]
pub(crate) fn avx512f_decode(lut: &[u32], offsets: &[u8]) -> Vec<u32> {
    let w = offsets.len();
    assert!(w >= SECTOR_SIZE, "Input CTEX data must have a width greater than {}!", SECTOR_SIZE - 1);
    assert_eq!(w % SECTOR_SIZE, 0, "Input CTEX data must have a width divisible by {}!", SECTOR_SIZE);
    assert_ne!(lut.len(), 0, "Input lut must not have a length of zero!");

    let mut out = Vec::with_capacity(4 * w);

    let off_ptr = offsets.as_ptr() as *const i32;
    let out_ptr = out.as_mut_ptr() as *mut i32;
    let lut_ptr = lut.as_ptr() as *const u8;

    unsafe {
        out.set_len(w);

        for i in 0..(w / SECTOR_SIZE) {
            let sect_0 = _mm512_loadu_si512(off_ptr.add(i));

            let     shfl_0 = _mm512_and_si512(sect_0, _mm512_set1_epi32(0x00_00_00_FF));
            let mut shfl_1 = _mm512_and_si512(sect_0, _mm512_set1_epi32(0x00_00_FF_00));
            let mut shfl_2 = _mm512_and_si512(sect_0, _mm512_set1_epi32(0x00_FF_00_00));
            let     shfl_3;

            shfl_1 = _mm512_srli_epi32(shfl_1, 08);
            shfl_2 = _mm512_srli_epi32(shfl_2, 16);
            shfl_3 = _mm512_srli_epi32(sect_0, 24);

            let gath_0 = _mm512_i32gather_epi32(shfl_0, lut_ptr, 4);
            let gath_1 = _mm512_i32gather_epi32(shfl_1, lut_ptr, 4);
            let gath_2 = _mm512_i32gather_epi32(shfl_2, lut_ptr, 4);
            let gath_3 = _mm512_i32gather_epi32(shfl_3, lut_ptr, 4);

            _mm512_storeu_si512(out_ptr.add(16 * (4 * i + 0)), gath_0);
            _mm512_storeu_si512(out_ptr.add(16 * (4 * i + 1)), gath_1);
            _mm512_storeu_si512(out_ptr.add(16 * (4 * i + 2)), gath_2);
            _mm512_storeu_si512(out_ptr.add(16 * (4 * i + 3)), gath_3);
        }
    }

    out
}
