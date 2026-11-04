//! Actual palette pixels are checked against exact planar projections.
use super::{Workload, probe::HelperProbe};
use martensite_raster_core::{
    RasterFailureContext, RasterViewer, VirtualWindowSystem, drawable::RasterDrawable,
};
use std::rc::Rc;
pub fn capture(label: &str, workload: Workload) {
    let mut viewer = viewer(workload.spatial());
    let probe: Rc<dyn RasterDrawable> = Rc::new(HelperProbe::new(workload));
    viewer.display(&probe).unwrap();
    viewer.repaint_view(1).unwrap();
    let bytes = saved(&mut viewer, label);
    assert!(bytes[15..].as_chunks::<3>().0.contains(&[255, 0, 0]));
    assert_expected_pixels(&bytes, workload);
    viewer.delete_view(1);
}
fn viewer(spatial: bool) -> RasterViewer {
    let system = VirtualWindowSystem::<RasterFailureContext>::connect_virtual().unwrap();
    let mut viewer = RasterViewer::with_adapter(Some(Rc::new(system)));
    viewer
        .make_view(1, if spatial { "+X+Y" } else { "-2D-" }, 0, 0, 400, 400)
        .unwrap();
    viewer.set_zoom(1, 10.);
    viewer
}
fn saved(viewer: &mut RasterViewer, label: &str) -> Vec<u8> {
    let root = std::env::var("MARTENSITE_GEOMETRY_ARTIFACT_DIR")
        .unwrap_or_else(|_| format!("/tmp/curve-style-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let path = format!("{root}/{label}.ppm");
    assert!(viewer.save_view(1, &path).unwrap());
    let bytes = std::fs::read(path).unwrap();
    assert!(bytes.starts_with(b"P6\n400 400\n255\n"));
    assert_eq!(bytes.len(), 480015);
    bytes
}
fn assert_expected_pixels(bytes: &[u8], workload: Workload) {
    match workload {
        Workload::Polygon(_) => {
            assert!(red(bytes, 150, 230) && red(bytes, 200, 150) && red(bytes, 250, 230))
        }
        Workload::Knots(_, _) => assert!(
            red(bytes, 145, 200)
                && red(bytes, 155, 200)
                && red(bytes, 245, 200)
                && red(bytes, 255, 200)
        ),
        Workload::Net(_) => assert!(red(bytes, 180, 180) && red(bytes, 220, 220)),
        Workload::Curvature(_, capped) => assert_radius(bytes, capped),
    }
}
fn assert_radius(bytes: &[u8], capped: bool) {
    let first = if capped { 245 } else { 240 };
    for x in first..=250 {
        assert!(red(bytes, x, 200));
    }
    assert!(!red(bytes, first - 1, 200));
    assert!(!red(bytes, 251, 200));
}
fn red(bytes: &[u8], x: usize, y: usize) -> bool {
    let offset = 15 + (y * 400 + x) * 3;
    bytes[offset..offset + 3] == [255, 0, 0]
}
