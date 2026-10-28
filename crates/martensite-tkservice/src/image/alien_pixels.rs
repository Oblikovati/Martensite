//! Pixel conversion between decoded image files and `ImageAlienPixMap` rows.
//!
//! Loading keeps the layout of a classic device-independent bitmap: 8-bit gray stays gray,
//! 16-bit gray stays 16-bit, colour is stored blue-green-red, and four-channel images become
//! `Bgra`, or `Bgr32` when every pixel is opaque. Colour with 16 bits per channel keeps the
//! high byte of red, green and blue and drops alpha. Writing reverses the channel order and
//! reduces to 8 bits: float and 16-bit gray scale linearly from their range onto 0..255, float
//! colour clamps to [0, 1].
use super::alien_codec::CodecFormat;
use super::alien_palette::IndexedKind;
use super::{ImageFormat, ImagePixMap};
use image::{DynamicImage, GrayImage, RgbImage, RgbaImage};

/// Decoded pixels, packed and top row first; `rows.pixels.len() == width * height * bpp`.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct DecodedRows {
    pub format: ImageFormat,
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

/// The pixmap form of a decoded image, or `None` for a pixel type without one; `indexed`
/// is the palette kind of an indexed file (see `alien_palette`).
///
/// Example: `decoded_rows(&DynamicImage::new_rgb8(1, 1), None).unwrap().format == ImageFormat::Bgr`.
pub(super) fn decoded_rows(
    image: &DynamicImage,
    indexed: Option<IndexedKind>,
) -> Option<DecodedRows> {
    let (format, pixels) = match indexed {
        Some(kind) => indexed_pixels(image, kind),
        None => direct_pixels(image)?,
    };
    let (width, height) = (image.width() as usize, image.height() as usize);
    Some(DecodedRows {
        format,
        width,
        height,
        pixels,
    })
}

/// A gray ramp's index is the red channel of its decoded colour; other palettes drop alpha.
fn indexed_pixels(image: &DynamicImage, kind: IndexedKind) -> (ImageFormat, Vec<u8>) {
    let rgb = image.to_rgb8();
    match kind {
        IndexedKind::GrayRamp => (
            ImageFormat::Gray,
            rgb.as_raw()
                .as_chunks::<3>()
                .0
                .iter()
                .map(|p| p[0])
                .collect(),
        ),
        IndexedKind::Colour => (ImageFormat::Bgr, reversed_colour(rgb.as_raw(), 3)),
    }
}

fn direct_pixels(image: &DynamicImage) -> Option<(ImageFormat, Vec<u8>)> {
    Some(match image {
        DynamicImage::ImageLuma8(gray) => (ImageFormat::Gray, gray.as_raw().clone()),
        DynamicImage::ImageLuma16(gray) => (ImageFormat::Gray16, native_u16(gray.as_raw())),
        DynamicImage::ImageRgb8(rgb) => (ImageFormat::Bgr, reversed_colour(rgb.as_raw(), 3)),
        DynamicImage::ImageRgba8(rgba) => four_channel(rgba.as_raw()),
        DynamicImage::ImageLumaA8(_) => four_channel(image.to_rgba8().as_raw()),
        DynamicImage::ImageRgb16(_)
        | DynamicImage::ImageRgba16(_)
        | DynamicImage::ImageLumaA16(_) => (ImageFormat::Bgr, high_bytes_bgr(image)),
        DynamicImage::ImageRgb32F(rgb) => (ImageFormat::Rgbf, native_f32(rgb.as_raw())),
        DynamicImage::ImageRgba32F(rgba) => (ImageFormat::Rgbaf, native_f32(rgba.as_raw())),
        _ => return None,
    })
}

/// RGB(A) bytes with red and blue exchanged in each `channels`-byte pixel.
fn reversed_colour(bytes: &[u8], channels: usize) -> Vec<u8> {
    let mut out = bytes.to_vec();
    out.chunks_exact_mut(channels)
        .for_each(|pixel| pixel.swap(0, 2));
    out
}

/// `Bgra`, or `Bgr32` when no pixel is transparent, so opaque files stay opaque.
fn four_channel(rgba: &[u8]) -> (ImageFormat, Vec<u8>) {
    let opaque = rgba
        .as_chunks::<4>()
        .0
        .iter()
        .all(|pixel| pixel[3] == u8::MAX);
    let format = if opaque {
        ImageFormat::Bgr32
    } else {
        ImageFormat::Bgra
    };
    (format, reversed_colour(rgba, 4))
}

fn high_bytes_bgr(image: &DynamicImage) -> Vec<u8> {
    let rgb = image.to_rgb16();
    let mut out = Vec::with_capacity(rgb.as_raw().len());
    for pixel in rgb.as_raw().as_chunks::<3>().0 {
        out.extend([pixel[2], pixel[1], pixel[0]].map(|channel| (channel >> 8) as u8));
    }
    out
}

fn native_u16(values: &[u16]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_ne_bytes())
        .collect()
}

fn native_f32(values: &[f32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_ne_bytes())
        .collect()
}

/// The 8-bit image `encode` writes for `pixmap` in `format`, top row first; `None` for a
/// pixmap format without an 8-bit form. JPEG and GIF take three channels.
///
/// Example: `encodable_image(&pixmap, CodecFormat::Png)`.
pub(super) fn encodable_image(pixmap: &ImagePixMap, format: CodecFormat) -> Option<DynamicImage> {
    let image = eight_bit_image(pixmap)?;
    if matches!(format, CodecFormat::Jpeg | CodecFormat::Gif) {
        return Some(DynamicImage::ImageRgb8(image.to_rgb8()));
    }
    Some(image)
}

fn eight_bit_image(pixmap: &ImagePixMap) -> Option<DynamicImage> {
    use ImageFormat::*;
    let format = pixmap.format();
    let (width, height) = (pixmap.width() as u32, pixmap.height() as u32);
    let pixels = logical_pixels(pixmap);
    Some(match format {
        Gray | Alpha => DynamicImage::ImageLuma8(GrayImage::from_raw(width, height, pixels)?),
        Gray16 | GrayF | AlphaF => {
            let scaled = linear_bytes(&scalar_values(&pixels, format));
            DynamicImage::ImageLuma8(GrayImage::from_raw(width, height, scaled)?)
        }
        Rgb | Bgr | Rgb32 | Bgr32 | Rgbf | Bgrf => {
            let rgb = colour_bytes(&pixels, format, 3);
            DynamicImage::ImageRgb8(RgbImage::from_raw(width, height, rgb)?)
        }
        Rgba | Bgra | Rgbaf | Bgraf => {
            let rgba = colour_bytes(&pixels, format, 4);
            DynamicImage::ImageRgba8(RgbaImage::from_raw(width, height, rgba)?)
        }
        _ => return None,
    })
}

/// Every pixel's bytes, rows top first, without row padding.
fn logical_pixels(pixmap: &ImagePixMap) -> Vec<u8> {
    let state = pixmap.data();
    let mut out = Vec::with_capacity(state.data.size_x * state.data.size_y * state.data.size_bpp);
    for row in 0..state.data.size_y {
        for col in 0..state.data.size_x {
            out.extend_from_slice(state.data.value(row, col).unwrap_or_default());
        }
    }
    out
}

/// The single channel of 16-bit or float gray pixels as `f64`.
fn scalar_values(pixels: &[u8], format: ImageFormat) -> Vec<f64> {
    if format == ImageFormat::Gray16 {
        let values = pixels
            .as_chunks::<2>()
            .0
            .iter()
            .map(|b| u16::from_ne_bytes(*b));
        return values.map(f64::from).collect();
    }
    let values = pixels
        .as_chunks::<4>()
        .0
        .iter()
        .map(|b| f32::from_ne_bytes(*b));
    values.map(f64::from).collect()
}

/// `values` mapped linearly from their [min, max] onto 0..255, rounded; a constant image
/// maps through [0, 255] unchanged, as a range of zero width has no scale.
fn linear_bytes(values: &[f64]) -> Vec<u8> {
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let (min, max) = if max > min { (min, max) } else { (0.0, 255.0) };
    let scale = 255.0 / (max - min);
    values
        .iter()
        .map(|value| (scale * (value - min) + 0.5) as u8)
        .collect()
}

/// RGB(A) bytes in red-green-blue order from any colour pixmap format, `channels` per pixel;
/// float channels clamp to [0, 1] and round.
fn colour_bytes(pixels: &[u8], format: ImageFormat, channels: usize) -> Vec<u8> {
    use ImageFormat::*;
    let float = matches!(format, Rgbf | Bgrf | Rgbaf | Bgraf);
    let stride = ImagePixMap::size_pixel_bytes_of(format);
    let component = if float { 4 } else { 1 };
    let reversed = matches!(format, Bgr | Bgr32 | Bgra | Bgrf | Bgraf);
    let mut out = Vec::with_capacity(pixels.len() / stride * channels);
    for pixel in pixels.chunks_exact(stride) {
        let read = |index: usize| channel_byte(&pixel[index * component..], float);
        let colour = [read(0), read(1), read(2)];
        out.extend(if reversed {
            [colour[2], colour[1], colour[0]]
        } else {
            colour
        });
        if channels == 4 {
            out.push(read(3));
        }
    }
    out
}

fn channel_byte(bytes: &[u8], float: bool) -> u8 {
    if !float {
        return bytes[0];
    }
    let value = f32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    (255.0 * value.clamp(0.0, 1.0) + 0.5) as u8
}

#[cfg(test)]
#[path = "alien_pixels_tests.rs"]
mod tests;
