//! Hand-computed charts from GlBackgroundArray::Render.
use super::*;
use martensite_tkernel::ncollection::Vec2;

/// A camera on the tile at pixel offset (128, 64) of a 200x100 dump in 64-pixel tiles.
fn tiled_camera() -> Graphic3dCamera {
    let camera = Graphic3dCamera::default();
    camera.set_tile(&Graphic3dCameraTile {
        total_size: Vec2::new(200, 100),
        tile_size: Vec2::new(64, 64),
        offset: Vec2::new(128, 64),
        is_top_down: false,
    });
    camera
}

#[test]
fn untiled_texture_chart_scales_pixels_to_clip_about_the_view_centre() {
    let camera = Graphic3dCamera::default();
    let chart = BackgroundChart::new(&camera, [200, 100], BackgroundGeometry::Texture).unwrap();
    let matrix = chart.world_view();
    assert_eq!(chart.view_size, [200., 100.]);
    assert_eq!(
        (matrix[0], matrix[5], matrix[10], matrix[15]),
        (0.01, 0.02, 1., 1.)
    );
    assert_eq!((matrix[12], matrix[13]), (0., 0.));
}
#[test]
fn untiled_gradient_chart_starts_at_the_lower_left_corner() {
    let camera = Graphic3dCamera::default();
    let chart = BackgroundChart::new(&camera, [97, 43], BackgroundGeometry::Gradient).unwrap();
    assert_eq!(chart.pixel_size(), [97, 43]);
    assert_eq!(chart.scale, [2_f32 / 97., 2_f32 / 43.]);
    assert_eq!(chart.shift, [-1., -1.]);
}
#[test]
fn camera_tile_texture_chart_spans_the_total_size_and_shifts_by_the_tile_offset() {
    let chart = BackgroundChart::new(&tiled_camera(), [64, 64], BackgroundGeometry::Texture);
    let chart = chart.unwrap();
    assert_eq!(chart.view_size, [200., 100.]);
    assert_eq!(chart.scale, [2. / 64., 2. / 64.]);
    // -1 + 200/64 - 2*128/64 and -1 + 100/64 - 2*64/64.
    assert_eq!(chart.shift, [-1.875, -1.4375]);
}
#[test]
fn camera_tile_gradient_chart_spans_the_total_size_from_the_tile_corner() {
    let chart = BackgroundChart::new(&tiled_camera(), [64, 64], BackgroundGeometry::Gradient);
    let chart = chart.unwrap();
    assert_eq!(chart.pixel_size(), [200, 100]);
    assert_eq!(chart.scale, [2. / 64., 2. / 64.]);
    // -1 - 2*128/64 and -1 - 2*64/64: the tile's lower-left pixel maps to clip -1.
    assert_eq!(chart.shift, [-5., -3.]);
}
#[test]
fn empty_viewports_are_rejected_with_their_extent() {
    let camera = Graphic3dCamera::default();
    for extent in [[0, 43], [97, 0]] {
        for geometry in [BackgroundGeometry::Gradient, BackgroundGeometry::Texture] {
            let error = BackgroundChart::new(&camera, extent, geometry).unwrap_err();
            assert!(error.to_string().contains(&format!("{extent:?}")));
            assert!(error.to_string().contains("positive background viewport"));
        }
    }
}
