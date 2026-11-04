//! Copies checked GPU captures into caller-owned ImagePixMap rows, including padded/bottom-up images.
use crate::FrameCapture;
use crate::RenderError;
use martensite_tkservice::{
    graphic3d::Graphic3dBufferType,
    image::{ImageFormat, ImagePixMap, ImagePixMapData},
};
use std::fmt::Write as _;

pub(crate) fn report_capture_failure(error: &RenderError) {
    eprintln!("{}", capture_failure_record(error));
}

fn capture_failure_record(error: &RenderError) -> String {
    let cause = std::error::Error::source(error).map_or_else(
        || "null".to_owned(),
        |source| json_string(&source.to_string()),
    );
    format!(
        "{{\"event\":\"vulkan_frame_capture_failed\",\"error\":{},\"cause\":{cause}}}",
        json_string(&error.to_string())
    )
}

fn json_string(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len() + 2);
    encoded.push('"');
    for character in value.chars() {
        encode_json_character(character, &mut encoded);
    }
    encoded.push('"');
    encoded
}

fn encode_json_character(character: char, output: &mut String) {
    match character {
        '"' => output.push_str("\\\""),
        '\\' => output.push_str("\\\\"),
        '\n' => output.push_str("\\n"),
        '\r' => output.push_str("\\r"),
        '\t' => output.push_str("\\t"),
        value if value < ' ' => write!(output, "\\u{:04x}", value as u32).unwrap(),
        value => output.push(value),
    }
}

pub(crate) fn dump_capture(
    image: &ImagePixMap,
    kind: &Graphic3dBufferType,
    capture: &FrameCapture,
) -> bool {
    if checked_capture_pixels(capture).is_none()
        || image.width() != capture.width as usize
        || image.height() != capture.height as usize
    {
        return false;
    }
    let Some(bytes) = capture_bytes(kind, image.format(), capture) else {
        return false;
    };
    let row_size = capture.width as usize * image.size_pixel_bytes();
    let mut target = image.change_data();
    copy_capture_rows(&mut target.data, &bytes, row_size)
}
pub(crate) fn copy_capture_rows(
    target: &mut ImagePixMapData,
    bytes: &[u8],
    row_size: usize,
) -> bool {
    for (y, row) in bytes.chunks_exact(row_size).enumerate() {
        let Some(destination) = target.change_row(y) else {
            return false;
        };
        destination[..row_size].copy_from_slice(row);
    }
    true
}
pub(crate) fn capture_bytes(
    kind: &Graphic3dBufferType,
    format: ImageFormat,
    capture: &FrameCapture,
) -> Option<Vec<u8>> {
    if let Some(channels) = float_color_channel_count(kind, format) {
        return Some(color_float_bytes(capture, channels));
    }
    if format != capture_image_format(kind)? {
        return None;
    }
    byte_or_depth_capture_bytes(kind, capture)
}

fn byte_or_depth_capture_bytes(
    kind: &Graphic3dBufferType,
    capture: &FrameCapture,
) -> Option<Vec<u8>> {
    match kind {
        Graphic3dBufferType::Rgba => Some(capture.rgba.clone()),
        Graphic3dBufferType::Rgb => Some(color_channels(capture, 3)),
        Graphic3dBufferType::Red => Some(color_channels(capture, 1)),
        Graphic3dBufferType::Depth => {
            Some(capture.depth.iter().flat_map(|v| v.to_ne_bytes()).collect())
        }
        _ => None,
    }
}

fn float_color_channel_count(kind: &Graphic3dBufferType, format: ImageFormat) -> Option<usize> {
    if !matches!(
        kind,
        Graphic3dBufferType::Rgba | Graphic3dBufferType::Rgb | Graphic3dBufferType::Red
    ) {
        return None;
    }
    // Destination storage chooses channels; GrayF color reads red rather than luminance.
    match format {
        ImageFormat::Rgbaf => Some(4),
        ImageFormat::Rgbf => Some(3),
        ImageFormat::GrayF => Some(1),
        _ => None,
    }
}

fn color_float_bytes(capture: &FrameCapture, channels: usize) -> Vec<u8> {
    // Normalize encoded bytes without implying HDR radiance or a gamma decode.
    capture
        .rgba
        .as_chunks::<4>()
        .0
        .iter()
        .flat_map(|pixel| pixel[..channels].iter())
        .flat_map(|value| (f32::from(*value) / 255.).to_ne_bytes())
        .collect()
}

fn color_channels(capture: &FrameCapture, count: usize) -> Vec<u8> {
    capture
        .rgba
        .as_chunks::<4>()
        .0
        .iter()
        .flat_map(|pixel| pixel[..count].iter().copied())
        .collect()
}
pub(crate) fn checked_capture_pixels(capture: &FrameCapture) -> Option<usize> {
    let pixels = (capture.width as usize).checked_mul(capture.height as usize)?;
    (pixels != 0
        && pixels.checked_mul(4) == Some(capture.rgba.len())
        && capture.depth.len() == pixels)
        .then_some(pixels)
}

pub(crate) fn capture_image_format(kind: &Graphic3dBufferType) -> Option<ImageFormat> {
    match kind {
        Graphic3dBufferType::Rgba => Some(ImageFormat::Rgba),
        Graphic3dBufferType::Rgb => Some(ImageFormat::Rgb),
        Graphic3dBufferType::Red => Some(ImageFormat::Gray),
        Graphic3dBufferType::Depth => Some(ImageFormat::GrayF),
        _ => None,
    }
}

#[cfg(test)]
#[path = "view_capture_error_tests.rs"]
mod error_tests;

#[cfg(test)]
#[path = "view_capture_float_tests.rs"]
mod float_tests;

#[cfg(test)]
#[path = "view_capture_color_float_tests.rs"]
mod color_float_tests;
