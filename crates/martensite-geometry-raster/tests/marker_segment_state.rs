//! Coordinate/style mutation never aliases another drawable or its screen state.
use martensite_geometry_raster::{PlanarSegment, PointGlyph, SpatialSegment};
use martensite_raster_core::{MarkerShape, PaletteColor, PaletteKind, drawable::RasterDrawable};
use martensite_tkmath::gp::{Point2, Point3};
fn red() -> PaletteColor {
    PaletteColor::new(PaletteKind::Rouge)
}
#[test]
fn point_dimension_and_style_mutations_are_local() {
    let point = PointGlyph::new(Point3::new(1., 2., 3.), MarkerShape::Plus, red());
    let other = PointGlyph::new_2d(Point2::new(4., 5.), MarkerShape::X, red());
    point.set_point_2d(Point2::new(-2., -3.));
    assert!(!point.is_3d());
    assert_eq!(point.point(), Point3::new(-2., -3., 0.));
    point.set_point(Point3::new(6., 7., 8.));
    point.set_color(PaletteColor::new(PaletteKind::Bleu));
    point.set_shape(MarkerShape::Square);
    assert!(point.is_3d());
    assert_eq!(point.point_2d(), Point2::new(6., 7.));
    assert_eq!(point.shape(), MarkerShape::Square);
    assert_eq!(point.color(), PaletteColor::new(PaletteKind::Bleu));
    assert_eq!(other.point_2d(), Point2::new(4., 5.));
    assert_eq!((other.shape(), other.color()), (MarkerShape::X, red()));
}
#[test]
fn point_screen_membership_and_bounds_are_per_instance() {
    let point = PointGlyph::new_2d(Point2::new(1., 2.), MarkerShape::Plus, red());
    let other = PointGlyph::new_2d(Point2::new(1., 2.), MarkerShape::Plus, red());
    point.set_bounds([1., 2., 3., 4.]);
    point.set_visible(true);
    assert_eq!(point.bounds(), [1., 2., 3., 4.]);
    assert!(point.visible());
    assert_eq!(other.bounds(), [0.; 4]);
    assert!(!other.visible());
}
#[test]
fn typed_segment_endpoints_and_screen_state_mutate_independently() {
    let space = SpatialSegment::new(Point3::new(0., 0., 0.), Point3::new(1., 2., 3.), red());
    space.set_first(Point3::new(-1., -2., -3.));
    space.set_last(Point3::new(4., 5., 6.));
    assert_eq!(space.first(), Point3::new(-1., -2., -3.));
    assert_eq!(space.last(), Point3::new(4., 5., 6.));
    assert!(space.is_3d());
    segment_screen_state(&space);
    planar_segment_state();
}
fn planar_segment_state() {
    let plane = PlanarSegment::new(Point2::new(0., 0.), Point2::new(1., 2.), red());
    plane.set_first(Point2::new(-1., -2.));
    plane.set_last(Point2::new(3., 4.));
    assert_eq!(plane.first(), Point2::new(-1., -2.));
    assert_eq!(plane.last(), Point2::new(3., 4.));
    assert!(!plane.is_3d());
    assert!(!plane.visible());
    segment_screen_state(&plane);
}
fn segment_screen_state(segment: &dyn RasterDrawable) {
    assert_eq!(segment.bounds(), [0.; 4]);
    segment.set_bounds([1., 2., 3., 4.]);
    segment.set_visible(true);
    assert_eq!(segment.bounds(), [1., 2., 3., 4.]);
    assert!(segment.visible());
}
