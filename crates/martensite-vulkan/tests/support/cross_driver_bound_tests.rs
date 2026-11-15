use super::*;
fn capture(rgba: Vec<u8>, depth: Vec<f32>) -> FrameCapture {
    FrameCapture {
        width: 2,
        height: 1,
        rgba,
        depth,
    }
}
fn bytes(depth: [f32; 2]) -> Vec<u8> {
    depth.iter().flat_map(|v| v.to_le_bytes()).collect()
}
const NATIVE: [u8; 8] = [0, 0, 0, 255, 100, 50, 25, 255];
#[test]
fn one_level_colour_and_depth_inside_the_snapping_bound_pass() {
    // Slope 0.256 per pixel: the bound is 1e-6 + 0.256 / 256 = 1.001e-3.
    let actual = capture(
        vec![0, 0, 0, 255, 101, 49, 25, 255],
        vec![0.5, 0.756 + 1e-3],
    );
    assert_cross_driver(&actual, &NATIVE, &bytes([0.5, 0.756]), "unit");
}
#[test]
fn mask_alpha_two_levels_or_depth_beyond_the_bound_fail() {
    for (rgba, depth) in [
        (vec![1, 0, 0, 255, 100, 50, 25, 255], [0.5, 0.756]),
        (vec![0, 0, 0, 255, 100, 50, 25, 254], [0.5, 0.756]),
        (vec![0, 0, 0, 255, 102, 50, 25, 255], [0.5, 0.756]),
        (vec![0, 0, 0, 255, 100, 50, 25, 255], [0.5, 0.756 + 2e-3]),
    ] {
        let actual = capture(rgba, depth.to_vec());
        let failed = shading_violation(&actual, &NATIVE)
            .or_else(|| depth_violation(&actual, &bytes([0.5, 0.756])));
        assert!(failed.is_some(), "{actual:?}");
    }
}
#[test]
fn background_samples_have_no_slope_allowance() {
    let actual = capture(NATIVE.to_vec(), vec![1. - 1e-3, 0.5]);
    assert!(depth_violation(&actual, &bytes([1., 0.5])).is_some());
}
