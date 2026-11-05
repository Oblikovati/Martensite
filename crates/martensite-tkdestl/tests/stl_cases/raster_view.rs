//! The drawing of a shape in a memory raster, which needs no display.
use martensite_geometry_raster::{TopologyDrawable, TopologyStyle};
use martensite_raster_core::{
    MemoryWindowSystem, RasterReports, RasterViewer, ZeroWidthLineRule, drawable::RasterDrawable,
};
use martensite_tkbrep::topods::Shape;
use std::rc::Rc;

/// Drawing diagnostics go to the case's own streams.
struct CaseStreams;
impl RasterReports for CaseStreams {
    fn write_out(&self, text: &str) {
        print!("{text}");
    }
    fn write_err(&self, text: &str) {
        eprint!("{text}");
    }
    fn flush(&self) {}
}

const VIEW: i32 = 1;
/// `smallview`: the axonometric view at (728, 450), 400 pixels wide and high.
const SMALL_VIEW: (&str, i32, i32, i32, i32) = ("AXON", 728, 450, 400, 400);
const FIT_FRAME: i32 = 10;
const BACKGROUND: [u8; 3] = [0, 0, 0];

/// Draws the shape, captures the view and refuses an empty image.
pub fn assert_shape_draws(shape: &Shape, case: &str) {
    let system = MemoryWindowSystem::with_line_rule(ZeroWidthLineRule::Accelerated);
    let mut viewer: RasterViewer = RasterViewer::with_adapter(Some(Rc::new(system)));
    let (kind, x, y, width, height) = SMALL_VIEW;
    viewer.make_view(VIEW, kind, x, y, width, height).unwrap();
    let drawable: Rc<dyn RasterDrawable> = Rc::new(TopologyDrawable::new(
        shape.clone(),
        TopologyStyle::default(),
        Rc::new(CaseStreams),
    ));
    viewer.display(&drawable).unwrap();
    viewer.fit_view(VIEW, FIT_FRAME).unwrap();
    viewer.repaint_view(VIEW).unwrap();
    let path = std::env::temp_dir().join(format!("tkdestl-{}-{case}.ppm", std::process::id()));
    assert!(viewer.save_view(VIEW, path.to_str().unwrap()).unwrap());
    let bytes = std::fs::read(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert!(drawn_pixels(&bytes) > 0, "{case}: expected a drawn image");
}

/// The pixels of a binary pixmap that are not the background.
fn drawn_pixels(image: &[u8]) -> usize {
    let header = format!("P6\n{} {}\n255\n", SMALL_VIEW.3, SMALL_VIEW.4);
    assert!(
        image.starts_with(header.as_bytes()),
        "expected a pixmap of the window"
    );
    let (pixels, _) = image[header.len()..].as_chunks::<3>();
    pixels.iter().filter(|pixel| **pixel != BACKGROUND).count()
}
