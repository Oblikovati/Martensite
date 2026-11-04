//! Owned coordinate fixtures pin input independence and display-list identity.
use crate::{BoxGlyph, BoxRendering};
use martensite_raster_core::{
    MemoryWindowSystem, PaletteColor, PaletteKind, RasterViewer, drawable::RasterDrawable,
};
use martensite_tkmath::{
    bnd::OrientedBoundingBox,
    gp::{Point3, dx, dy, dz},
};
use std::rc::Rc;
fn box_value() -> OrientedBoundingBox {
    OrientedBoundingBox::new(Point3::new(1., 2., 3.), [dy(), dz(), dx()], 1., 2., 3.)
}
fn orange() -> PaletteColor {
    PaletteColor::new(PaletteKind::Orange)
}
#[test]
fn box_coordinates_and_axis_moves_follow_the_owned_rotated_frame() {
    let box_rendering = BoxRendering::new(box_value(), orange());
    let corner = box_rendering.to_wcs(-1., -2., -3.);
    assert_eq!(corner, Point3::new(-2., 1., 1.));
    let opposite = box_rendering.move_z(
        6.,
        box_rendering.move_y(4., box_rendering.move_x(2., corner)),
    );
    assert_eq!(opposite, Point3::new(4., 3., 5.));
}
#[test]
fn later_source_and_returned_box_copies_do_not_change_rendering_input() {
    let mut source = box_value();
    let glyph = BoxGlyph::new(source, orange());
    let rendering = BoxRendering::new(source, orange());
    source.set_center(Point3::new(99., 98., 97.));
    let mut returned = *glyph.obb();
    returned.set_void();
    assert_eq!(glyph.obb(), &box_value());
    assert_eq!(rendering.obb(), &box_value());
    assert_ne!(source, *rendering.obb());
}
#[test]
fn box_screen_state_is_thread_confined_and_per_instance() {
    let first = BoxGlyph::new(box_value(), orange());
    let second = BoxGlyph::new(box_value(), orange());
    first.set_bounds([1., 2., 3., 4.]);
    first.set_visible(true);
    assert!(first.is_3d());
    assert_eq!(first.bounds(), [1., 2., 3., 4.]);
    assert!(first.visible());
    assert!(!second.visible());
    assert_eq!(second.bounds(), [0.; 4]);
}
#[test]
fn raster_viewer_retains_the_same_rc_and_removes_only_that_instance() {
    let first: Rc<dyn RasterDrawable> = Rc::new(BoxGlyph::new(box_value(), orange()));
    let other: Rc<dyn RasterDrawable> = Rc::new(BoxGlyph::new(box_value(), orange()));
    let mut viewer: RasterViewer =
        RasterViewer::with_adapter(Some(Rc::new(MemoryWindowSystem::new())));
    viewer.add_drawable(&first);
    viewer.add_drawable(&other);
    assert!(Rc::ptr_eq(&viewer.drawables()[0], &first));
    assert!(Rc::ptr_eq(&viewer.drawables()[1], &other));
    viewer.remove_drawable(&first);
    assert!(!first.visible());
    assert!(other.visible());
    assert_eq!(viewer.drawables().len(), 1);
    assert!(Rc::ptr_eq(&viewer.drawables()[0], &other));
}
#[test]
fn immutable_rendering_values_are_send_sync() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<BoxRendering>();
}
