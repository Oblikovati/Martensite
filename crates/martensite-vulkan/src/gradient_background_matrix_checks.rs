//! Depth/color invariants are independent of exact native raster comparisons.
use super::matrix::GradientMatrixCase;
use crate::FrameCapture;

/// Preserve foreground state across background changes; `assert_matrix_pair(case, &gradient, &solid)`.
pub(super) fn assert_matrix_pair(
    case: GradientMatrixCase,
    gradient: &FrameCapture,
    solid: &FrameCapture,
) {
    assert_eq!([gradient.width, gradient.height], case.extent);
    assert_eq!([solid.width, solid.height], case.extent);
    assert_eq!(
        gradient.depth, solid.depth,
        "{}: background changed model depth",
        case.label
    );
    let (pixels, remainder) = gradient.rgba.as_chunks::<4>();
    assert!(remainder.is_empty());
    assert!(pixels.iter().all(|pixel| pixel[3] == 255));
    assert_foreground_color_depth(case, gradient, solid);
}
fn assert_foreground_color_depth(
    case: GradientMatrixCase,
    gradient: &FrameCapture,
    solid: &FrameCapture,
) {
    let (colors, _) = gradient.rgba.as_chunks::<4>();
    let (cleared, _) = solid.rgba.as_chunks::<4>();
    let mut covered = 0;
    for ((color, clear), depth) in colors.iter().zip(cleared).zip(&gradient.depth) {
        if *depth == 1. {
            continue;
        }
        covered += 1;
        assert_foreground_pixel(color, clear, *depth);
    }
    let expected = case.foreground;
    let label = case.label;
    assert_eq!(covered > 0, expected, "{label}: covered={covered}");
}
fn assert_foreground_pixel(color: &[u8; 4], cleared: &[u8; 4], depth: f32) {
    assert_eq!(color, &[255, 0, 0, 255]);
    assert_eq!(color, cleared);
    // Eye20, modelZ0 and near/far1/100 give the independent orthographic depth19/99.
    assert!(
        (depth - 19. / 99.).abs() <= 1e-6,
        "orthographic foreground depth={depth}; expected19/99 within1e-6"
    );
}
