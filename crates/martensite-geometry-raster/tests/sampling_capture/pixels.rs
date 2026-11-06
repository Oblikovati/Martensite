//! Actual P6 captures also check a finite line against exact projected pixel endpoints.
use super::{Geometry, SamplingDriver, probe::SamplingProbe};
use martensite_raster_core::{
    RasterFailureContext, RasterViewer, VirtualWindowSystem, drawable::RasterDrawable,
};
use std::rc::Rc;
pub fn capture<S: SamplingDriver>(label: &str, geometry: Geometry, mode: i32) {
    let mut viewer = live_viewer(geometry.spatial());
    let probe: Rc<dyn RasterDrawable> = Rc::new(SamplingProbe::<S>::new(geometry, mode));
    viewer.display(&probe).unwrap();
    viewer.repaint_view(1).unwrap();
    let bytes = saved(&mut viewer, label);
    if label.contains("-line-") {
        assert_line(&bytes, probe.bounds());
    }
    assert!(bytes[15..].as_chunks::<3>().0.contains(&[255, 0, 0]));
    viewer.delete_view(1);
}
pub fn live_viewer(spatial: bool) -> RasterViewer {
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
        .unwrap_or_else(|_| format!("/tmp/sampling-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let path = format!("{root}/{label}.ppm");
    assert!(viewer.save_view(1, &path).unwrap());
    let bytes = std::fs::read(path).unwrap();
    assert!(bytes.starts_with(b"P6\n400 400\n255\n"));
    assert_eq!(bytes.len(), 480015);
    bytes
}
fn assert_line(bytes: &[u8], bounds: [f64; 4]) {
    // Bounds are zoom-scaled view coordinates; [-10,10] at zoom10 gives [-100,100].
    assert_eq!(bounds, [-100., 100., 0., 0.]);
    for x in [100, 200, 300] {
        assert!(red(bytes, x, 200));
    }
    assert!(!red(bytes, 99, 200) && !red(bytes, 301, 200));
    assert!(!red(bytes, 200, 199) && !red(bytes, 200, 201));
}
fn red(bytes: &[u8], x: usize, y: usize) -> bool {
    let offset = 15 + (y * 400 + x) * 3;
    bytes[offset..offset + 3] == [255, 0, 0]
}
