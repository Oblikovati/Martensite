//! The same retained object and view identity are used before and after changes.
use super::super::{
    curve_api::{self, CurveDrawableContract, PlanarCurve, SpatialCurve, Viewer},
    fixtures,
};
use std::rc::Rc;
pub fn spatial(curve: Rc<SpatialCurve>, label: &str) -> Vec<u8> {
    let mut viewer = curve_api::viewer(true);
    let handle = curve_api::spatial_handle(curve.clone());
    viewer.display(&handle).unwrap();
    assert!(Rc::ptr_eq(&viewer.drawables()[0], &handle));
    let bytes = capture(&mut viewer, label);
    assert!(curve.visible());
    viewer.delete_view(1);
    bytes
}
pub fn planar(curve: Rc<PlanarCurve>, label: &str) -> Vec<u8> {
    let mut viewer = curve_api::viewer(false);
    let handle = curve_api::planar_handle(curve.clone());
    viewer.display(&handle).unwrap();
    assert!(Rc::ptr_eq(&viewer.drawables()[0], &handle));
    let bytes = capture(&mut viewer, label);
    assert!(curve.visible());
    viewer.delete_view(1);
    bytes
}
pub fn capture(viewer: &mut Viewer, label: &str) -> Vec<u8> {
    viewer.repaint_view(1).unwrap();
    let root = std::env::var("MARTENSITE_GEOMETRY_ARTIFACT_DIR")
        .unwrap_or_else(|_| format!("/tmp/generic-curves-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let path = format!("{root}/{label}.ppm");
    assert!(viewer.save_view(1, &path).unwrap());
    let bytes = std::fs::read(path).unwrap();
    assert!(bytes.starts_with(b"P6\n400 400\n255\n"));
    assert_eq!(bytes.len(), 480015);
    assert!(bytes[15..].iter().any(|value| *value != 0));
    bytes
}
pub fn red(bytes: &[u8], x: usize, y: usize) -> bool {
    let offset = 15 + (y * 400 + x) * 3;
    bytes[offset..offset + 3] == [255, 0, 0]
}
pub fn finite_line(bytes: &[u8]) {
    for x in [100, 200, 300] {
        assert!(red(bytes, x, 200));
    }
    assert!(!red(bytes, 99, 200) && !red(bytes, 301, 200));
    assert!(!red(bytes, 200, 199));
}
#[test]
fn failed_display_preserves_the_existing_membership_state() {
    let curve = Rc::new(SpatialCurve::with_look(
        Default::default(),
        fixtures::red_look(),
        (30, 0.01, 0),
    ));
    let mut viewer = curve_api::viewer(true);
    let handle = curve_api::spatial_handle(curve.clone());
    let error = viewer.display(&handle).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_NullObject");
    assert_eq!(error.message(), "null geometry in a drawable");
    assert!(curve.visible());
    assert!(Rc::ptr_eq(&viewer.drawables()[0], &handle));
    viewer.delete_view(1);
}
