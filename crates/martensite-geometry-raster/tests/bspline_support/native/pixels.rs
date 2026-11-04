//! Capture the rendered surface, including custom ranged drawings without repainting.
use super::super::spline_api::{self, DrawableHandle, Planar, Spatial, Viewer};
use martensite_raster_core::RasterDisplay;
use martensite_tkernel::standard::Failure;
use std::rc::Rc;
pub fn spatial(curve: Rc<Spatial>, label: &str) -> Vec<u8> {
    normal(spline_api::space_handle(curve), true, label)
}
pub fn planar(curve: Rc<Planar>, label: &str) -> Vec<u8> {
    normal(spline_api::plane_handle(curve), false, label)
}
fn normal(handle: DrawableHandle, space: bool, label: &str) -> Vec<u8> {
    let mut viewer = spline_api::viewer(space);
    spline_api::reset_visibility(&handle);
    viewer.display(&handle).unwrap();
    assert!(Rc::ptr_eq(&viewer.drawables()[0], &handle));
    viewer.repaint_view(1).unwrap();
    let bytes = save(&mut viewer, label);
    viewer.delete_view(1);
    bytes
}
pub fn manual(
    label: &str,
    draw: impl FnOnce(&mut RasterDisplay<'_>) -> Result<(), Failure>,
) -> Vec<u8> {
    let mut viewer = spline_api::viewer(true);
    {
        let mut display = viewer.make_display(1);
        draw(&mut display).unwrap();
        display.flush();
    }
    let bytes = save(&mut viewer, label);
    viewer.delete_view(1);
    bytes
}
fn save(viewer: &mut Viewer, label: &str) -> Vec<u8> {
    let root = std::env::var("MARTENSITE_GEOMETRY_ARTIFACT_DIR")
        .unwrap_or_else(|_| format!("/tmp/bspline-curves-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let path = format!("{root}/{label}.ppm");
    assert!(viewer.save_view(1, &path).unwrap());
    let bytes = std::fs::read(path).unwrap();
    assert!(bytes.starts_with(b"P6\n400 400\n255\n"));
    assert_eq!(bytes.len(), 480015);
    bytes
}
pub fn color(bytes: &[u8], x: usize, y: usize) -> [u8; 3] {
    let offset = 15 + (400 * y + x) * 3;
    bytes[offset..offset + 3].try_into().unwrap()
}
