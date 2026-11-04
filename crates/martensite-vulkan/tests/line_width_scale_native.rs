//! Line width under a HiDPI resolution ratio on real devices: native GlContext::SetLineWidth
//! multiplies the aspect width by max(1, round(ResolutionRatio)).
//!
//! Native lit-row counts come from the recorded line_width_scale capture on
//! llvmpipe and AMD radeonsi, kept in fixtures/line_width_scale/native.txt run
//! with DISPLAY=:1, LIBGL_ALWAYS_SOFTWARE=1 for llvmpipe).
#[path = "../examples/support/device_options.rs"]
mod device_options;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::{quantity::ColorRgba, standard::Handle};
use martensite_tkservice::{graphic3d::*, image::ImagePixMap};
use martensite_vulkan::Graphic3dRenderDriver;

const NATIVE: &str = include_str!("fixtures/line_width_scale/native.txt");

/// Lit rows of column 32 (red above 128).
fn lit_rows(scene: &scene::ViewerScene) -> usize {
    let image = ImagePixMap::default();
    image.set_top_down(true);
    let view = scene.view.get().unwrap();
    scene.render_view().try_redraw().unwrap();
    let dumped = view.to_pix_map_sized(&image, 64, 64, Graphic3dBufferType::Rgba, true);
    assert!(dumped.unwrap());
    let data = image.data().data.data().to_vec();
    (0..64)
        .filter(|row| data[(row * 64 + 32) * 4] > 128)
        .count()
}

/// A red horizontal segment of `width` at y = 0.1.
fn line_scene(width: f32) -> (Handle<Graphic3dRenderDriver>, scene::ViewerScene) {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let scene = scene::ViewerScene::new(&driver);
    scene.group.clear(true);
    let style = Handle::new(Graphic3dAspects::default());
    style.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    // Native Graphic3dAspectLine3d colours the line with the interior colour.
    style.set_interior_color_rgba(ColorRgba::new(1., 0., 0., 1.).unwrap());
    style.set_line_width(width).unwrap();
    scene.group.set_group_primitives_aspect(&style.cast());
    let segment = Handle::new(
        Graphic3dArrayOfPrimitives::new(Graphic3dTypeOfPrimitiveArray::Segments, 2, 0, 0, 0)
            .unwrap(),
    );
    segment.add_vertex(-10., 0.1, 0.).unwrap();
    segment.add_vertex(10., 0.1, 0.).unwrap();
    scene.group.add_primitive_array(&segment, true);
    scene.view.get().unwrap().camera().set_scale(40.);
    (driver, scene)
}

/// The native count for one width and resolution; both native drivers must agree.
fn native_rows(width: f32, resolution: u32) -> usize {
    let key = format!("width={width} resolution={resolution} ");
    let values: Vec<usize> = NATIVE
        .lines()
        .filter(|line| line.starts_with(&key))
        .map(|line| line.rsplit_once("lit_rows=").unwrap().1.parse().unwrap())
        .collect();
    assert_eq!(
        values.len(),
        2,
        "received {values:?} for {key}; expected sw and host"
    );
    assert_eq!(values[0], values[1], "native drivers disagree on {key}");
    values[0]
}

/// Compare every (width, resolution) case on one fresh scene per width.
fn assert_native_rows(cases: &[(f32, &[u32])]) {
    for &(width, resolutions) in cases {
        let (driver, scene) = line_scene(width);
        for &resolution in resolutions {
            let view = scene.view.get().unwrap();
            view.change_rendering_params(&mut |params| params.resolution = resolution);
            let rows = lit_rows(&scene);
            let expected = native_rows(width, resolution);
            assert_eq!(rows, expected, "width {width} at {resolution}");
        }
        assert!(driver.validation_errors().unwrap().is_empty());
    }
}

#[test]
fn line_width_follows_the_native_resolution_scale() {
    // Scaled widths 1, 2, 3, 3 and 6 px: inside the wide-line range of both devices.
    assert_native_rows(&[(1., &[72, 144, 180]), (3., &[72, 144])]);
}

#[test]
fn line_width_beyond_the_device_line_range_follows_native() {
    // Width 3 at ratio 2.5 scales to 9 px.
    assert_native_rows(&[(3., &[180])]);
}
