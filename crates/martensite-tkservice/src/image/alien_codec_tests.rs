use super::*;
use image::{DynamicImage, RgbImage};

/// A 2x1 RGB image encoded in `format`: red, then blue.
fn two_pixel_file(format: FileFormat) -> Vec<u8> {
    let image = RgbImage::from_raw(2, 1, vec![255, 0, 0, 0, 0, 255]).unwrap();
    let mut out = Cursor::new(Vec::new());
    DynamicImage::ImageRgb8(image)
        .write_to(&mut out, format)
        .unwrap();
    out.into_inner()
}

/// A pixmap of the given format and packed bytes, one row.
fn pixmap(format: ImageFormat, width: usize, bytes: &[u8]) -> ImagePixMap {
    let pixmap = ImagePixMap::default();
    assert!(pixmap.init_trash(format, width, 1, 0));
    pixmap.change_data().data.change_data()[..bytes.len()].copy_from_slice(bytes);
    pixmap
}

#[test]
fn names_select_formats_by_their_last_extension_without_case() {
    assert_eq!(
        CodecFormat::from_name(b"dir.v2/a.PNG"),
        Some(CodecFormat::Png)
    );
    for name in [&b"x.jpg"[..], b"x.JPEG", b"x.jpe", b"x.jif"] {
        assert_eq!(CodecFormat::from_name(name), Some(CodecFormat::Jpeg));
    }
    assert_eq!(CodecFormat::from_name(b"bmp"), Some(CodecFormat::Bmp));
    assert_eq!(CodecFormat::from_name(b".gif"), Some(CodecFormat::Gif));
    assert_eq!(CodecFormat::from_name(b"a.tga"), None);
    assert_eq!(CodecFormat::from_name(b"a.png.txt"), None);
}

#[test]
fn signatures_select_formats_before_names() {
    assert_eq!(
        CodecFormat::from_signature(b"GIF89a...."),
        Some(CodecFormat::Gif)
    );
    assert_eq!(
        CodecFormat::from_signature(b"BM\0\0\0\0"),
        Some(CodecFormat::Bmp)
    );
    assert_eq!(
        CodecFormat::from_signature(&[0xff, 0xd8, 0xff, 0xe0]),
        Some(CodecFormat::Jpeg)
    );
    assert_eq!(CodecFormat::from_signature(b"P6\n1 1\n255\n"), None);
    let png = two_pixel_file(FileFormat::Png);
    assert_eq!(CodecFormat::from_signature(&png), Some(CodecFormat::Png));
    assert_eq!(
        decode(Some(&png), b"misnamed.gif").unwrap().format,
        ImageFormat::Bgr
    );
}

#[test]
fn failures_distinguish_format_file_and_pixels() {
    assert_eq!(
        decode(None, b"absent.png"),
        Err(DecodeFailure::MissingOrInvalid)
    );
    assert_eq!(
        decode(None, b"absent.xyz"),
        Err(DecodeFailure::UnsupportedFile)
    );
    assert_eq!(
        decode(Some(b"not an image"), b"a.xyz"),
        Err(DecodeFailure::UnsupportedFile)
    );
    assert_eq!(
        decode(Some(b"not an image"), b"a.bmp"),
        Err(DecodeFailure::MissingOrInvalid)
    );
}

#[test]
fn lossless_formats_round_trip_exactly() {
    for (format, file) in [
        (CodecFormat::Png, FileFormat::Png),
        (CodecFormat::Bmp, FileFormat::Bmp),
        (CodecFormat::Gif, FileFormat::Gif),
    ] {
        let decoded = decode(Some(&two_pixel_file(file)), b"").unwrap();
        let expected = if format == CodecFormat::Gif {
            (ImageFormat::Bgr32, vec![0, 0, 255, 255, 255, 0, 0, 255])
        } else {
            (ImageFormat::Bgr, vec![0, 0, 255, 255, 0, 0])
        };
        assert_eq!(
            (decoded.format, decoded.pixels.clone()),
            expected,
            "{format:?}"
        );
        let source = pixmap(decoded.format, 2, &decoded.pixels);
        let again = decode(Some(&encode(&source, format).unwrap()), b"").unwrap();
        assert_eq!(again, decoded, "{format:?}");
    }
}

#[test]
fn jpeg_keeps_a_flat_colour_within_two_levels() {
    let flat = pixmap(ImageFormat::Rgb, 16, &[200, 40, 90].repeat(16));
    let decoded = decode(Some(&encode(&flat, CodecFormat::Jpeg).unwrap()), b"").unwrap();
    assert_eq!(decoded.format, ImageFormat::Bgr);
    for pixel in decoded.pixels.as_chunks::<3>().0 {
        for (value, expected) in pixel.iter().zip([90u8, 40, 200]) {
            assert!(
                value.abs_diff(expected) <= 2,
                "{pixel:?} vs BGR [90, 40, 200]"
            );
        }
    }
}

#[test]
fn half_float_pixmaps_have_no_file_form() {
    let half = pixmap(ImageFormat::RgfHalf, 1, &[0; 4]);
    assert_eq!(
        encode(&half, CodecFormat::Png),
        Err(EncodeFailure::UnsupportedPixels(ImageFormat::RgfHalf))
    );
}
