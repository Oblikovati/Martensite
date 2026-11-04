//! Hand-computed expectations from createTextureArray, Render and drawBackground.
use super::*;
use martensite_tkservice::aspect::{AspectGradientBackground, AspectGradientFillMethod};
use martensite_tkservice::image::{ImageFormat, ImagePixMap};

struct ImageUnusedBackend;
impl crate::RenderBackend for ImageUnusedBackend {
    fn render(&mut self, _frame: &crate::RenderFrame) -> Result<crate::FrameCapture, RenderError> {
        panic!("image background extraction must not render; expected a pure layer extraction")
    }
}
fn retained_view() -> Handle<dyn Graphic3dCView> {
    let driver = crate::Graphic3dRenderDriver::with_backend(ImageUnusedBackend, 64).unwrap();
    let manager = Handle::new(Graphic3dStructureManager::new(&driver.cast()).unwrap());
    driver.create_view(&manager).unwrap()
}
/// A 5x3 RGB texture of one grey level.
fn small_texture() -> Handle<dyn Graphic3dTextureMap> {
    let image = Handle::new(ImagePixMap::default());
    assert!(image.init_wrapper(ImageFormat::Rgb, Some(vec![90; 5 * 3 * 3]), 5, 3, 15));
    Handle::new(Graphic3dTexture2D::from_pix_map(image)).cast()
}

#[test]
fn stretched_quad_spans_the_view_and_maps_the_texture_once() {
    let quad = texture_quad(AspectFillMethod::Stretch, [203., 151.], [37., 23.]);
    assert_eq!(
        quad,
        [
            ([101.5, 75.5], [1., 0.]),
            ([101.5, -75.5], [1., -1.]),
            ([-101.5, 75.5], [0., 0.]),
            ([-101.5, -75.5], [0., -1.]),
        ]
    );
}
#[test]
fn centered_quad_spans_the_texture_size() {
    let quad = texture_quad(AspectFillMethod::Centered, [203., 151.], [37., 23.]);
    assert_eq!(quad[0], ([18.5, 11.5], [1., 0.]));
    assert_eq!(quad[3], ([-18.5, -11.5], [0., -1.]));
}
#[test]
fn tiled_quad_repeats_per_texture_size_from_the_top_left_corner() {
    let quad = texture_quad(AspectFillMethod::Tiled, [200., 100.], [40., 25.]);
    // The top-left corner samples (0, 0); the texture repeats 5 times across and 4 times down.
    assert_eq!(quad[2], ([-100., 50.], [0., 0.]));
    assert_eq!(quad[1], ([100., -50.], [5., -4.]));
}
#[test]
fn gradient_draws_under_a_centered_or_undrawn_image_only() {
    let view = retained_view();
    assert!(gradient_under_image(&*view));
    view.set_background_image(&small_texture(), true);
    for (style, under) in [
        (AspectFillMethod::Centered, true),
        (AspectFillMethod::None, true),
        (AspectFillMethod::Stretch, false),
        (AspectFillMethod::Tiled, false),
    ] {
        view.set_background_image_style(style);
        assert_eq!(gradient_under_image(&*view), under, "{style:?}");
    }
}
#[test]
fn image_layer_is_an_unlit_double_sided_textured_quad_without_depth() {
    let view = retained_view();
    view.set_background_image(&small_texture(), true);
    assert!(image_background_layer(&*view, [64, 32]).unwrap().is_none());
    view.set_background_image_style(AspectFillMethod::Centered);
    let layer = image_background_layer(&*view, [64, 32]).unwrap().unwrap();
    assert!(!layer.depth_test && !layer.depth_write);
    let draw = &layer.draws[0];
    assert_eq!(draw.indices, QUAD_TRIANGLES);
    assert_eq!(draw.face_culling, crate::RenderFaceCulling::None);
    assert!(draw.lighting.is_none());
    let texture = draw.surface_texture.as_ref().unwrap();
    assert_eq!(texture.image.extent, [5, 3]);
    assert!(texture.sampler.repeat);
    assert_eq!(draw.vertices[0].position, [2.5, 1.5, 0.]);
}
#[test]
fn image_follows_the_gradient_in_the_prepended_layers() {
    let view = retained_view();
    view.set_background_image(&small_texture(), true);
    view.set_background_image_style(AspectFillMethod::Centered);
    let colors = (
        martensite_tkernel::quantity::Color::default(),
        martensite_tkernel::quantity::Color::default(),
    );
    let gradient =
        AspectGradientBackground::new(colors.0, colors.1, AspectGradientFillMethod::Vertical);
    view.set_gradient_background(&gradient);
    let mut passes = Vec::new();
    crate::gradient_background::prepend_background(&*view, [64, 32], &mut passes).unwrap();
    assert_eq!(passes.len(), 2);
    assert!(passes[0].draws[0].surface_texture.is_none());
    assert!(passes[1].draws[0].surface_texture.is_some());
    view.set_background_image_style(AspectFillMethod::Stretch);
    passes.clear();
    crate::gradient_background::prepend_background(&*view, [64, 32], &mut passes).unwrap();
    assert_eq!(passes.len(), 1);
    assert!(passes[0].draws[0].surface_texture.is_some());
}
