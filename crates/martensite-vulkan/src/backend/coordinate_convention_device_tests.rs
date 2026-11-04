//! Four independently selected conventions preserve one closed-form captured image.
use super::*;
use crate::{
    PrimitiveDraw, RenderCoordinateConvention, RenderPipelinePolicies, RenderRasterPolicy,
    RenderVertex,
};

#[path = "line_depth_interpolation_device_tests.rs"]
mod line_depth_interpolation;

fn convention_renderer(
    raster: RenderRasterPolicy,
    coordinates: RenderCoordinateConvention,
) -> VulkanRenderer {
    let options = coordinate_device_options();
    VulkanRenderer::new_with_conventions(options, raster, coordinates).unwrap()
}

fn coordinate_device_options() -> VulkanOptions {
    // Every cell must exercise the same device and validation contract.
    VulkanOptions {
        validation: true,
        device_index: Some(0),
    }
}

fn asymmetric_coordinate_frame() -> RenderFrame {
    let points = [
        [-0.75, -0.5, 0.],
        [0.25, -0.5, 0.],
        [0.25, 0.75, 0.],
        [-0.75, 0.75, 0.],
    ];
    let vertices = points
        .map(|p| RenderVertex::new(p, [1., 0., 0., 1.]))
        .to_vec();
    let draw =
        PrimitiveDraw::new(vertices, vec![0, 1, 2, 0, 2, 3], crate::identity_clip()).unwrap();
    RenderFrame::new(16, 16, [0., 0., 1., 1.], vec![draw]).unwrap()
}

fn assert_asymmetric_capture(capture: &FrameCapture, half: f32) {
    for (index, pixel) in capture.rgba.as_chunks::<4>().0.iter().enumerate() {
        let x = index % 16;
        let y = index / 16;
        let inside = (2..10).contains(&x) && (2..12).contains(&y);
        let expected = if inside {
            [255, 0, 0, 255]
        } else {
            [0, 0, 255, 255]
        };
        assert_eq!(*pixel, expected, "pixel {x},{y}");
        assert_eq!(
            capture.depth[index],
            if inside { half } else { 1. },
            "depth {x},{y}"
        );
    }
}

#[test]
fn four_coordinate_provoking_cells_keep_logical_capture_and_owned_selection() {
    for raster in [
        RenderRasterPolicy::LegacyFirstV1,
        RenderRasterPolicy::LastProvokingVertexV2,
    ] {
        for coordinates in [
            RenderCoordinateConvention::LegacyBottomUpV1,
            RenderCoordinateConvention::TopLeftLogicalV2,
        ] {
            assert_coordinate_cell(raster, coordinates);
        }
    }
}

fn assert_coordinate_cell(raster: RenderRasterPolicy, coordinates: RenderCoordinateConvention) {
    let mut renderer = convention_renderer(raster, coordinates);
    assert_eq!(renderer.raster_policy(), raster);
    assert_eq!(renderer.coordinate_convention(), coordinates);
    let half = renderer.stored_depth(0.5);
    assert_asymmetric_capture(
        &renderer.render(&asymmetric_coordinate_frame()).unwrap(),
        half,
    );
    assert!(
        renderer.validation_errors().unwrap().is_empty(),
        "{:?}",
        renderer.validation_errors().unwrap()
    );
}

#[test]
fn additive_driver_coordinate_selection_keeps_independent_raster_choice() {
    let selection = RenderPipelinePolicies {
        raster: RenderRasterPolicy::LastProvokingVertexV2,
        ..Default::default()
    };
    let coordinates = RenderCoordinateConvention::TopLeftLogicalV2;
    let driver = crate::Graphic3dRenderDriver::with_coordinate_configuration(
        coordinate_device_options(),
        selection,
        coordinates,
    )
    .unwrap();
    assert_eq!(driver.coordinate_convention(), Some(coordinates));
    assert_eq!(driver.raster_policy(), Some(selection.raster));
    assert!(
        driver.validation_errors().unwrap().is_empty(),
        "{:?}",
        driver.validation_errors().unwrap()
    );
}

#[test]
fn four_coordinate_cells_keep_real_multisample_resolve_orientation() {
    for raster in [
        RenderRasterPolicy::LegacyFirstV1,
        RenderRasterPolicy::LastProvokingVertexV2,
    ] {
        for coordinates in [
            RenderCoordinateConvention::LegacyBottomUpV1,
            RenderCoordinateConvention::TopLeftLogicalV2,
        ] {
            assert_multisample_cell(raster, coordinates);
        }
    }
}
fn assert_multisample_cell(raster: RenderRasterPolicy, coordinates: RenderCoordinateConvention) {
    let mut renderer = convention_renderer(raster, coordinates);
    let mut frame = asymmetric_coordinate_frame();
    frame.raster.requested_samples = 4;
    let plan = super::frame_raster_plan::frame_raster_plan(&renderer.runtime, &frame, false);
    assert_eq!(plan.samples, ash::vk::SampleCountFlags::TYPE_4);
    let half = renderer.stored_depth(0.5);
    assert_asymmetric_capture(&renderer.render(&frame).unwrap(), half);
    assert!(
        renderer.validation_errors().unwrap().is_empty(),
        "{:?}",
        renderer.validation_errors().unwrap()
    );
}
