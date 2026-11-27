//! Ordinary encoded color readback has floating storage without adding HDR radiance.
use super::{capture_bytes, dump_capture};
use crate::FrameCapture;
use martensite_tkservice::{
    graphic3d::Graphic3dBufferType,
    image::{ImageFormat, ImagePixMap},
};

fn float_capture_fixture() -> FrameCapture {
    FrameCapture {
        width: 2,
        height: 2,
        rgba: vec![64, 128, 192, 255, 64, 128, 192, 255, 1, 2, 3, 4, 1, 2, 3, 4],
        depth: vec![0.25; 4],
    }
}

fn assert_normalized_byte_values(bytes: &[u8]) {
    for (value, encoded) in bytes.as_chunks::<4>().0.iter().zip(0..=255) {
        let actual = f32::from_ne_bytes(*value);
        let expected = (f64::from(encoded) / 255.) as f32;
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
    assert_eq!(&bytes[..4], &0_f32.to_ne_bytes());
    assert_eq!(&bytes[1020..], &1_f32.to_ne_bytes());
}

#[test]
fn encoded_rgba_normalization_covers_every_byte_without_gamma_decode() {
    let mut capture = float_capture_fixture();
    capture.width = 64;
    capture.height = 1;
    capture.depth = vec![0.; 64];
    capture.rgba = (0..=255).collect();
    for kind in [
        Graphic3dBufferType::Rgba,
        Graphic3dBufferType::Rgb,
        Graphic3dBufferType::Red,
    ] {
        let bytes = capture_bytes(&kind, ImageFormat::Rgbaf, &capture).unwrap();
        assert_normalized_byte_values(&bytes);
    }
}

fn assert_float_logical_rows(image: &ImagePixMap) {
    let storage = image.data();
    for (row, values) in [[64_u8, 128, 192, 255], [1, 2, 3, 4]].iter().enumerate() {
        let bytes: Vec<u8> = values
            .iter()
            .cycle()
            .take(8)
            .flat_map(|value| (f32::from(*value) / 255.).to_ne_bytes())
            .collect();
        assert_eq!(&storage.data.row(row).unwrap()[..32], bytes);
        assert_eq!(&storage.data.row(row).unwrap()[32..], &[167; 4]);
    }
}

#[test]
fn float_destination_preserves_padding_orientation_and_completed_capture() {
    let capture = float_capture_fixture();
    for top_down in [false, true] {
        let image = ImagePixMap::default();
        image.set_top_down(top_down);
        assert!(image.init_zero(ImageFormat::Rgbaf, 2, 2, 36, 167));
        assert!(dump_capture(&image, &Graphic3dBufferType::Rgba, &capture));
        assert_float_logical_rows(&image);
        let expected_first = if top_down { 64_f32 } else { 1_f32 } / 255.;
        assert_eq!(
            &image.data().data.data()[..4],
            &expected_first.to_ne_bytes()
        );
    }
    assert_eq!(capture.rgba, float_capture_fixture().rgba);
}

fn assert_float_kind_rejected(image: &ImagePixMap, kind: Graphic3dBufferType) {
    let original = image.data().clone();
    let capture = float_capture_fixture();
    assert!(!dump_capture(image, &kind, &capture));
    assert_eq!(*image.data(), original);
}

#[test]
fn float_destination_rejections_leave_every_destination_byte_unchanged() {
    let image = ImagePixMap::default();
    assert!(image.init_zero(ImageFormat::Rgbaf, 2, 2, 36, 167));
    for kind in [
        Graphic3dBufferType::Depth,
        Graphic3dBufferType::ShadowMap,
        Graphic3dBufferType::RgbRayTraceHdrLeft,
    ] {
        assert_float_kind_rejected(&image, kind);
    }
    let original = image.data().clone();
    let mut malformed = float_capture_fixture();
    malformed.rgba.pop();
    assert!(!dump_capture(
        &image,
        &Graphic3dBufferType::Rgba,
        &malformed
    ));
    assert_eq!(*image.data(), original);
}
