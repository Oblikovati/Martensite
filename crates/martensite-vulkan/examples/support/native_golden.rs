//! Device-bound recorded assets are replayed in Rust alone.
#[path = "../../tests/support/reference_cpu.rs"]
mod reference_cpu;
use martensite_vulkan::FrameCapture;
/// Check exact native RGB/alpha and bounded depth; `assert_capture(&capture, rgba, depth, mode)`.
pub fn assert_capture(capture: &FrameCapture, rgba: &[u8], depth: &[u8], mode: &str) {
    assert_eq!(
        capture.rgba.len(),
        rgba.len(),
        "received wrong byte count for {mode}; expected native shape"
    );
    let difference = capture
        .rgba
        .iter()
        .zip(rgba)
        .position(|(actual, native)| actual != native);
    reference_cpu::require_exact(difference.is_none(), || {
        format!(
            "received different pixels for {mode} at byte {difference:?}; expected pinned native RGBA"
        )
    });
    assert_golden_depth(&capture.depth, depth, mode);
}
/// Preserve the native depth gate independently of RGB; `assert_golden_depth(&values, bytes, mode)`.
pub fn assert_golden_depth(actual: &[f32], expected: &[u8], mode: &str) {
    assert_eq!(actual.len() * 4, expected.len());
    for (actual, bytes) in actual.iter().zip(expected.as_chunks::<4>().0) {
        let native = f32::from_le_bytes(*bytes);
        assert!(
            (actual - native).abs() <= 1e-6,
            "received {actual} depth for {mode}; expected {native} within 1e-6"
        );
    }
}
#[cfg(test)]
support_unit_tests! {
    #[path = "native_golden_tests.rs"]
    mod tests;
}
