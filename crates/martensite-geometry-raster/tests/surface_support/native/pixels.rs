//! These captures run the real viewer paths, with no recording renderer in their place.
use super::super::surface_api::{self, Surface, Viewer};
use std::rc::Rc;
pub fn normal(surface: Rc<Surface>, zoom: f64, label: &str) -> Vec<u8> {
    let mut viewer = surface_api::viewer(zoom);
    let handle = surface_api::handle(surface);
    surface_api::reset_visibility(&handle);
    viewer.display(&handle).unwrap();
    assert!(Rc::ptr_eq(&viewer.drawables()[0], &handle));
    viewer.repaint_view(1).unwrap();
    let bytes = save(&mut viewer, label);
    viewer.delete_view(1);
    bytes
}
pub fn manual(surface: &Surface, zoom: f64, isos: bool, label: &str) -> Vec<u8> {
    let mut viewer = surface_api::viewer(zoom);
    {
        let mut display = viewer.make_display(1);
        surface.draw_on_with_isos(&mut display, isos).unwrap();
        display.flush();
    }
    let bytes = save(&mut viewer, label);
    viewer.delete_view(1);
    bytes
}
fn save(viewer: &mut Viewer, label: &str) -> Vec<u8> {
    let root = std::env::var("MARTENSITE_GEOMETRY_ARTIFACT_DIR")
        .unwrap_or_else(|_| format!("/tmp/surface-curves-{}", std::process::id()));
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
