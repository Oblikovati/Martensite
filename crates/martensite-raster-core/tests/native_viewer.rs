//! Independent planar and rotated-line captures exercise the standalone drawable contract.
#![cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
#[path = "support/line_drawable.rs"]
mod line_drawable;
use line_drawable::LineDrawable;
use martensite_raster_core::{
    FormattedText, RasterFailureContext, RasterViewer, VirtualWindowSystem, display::DrawingMode,
    drawable::RasterDrawable,
};
use martensite_tkmath::gp::{Point3, dz};
use std::rc::Rc;

fn viewer(kind: &str) -> RasterViewer {
    let system = VirtualWindowSystem::<RasterFailureContext>::connect_virtual().unwrap();
    let mut viewer = RasterViewer::with_adapter(Some(Rc::new(system)));
    viewer.make_view(1, kind, 0, 0, 200, 100).unwrap();
    viewer
}
fn captured(viewer: &mut RasterViewer, label: &str) -> Vec<u8> {
    let path =
        std::env::temp_dir().join(format!("raster-viewer-{label}-{}.ppm", std::process::id()));
    assert!(viewer.save_view(1, path.to_str().unwrap()).unwrap());
    let pixels = std::fs::read(&path).unwrap();
    std::fs::remove_file(path).unwrap();
    assert!(pixels.starts_with(b"P6\n200 100\n255\n"));
    assert_eq!(pixels.len(), 15 + 200 * 100 * 3);
    pixels[15..].to_vec()
}
fn red(pixels: &[u8], x: usize, y: usize) -> bool {
    let offset = (y * 200 + x) * 3;
    pixels[offset..offset + 3] == [255, 0, 0]
}
#[test]
fn planar_fit_retains_shared_identity_and_exact_pixel_line() {
    let mut viewer = viewer("-2D-");
    let line: Rc<dyn RasterDrawable> = Rc::new(LineDrawable::new(false, false));
    viewer.display(&line).unwrap();
    assert!(Rc::ptr_eq(&viewer.drawables()[0], &line));
    assert_eq!(line.bounds(), [0., 10., 0., 0.]);
    viewer.fit_view(1, 10).unwrap();
    assert_eq!((viewer.zoom(1), viewer.pan(1)), (18., Some((10, -50))));
    viewer.repaint_view(1).unwrap();
    let pixels = captured(&mut viewer, "planar");
    assert!(red(&pixels, 10, 50) && red(&pixels, 100, 50) && red(&pixels, 190, 50));
    assert!(!red(&pixels, 100, 49));
    viewer.clear_2d();
    assert!(!line.visible());
    viewer.delete_view(1);
    assert!(!viewer.has_view(1));
}
#[test]
fn quarter_turn_projects_spatial_line_vertically() {
    let mut viewer = viewer("+X+Y");
    let line: Rc<dyn RasterDrawable> = Rc::new(LineDrawable::new(true, false));
    viewer.display(&line).unwrap();
    viewer.rotate_view(
        1,
        Point3::new(0., 0., 0.),
        dz(),
        std::f64::consts::FRAC_PI_2,
    );
    viewer.repaint_view(1).unwrap();
    let pixels = captured(&mut viewer, "rotation");
    assert!(red(&pixels, 100, 50) && red(&pixels, 100, 45) && red(&pixels, 100, 40));
    assert!(!red(&pixels, 105, 50));
    viewer.delete_view(1);
}
#[test]
fn failed_geometry_keeps_error_and_restores_output_modes() {
    let mut viewer = viewer("+X+Y");
    let line: Rc<dyn RasterDrawable> = Rc::new(LineDrawable::new(true, true));
    let failure = viewer.display(&line).unwrap_err();
    assert_eq!(failure.message(), "line: expected available geometry");
    assert!(line.visible());
    assert!(viewer.pick(1, 0, 0, 2, 0).is_err());
    assert_eq!(viewer.display_state().mode, DrawingMode::Draw);
    let mut output = FormattedText::new();
    assert!(
        viewer
            .postscript_view(1, [0, 0, 100, 100], [0, 0, 200, 200], &mut output)
            .is_err()
    );
    assert_eq!(viewer.display_state().mode, DrawingMode::Draw);
    viewer.delete_view(1);
}
