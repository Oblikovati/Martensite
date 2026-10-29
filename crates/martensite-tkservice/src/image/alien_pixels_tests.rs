use super::*;
use image::{ImageBuffer, LumaA, Rgb, Rgba};

fn pixmap(format: ImageFormat, width: usize, bytes: &[u8]) -> ImagePixMap {
    let pixmap = ImagePixMap::default();
    assert!(pixmap.init_trash(format, width, 1, 0));
    pixmap.change_data().data.change_data()[..bytes.len()].copy_from_slice(bytes);
    pixmap
}

fn floats(values: &[f32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_ne_bytes()).collect()
}

#[test]
fn eight_bit_colour_loads_blue_first_and_opaque_four_channels_as_bgr32() {
    let rgb = decoded_rows(
        &DynamicImage::ImageRgb8(RgbImage::from_raw(1, 1, vec![1, 2, 3]).unwrap()),
        None,
    );
    assert_eq!(rgb.unwrap().pixels, vec![3, 2, 1]);
    let opaque = RgbaImage::from_raw(2, 1, vec![1, 2, 3, 255, 4, 5, 6, 255]).unwrap();
    let rows = decoded_rows(&DynamicImage::ImageRgba8(opaque), None).unwrap();
    assert_eq!(
        (rows.format, rows.pixels),
        (ImageFormat::Bgr32, vec![3, 2, 1, 255, 6, 5, 4, 255])
    );
    let clear = RgbaImage::from_raw(1, 1, vec![1, 2, 3, 0]).unwrap();
    assert_eq!(
        decoded_rows(&DynamicImage::ImageRgba8(clear), None)
            .unwrap()
            .format,
        ImageFormat::Bgra
    );
}

#[test]
fn gray_keeps_its_depth_and_gray_alpha_becomes_colour() {
    let gray16 = image::ImageBuffer::<image::Luma<u16>, _>::from_raw(1, 1, vec![0x1234]).unwrap();
    let rows = decoded_rows(&DynamicImage::ImageLuma16(gray16), None).unwrap();
    assert_eq!(
        (rows.format, rows.pixels),
        (ImageFormat::Gray16, 0x1234u16.to_ne_bytes().to_vec())
    );
    let gray_alpha = ImageBuffer::<LumaA<u8>, _>::from_raw(1, 1, vec![7, 9]).unwrap();
    let rows = decoded_rows(&DynamicImage::ImageLumaA8(gray_alpha), None).unwrap();
    assert_eq!(
        (rows.format, rows.pixels),
        (ImageFormat::Bgra, vec![7, 7, 7, 9])
    );
}

#[test]
fn sixteen_bit_colour_keeps_high_bytes_and_drops_alpha() {
    let rgba =
        ImageBuffer::<Rgba<u16>, _>::from_raw(1, 1, vec![0x1280, 0x34ff, 0x5600, 0]).unwrap();
    let rows = decoded_rows(&DynamicImage::ImageRgba16(rgba), None).unwrap();
    assert_eq!(
        (rows.format, rows.pixels),
        (ImageFormat::Bgr, vec![0x56, 0x34, 0x12])
    );
    let rgb = ImageBuffer::<Rgb<u16>, _>::from_raw(1, 1, vec![0xffff, 0, 0x0100]).unwrap();
    assert_eq!(
        decoded_rows(&DynamicImage::ImageRgb16(rgb), None)
            .unwrap()
            .pixels,
        vec![1, 0, 255]
    );
}

#[test]
fn writing_reorders_channels_and_jpeg_or_gif_drop_alpha() {
    let bgra = pixmap(ImageFormat::Bgra, 1, &[1, 2, 3, 4]);
    let png = encodable_image(&bgra, CodecFormat::Png).unwrap();
    assert_eq!(png.as_bytes(), &[3, 2, 1, 4]);
    let jpeg = encodable_image(&bgra, CodecFormat::Jpeg).unwrap();
    assert_eq!(jpeg.as_bytes(), &[3, 2, 1]);
    let bgr32 = pixmap(ImageFormat::Bgr32, 1, &[1, 2, 3, 0]);
    assert_eq!(
        encodable_image(&bgr32, CodecFormat::Bmp)
            .unwrap()
            .as_bytes(),
        &[3, 2, 1]
    );
}

#[test]
fn wide_gray_scales_linearly_onto_eight_bits() {
    let gray16: Vec<u8> = [10u16, 61, 265]
        .iter()
        .flat_map(|v| v.to_ne_bytes())
        .collect();
    let image = encodable_image(&pixmap(ImageFormat::Gray16, 3, &gray16), CodecFormat::Png);
    assert_eq!(image.unwrap().as_bytes(), &[0, 51, 255]);
    // A constant image has no range: values map through [0, 255], so 0.5 rounds to 1.
    let flat = pixmap(ImageFormat::GrayF, 2, &floats(&[0.5, 0.5]));
    assert_eq!(
        encodable_image(&flat, CodecFormat::Png).unwrap().as_bytes(),
        &[1, 1]
    );
}

#[test]
fn float_colour_clamps_to_the_unit_range() {
    let rgbf = pixmap(ImageFormat::Rgbf, 1, &floats(&[-1.0, 0.5, 2.0]));
    assert_eq!(
        encodable_image(&rgbf, CodecFormat::Png).unwrap().as_bytes(),
        &[0, 128, 255]
    );
    let bgraf = pixmap(ImageFormat::Bgraf, 1, &floats(&[0.0, 0.0, 1.0, 0.25]));
    assert_eq!(
        encodable_image(&bgraf, CodecFormat::Png)
            .unwrap()
            .as_bytes(),
        &[255, 0, 0, 64]
    );
}
