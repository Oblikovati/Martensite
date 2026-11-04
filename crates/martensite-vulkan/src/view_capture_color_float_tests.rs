//! Floating color destinations select stored channels without changing encoded color values.
use super::{capture_bytes, dump_capture};
use crate::FrameCapture;
use martensite_tkservice::{
    graphic3d::Graphic3dBufferType,
    image::{ImageFormat, ImagePixMap},
};

fn channel_ramp_capture() -> FrameCapture {
    FrameCapture {
        width: 256,
        height: 1,
        rgba: (0..=255_u8)
            .flat_map(|red| [red, red.wrapping_add(37), 255 - red, 19])
            .collect(),
        depth: vec![0.25; 256],
    }
}

fn ordinary_color_kinds() -> [Graphic3dBufferType; 3] {
    [
        Graphic3dBufferType::Rgba,
        Graphic3dBufferType::Rgb,
        Graphic3dBufferType::Red,
    ]
}

fn float_color_layouts() -> [(ImageFormat, usize); 3] {
    [
        (ImageFormat::Rgbaf, 4),
        (ImageFormat::Rgbf, 3),
        (ImageFormat::GrayF, 1),
    ]
}

fn expected_float_channel_bytes(capture: &FrameCapture, channels: usize) -> Vec<u8> {
    capture
        .rgba
        .as_chunks::<4>()
        .0
        .iter()
        .flat_map(|pixel| pixel[..channels].iter())
        .flat_map(|value| ((f64::from(*value) / 255.) as f32).to_ne_bytes())
        .collect()
}

#[test]
fn float_color_layouts_preserve_all_encoded_channel_values() {
    let capture = channel_ramp_capture();
    for kind in ordinary_color_kinds() {
        for (format, channels) in float_color_layouts() {
            let expected = expected_float_channel_bytes(&capture, channels);
            assert_eq!(capture_bytes(&kind, format, &capture), Some(expected));
        }
    }
}

fn padded_float_destination(format: ImageFormat, channels: usize, top_down: bool) -> ImagePixMap {
    let image = ImagePixMap::default();
    image.set_top_down(top_down);
    assert!(image.init_zero(format, 256, 2, 256 * channels * 4 + 8, 167));
    image
}

fn two_row_channel_capture() -> FrameCapture {
    let mut capture = channel_ramp_capture();
    capture.height = 2;
    capture
        .rgba
        .extend(channel_ramp_capture().rgba.into_iter().rev());
    capture.depth.resize(512, 0.75);
    capture
}

fn assert_padded_float_rows(image: &ImagePixMap, expected: &[u8], row_bytes: usize) {
    let storage = image.data();
    for (index, source) in expected.chunks_exact(row_bytes).enumerate() {
        let destination = storage.data.row(index).unwrap();
        assert_eq!(&destination[..row_bytes], source);
        assert_eq!(&destination[row_bytes..], &[167; 8]);
    }
    let first_row = if image.is_top_down() { 0 } else { row_bytes };
    assert_eq!(
        &storage.data.data()[..row_bytes],
        &expected[first_row..first_row + row_bytes]
    );
}

#[test]
fn float_color_rows_preserve_padding_orientation_and_source_ownership() {
    let capture = two_row_channel_capture();
    for (format, channels) in float_color_layouts() {
        assert_float_layout_rows(&capture, format, channels);
    }
    assert_eq!(capture.rgba, two_row_channel_capture().rgba);
    assert_eq!(capture.depth, two_row_channel_capture().depth);
}

fn assert_float_layout_rows(capture: &FrameCapture, format: ImageFormat, channels: usize) {
    let expected = expected_float_channel_bytes(capture, channels);
    for top_down in [false, true] {
        for kind in ordinary_color_kinds() {
            let image = padded_float_destination(format, channels, top_down);
            assert!(dump_capture(&image, &kind, capture));
            assert_padded_float_rows(&image, &expected, 256 * channels * 4);
        }
    }
}

fn assert_float_refusal_unchanged(
    image: &ImagePixMap,
    kind: Graphic3dBufferType,
    capture: &FrameCapture,
) {
    let before = image.data().clone();
    assert!(!dump_capture(image, &kind, capture));
    assert_eq!(*image.data(), before);
}

#[test]
fn float_color_invalid_and_unsupported_destinations_remain_unchanged() {
    let capture = two_row_channel_capture();
    for (format, channels) in float_color_layouts() {
        let image = padded_float_destination(format, channels, true);
        for kind in [
            Graphic3dBufferType::ShadowMap,
            Graphic3dBufferType::RgbRayTraceHdrLeft,
        ] {
            assert_float_refusal_unchanged(&image, kind, &capture);
        }
        assert_invalid_float_captures(&image);
    }
}

fn assert_invalid_float_captures(image: &ImagePixMap) {
    let mut malformed = two_row_channel_capture();
    malformed.rgba.pop();
    assert_float_refusal_unchanged(image, Graphic3dBufferType::Rgb, &malformed);
    malformed = two_row_channel_capture();
    malformed.depth.pop();
    assert_float_refusal_unchanged(image, Graphic3dBufferType::Red, &malformed);
    malformed = channel_ramp_capture();
    assert_float_refusal_unchanged(image, Graphic3dBufferType::Rgba, &malformed);
}
