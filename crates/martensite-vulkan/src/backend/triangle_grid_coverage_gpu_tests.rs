use super::{VulkanOptions, VulkanRenderer};
use crate::{
    PrimitiveDraw, RenderBackend, RenderBehaviorConfiguration, RenderCoordinateConvention,
    RenderFrame, RenderLightSet, RenderLighting, RenderMaterial, RenderMaterialCommon,
    RenderPhongInterpolationBehavior, RenderShading, RenderTriangleCoverageBehavior,
    RenderTriangleGridOrigin, RenderVertex, identity_clip,
};
#[test]
fn conservative_carrier_matches_q12_samples_and_preserves_color_and_depth() {
    if super::phong_interpolation_device::skip_without_barycentric() {
        return;
    }
    let mut selected = renderer(coverage_behavior());
    let bits = selected.runtime.limits.sub_pixel_precision_bits;
    assert!(
        bits <= 10,
        "device raster precision {bits}; expected 0..=10 for distinct Q12 control"
    );
    assert_live_coverage_capabilities(&selected);
    let frame = coverage_frame(bits);
    let actual = selected.render(&frame).unwrap();
    let mut hardware = renderer(hardware_behavior());
    let control = hardware.render(&frame).unwrap();
    retain_capture("fixed-grid-v2", &actual);
    retain_capture("hardware-v1", &control);
    assert_selected_image(&actual, bits);
    assert_hardware_control(&control, bits);
    assert_shared_pixels_unchanged(&actual, &control, bits);
    assert!(selected.validation_errors().unwrap().is_empty());
    assert!(hardware.validation_errors().unwrap().is_empty());
}

#[test]
fn near_collinear_carrier_matches_one_closed_form_sample_and_control_depth() {
    if super::phong_interpolation_device::skip_without_barycentric() {
        return;
    }
    let mut selected = renderer(coverage_behavior());
    assert_live_degenerate_property(&selected);
    let frame = near_collinear_frame();
    let candidate = selected.render(&frame).unwrap();
    let mut control = renderer(hardware_behavior());
    let hardware = control.render(&frame).unwrap();
    retain_capture("near-collinear-fixed-grid-v2", &candidate);
    retain_capture("near-collinear-hardware-v1", &hardware);
    assert_one_red_sample(&candidate, 255, 47);
    assert_red_at(&hardware, 255, 47);
    assert_depth_unchanged(&candidate, &hardware, 255, 47);
    assert!(selected.validation_errors().unwrap().is_empty());
    assert!(control.validation_errors().unwrap().is_empty());
}

#[test]
fn native_grid_v4_restores_logical_depth_for_a_q8_collapsed_triangle() {
    if super::phong_interpolation_device::skip_without_barycentric() {
        return;
    }
    let mut selected = renderer(native_grid_behavior());
    assert_live_degenerate_property(&selected);
    assert_eq!(selected.runtime.limits.sub_pixel_precision_bits, 8);
    let capture = selected.render(&q8_collapsed_depth_frame()).unwrap();
    assert_single_q12_sample(&capture);
    assert!(selected.validation_errors().unwrap().is_empty());
}

#[test]
fn native_grid_v4_saturates_shader_bias_to_each_viewport_depth_bound() {
    if super::phong_interpolation_device::skip_without_barycentric() {
        return;
    }
    let behavior =
        native_grid_behavior().with_depth_bias(crate::RenderDepthBiasBehavior::Native24UnitsV2);
    let mut selected = renderer(behavior);
    for (units, bound) in [(1.0e30, 1.0), (-1.0e30, 0.0)] {
        let capture = selected.render(&v4_bias_frame(units)).unwrap();
        assert_filled_depth(&capture, bound);
    }
    assert!(selected.validation_errors().unwrap().is_empty());
}

fn v4_bias_frame(units: f32) -> RenderFrame {
    let vertices = [
        RenderVertex::new([-0.75, -0.75, 0.5], [1.0; 4]),
        RenderVertex::new([0.75, -0.75, 0.5], [1.0; 4]),
        RenderVertex::new([0.0, 0.75, 0.5], [1.0; 4]),
    ];
    let draw = triangle_draw(vertices)
        .with_polygon_offset(crate::RenderPolygonOffset::new(1, 0.0, units).unwrap());
    RenderFrame::new(8, 8, [0.0; 4], vec![draw]).unwrap()
}

fn assert_filled_depth(capture: &crate::FrameCapture, expected: f32) {
    let mut covered = 0;
    for (index, pixel) in capture.rgba.as_chunks::<4>().0.iter().enumerate() {
        if pixel[0] != 255 {
            continue;
        }
        covered += 1;
        assert_eq!(capture.depth[index], expected);
    }
    assert!(covered > 0, "expected a covered triangle");
}

#[test]
fn native_grid_v4_selects_two_sided_material_from_q12_winding() {
    if super::phong_interpolation_device::skip_without_barycentric() {
        return;
    }
    let mut selected = renderer(native_grid_behavior());
    assert_live_degenerate_property(&selected);
    let positive = selected.render(&q8_two_sided_frame(false)).unwrap();
    let negative = selected.render(&q8_two_sided_frame(true)).unwrap();
    assert_pixel_rgba(&positive, [0, 255, 0, 255]);
    assert_pixel_rgba(&negative, [255, 0, 0, 255]);
}

fn q8_two_sided_frame(reverse: bool) -> RenderFrame {
    let points: [[f32; 2]; 3] = [[32.25, 32.499_023], [32.75, 32.499_023], [32.5, 32.500_977]];
    let points = if reverse {
        [points[0], points[2], points[1]]
    } else {
        points
    };
    let vertices = points.map(|point| RenderVertex::new(window_clip(point, 0.), [1.0; 4]));
    let draw = triangle_draw_with_sides(vertices);
    RenderFrame::new(64, 64, [0.0, 0.0, 0.0, 1.0], vec![draw]).unwrap()
}

fn assert_pixel_rgba(capture: &crate::FrameCapture, expected: [u8; 4]) {
    let sample = 32 * capture.width as usize + 32;
    assert_eq!(&capture.rgba[sample * 4..sample * 4 + 4], &expected);
}

pub(super) fn triangle_draw_with_sides(vertices: [RenderVertex; 3]) -> PrimitiveDraw {
    let draw = PrimitiveDraw::new(vertices.to_vec(), vec![0, 1, 2], identity_clip()).unwrap();
    draw.with_lighting(two_sided_emission()).unwrap()
}

fn two_sided_emission() -> RenderLighting {
    lighting_for_material(two_sided_emission_material())
}

fn lighting_for_material(material: RenderMaterial) -> RenderLighting {
    RenderLighting::new(
        vec![[0.0, 0.0, 1.0]; 3],
        material,
        identity_clip(),
        identity_clip(),
        false,
        RenderShading::Phong,
    )
    .unwrap()
    .with_lights(RenderLightSet {
        ambient: [0.0; 4],
        sources: Vec::new(),
    })
}

fn two_sided_emission_material() -> RenderMaterial {
    RenderMaterial {
        common: [
            side_emission([1., 0., 0., 1.]),
            side_emission([0., 1., 0., 1.]),
        ],
        ..Default::default()
    }
}

fn side_emission(color: [f32; 4]) -> RenderMaterialCommon {
    RenderMaterialCommon {
        diffuse: [0.0, 0.0, 0.0, 1.0],
        emission: color,
        specular_shininess: [0.0; 4],
        ambient: [0.0; 4],
    }
}

fn q8_collapsed_depth_frame() -> RenderFrame {
    let vertices = [
        RenderVertex::new(window_clip([32.25, 32.499_023], -0.2), [1.0; 4]),
        RenderVertex::new(window_clip([32.75, 32.499_023], 0.2), [1.0; 4]),
        RenderVertex::new(window_clip([32.5, 32.500_977], 0.4), [1.0; 4]),
    ];
    RenderFrame::new(64, 64, [0.0, 0.0, 0.0, 1.0], vec![triangle_draw(vertices)]).unwrap()
}

pub(super) fn window_clip([x, y]: [f32; 2], z: f32) -> [f32; 3] {
    [2.0 * x / 64.0 - 1.0, 1.0 - 2.0 * y / 64.0, z]
}

fn assert_single_q12_sample(capture: &crate::FrameCapture) {
    let red = capture
        .rgba
        .as_chunks::<4>()
        .0
        .iter()
        .filter(|pixel| pixel[..] == [255, 0, 0, 255])
        .count();
    assert_eq!(red, 1, "Q12 retained sample count");
    let sample = 32 * capture.width as usize + 32;
    assert_eq!(&capture.rgba[sample * 4..sample * 4 + 4], &[255, 0, 0, 255]);
    assert!((capture.depth[sample] - 0.6).abs() < 1.0e-5);
}

fn near_collinear_frame() -> RenderFrame {
    let vertices = [
        RenderVertex::new(clip_vertex([0x3e8b7008, 0x3f3be650, 0xbf302148]), [1.0; 4]),
        RenderVertex::new(clip_vertex([0x3e5c422c, 0x3f5163d4, 0xbf3934aa]), [1.0; 4]),
        RenderVertex::new(clip_vertex([0x3e77a320, 0x3f4764ca, 0xbf3f0eae]), [1.0; 4]),
    ];
    let draw = triangle_draw(vertices);
    RenderFrame::new(409, 409, [0.0, 0.0, 0.0, 1.0], vec![draw]).unwrap()
}

fn clip_vertex(bits: [u32; 3]) -> [f32; 3] {
    bits.map(f32::from_bits)
}

fn assert_live_degenerate_property(renderer: &VulkanRenderer) {
    let live = super::triangle_grid_coverage_device::degenerate_triangles_rasterized(
        &renderer.runtime.instance,
        renderer.runtime.physical,
    );
    println!("{{\"degenerate_triangles_rasterized\":{live}}}");
    assert!(live, "expected live conservative-raster degenerate support");
}

fn assert_one_red_sample(capture: &crate::FrameCapture, x: usize, y: usize) {
    assert_red_at(capture, x, y);
    let red_pixels = capture
        .rgba
        .as_chunks::<4>()
        .0
        .iter()
        .filter(|pixel| pixel[..] == [255, 0, 0, 255])
        .count();
    assert_eq!(red_pixels, 1, "near-collinear Q12 coverage sample count");
}

fn assert_red_at(capture: &crate::FrameCapture, x: usize, y: usize) {
    let offset = (y * 409 + x) * 4;
    assert_eq!(&capture.rgba[offset..offset + 4], &[255, 0, 0, 255]);
}

fn assert_depth_unchanged(
    candidate: &crate::FrameCapture,
    control: &crate::FrameCapture,
    x: usize,
    y: usize,
) {
    let index = y * 409 + x;
    assert_eq!(
        candidate.depth[index].to_bits(),
        control.depth[index].to_bits()
    );
}

fn renderer(configuration: RenderBehaviorConfiguration) -> VulkanRenderer {
    renderer_with_coordinates(configuration, RenderCoordinateConvention::TopLeftLogicalV2)
}

pub(super) fn renderer_with_coordinates(
    configuration: RenderBehaviorConfiguration,
    coordinates: RenderCoordinateConvention,
) -> VulkanRenderer {
    VulkanRenderer::new_with_render_behavior(
        VulkanOptions {
            validation: true,
            device_index: Some(0),
        },
        Default::default(),
        coordinates,
        Default::default(),
        configuration,
    )
    .unwrap()
}

fn coverage_behavior() -> RenderBehaviorConfiguration {
    hardware_behavior().with_triangle_coverage(RenderTriangleCoverageBehavior::FixedGrid12V2 {
        origin: RenderTriangleGridOrigin::TopDown,
    })
}

pub(super) fn native_grid_behavior() -> RenderBehaviorConfiguration {
    hardware_behavior().with_triangle_coverage(RenderTriangleCoverageBehavior::NativeGrid12V4 {
        origin: RenderTriangleGridOrigin::TopDown,
    })
}

pub(super) fn hardware_behavior() -> RenderBehaviorConfiguration {
    RenderBehaviorConfiguration::default()
        .with_phong_interpolation(RenderPhongInterpolationBehavior::UnsnappedV2)
}

fn coverage_frame(subpixel_bits: u32) -> RenderFrame {
    let delta = martensite_cmath::powi(f64::from(2.0_f32), -(subpixel_bits as i32 + 2)) as f32;
    let top = 0.75 - delta / 2.0;
    let vertices = [
        RenderVertex::new([-0.75, top, -0.5], [1.0; 4]),
        RenderVertex::new([0.75, top, 0.0], [1.0; 4]),
        RenderVertex::new([-0.75, -0.75, 0.5], [1.0; 4]),
    ];
    let draw = triangle_draw(vertices);
    RenderFrame::new(4, 4, [0.0, 0.0, 0.0, 1.0], vec![draw]).unwrap()
}

fn triangle_draw(vertices: [RenderVertex; 3]) -> PrimitiveDraw {
    let draw = PrimitiveDraw::new(vertices.to_vec(), vec![0, 1, 2], identity_clip()).unwrap();
    draw.with_lighting(phong_emission()).unwrap()
}

fn phong_emission() -> RenderLighting {
    let emission = RenderMaterialCommon {
        diffuse: [0.0, 0.0, 0.0, 1.0],
        emission: [1.0, 0.0, 0.0, 1.0],
        specular_shininess: [0.0; 4],
        ambient: [0.0; 4],
    };
    RenderLighting::new(
        vec![[0.0, 0.0, 1.0]; 3],
        RenderMaterial {
            common: [emission; 2],
            ..Default::default()
        },
        identity_clip(),
        identity_clip(),
        false,
        RenderShading::Phong,
    )
    .unwrap()
    .with_lights(RenderLightSet {
        ambient: [0.0; 4],
        sources: Vec::new(),
    })
}

fn assert_live_coverage_capabilities(renderer: &VulkanRenderer) {
    assert!(
        super::triangle_grid_coverage_device::extension_offered(
            &renderer.runtime.instance,
            renderer.runtime.physical,
        )
        .unwrap()
    );
    assert!(super::triangle_grid_coverage_device::shader_int64(
        &renderer.runtime.instance,
        renderer.runtime.physical,
    ));
    let size = super::triangle_grid_coverage_device::overestimate_size(
        &renderer.runtime.instance,
        renderer.runtime.physical,
    );
    assert!(size.is_finite() && size >= 0.0, "overestimate size {size}");
    println!(
        "{{\"device\":{:?},\"subpixel_bits\":{},\"overestimate_size\":{size}}}",
        renderer.device_name(),
        renderer.runtime.limits.sub_pixel_precision_bits
    );
}

fn assert_selected_image(capture: &crate::FrameCapture, subpixel_bits: u32) {
    let delta = martensite_cmath::powi(2.0_f64, -(subpixel_bits as i32 + 2));
    let raster_top = snapped_top(delta, subpixel_bits);
    for y in 0..4 {
        for x in 0..4 {
            let selected = q12_membership(x, y);
            assert_pixel_color(capture, x, y, selected);
            if selected {
                assert_pixel_depth(capture, x, y, expected_depth(x, y, raster_top));
            } else {
                assert_pixel_depth(capture, x, y, 1.0);
            }
        }
    }
}

fn q12_membership(x: usize, y: usize) -> bool {
    // For this triangle, Q12 admits the two inner BC samples but excludes vertex C=(0,3).
    y > 0 && (x + y < 3 || (x > 0 && x + y == 3))
}

fn expected_depth(x: usize, y: usize, screen_y: f64) -> f32 {
    let weight_y = (y as f64 + 0.5 - screen_y) / (3.5 - screen_y);
    let weight_x = x as f64 / 3.0;
    let weight_origin = 1.0 - weight_x - weight_y;
    let ndc_z = weight_origin * -0.5 + weight_x * 0.0 + weight_y * 0.5;
    ((ndc_z + 1.0) * 0.5) as f32
}

fn assert_pixel_color(capture: &crate::FrameCapture, x: usize, y: usize, selected: bool) {
    let offset = (y * 4 + x) * 4;
    let expected = if selected {
        [255, 0, 0, 255]
    } else {
        [0, 0, 0, 255]
    };
    assert_eq!(
        &capture.rgba[offset..offset + 4],
        &expected,
        "sample ({x},{y})"
    );
}

fn assert_pixel_depth(capture: &crate::FrameCapture, x: usize, y: usize, expected: f32) {
    let actual = capture.depth[y * 4 + x];
    assert!(
        (actual - expected).abs() <= 1.0e-6,
        "sample ({x},{y}) depth {actual}; expected {expected}"
    );
}

fn snapped_top(delta: f64, subpixel_bits: u32) -> f64 {
    let scale = martensite_cmath::powi(2.0_f64, subpixel_bits as i32);
    ((0.5 + delta) * scale).round_ties_even() / scale
}

fn assert_hardware_control(capture: &crate::FrameCapture, subpixel_bits: u32) {
    let delta = martensite_cmath::powi(2.0_f64, -(subpixel_bits as i32 + 2));
    let raster_top = snapped_top(delta, subpixel_bits);
    for y in 0..4 {
        for x in 0..4 {
            let covered = x + y < 3;
            assert_pixel_color(capture, x, y, covered);
            if covered {
                assert_pixel_depth(capture, x, y, expected_depth(x, y, raster_top));
            }
        }
    }
}

fn assert_shared_pixels_unchanged(
    candidate: &crate::FrameCapture,
    control: &crate::FrameCapture,
    subpixel_bits: u32,
) {
    let delta = martensite_cmath::powi(2.0_f64, -(subpixel_bits as i32 + 2));
    let raster_top = snapped_top(delta, subpixel_bits);
    for y in 0..4 {
        for x in 0..4 {
            if y == 0 || x + y >= 3 {
                continue;
            }
            let index = y * 4 + x;
            assert_eq!(
                candidate.depth[index].to_bits(),
                control.depth[index].to_bits()
            );
            assert_eq!(control.depth[index], expected_depth(x, y, raster_top));
        }
    }
}

pub(super) fn retain_capture(label: &str, capture: &crate::FrameCapture) {
    let Some(directory) = std::env::var_os("MARTENSITE_GEOMETRY_ARTIFACT_DIR") else {
        return;
    };
    let directory = std::path::PathBuf::from(directory);
    std::fs::create_dir_all(&directory).unwrap();
    std::fs::write(directory.join(format!("{label}.rgba")), &capture.rgba).unwrap();
    let depth: Vec<_> = capture
        .depth
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect();
    std::fs::write(directory.join(format!("{label}.depth")), depth).unwrap();
}

#[path = "triangle_grid_empty_pruning_gpu_tests.rs"]
mod empty_pruning;
