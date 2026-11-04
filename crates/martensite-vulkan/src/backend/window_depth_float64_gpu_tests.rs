//! Exact closed forms exercise the real V5 transport, bias and completion route.
use super::{VulkanOptions, VulkanRenderer, depth_plane_operand_gpu_tests::operand_frame};
use crate::{
    RenderBackend, RenderBehaviorConfiguration, RenderCoordinateConvention,
    RenderDepthBiasBehavior, RenderViewportGridPrecision as Grid,
    RenderWindowDepthBehavior as Depth,
};

fn behavior() -> Depth {
    Depth::Float64ReferenceWindowDepthV5 {
        reference_grid: Grid::Fractional12,
        raster_grid: Grid::Fractional8,
    }
}
fn renderer(depth: Depth) -> VulkanRenderer {
    VulkanRenderer::new_with_render_behavior(
        VulkanOptions {
            validation: true,
            device_index: Some(0),
        },
        crate::RenderRasterPolicy::LastProvokingVertexV2,
        RenderCoordinateConvention::LegacyBottomUpV1,
        Default::default(),
        RenderBehaviorConfiguration::default()
            .with_depth_bias(RenderDepthBiasBehavior::Native24UnitsV2)
            .with_window_depth(depth),
    )
    .unwrap()
}
#[test]
fn float64_closed_form_plane_and_four_bias_terms_preserve_coverage() {
    let mut actual = renderer(behavior());
    let mut baseline = renderer(Depth::HardwareV1);
    for (factor, units) in [(0., 0.), (1., 0.), (0., 1.), (1., 1.)] {
        let mut frame = operand_frame(factor, units);
        let points = [[-0.5, -0.5, -0.5], [0.5, -0.5, 0.0], [-0.5, 0.5, 0.5]];
        for (vertex, point) in frame.draws[0].vertices.iter_mut().zip(points) {
            vertex.position = point;
        }
        let old = baseline.render(&frame).unwrap();
        let result = actual.render(&frame).unwrap();
        assert_eq!(result.rgba, old.rgba);
        let depth = 0.25 + (0.25 / 204.5 * (180.5 - 102.25) + 0.5 / 204.5 * (178.5 - 102.25));
        let expected =
            (depth + 0.5 / 204.5 * f64::from(factor) + f64::from(units) / 8388608.) as f32;
        assert_eq!(result.depth[230 * 409 + 180].to_bits(), expected.to_bits());
    }
    assert!(actual.validation_errors().unwrap().is_empty());
}
#[test]
fn float64_actual_pair_uses_raw_clip_and_preserves_rational_sector_order() {
    let mut actual = renderer(behavior());
    let mut frame = operand_frame(1., 1.);
    let ring = crate::window_depth_float64_fixture::RING.map(|p| [p[0], p[1], p[2]]);
    let sector = crate::window_depth_float64_fixture::SECTOR.map(|p| [p[0], p[1], p[2]]);
    for (vertex, p) in frame.draws[0].vertices.iter_mut().zip(ring) {
        vertex.position = p;
        vertex.color = [1., 0., 0., 1.];
    }
    let mut front = frame.draws[0].clone();
    for (vertex, p) in front.vertices.iter_mut().zip(sector) {
        vertex.position = p;
        vertex.color = [0., 1., 0., 0.5];
    }
    frame.draws.push(front.with_surface_blending(true).unwrap());
    let frame =
        crate::RenderFrame::new(frame.width, frame.height, frame.clear_color, frame.draws).unwrap();
    let image = actual.render(&frame).unwrap();
    assert!(
        image.rgba[(186 * 409 + 185) * 4 + 1] > 0,
        "rationally nearer sector must pass ring"
    );
    assert!(actual.validation_errors().unwrap().is_empty());
}

#[test]
fn float64_reference_degeneracy_refuses_and_declared_empty_stays_finite() {
    let mut actual = renderer(behavior());
    let mut frame = operand_frame(0., 0.);
    let window = [
        [100., 100.],
        [200.00048828125, 133.33349609375],
        [300.0009765625, 166.6669921875],
    ];
    for (vertex, p) in frame.draws[0].vertices.iter_mut().zip(window) {
        vertex.position = [
            (2. * p[0] / 409. - 1.) as f32,
            (2. * p[1] / 409. - 1.) as f32,
            0.2,
        ];
    }
    let error = actual
        .render(&frame)
        .expect_err("Q12-degenerate nonempty raster must refuse");
    assert!(
        matches!(error.window_depth_failure(),Some(crate::RenderWindowDepthFailure::GpuDraw{cause,..})
        if matches!(cause.as_ref(),crate::RenderWindowDepthFailure::GpuTriangle{flags,..} if flags&4!=0))
    );
    for vertex in &mut frame.draws[0].vertices {
        vertex.position = [0.1, 0.1, 0.2];
    }
    let baseline = renderer(Depth::HardwareV1).render(&frame).unwrap();
    let image = actual.render(&frame).unwrap();
    assert_eq!(image.rgba, baseline.rgba);
    assert_eq!(image.depth, baseline.depth);
    assert!(actual.validation_errors().unwrap().is_empty());
}
