use super::*;
use crate::{PrimitiveDraw, RenderFrame, RenderLayer, RenderMaterial, RenderShading, RenderVertex};

#[test]
fn v4_rejects_enabled_nonzero_units_without_a_declared_unit_scale() {
    let frame = frame_with_offset(1, 0., 2.);
    let error = validate_frame(
        RenderTriangleCoverageBehavior::NativeGrid12V4 {
            origin: crate::RenderTriangleGridOrigin::TopDown,
        },
        RenderDepthBiasBehavior::HardwareFormatV1,
        &frame,
    )
    .unwrap_err();
    assert_eq!(
        error.depth_bias_failure(),
        Some(&RenderDepthBiasFailure::UnresolvedHardwareUnits {
            units_bits: 2.0f32.to_bits(),
        })
    );
    assert!(error.to_string().contains("Native24UnitsV2"));
}

#[test]
fn v4_accepts_zero_or_inactive_units_and_explicit_native24_units() {
    for (mode, units, behavior) in [
        (1, 0., RenderDepthBiasBehavior::HardwareFormatV1),
        (0, 2., RenderDepthBiasBehavior::HardwareFormatV1),
        (1, 2., RenderDepthBiasBehavior::Native24UnitsV2),
    ] {
        assert!(
            validate_frame(
                RenderTriangleCoverageBehavior::NativeGrid12V4 {
                    origin: crate::RenderTriangleGridOrigin::TopDown,
                },
                behavior,
                &frame_with_offset(mode, 1., units),
            )
            .is_ok()
        );
    }
}

#[test]
fn v4_refuses_finite_slope_factors_outside_the_proven_arithmetic_bound() {
    let frame = frame_with_offset(1, 1.0e26, 0.);
    let error = validate_frame(
        RenderTriangleCoverageBehavior::NativeGrid12V4 {
            origin: crate::RenderTriangleGridOrigin::TopDown,
        },
        RenderDepthBiasBehavior::Native24UnitsV2,
        &frame,
    )
    .unwrap_err();
    assert_eq!(
        error.depth_bias_failure(),
        Some(&RenderDepthBiasFailure::UnboundedSlopeFactor {
            factor_bits: 1.0e26f32.to_bits(),
        })
    );
}

#[test]
fn v4_worst_case_bias_arithmetic_remains_finite_before_saturation() {
    let maximum_slope = martensite_cmath::powi(f64::from(2.0_f32), 44) as f32;
    let factor_component = maximum_slope * super::MAX_V4_SLOPE_FACTOR;
    let units_component = f32::MAX * (1.0 / 8_388_608.0);
    assert!(factor_component.is_finite());
    assert!((factor_component + units_component + 1.0).is_finite());
}

#[test]
fn v4_slope_factor_bound_keeps_positive_and_negative_edges() {
    assert!(super::refuse_slope_factor(1.0e25).is_ok());
    assert!(super::refuse_slope_factor(-1.0e25).is_ok());
    assert!(super::refuse_slope_factor(1.0e25 * 2.0).is_err());
}

#[test]
fn v4_checks_the_effective_temporary_shadow_units() {
    let frame = shadow_offset_frame();
    let error = validate_frame(
        RenderTriangleCoverageBehavior::NativeGrid12V4 {
            origin: crate::RenderTriangleGridOrigin::TopDown,
        },
        RenderDepthBiasBehavior::HardwareFormatV1,
        &frame,
    )
    .unwrap_err();
    assert_eq!(
        error.depth_bias_failure(),
        Some(&RenderDepthBiasFailure::UnresolvedHardwareUnits {
            units_bits: 3.0f32.to_bits(),
        })
    );
}

#[test]
fn v4_preflight_preserves_offset_transitions_from_nonselected_draws() {
    let frame = frame_with_prior_offset();
    let error = validate_frame(
        RenderTriangleCoverageBehavior::NativeGrid12V4 {
            origin: crate::RenderTriangleGridOrigin::TopDown,
        },
        RenderDepthBiasBehavior::HardwareFormatV1,
        &frame,
    )
    .unwrap_err();
    assert_eq!(
        error.depth_bias_failure(),
        Some(&RenderDepthBiasFailure::UnresolvedHardwareUnits {
            units_bits: 9.0f32.to_bits(),
        })
    );
}

fn shadow_offset_frame() -> RenderFrame {
    let mut draw = phong_draw(triangle_vertices())
        .with_polygon_offset(crate::RenderPolygonOffset::new(1, 1., 2.).unwrap());
    draw.text_shadow_bias = true;
    RenderFrame::with_layers(
        8,
        8,
        [0.; 4],
        vec![RenderLayer::new(vec![draw], true, true, false)],
    )
    .unwrap()
}

fn frame_with_prior_offset() -> RenderFrame {
    let first = PrimitiveDraw::new(triangle_vertices(), vec![0, 1, 2], crate::identity_clip())
        .unwrap()
        .with_polygon_offset(crate::RenderPolygonOffset::new(1, 0., 9.).unwrap());
    let selected = phong_draw(triangle_vertices())
        .with_polygon_offset(crate::RenderPolygonOffset::new(8, 1., 1.).unwrap());
    RenderFrame::with_layers(
        8,
        8,
        [0.; 4],
        vec![RenderLayer::new(vec![first, selected], true, true, false)],
    )
    .unwrap()
}

fn frame_with_offset(mode: i32, factor: f32, units: f32) -> RenderFrame {
    let vertices = vec![
        RenderVertex::new([-0.5, -0.5, 0.], [1.; 4]),
        RenderVertex::new([0.5, -0.5, 0.], [1.; 4]),
        RenderVertex::new([0., 0.5, 0.], [1.; 4]),
    ];
    let draw = phong_draw(vertices)
        .with_polygon_offset(crate::RenderPolygonOffset::new(mode, factor, units).unwrap());
    let layer = RenderLayer::new(vec![draw], true, true, false);
    RenderFrame::with_layers(8, 8, [0.; 4], vec![layer]).unwrap()
}

fn phong_draw(vertices: Vec<RenderVertex>) -> PrimitiveDraw {
    let mut draw = PrimitiveDraw::new(vertices, vec![0, 1, 2], crate::identity_clip()).unwrap();
    let lighting = crate::RenderLighting::new(
        vec![[0., 0., 1.]; 3],
        RenderMaterial::default(),
        crate::identity_clip(),
        crate::identity_clip(),
        false,
        RenderShading::Phong,
    )
    .unwrap();
    draw = draw.with_lighting(lighting).unwrap();
    draw
}

fn triangle_vertices() -> Vec<RenderVertex> {
    vec![
        RenderVertex::new([-0.5, -0.5, 0.], [1.; 4]),
        RenderVertex::new([0.5, -0.5, 0.], [1.; 4]),
        RenderVertex::new([0., 0.5, 0.], [1.; 4]),
    ]
}
