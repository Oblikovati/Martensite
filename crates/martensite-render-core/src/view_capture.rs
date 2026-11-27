//! Copies checked GPU captures into caller-owned ImagePixMap rows, including padded/bottom-up images.
use crate::FrameCapture;
use crate::RenderError;
use martensite_tkservice::{
    graphic3d::Graphic3dBufferType,
    image::{ImageFormat, ImagePixMap, ImagePixMapData},
};
use std::fmt::Write as _;

/// The host-copy steps of one redraw's capture, kept until its pixels are read (#594).
pub struct HostCaptureSteps {
    /// The logical frame size.
    pub size: [u32; 2],
    /// The offscreen capture a render scale blitted, whose depth the capture takes.
    pub scaled: Option<FrameCapture>,
    pub subview_blits: Vec<crate::subview_composition::RenderSubviewBlit>,
    pub flip_output: bool,
}

impl HostCaptureSteps {
    /// Takes the subview blits of a rendered `frame`; `HostCaptureSteps::taken(&mut frame, scaled)`.
    pub fn taken(frame: &mut crate::RenderFrame, scaled: Option<FrameCapture>) -> Self {
        Self {
            size: [frame.width, frame.height],
            scaled,
            subview_blits: std::mem::take(&mut frame.subview_blits),
            flip_output: frame.flip_output,
        }
    }
    /// Applies the steps in GlView order: the scaled depth blit, the subview blits, then the
    /// flipped output; `steps.apply(&mut capture)`.
    pub fn apply(&self, capture: &mut FrameCapture) {
        if let Some(scaled) = &self.scaled {
            capture.depth = crate::render_scale::blit_depth(scaled, self.size);
        }
        self.apply_color(capture);
    }
    /// Applies the steps that change colour, the subview blits and the flipped output, to a
    /// capture whose depth may be empty; `steps.apply_color(&mut capture)`.
    pub fn apply_color(&self, capture: &mut FrameCapture) {
        crate::subview_composition::apply_subview_blits(capture, &self.subview_blits);
        if self.flip_output {
            capture.reverse_rows();
        }
    }
}

pub fn report_capture_failure(error: &RenderError) {
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

pub fn dump_capture(
    image: &ImagePixMap,
    kind: &Graphic3dBufferType,
    capture: &FrameCapture,
) -> bool {
    if checked_dump_pixels(kind, capture).is_none()
        || image.width() != capture.width as usize
        || image.height() != capture.height as usize
    {
        return false;
    }
    let format = image.format();
    if (*kind, format) == (Graphic3dBufferType::Rgb, ImageFormat::Rgb) {
        return dump_rgb_rows(&mut image.change_data().data, capture);
    }
    let Some(bytes) = capture_bytes(kind, format, capture) else {
        return false;
    };
    let row_size = capture.width as usize * image.size_pixel_bytes();
    let mut target = image.change_data();
    copy_capture_rows(&mut target.data, &bytes, row_size)
}

fn dump_rgb_rows(target: &mut ImagePixMapData, capture: &FrameCapture) -> bool {
    let width = capture.width as usize;
    for (y, source) in capture.rgba.chunks_exact(width * 4).enumerate() {
        let Some(destination) = target.change_row(y) else {
            return false;
        };
        rgba_row_to_rgb(source, &mut destination[..width * 3]);
    }
    true
}
/// Drop the alpha byte of every pixel in a row; `rgba_row_to_rgb(&rgba, &mut rgb)`. Four pixels
/// become three little-endian words instead of twelve byte copies (#594).
pub fn rgba_row_to_rgb(rgba: &[u8], rgb: &mut [u8]) {
    let (quads, rest) = rgba.as_chunks::<16>();
    let (triples, rgb_rest) = rgb.as_chunks_mut::<12>();
    for (out, quad) in triples.iter_mut().zip(quads) {
        *out = rgb_quad(quad);
    }
    for (out, pixel) in rgb_rest
        .as_chunks_mut::<3>()
        .0
        .iter_mut()
        .zip(rest.as_chunks::<4>().0)
    {
        *out = [pixel[0], pixel[1], pixel[2]];
    }
}
/// Four pixels without their alpha bytes, as three little-endian words.
fn rgb_quad(quad: &[u8; 16]) -> [u8; 12] {
    let word = |at: usize| u32::from_le_bytes([quad[at], quad[at + 1], quad[at + 2], quad[at + 3]]);
    let [a, b, c, d] = [word(0), word(4), word(8), word(12)];
    let packed = [
        (a & 0x00ff_ffff) | (b << 24),
        ((b >> 8) & 0xffff) | (c << 16),
        ((c >> 16) & 0xff) | (d << 8),
    ];
    let mut out = [0; 12];
    for (slot, value) in out.as_chunks_mut::<4>().0.iter_mut().zip(packed) {
        *slot = value.to_le_bytes();
    }
    out
}
/// Whether a dump of `kind` into `image` can take packed RGB rows from the device.
pub fn is_direct_rgb_dump(image: &ImagePixMap, kind: &Graphic3dBufferType) -> bool {
    (*kind, image.format()) == (Graphic3dBufferType::Rgb, ImageFormat::Rgb)
}
/// Copy device-packed RGB rows into `image`, or `None` when the frame must be dumped from
/// its capture instead; `dump_rgb_rows_from(&image, &publication, &device)`.
pub fn dump_rgb_rows_from(
    image: &ImagePixMap,
    publication: &crate::redraw_capture::RedrawCapture,
    device: &crate::driver_backend::SharedRenderDevice,
) -> Option<Result<bool, RenderError>> {
    let [width, height] = publication.size();
    if width == 0
        || height == 0
        || image.width() != width as usize
        || image.height() != height as usize
    {
        return Some(Ok(false));
    }
    let mut target = image.change_data();
    let mut write_row = |y: usize, row: &[u8]| {
        target
            .data
            .change_row(y)
            .map(|destination| destination[..row.len()].copy_from_slice(row))
            .is_some()
    };
    publication.dump_rgb(device, &mut write_row)
}
pub fn copy_capture_rows(target: &mut ImagePixMapData, bytes: &[u8], row_size: usize) -> bool {
    for (y, row) in bytes.chunks_exact(row_size).enumerate() {
        let Some(destination) = target.change_row(y) else {
            return false;
        };
        destination[..row_size].copy_from_slice(row);
    }
    true
}
pub fn capture_bytes(
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
/// The attachments a dump of `kind` reads; only a depth dump needs the depth copy (#594).
pub fn dump_planes(kind: &Graphic3dBufferType) -> crate::device_backend::CapturePlanes {
    match kind {
        Graphic3dBufferType::Depth => crate::device_backend::CapturePlanes::ColorAndDepth,
        _ => crate::device_backend::CapturePlanes::Color,
    }
}
/// The pixel count of a capture whose planes hold what a dump of `kind` reads.
fn checked_dump_pixels(kind: &Graphic3dBufferType, capture: &FrameCapture) -> Option<usize> {
    if dump_planes(kind) == crate::device_backend::CapturePlanes::ColorAndDepth {
        return checked_capture_pixels(capture);
    }
    // An empty depth plane was not read; any other depth must still match the colour.
    let pixels = (capture.width as usize).checked_mul(capture.height as usize)?;
    (pixels != 0
        && pixels.checked_mul(4) == Some(capture.rgba.len())
        && (capture.depth.is_empty() || capture.depth.len() == pixels))
        .then_some(pixels)
}
pub fn checked_capture_pixels(capture: &FrameCapture) -> Option<usize> {
    let pixels = (capture.width as usize).checked_mul(capture.height as usize)?;
    (pixels != 0
        && pixels.checked_mul(4) == Some(capture.rgba.len())
        && capture.depth.len() == pixels)
        .then_some(pixels)
}

pub fn capture_image_format(kind: &Graphic3dBufferType) -> Option<ImageFormat> {
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
