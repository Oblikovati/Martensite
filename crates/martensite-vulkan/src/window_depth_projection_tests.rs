use super::*;
const REFERENCE: RenderViewportGridPrecision = RenderViewportGridPrecision::Fractional12;
const RASTER: RenderViewportGridPrecision = RenderViewportGridPrecision::Fractional8;
const VIEWPORT: [f32; 4] = [0., 0., 409., 409.];

fn clip_triangle() -> [[f32; 4]; 3] {
    [
        [-0.4, -0.3, 0.2, 1.],
        [0.3, -0.2, 0.4, 1.],
        [-0.2, 0.4, 0.6, 1.],
    ]
}

#[test]
fn compensation_preserves_every_xy_w_word() {
    let clip = clip_triangle();
    let corrected = compensated_clip_triangle(clip, VIEWPORT, false, REFERENCE, RASTER).unwrap();
    for (original, output) in clip.into_iter().zip(corrected) {
        assert_eq!(
            [original[0], original[1], original[3]].map(f32::to_bits),
            [output[0], output[1], output[3]].map(f32::to_bits)
        );
        assert!((0. ..=output[3]).contains(&output[2]));
    }
}

#[test]
fn perspective_w_and_negative_viewport_are_supported_explicitly() {
    let clip = clip_triangle().map(|p| p.map(|v| v * 2.));
    let corrected =
        compensated_clip_triangle(clip, [0., 409., 409., -409.], false, REFERENCE, RASTER).unwrap();
    assert!(
        corrected
            .iter()
            .all(|p| p[3] == 2. && p[2] > 0. && p[2] < 2.)
    );
}

#[test]
fn clip_boundaries_and_nonpositive_w_are_not_silently_clamped() {
    for rejected in [[2., 0., 0.5, 1.], [0., 0., -0.1, 1.], [0., 0., 0., 0.]] {
        let mut clip = clip_triangle();
        clip[0] = rejected;
        assert!(matches!(
            compensated_clip_triangle(clip, VIEWPORT, false, REFERENCE, RASTER),
            Err(WindowDepthProjectionFailure::Clip { vertex: 0, .. })
        ));
    }
}

#[test]
fn viewport_overflow_and_nonfinite_values_refuse_before_publication() {
    for viewport in [[0., 0., 0., 409.], [0., 0., f32::INFINITY, 409.]] {
        assert!(matches!(
            compensated_clip_triangle(clip_triangle(), viewport, false, REFERENCE, RASTER),
            Err(WindowDepthProjectionFailure::Viewport { .. })
        ));
    }
    assert!(matches!(
        compensated_clip_triangle(
            clip_triangle(),
            [f32::MAX, 0., f32::MAX, 409.],
            false,
            REFERENCE,
            RASTER
        ),
        Err(WindowDepthProjectionFailure::WindowRange { .. })
    ));
}

#[test]
fn sample_relevant_grid_collapse_remains_an_explicit_refusal() {
    let clip = [
        [0., 0., 0.2, 1.],
        [0.000001, 0., 0.3, 1.],
        [0., 0.000001, 0.4, 1.],
    ];
    assert!(matches!(
        compensated_clip_triangle(clip, VIEWPORT, false, REFERENCE, RASTER),
        Err(WindowDepthProjectionFailure::Plane(
            WindowDepthPlaneFailure::Degenerate { .. }
        ))
    ));
}
