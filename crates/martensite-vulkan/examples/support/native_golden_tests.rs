use super::*;
#[test]
fn native_shape_and_depth_gates() {
    let capture = FrameCapture {
        width: 1,
        height: 1,
        rgba: vec![0, 0, 0, 255],
        depth: vec![0.25],
    };
    assert_capture(&capture, &[0, 0, 0, 255], &0.25f32.to_le_bytes(), "test");
    assert_golden_depth(&[0.2500005], &0.25f32.to_le_bytes(), "test");
}
