//! Closed-form primary-color checks use real extracted point inputs and actual GPU captures.
use crate::point_surface_texture_tests::fixture::{PointTextureFixture, point_draw};
#[path = "../tests/support/hatch_capture.rs"]
mod capture_support;
use crate::*;
use martensite_tkservice::graphic3d::{
    Graphic3dAspectsLineMembers, Graphic3dAspectsMembers, Graphic3dTypeOfShadingModel as Model,
};

fn point_renderer() -> VulkanRenderer {
    let device_index = std::env::var("MARTENSITE_VULKAN_DEVICE")
        .ok()
        .map(|value| value.parse().expect("device index must be an integer"));
    let renderer = VulkanRenderer::new(VulkanOptions {
        validation: true,
        device_index,
    })
    .unwrap();
    println!("MARTENSITE_DEVICE={}", renderer.device_name());
    renderer
}
fn palette_draw(model: Model, size: f32) -> PrimitiveDraw {
    let mut draw = point_draw(model, size);
    place_palette_points(&mut draw);
    if let Some(lighting) = draw.lighting.as_mut() {
        lighting.lights = RenderLightSet {
            ambient: [1.; 4],
            sources: vec![],
        };
        lighting.material.common = [ambient_material(); 2];
    }
    draw
}
fn place_palette_points(draw: &mut PrimitiveDraw) {
    draw.clip_transform = identity_clip();
    for (vertex, x) in draw.vertices.iter_mut().zip([16.5_f32, 48.5]) {
        vertex.position = [x / 32. - 1., 1. - 32.5 / 32., 0.5];
    }
}
fn ambient_material() -> RenderMaterialCommon {
    RenderMaterialCommon {
        ambient: [1.; 4],
        diffuse: [0., 0., 0., 1.],
        emission: [0.; 4],
        specular_shininess: [0.; 4],
    }
}
fn require_palette(model: Model) {
    let mut renderer = point_renderer();
    for size in [1., 3., 7.] {
        let frame =
            RenderFrame::new(64, 64, [0., 0., 0., 1.], vec![palette_draw(model, size)]).unwrap();
        let capture = renderer.render(&frame).unwrap();
        require_palette_capture(&capture, size);
    }
    assert!(renderer.validation_errors().unwrap().is_empty());
}
fn require_palette_capture(capture: &FrameCapture, size: f32) {
    require_palette_centres(capture, size);
    let lit = capture
        .rgba
        .as_chunks::<4>()
        .0
        .iter()
        .filter(|pixel| pixel[..3] != [0, 0, 0])
        .count();
    assert_eq!(
        lit,
        2 * (size as usize).pow(2),
        "requested point size {size}"
    );
}
fn require_palette_centres(capture: &FrameCapture, size: f32) {
    let pixels = capture.rgba.as_chunks::<4>().0;
    let observed = palette_pixels(capture);
    assert_eq!(
        &pixels[32 * 64 + 16][..3],
        &[255, 0, 0],
        "size {size}; actual {observed:?}"
    );
    assert_eq!(
        &pixels[32 * 64 + 48][..3],
        &[0, 255, 0],
        "size {size}; actual {observed:?}"
    );
}
fn palette_pixels(capture: &FrameCapture) -> Vec<(usize, usize, [u8; 4])> {
    capture
        .rgba
        .as_chunks::<4>()
        .0
        .iter()
        .enumerate()
        .filter(|(_, pixel)| pixel[..3] != [0, 0, 0])
        .map(|(index, pixel)| (index % 64, index / 64, *pixel))
        .collect()
}
#[test]
fn unlit_point_uv_palette_and_requested_sizes() {
    require_palette(Model::Unlit);
}
#[test]
fn gouraud_point_uv_palette_and_requested_sizes() {
    require_palette(Model::Gouraud);
}
#[test]
fn phong_point_uv_palette_and_requested_sizes() {
    require_palette(Model::Phong);
}

#[test]
fn mapped_point_vertex_colors_override_sampled_palette() {
    let mut renderer = point_renderer();
    let fixture = PointTextureFixture::new(true, true);
    fixture.aspect.set_shading_model(Model::Unlit);
    fixture.aspect.set_marker_scale(1.).unwrap();
    let capture = capture_colored_point_palette(&mut renderer, fixture.draws().unwrap().remove(0));
    let pixels = capture.rgba.as_chunks::<4>().0;
    assert_eq!(&pixels[32 * 64 + 16][..3], &[0, 255, 255]);
    assert_eq!(&pixels[32 * 64 + 48][..3], &[255, 0, 255]);
    assert_eq!(palette_pixels(&capture).len(), 2);
    assert!(renderer.validation_errors().unwrap().is_empty());
}

fn capture_colored_point_palette(
    renderer: &mut VulkanRenderer,
    mut draw: PrimitiveDraw,
) -> FrameCapture {
    place_palette_points(&mut draw);
    let frame = RenderFrame::new(64, 64, [0., 0., 0., 1.], vec![draw]).unwrap();
    let capture = renderer.render(&frame).unwrap();
    capture_support::record("vertex-color-precedence", &capture);
    capture
}

#[test]
fn unavailable_gouraud_point_image_preserves_emission_without_source_colors() {
    require_missing_image_mode(Model::Gouraud, false, [[255, 0, 0]; 2]);
}
#[test]
fn unavailable_gouraud_point_image_preserves_real_colors_and_emission() {
    require_missing_image_mode(Model::Gouraud, true, [[255, 255, 255], [255, 0, 255]]);
}
#[test]
fn unavailable_phong_point_image_preserves_emission_without_source_colors() {
    require_missing_image_mode(Model::Phong, false, [[255, 0, 0]; 2]);
}
#[test]
fn unavailable_phong_point_image_preserves_real_colors_and_emission() {
    require_missing_image_mode(Model::Phong, true, [[255, 255, 255], [255, 0, 255]]);
}
#[test]
fn unavailable_unlit_point_image_writes_opaque_black_without_source_colors() {
    require_missing_image_mode(Model::Unlit, false, [[0, 0, 0]; 2]);
}
#[test]
fn unavailable_unlit_point_image_preserves_real_source_colors() {
    require_missing_image_mode(Model::Unlit, true, [[0, 255, 255], [255, 0, 255]]);
}
fn require_missing_image_mode(model: Model, colors: bool, expected: [[u8; 3]; 2]) {
    let mut renderer = point_renderer();
    require_missing_image_frame(&mut renderer, model, colors, expected);
    assert!(renderer.validation_errors().unwrap().is_empty());
}
fn require_missing_image_frame(
    renderer: &mut VulkanRenderer,
    model: Model,
    colors: bool,
    expected: [[u8; 3]; 2],
) {
    let draw = missing_point_image_draw(model, colors);
    let frame = RenderFrame::new(64, 64, [0., 0., 0., 1.], vec![draw]).unwrap();
    let capture = renderer.render(&frame).unwrap();
    capture_support::record(&format!("missing-{model:?}-colors-{colors}"), &capture);
    require_missing_image_pixels(&capture, expected);
}
fn missing_point_image_draw(model: Model, colors: bool) -> PrimitiveDraw {
    let fixture = PointTextureFixture::unavailable_base_image(colors);
    fixture.aspect.set_shading_model(model);
    fixture.aspect.set_marker_scale(1.).unwrap();
    let mut draw = fixture.draws().unwrap().remove(0);
    place_palette_points(&mut draw);
    if let Some(lighting) = draw.lighting.as_mut() {
        lighting.lights = RenderLightSet {
            ambient: [1.; 4],
            sources: vec![],
        };
        let mut material = ambient_material();
        material.emission = [1., 0., 0., 0.];
        lighting.material.common = [material; 2];
    }
    draw
}
fn require_missing_image_pixels(capture: &FrameCapture, expected: [[u8; 3]; 2]) {
    let pixels = capture.rgba.as_chunks::<4>().0;
    for (x, rgb) in [16, 48].into_iter().zip(expected) {
        assert_eq!(&pixels[32 * 64 + x][..3], &rgb);
        assert_eq!(capture.depth[32 * 64 + x], 0.75);
    }
    assert_eq!(
        capture.depth.iter().filter(|value| **value != 1.).count(),
        2
    );
}
