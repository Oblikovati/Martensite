use martensite_geometry_raster::{TopologyDrawable, TopologyStyle, axes::AxisGlyph};
use martensite_raster_core::{
    PaletteColor, PaletteKind, RasterFailureContext, RasterReports, RasterViewer,
    VirtualWindowSystem, drawable::RasterDrawable,
};
use martensite_tkbrep::topods::Shape;
use std::{cell::RefCell, path::PathBuf, rc::Rc};

#[derive(Default)]
struct RecordingGeometryReports {
    output: RefCell<String>,
    errors: RefCell<String>,
}
impl RasterReports for RecordingGeometryReports {
    fn write_out(&self, text: &str) {
        self.output.borrow_mut().push_str(text);
    }
    fn write_err(&self, text: &str) {
        self.errors.borrow_mut().push_str(text);
    }
    fn flush(&self) {}
}
pub(super) fn viewer(kind: &str) -> RasterViewer {
    let system = VirtualWindowSystem::<RasterFailureContext>::connect_virtual().unwrap();
    let mut viewer = RasterViewer::with_adapter(Some(Rc::new(system)));
    viewer.make_view(1, kind, 728, 450, 400, 400).unwrap();
    viewer
}
pub(super) fn drawable(shape: Shape) -> Rc<TopologyDrawable> {
    Rc::new(TopologyDrawable::new(
        shape,
        TopologyStyle::default(),
        Rc::new(RecordingGeometryReports::default()),
    ))
}
fn artifact_path(identity: &str) -> PathBuf {
    if let Some(path) = std::env::var_os("MARTENSITE_GEOMETRY_ARTIFACT_DIR") {
        std::fs::create_dir_all(&path).unwrap();
        return PathBuf::from(path).join(format!("{identity}.png"));
    }
    std::env::temp_dir().join(format!(
        "geometry-renderer-{}-{identity}.ppm",
        std::process::id()
    ))
}
pub fn capture(shape: Shape, identity: &str) {
    let mut viewer = viewer("AXON");
    viewer.clear();
    let shape: Rc<dyn RasterDrawable> = drawable(shape);
    viewer.display(&shape).unwrap();
    let blue = PaletteColor::new(PaletteKind::Bleu);
    for axes in [AxisGlyph::spatial(blue, 20), AxisGlyph::planar(blue, 20)] {
        let axes: Rc<dyn RasterDrawable> = Rc::new(axes);
        viewer.display(&axes).unwrap();
    }
    viewer.fit_view(1, 10).unwrap();
    save_view(&mut viewer, identity);
    viewer.delete_view(1);
    assert!(!viewer.has_view(1));
}
fn save_view(viewer: &mut RasterViewer, identity: &str) {
    let path = artifact_path(identity);
    viewer.repaint_view(1).unwrap();
    assert!(viewer.save_view(1, path.to_str().unwrap()).unwrap());
    let pixels = std::fs::read(&path).unwrap();
    assert!(pixels.starts_with(b"P6\n400 400\n255\n"));
    assert_eq!(pixels.len(), 480015);
    assert!(
        pixels[15..]
            .as_chunks::<3>()
            .0
            .iter()
            .any(|rgb| rgb[0] != 0 || rgb[1] != 0)
    );
    if std::env::var_os("MARTENSITE_GEOMETRY_ARTIFACT_DIR").is_none() {
        std::fs::remove_file(path).unwrap();
    }
}

pub fn replay(identity: &str, image: &str, geometry: impl FnOnce() -> Shape) {
    crate::process_limit::run_with_limit(identity, 1000, || {
        capture(geometry(), image);
        Ok(())
    });
}
