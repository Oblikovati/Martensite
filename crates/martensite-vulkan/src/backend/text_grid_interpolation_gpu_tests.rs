use super::{VulkanOptions, VulkanRenderer};
use crate::{
    PrimitiveDraw, RasterImage, RenderBackend, RenderBehaviorConfiguration,
    RenderCoordinateConvention, RenderFrame, RenderTextInterpolationBehavior, RenderVertex,
    TexturedStyle, identity_clip,
};

#[test]
fn projected_grid_quad_samples_its_closed_form_center_uv() {
    if super::phong_interpolation_device::skip_without_barycentric() {
        return;
    }
    let mut renderer = text_grid_renderer();
    let frame = gradient_quad_frame();
    let capture = renderer.render(&frame).unwrap();
    let pixel = &capture.rgba[(4 * 8 + 4) * 4..(4 * 8 + 4) * 4 + 4];
    // Quad UV is 4.5/8; 2-texel bilinear sampling gives 0.625 linear red, then SRGB8 encodes.
    let expected = [srgb_byte(0.625), 0, 0, 255];
    assert_eq!(expected, [207, 0, 0, 255]);
    assert_eq!(pixel, expected);
    let validation_errors = renderer.validation_errors().unwrap();
    assert!(
        validation_errors.is_empty(),
        "validation errors: {validation_errors:?}"
    );
}

fn srgb_byte(linear: f32) -> u8 {
    let encoded = if linear <= 0.003_130_8 {
        12.92 * linear
    } else {
        1.055 * martensite_cmath::powf(linear, 1.0 / 2.4) - 0.055
    };
    (encoded * 255.0).round() as u8
}

fn text_grid_renderer() -> VulkanRenderer {
    let behavior = RenderBehaviorConfiguration::default()
        .with_text_interpolation(RenderTextInterpolationBehavior::ProjectedGrid12V2);
    VulkanRenderer::new_with_render_behavior(
        VulkanOptions {
            validation: true,
            device_index: Some(0),
        },
        Default::default(),
        RenderCoordinateConvention::TopLeftLogicalV2,
        Default::default(),
        behavior,
    )
    .unwrap()
}

fn gradient_quad_frame() -> RenderFrame {
    let positions = [[-1., -1., 0.], [1., -1., 0.], [1., 1., 0.], [-1., 1., 0.]];
    let vertices = positions.map(|position| RenderVertex::new(position, [1.; 4]));
    let image = RasterImage::new(
        [2, 2],
        vec![
            [0., 0., 0., 1.],
            [1., 0., 0., 1.],
            [0., 0., 0., 1.],
            [1., 0., 0., 1.],
        ],
    )
    .unwrap();
    let style =
        TexturedStyle::new(image, vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]], 0.285).unwrap();
    let draw = PrimitiveDraw::textured(
        vertices.to_vec(),
        vec![0, 1, 2, 0, 2, 3],
        identity_clip(),
        style,
    )
    .unwrap();
    RenderFrame::new(8, 8, [0., 0., 0., 1.], vec![draw]).unwrap()
}
