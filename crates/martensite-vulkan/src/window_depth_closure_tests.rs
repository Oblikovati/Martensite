use super::*;
const REFERENCE: RenderViewportGridPrecision = RenderViewportGridPrecision::Fractional12;
const RASTER: RenderViewportGridPrecision = RenderViewportGridPrecision::Fractional8;

fn triangle() -> [[Bound; 4]; 3] {
    [
        [-0.4, -0.3, 0.2, 1.],
        [0.3, -0.2, 0.4, 1.],
        [-0.2, 0.4, 0.6, 1.],
    ]
    .map(|vertex| vertex.map(|value| Bound::source(value).unwrap()))
}

#[test]
fn full_rounding_and_ftz_hull_proves_the_nonempty_operation() {
    assert_eq!(
        prove_window_depth_closure(triangle(), [0., 0., 409., 409.], false, REFERENCE, RASTER),
        Ok(WindowDepthClosure::ReferencePlane)
    );
}

#[test]
fn exact_physical_empty_triangle_completes_without_reference_division() {
    let clip = [[0.0001, 0.0001, 0.2, 1.]; 3].map(|v| v.map(|x| Bound::source(x).unwrap()));
    assert_eq!(
        prove_window_depth_closure(clip, [0., 0., 409., 409.], false, REFERENCE, RASTER),
        Ok(WindowDepthClosure::RasterEmpty)
    );
}

#[test]
fn original_clipping_and_viewport_arithmetic_overflow_refuse() {
    let mut clipped = triangle();
    clipped[0][2] = Bound::source(-0.1).unwrap();
    assert_eq!(
        prove_window_depth_closure(clipped, [0., 0., 409., 409.], false, REFERENCE, RASTER),
        Err(WindowDepthClosureFailure::OriginalClip)
    );
    assert_eq!(
        prove_window_depth_closure(
            triangle(),
            [f32::MAX, 0., f32::MAX, 409.],
            false,
            REFERENCE,
            RASTER
        ),
        Err(WindowDepthClosureFailure::Arithmetic)
    );
}

#[test]
fn negative_viewport_and_gl_clip_mapping_have_explicit_closure() {
    let mut clip = triangle();
    for vertex in &mut clip {
        vertex[2] = Bound::source(0.2).unwrap();
    }
    assert_eq!(
        prove_window_depth_closure(clip, [0., 409., 409., -409.], true, REFERENCE, RASTER),
        Ok(WindowDepthClosure::ReferencePlane)
    );
}

#[test]
fn verified_identical_source_pair_preserves_correlation_without_bypassing_clip_or_ranges() {
    let x = 0.00390625_f32 / 409.;
    let clip = [[x, 0.2, 0.4, 1.]; 3].map(|v| v.map(|x| Bound::source(x).unwrap()));
    assert_eq!(
        prove_fragment_window_depth_closure(
            clip,
            [0., 0., 409., 409.],
            false,
            REFERENCE,
            RASTER,
            false
        ),
        Ok(WindowDepthClosure::RequiresActualValidation)
    );
    assert_eq!(
        prove_fragment_window_depth_closure(
            clip,
            [0., 0., 409., 409.],
            false,
            REFERENCE,
            RASTER,
            true
        ),
        Ok(WindowDepthClosure::RasterEmpty)
    );
    let mut clipped = clip;
    clipped[0][2] = Bound::source(-0.1).unwrap();
    assert_eq!(
        prove_fragment_window_depth_closure(
            clipped,
            [0., 0., 409., 409.],
            false,
            REFERENCE,
            RASTER,
            true
        ),
        Err(WindowDepthClosureFailure::OriginalClip)
    );
    assert_eq!(
        prove_fragment_window_depth_closure(
            clip,
            [f32::MAX, 0., f32::MAX, 409.],
            false,
            REFERENCE,
            RASTER,
            true
        ),
        Err(WindowDepthClosureFailure::Arithmetic)
    );
}
#[test]
fn fragment_evaluation_proves_its_own_finite_plane_instead_of_unused_corrected_clip_z() {
    let mut clip = triangle();
    clip[0][2] = Bound::source(0.).unwrap();
    assert_eq!(
        prove_window_depth_closure(clip, [0., 0., 409., 409.], false, REFERENCE, RASTER),
        Err(WindowDepthClosureFailure::CorrectedClip)
    );
    assert_eq!(
        prove_fragment_window_depth_closure(
            clip,
            [0., 0., 409., 409.],
            false,
            REFERENCE,
            RASTER,
            false
        ),
        Ok(WindowDepthClosure::ReferencePlane)
    );
}
