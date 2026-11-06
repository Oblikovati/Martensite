//! Independent corner, depth and retained-view expectations for the two linear fills.
use super::*;
use crate::identity_clip;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectGradientBackground;
const FIRST: [f32; 4] = [0.125, 0.25, 0.75, 1.];
const SECOND: [f32; 4] = [0.75, 0.5, 0.125, 1.];

const CORNER_CASES: [(AspectGradientFillMethod, [[f32; 4]; 4]); 2] = [
    (
        AspectGradientFillMethod::Horizontal,
        [SECOND, SECOND, FIRST, FIRST],
    ),
    (
        AspectGradientFillMethod::Vertical,
        [SECOND, FIRST, FIRST, SECOND],
    ),
];
#[test]
fn linear_background_corners_preserve_native_orientation_and_disable_depth() {
    for (method, expected) in CORNER_CASES {
        assert_gradient_layer(method, expected);
    }
}
fn assert_gradient_layer(method: AspectGradientFillMethod, expected: [[f32; 4]; 4]) {
    let layer = gradient_layer(method, [FIRST, SECOND], [64, 64])
        .unwrap()
        .unwrap();
    assert!(!layer.depth_test && !layer.depth_write && !layer.clear_depth);
    assert_eq!(layer.draws[0].indices, [0, 1, 3, 1, 2, 3]);
    let actual: Vec<_> = layer.draws[0]
        .vertices
        .iter()
        .map(|vertex| vertex.color)
        .collect();
    assert_eq!(actual, expected);
}
#[test]
fn disabled_gradient_preserves_solid_and_other_modes_stay_explicit() {
    assert!(
        gradient_layer(AspectGradientFillMethod::None, [FIRST, SECOND], [64, 64])
            .unwrap()
            .is_none()
    );
    for method in [
        AspectGradientFillMethod::Corner2,
        AspectGradientFillMethod::Corner1,
    ] {
        let failure = gradient_layer(method, [FIRST, SECOND], [64, 64]).unwrap_err();
        assert!(failure.to_string().contains(&format!("{method:?}")));
        assert!(
            failure
                .to_string()
                .contains("None, Horizontal, Vertical, Diagonal1 or Diagonal2")
        );
    }
}
struct GradientUnusedBackend;
impl crate::RenderBackend for GradientUnusedBackend {
    fn render(&mut self, _frame: &crate::RenderFrame) -> Result<crate::FrameCapture, RenderError> {
        panic!(
            "background extraction invoked the unused GPU backend; expected pure retained-view extraction without a rendering request"
        )
    }
}
fn retained_gradient_view() -> Handle<dyn Graphic3dCView> {
    let driver = crate::Graphic3dRenderDriver::with_backend(GradientUnusedBackend, 64).unwrap();
    let manager = Handle::new(Graphic3dStructureManager::new(&driver.cast()).unwrap());
    driver.create_view(&manager).unwrap()
}
fn set_gradient(view: &dyn Graphic3dCView, method: AspectGradientFillMethod) {
    use martensite_tkernel::quantity::{Color, ColorSpace};
    let first = Color::new(0.125, 0.25, 0.75, ColorSpace::Rgb).unwrap();
    let second = Color::new(0.75, 0.5, 0.125, ColorSpace::Rgb).unwrap();
    view.set_gradient_background(&AspectGradientBackground::new(first, second, method));
}
#[test]
fn retained_gradient_precedes_model_pass_without_mutating_scene_or_model_depth() {
    let view = retained_gradient_view();
    let before = view.layers().len();
    set_gradient(&*view, AspectGradientFillMethod::Vertical);
    let mut passes = vec![RenderLayer::new(Vec::new(), true, true, true)];
    prepend_background(&*view, [64, 64], &mut passes).unwrap();
    assert_eq!(passes.len(), 2);
    assert_eq!(passes[1].draws.len(), 0);
    assert!(passes[1].depth_test && passes[1].depth_write && passes[1].clear_depth);
    assert_eq!(view.layers().len(), before);
}
#[test]
fn retained_gradient_updates_and_solid_transition_do_not_retain_stale_draws() {
    let view = retained_gradient_view();
    for (method, count) in [
        (AspectGradientFillMethod::Horizontal, 1),
        (AspectGradientFillMethod::Vertical, 1),
        (AspectGradientFillMethod::Elliptical, 1),
        (AspectGradientFillMethod::None, 0),
    ] {
        set_gradient(&*view, method);
        let mut passes = Vec::new();
        prepend_background(&*view, [64, 64], &mut passes).unwrap();
        assert_eq!(passes.len(), count);
    }
}
#[test]
fn corner_gradients_become_the_frame_colored_quad_without_layers() {
    let view = retained_gradient_view();
    set_gradient(&*view, AspectGradientFillMethod::Corner3);
    let mut passes = Vec::new();
    let quad = prepend_background(&*view, [64, 32], &mut passes)
        .unwrap()
        .unwrap();
    assert!(passes.is_empty());
    assert_eq!((quad.corner, quad.view_size), (2, [64., 32.]));
    let mut chart = identity_clip();
    (chart[0], chart[5], chart[12], chart[13]) = (2. / 64., 2. / 32., -1., -1.);
    assert_eq!(quad.projection, chart);
    assert_eq!(quad.colors, [[0.125, 0.25, 0.75], [0.75, 0.5, 0.125]]);
}
#[test]
fn texture_background_without_an_image_draws_its_gradient() {
    let view = retained_gradient_view();
    set_gradient(&*view, AspectGradientFillMethod::Vertical);
    view.cview_base().fields.write().background_type = Graphic3dTypeOfBackground::Texture;
    let mut passes = Vec::new();
    assert!(
        prepend_background(&*view, [64, 64], &mut passes)
            .unwrap()
            .is_none()
    );
    assert_eq!(passes.len(), 1);
    assert_eq!(passes[0].draws[0].indices, [0, 1, 3, 1, 2, 3]);
}

#[test]
fn non_square_background_retains_the_native_pixel_chart_and_float_projection() {
    let method = AspectGradientFillMethod::Horizontal;
    let layer = gradient_layer(method, [FIRST, SECOND], [97, 43])
        .unwrap()
        .unwrap();
    let draw = &layer.draws[0];
    assert_eq!(draw.vertices[0].position, [97., 0., 0.]);
    assert_eq!(draw.vertices[2].position, [0., 43., 0.]);
    let mut expected = identity_clip();
    expected[0] = 2_f32 / 97.;
    expected[5] = 2_f32 / 43.;
    expected[12] = -1.;
    expected[13] = -1.;
    assert_eq!(draw.clip_transform(), &expected);
}
/// Put the view camera on the tile at (64, 0) of a 160x96 dump in 64-pixel tiles.
fn set_camera_tile(view: &dyn Graphic3dCView) {
    use martensite_tkernel::ncollection::Vec2;
    crate::view_render_camera::owned_render_camera(view).set_tile(&Graphic3dCameraTile {
        total_size: Vec2::new(160, 96),
        tile_size: Vec2::new(64, 64),
        offset: Vec2::new(64, 0),
        is_top_down: false,
    });
}
/// The tiled chart: scale 2/64 and the shift -1 - 2*offset/64 of the (bottom-up) tile offset
/// (64, 0): GlBackgroundArray::Render's gradient camera-tile branch.
fn tiled_gradient_chart() -> [f32; 16] {
    let mut chart = identity_clip();
    (chart[0], chart[5], chart[12], chart[13]) = (2. / 64., 2. / 64., -3., -1.);
    chart
}
#[test]
fn camera_tiles_draw_their_part_of_the_total_size_gradient() {
    let view = retained_gradient_view();
    set_gradient(&*view, AspectGradientFillMethod::Vertical);
    set_camera_tile(&*view);
    let mut passes = Vec::new();
    prepend_background(&*view, [64, 64], &mut passes).unwrap();
    let draw = &passes[0].draws[0];
    assert_eq!(draw.vertices[1].position, [160., 96., 0.]);
    assert_eq!(draw.clip_transform(), &tiled_gradient_chart());
}
#[test]
fn camera_tiles_draw_their_part_of_the_total_size_corner_gradient() {
    let view = retained_gradient_view();
    set_gradient(&*view, AspectGradientFillMethod::Corner1);
    set_camera_tile(&*view);
    let quad = prepend_background(&*view, [64, 64], &mut Vec::new())
        .unwrap()
        .unwrap();
    assert_eq!(quad.view_size, [160., 96.]);
    assert_eq!(quad.projection, tiled_gradient_chart());
}
