use super::{
    VulkanRenderer,
    triangle_grid_coverage_gpu_tests::{
        hardware_behavior, native_grid_behavior, renderer_with_coordinates,
        triangle_draw_with_sides, window_clip,
    },
};
use crate::{
    RenderBackend, RenderCoordinateConvention, RenderFaceCulling, RenderFrame, RenderVertex,
};

#[test]
fn native_grid_v4_front_face_matches_hardware_across_draw_conventions() {
    if super::phong_interpolation_device::skip_without_barycentric() {
        return;
    }
    for coordinates in [
        RenderCoordinateConvention::LegacyBottomUpV1,
        RenderCoordinateConvention::TopLeftLogicalV2,
    ] {
        assert_facing_origin_matches_hardware(coordinates);
    }
}

fn assert_facing_origin_matches_hardware(coordinates: RenderCoordinateConvention) {
    let mut hardware = renderer_with_coordinates(hardware_behavior(), coordinates);
    let mut selected = renderer_with_coordinates(native_grid_behavior(), coordinates);
    for reverse in [false, true] {
        for mirrored in [false, true] {
            assert_facing_cases_match(&mut hardware, &mut selected, reverse, mirrored);
        }
    }
    assert!(hardware.validation_errors().unwrap().is_empty());
    assert!(selected.validation_errors().unwrap().is_empty());
}

fn assert_facing_cases_match(
    hardware: &mut VulkanRenderer,
    selected: &mut VulkanRenderer,
    reverse: bool,
    mirrored: bool,
) {
    let frame = broad_two_sided_frame(reverse, mirrored, RenderFaceCulling::None);
    let control = hardware.render(&frame).unwrap();
    let actual = selected.render(&frame).unwrap();
    assert_interior_matches(
        &actual,
        &control,
        reverse,
        mirrored,
        RenderFaceCulling::None,
    );
    let visible = interior_pixel(&control);
    assert!(matches!(visible, [255, 0, 0, 255] | [0, 255, 0, 255]));
    assert_cull_modes_match(hardware, selected, reverse, mirrored, visible);
}

fn assert_cull_modes_match(
    hardware: &mut VulkanRenderer,
    selected: &mut VulkanRenderer,
    reverse: bool,
    mirrored: bool,
    visible: [u8; 4],
) {
    let mut culled = 0;
    for culling in [RenderFaceCulling::Front, RenderFaceCulling::Back] {
        let frame = broad_two_sided_frame(reverse, mirrored, culling);
        let control = hardware.render(&frame).unwrap();
        let actual = selected.render(&frame).unwrap();
        assert_interior_matches(&actual, &control, reverse, mirrored, culling);
        let pixel = interior_pixel(&control);
        culled += usize::from(pixel == [0, 0, 0, 255]);
        assert!(pixel == visible || pixel == [0, 0, 0, 255]);
    }
    assert_eq!(
        culled, 1,
        "one of front/back culling must remove the triangle"
    );
}

fn broad_two_sided_frame(reverse: bool, mirrored: bool, culling: RenderFaceCulling) -> RenderFrame {
    let points = [[8.0, 8.0], [56.0, 8.0], [8.0, 56.0]];
    let order = if reverse { [0, 2, 1] } else { [0, 1, 2] };
    let vertices = order.map(|index| RenderVertex::new(window_clip(points[index], 0.0), [1.0; 4]));
    let mut draw = triangle_draw_with_sides(vertices).with_face_culling(culling);
    draw.mirrored = mirrored;
    RenderFrame::new(64, 64, [0.0, 0.0, 0.0, 1.0], vec![draw]).unwrap()
}

fn assert_interior_matches(
    actual: &crate::FrameCapture,
    control: &crate::FrameCapture,
    reverse: bool,
    mirrored: bool,
    culling: RenderFaceCulling,
) {
    assert_eq!(
        interior_pixel(actual),
        interior_pixel(control),
        "reverse={reverse}, mirrored={mirrored}, culling={culling:?}"
    );
}

fn interior_pixel(capture: &crate::FrameCapture) -> [u8; 4] {
    let pixel = 16 * capture.width as usize + 16;
    capture.rgba[pixel * 4..pixel * 4 + 4].try_into().unwrap()
}
