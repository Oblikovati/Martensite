//! Owned closed-form bias contracts; run only under the canonical GPU lock.
use super::{VulkanOptions, VulkanRenderer};
use crate::{
    FrameCapture, PrimitiveDraw, RenderBackend, RenderBehaviorConfiguration,
    RenderCoordinateConvention as Coordinates, RenderDepthBiasBehavior as Behavior, RenderFrame,
    RenderPolygonOffset, RenderVertex, identity_clip,
};
const UNIT: f32 = 1.0 / 8_388_608.0;
const BASES: [f32; 4] = [1.0 / 1_048_576.0, 0.25, 0.5, 0.75];
#[test]
fn native24_unit_sweep_is_independent_of_exponent_origin_and_winding() {
    for origin in [Coordinates::LegacyBottomUpV1, Coordinates::TopLeftLogicalV2] {
        let mut fixed = renderer(Behavior::Native24UnitsV2, origin);
        let mut native = renderer(Behavior::HardwareFormatV1, origin);
        assert_eq!(fixed.runtime.depth, native.runtime.depth);
        assert!(fixed.runtime.depth_bias_command.is_some());
        assert!(native.runtime.depth_bias_command.is_none());
        for reverse in [false, true] {
            unit_sweep(&mut fixed, &mut native, reverse);
        }
        assert!(fixed.validation_errors().unwrap().is_empty());
        assert!(native.validation_errors().unwrap().is_empty());
    }
}
fn unit_sweep(fixed: &mut VulkanRenderer, native: &mut VulkanRenderer, reverse: bool) {
    for base in BASES {
        for units in [-2., -1., 0., 1., 2.] {
            let frame = frame(vec![plane(base, 0., units, 0., reverse)]);
            let selected = fixed.render(&frame).unwrap();
            assert_flat_depth(&selected, stored(fixed, base + units * UNIT));
            let control = native.render(&frame).unwrap();
            let exponent = ((base.to_bits() >> 23) & 255) as i32 - 127;
            let float_r = martensite_cmath::powi(f64::from(2.0_f32), exponent - 23) as f32;
            if is_float(native) {
                assert_flat_depth(&control, stored(native, base + units * float_r));
            } else if units == 0. {
                // V1 fixed-point r is implementation-defined; only V2 declares an absolute unit.
                assert_flat_depth(&control, stored(native, base));
            }
        }
    }
}
#[test]
fn native24_preserves_analytic_slopes_and_signed_combined_bias() {
    let mut fixed = renderer(Behavior::Native24UnitsV2, Coordinates::TopLeftLogicalV2);
    let mut native = renderer(Behavior::HardwareFormatV1, Coordinates::TopLeftLogicalV2);
    for factor in [-2., -1., 0., 1., 2.] {
        for units in [-2., 0., 2.] {
            let actual = fixed
                .render(&frame(vec![plane(0.5, 1.0 / 64.0, units, factor, false)]))
                .unwrap();
            assert_sloped_depth(&fixed, &actual, units, factor);
            if units == 0. {
                let control = native
                    .render(&frame(vec![plane(0.5, 1.0 / 64.0, 0., factor, false)]))
                    .unwrap();
                assert_eq!(actual.depth, control.depth);
                assert_eq!(actual.rgba, control.rgba);
            }
        }
    }
    assert!(fixed.validation_errors().unwrap().is_empty());
    assert!(native.validation_errors().unwrap().is_empty());
}
fn assert_sloped_depth(renderer: &VulkanRenderer, capture: &FrameCapture, units: f32, factor: f32) {
    for y in 0..8 {
        for x in 0..8 {
            let expected = 0.5 + (x as f32 + 0.5 - 4.) / 64. + factor / 64. + units * UNIT;
            assert_eq!(
                capture.depth[y * 8 + x],
                stored(renderer, expected),
                "sample=({x},{y}), units={units}, factor={factor}"
            );
        }
    }
}
#[test]
fn native24_constant_offset_clamps_only_at_window_depth_boundaries() {
    let mut renderer = renderer(Behavior::Native24UnitsV2, Coordinates::TopLeftLogicalV2);
    assert!(
        renderer.runtime.depth_clamp,
        "boundary control requires enabled depthClamp"
    );
    for base in [0., 1.] {
        for units in [-2., 0., 2.] {
            let capture = renderer
                .render(&clamped_boundary_frame(base, units))
                .unwrap();
            assert_flat_depth(
                &capture,
                stored(&renderer, (base + units * UNIT).clamp(0., 1.)),
            );
        }
    }
    assert!(renderer.validation_errors().unwrap().is_empty());
}
#[test]
fn native24_depth_order_allows_later_unbiased_point_to_cover_offset_plane() {
    for behavior in [Behavior::HardwareFormatV1, Behavior::Native24UnitsV2] {
        let mut renderer = renderer(behavior, Coordinates::TopLeftLogicalV2);
        let capture = renderer
            .render(&frame(vec![plane(BASES[0], 0., 1., 0., false), marker()]))
            .unwrap();
        let blue = capture
            .rgba
            .as_chunks::<4>()
            .0
            .iter()
            .filter(|p| **p == [0, 0, 255, 255])
            .count();
        if behavior == Behavior::Native24UnitsV2 {
            assert_eq!(blue, 1);
        } else if is_float(&renderer) {
            assert_eq!(blue, 0);
        }
        assert!(renderer.validation_errors().unwrap().is_empty());
    }
}
fn renderer(behavior: Behavior, origin: Coordinates) -> VulkanRenderer {
    let config = RenderBehaviorConfiguration::default()
        .with_depth_bias(behavior)
        .with_point_raster(crate::RenderPointRasterBehavior::FixedGridSprites12V4);
    VulkanRenderer::new_with_render_behavior(
        VulkanOptions {
            validation: true,
            device_index: Some(0),
        },
        Default::default(),
        origin,
        Default::default(),
        config,
    )
    .unwrap()
}
fn plane(base: f32, slope: f32, units: f32, factor: f32, reverse: bool) -> PrimitiveDraw {
    let vertices = [[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]].map(|[x, y]| {
        RenderVertex::new([x, y, 2. * (base + 4. * x * slope) - 1.], [1., 0., 0., 1.])
    });
    let indices = if reverse {
        vec![2, 1, 0, 3, 2, 0]
    } else {
        vec![0, 1, 2, 0, 2, 3]
    };
    PrimitiveDraw::new(vertices.to_vec(), indices, identity_clip())
        .unwrap()
        .with_polygon_offset(RenderPolygonOffset::new(1, factor, units).unwrap())
}
fn clamped_boundary_frame(base: f32, units: f32) -> RenderFrame {
    let mut draw = plane(base, 0., units, 0., false);
    draw.depth_clamp = true;
    frame(vec![draw])
}
fn marker() -> PrimitiveDraw {
    let vertex = RenderVertex::new(
        [0.125, 0.125, 2. * (BASES[0] + 1.0 / 33_554_432.0) - 1.],
        [0., 0., 1., 1.],
    );
    let style = crate::PointStyle::new(1., [1, 1], vec![[1.; 4]]).unwrap();
    PrimitiveDraw::points(vec![vertex], vec![0], identity_clip(), style)
        .unwrap()
        .with_polygon_offset(RenderPolygonOffset::new(1, 99., 99.).unwrap())
}
fn frame(draws: Vec<PrimitiveDraw>) -> RenderFrame {
    RenderFrame::new(8, 8, [0., 0., 0., 1.], draws).unwrap()
}
fn assert_flat_depth(capture: &FrameCapture, expected: f32) {
    assert!(
        capture.depth.iter().all(|v| *v == expected),
        "expected={expected:?}, actual={:?}",
        capture.depth
    );
    assert!(
        capture
            .rgba
            .as_chunks::<4>()
            .0
            .iter()
            .all(|p| *p == [255, 0, 0, 255])
    );
}
fn is_float(renderer: &VulkanRenderer) -> bool {
    renderer.runtime.depth.formats.plain == ash::vk::Format::D32_SFLOAT
}
pub(super) fn stored(renderer: &VulkanRenderer, depth: f32) -> f32 {
    if is_float(renderer) {
        return depth;
    }
    ((f64::from(depth) * 16_777_215.).round() / 16_777_215.) as f32
}
