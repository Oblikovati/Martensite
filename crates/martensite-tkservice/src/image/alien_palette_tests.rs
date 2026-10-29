use super::*;
use crate::image::ImageFormat;
use crate::image::alien_codec::decode;

fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = u32::MAX;
    for &byte in bytes {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            crc = if crc & 1 == 1 {
                (crc >> 1) ^ 0xedb8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

fn chunk(kind: &[u8], body: &[u8]) -> Vec<u8> {
    let mut out = (body.len() as u32).to_be_bytes().to_vec();
    out.extend_from_slice(kind);
    out.extend_from_slice(body);
    out.extend(crc32(&[kind, body].concat()).to_be_bytes());
    out
}

/// A zlib stream holding `raw` in one stored (uncompressed) block.
fn stored_zlib(raw: &[u8]) -> Vec<u8> {
    let length = raw.len() as u16;
    let mut out = vec![0x78, 0x01, 0x01];
    out.extend(length.to_le_bytes());
    out.extend((!length).to_le_bytes());
    out.extend_from_slice(raw);
    let (a, b) = raw.iter().fold((1u32, 0u32), |(a, b), &x| {
        let a = (a + u32::from(x)) % 65521;
        (a, (b + a) % 65521)
    });
    out.extend(((b << 16) | a).to_be_bytes());
    out
}

/// An 8-bit indexed PNG, one row of `indices`, with `palette` and optional tRNS alphas.
fn indexed_png(palette: &[[u8; 3]], indices: &[u8], alphas: &[u8]) -> Vec<u8> {
    let mut header = (indices.len() as u32).to_be_bytes().to_vec();
    header.extend([0, 0, 0, 1, 8, 3, 0, 0, 0]);
    let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
    out.extend(chunk(b"IHDR", &header));
    out.extend(chunk(b"PLTE", &palette.concat()));
    if !alphas.is_empty() {
        out.extend(chunk(b"tRNS", alphas));
    }
    out.extend(chunk(b"IDAT", &stored_zlib(&[&[0], indices].concat())));
    out.extend(chunk(b"IEND", &[]));
    out
}

/// A BMP of one row of 8-bit `indices` with a `palette` colour table.
fn indexed_bmp(depth: u16, palette: &[[u8; 3]], row: &[u8]) -> Vec<u8> {
    let padded = [row, &vec![0; (4 - row.len() % 4) % 4]].concat();
    let table: Vec<u8> = palette.iter().flat_map(|c| [c[2], c[1], c[0], 0]).collect();
    let offset = 14 + 40 + table.len() as u32;
    let width = if depth == 1 { 8 } else { row.len() as i32 };
    let mut out = b"BM".to_vec();
    out.extend((offset + padded.len() as u32).to_le_bytes());
    out.extend([0; 4]);
    out.extend(offset.to_le_bytes());
    out.extend(40u32.to_le_bytes());
    out.extend(width.to_le_bytes());
    out.extend(1i32.to_le_bytes());
    out.extend(1u16.to_le_bytes());
    out.extend(depth.to_le_bytes());
    out.extend([0; 4]);
    out.extend((padded.len() as u32).to_le_bytes());
    out.extend([0; 8]);
    out.extend((palette.len() as u32).to_le_bytes());
    out.extend([0; 4]);
    out.extend(table);
    out.extend(padded);
    out
}

fn ramp(count: usize) -> Vec<[u8; 3]> {
    (0..count).map(|i| [i as u8; 3]).collect()
}

#[test]
fn only_identity_ramps_and_black_white_bits_are_gray() {
    assert!(is_gray_ramp(8, &ramp(256)));
    assert!(
        is_gray_ramp(8, &ramp(201)),
        "a short ramp counts: only listed entries"
    );
    assert!(is_gray_ramp(1, &[[0; 3], [255; 3]]));
    assert!(!is_gray_ramp(1, &[[255; 3], [0; 3]]));
    let reversed: Vec<[u8; 3]> = (0..256).map(|i| [255 - i as u8; 3]).collect();
    assert!(!is_gray_ramp(8, &reversed));
    let scaled: Vec<[u8; 3]> = (0..16).map(|i| [17 * i as u8; 3]).collect();
    assert!(!is_gray_ramp(4, &scaled));
}

#[test]
fn direct_colour_files_have_no_palette() {
    assert_eq!(indexed_kind(b"GIF89a"), None);
    let mut rgb_png = indexed_png(&ramp(2), &[0, 1], &[]);
    rgb_png[25] = 2;
    assert_eq!(indexed_kind(&rgb_png), None);
}

#[test]
fn indexed_png_loads_gray_or_opaque_colour() {
    let gray = decode(Some(&indexed_png(&ramp(256), &[0, 100, 255], &[])), b"").unwrap();
    assert_eq!(
        (gray.format, gray.pixels),
        (ImageFormat::Gray, vec![0, 100, 255])
    );
    let palette = [[10, 20, 30], [40, 50, 60]];
    let colour = decode(Some(&indexed_png(&palette, &[1, 0], &[0, 128])), b"").unwrap();
    assert_eq!(
        (colour.format, colour.pixels),
        (ImageFormat::Bgr, vec![60, 50, 40, 30, 20, 10])
    );
}

#[test]
fn indexed_bmp_loads_gray_or_colour() {
    let gray = decode(Some(&indexed_bmp(8, &ramp(256), &[3, 200])), b"").unwrap();
    assert_eq!(
        (gray.format, gray.pixels),
        (ImageFormat::Gray, vec![3, 200])
    );
    let bits = decode(
        Some(&indexed_bmp(1, &[[0; 3], [255; 3]], &[0b1010_0000])),
        b"",
    )
    .unwrap();
    assert_eq!(
        (bits.format, &bits.pixels[..3]),
        (ImageFormat::Gray, &[255, 0, 255][..])
    );
    let colour = decode(Some(&indexed_bmp(8, &[[1, 2, 3]], &[0])), b"").unwrap();
    assert_eq!(
        (colour.format, colour.pixels),
        (ImageFormat::Bgr, vec![3, 2, 1])
    );
}
