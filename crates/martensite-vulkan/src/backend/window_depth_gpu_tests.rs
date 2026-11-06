//! A closed triangle operation checks activation, coverage and all independent bias terms.
use super::{VulkanOptions, VulkanRenderer, depth_plane_operand_gpu_tests::operand_frame};
use crate::{
    RenderBackend, RenderBehaviorConfiguration, RenderCoordinateConvention as Coordinates,
    RenderDepthBiasBehavior as Bias, RenderViewportGridPrecision as Grid,
    RenderWindowDepthBehavior as Depth,
};

fn selected() -> Depth {
    Depth::UnsnappedWindowDepthV2 {
        reference_grid: Grid::Fractional12,
        raster_grid: Grid::Fractional8,
    }
}

fn renderer(bias: Bias, depth: Depth) -> VulkanRenderer {
    VulkanRenderer::new_with_render_behavior(
        VulkanOptions {
            validation: true,
            device_index: Some(0),
        },
        crate::RenderRasterPolicy::LastProvokingVertexV2,
        Coordinates::LegacyBottomUpV1,
        Default::default(),
        RenderBehaviorConfiguration::default()
            .with_depth_bias(bias)
            .with_window_depth(depth),
    )
    .unwrap()
}

#[test]
fn reference_depth_records_each_bias_term_without_changing_coverage() {
    for bias in [Bias::HardwareFormatV1, Bias::Native24UnitsV2] {
        let mut baseline = renderer(bias, Depth::HardwareV1);
        let mut corrected = renderer(bias, selected());
        for (factor, units) in [(0., 0.), (1., 0.), (0., 1.), (1., 1.)] {
            compare_operation(&mut baseline, &mut corrected, bias, factor, units);
        }
        assert!(baseline.validation_errors().unwrap().is_empty());
        assert!(corrected.validation_errors().unwrap().is_empty());
    }
}

fn compare_operation(
    baseline: &mut VulkanRenderer,
    corrected: &mut VulkanRenderer,
    bias: Bias,
    factor: f32,
    units: f32,
) {
    let source = operand_frame(factor, units);
    let old = baseline.render(&source).unwrap();
    let actual = corrected.render(&source).unwrap();
    assert_eq!(
        actual.rgba, old.rgba,
        "window-depth compensation changed XY coverage"
    );
    let pixel = 186 * 409 + 185;
    assert_ne!(
        actual.depth[pixel].to_bits(),
        old.depth[pixel].to_bits(),
        "selected depth operation was inactive"
    );
    println!(
        "{{\"kind\":\"window-depth-operand\",\"device\":{:?},\"format\":\"{:?}\",\"bias\":\"{bias:?}\",\"factor\":{factor},\"units\":{units},\"depth_bits\":{}}}",
        corrected.runtime.name,
        corrected.runtime.depth.formats.plain,
        actual.depth[pixel].to_bits()
    );
}

#[test]
fn physical_empty_triangle_completes_with_unchanged_color_and_depth() {
    let mut frame = operand_frame(0., 0.);
    for vertex in &mut frame.draws[0].vertices {
        vertex.position = [0.0001, 0.0001, 0.2];
    }
    let mut baseline = renderer(Bias::HardwareFormatV1, Depth::HardwareV1);
    let mut corrected = renderer(Bias::HardwareFormatV1, selected());
    let expected = baseline.render(&frame).unwrap();
    let actual = corrected.render(&frame).unwrap();
    assert_eq!(actual.rgba, expected.rgba);
    assert_eq!(actual.depth, expected.depth);
    assert!(corrected.validation_errors().unwrap().is_empty());
}

#[test]
fn direct_fragment_depth_records_reference_plane_and_effective_native_bias() {
    let mut baseline = renderer(Bias::Native24UnitsV2, Depth::HardwareV1);
    let mut direct = renderer(
        Bias::Native24UnitsV2,
        Depth::ReferenceWindowDepthV3 {
            reference_grid: Grid::Fractional12,
            raster_grid: Grid::Fractional8,
        },
    );
    for (factor, units) in [(0., 0.), (1., 0.), (0., 1.), (1., 1.)] {
        compare_operation(
            &mut baseline,
            &mut direct,
            Bias::Native24UnitsV2,
            factor,
            units,
        );
    }
    assert!(direct.validation_errors().unwrap().is_empty());
}

#[test]
fn invalid_actual_reference_plane_refuses_before_capture_publication() {
    let mut frame = operand_frame(0., 0.);
    let window = [
        [100., 100.],
        [200.00048828125, 133.33349609375],
        [300.0009765625, 166.6669921875],
    ];
    for (vertex, point) in frame.draws[0].vertices.iter_mut().zip(window) {
        vertex.position = [
            (2. * point[0] / 409. - 1.) as f32,
            (2. * point[1] / 409. - 1.) as f32,
            0.2,
        ];
    }
    let mut direct = renderer(
        Bias::Native24UnitsV2,
        Depth::ReferenceWindowDepthV3 {
            reference_grid: Grid::Fractional12,
            raster_grid: Grid::Fractional8,
        },
    );
    let error = match direct.render(&frame) {
        Err(error) => error,
        Ok(_) => panic!("logical Q12 degenerate with nonempty Q8 must not publish"),
    };
    assert!(
        matches!(error.window_depth_failure(),Some(crate::RenderWindowDepthFailure::GpuDraw{draw:0,cause})
        if matches!(cause.as_ref(),crate::RenderWindowDepthFailure::GpuTriangle{triangle:0,flags,..} if flags&4!=0))
    );
    assert!(direct.validation_errors().unwrap().is_empty());
}

#[test]
fn overflowing_effective_bias_refuses_but_disabled_bias_does_not_leak_failure() {
    let mut frame = operand_frame(f32::MAX, 0.);
    let window = [
        [185_f64, 185_f64, -0.8],
        [185.01, 185., 0.8],
        [185., 185.01, -0.8],
    ];
    for (vertex, p) in frame.draws[0].vertices.iter_mut().zip(window) {
        vertex.position = [
            (2. * p[0] / 409. - 1.) as f32,
            (2. * p[1] / 409. - 1.) as f32,
            p[2] as f32,
        ];
    }
    let policy = Depth::ReferenceWindowDepthV3 {
        reference_grid: Grid::Fractional12,
        raster_grid: Grid::Fractional8,
    };
    let mut direct = renderer(Bias::Native24UnitsV2, policy);
    let error = match direct.render(&frame) {
        Err(error) => error,
        Ok(_) => {
            panic!("finite factor times slope greater than one must refuse before publication")
        }
    };
    assert!(
        matches!(error.window_depth_failure(),Some(crate::RenderWindowDepthFailure::GpuDraw{draw:0,cause})
        if matches!(cause.as_ref(),crate::RenderWindowDepthFailure::GpuTriangle{triangle:0,flags,..} if flags&64!=0))
    );
    frame.draws[0] = frame.draws[0]
        .clone()
        .with_polygon_offset(crate::RenderPolygonOffset::new(0, f32::MAX, 0.).unwrap());
    assert!(direct.render(&frame).is_ok());
    assert!(direct.validation_errors().unwrap().is_empty());
}
