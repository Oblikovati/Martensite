//! Closed-form marker and segment pixels over independently owned retained state.
#![cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
use martensite_geometry_raster::{PlanarSegment, PointGlyph, SpatialSegment};
use martensite_raster_core::{
    MarkerShape, PaletteColor, PaletteKind, RasterFailureContext, RasterViewer,
    VirtualWindowSystem, drawable::RasterDrawable,
};
use martensite_tkmath::gp::{Point2, Point3};
use std::rc::Rc;

fn capture(label: &str, kind: &str, drawable: Rc<dyn RasterDrawable>) -> Vec<u8> {
    let system = VirtualWindowSystem::<RasterFailureContext>::connect_virtual().unwrap();
    let mut viewer = RasterViewer::with_adapter(Some(Rc::new(system)));
    viewer.make_view(1, kind, 0, 0, 200, 100).unwrap();
    viewer.display(&drawable).unwrap();
    viewer.repaint_view(1).unwrap();
    saved(&mut viewer, label)
}
fn saved(viewer: &mut RasterViewer, label: &str) -> Vec<u8> {
    let root = std::env::var("MARTENSITE_GEOMETRY_ARTIFACT_DIR")
        .unwrap_or_else(|_| format!("/tmp/marker-segment-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let path = format!("{root}/{label}.ppm");
    assert!(viewer.save_view(1, &path).unwrap());
    let bytes = std::fs::read(path).unwrap();
    assert!(bytes.starts_with(b"P6\n200 100\n255\n"));
    assert_eq!(bytes.len(), 60015);
    viewer.delete_view(1);
    bytes
}
fn red(bytes: &[u8], x: usize, y: usize) -> bool {
    let offset = 15 + (y * 200 + x) * 3;
    bytes[offset..offset + 3] == [255, 0, 0]
}
fn red_color() -> PaletteColor {
    PaletteColor::new(PaletteKind::Rouge)
}
#[test]
fn spatial_point_fixed_pixel_marker() {
    let point = PointGlyph::new(Point3::new(0., 0., 0.), MarkerShape::Plus, red_color());
    let bytes = capture("spatial_point", "+X+Y", Rc::new(point));
    assert!(red(&bytes, 95, 50) && red(&bytes, 105, 50));
    assert!(red(&bytes, 100, 45) && red(&bytes, 100, 55));
    assert!(!red(&bytes, 101, 51));
}
#[test]
fn planar_point_fixed_pixel_marker() {
    let point = PointGlyph::new_2d(Point2::new(0., 0.), MarkerShape::Plus, red_color());
    let bytes = capture("planar_point", "-2D-", Rc::new(point));
    assert!(red(&bytes, 95, 50) && red(&bytes, 105, 50));
    assert!(red(&bytes, 100, 45) && red(&bytes, 100, 55));
    assert!(!red(&bytes, 101, 51));
}
#[test]
fn spatial_segment_exact_endpoints() {
    let segment = SpatialSegment::new(
        Point3::new(0., 0., 0.),
        Point3::new(10., 0., 0.),
        red_color(),
    );
    let bytes = capture("spatial_segment", "+X+Y", Rc::new(segment));
    assert!(red(&bytes, 100, 50) && red(&bytes, 105, 50) && red(&bytes, 110, 50));
    assert!(!red(&bytes, 111, 50) && !red(&bytes, 105, 49));
}
#[test]
fn planar_segment_exact_endpoints() {
    let segment = PlanarSegment::new(Point2::new(0., 0.), Point2::new(10., 0.), red_color());
    let bytes = capture("planar_segment", "-2D-", Rc::new(segment));
    assert!(red(&bytes, 100, 50) && red(&bytes, 105, 50) && red(&bytes, 110, 50));
    assert!(!red(&bytes, 111, 50) && !red(&bytes, 105, 49));
}
