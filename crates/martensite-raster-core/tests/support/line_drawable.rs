use martensite_raster_core::{PaletteColor, PaletteKind, RasterDisplay, drawable::RasterDrawable};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Point2, Point3};
use std::cell::Cell;

pub struct LineDrawable {
    pub spatial: bool,
    pub fail: bool,
    bounds: Cell<[f64; 4]>,
    visible: Cell<bool>,
}
impl LineDrawable {
    pub fn new(spatial: bool, fail: bool) -> Self {
        Self {
            spatial,
            fail,
            bounds: Cell::new([0.; 4]),
            visible: Cell::new(false),
        }
    }
}
impl RasterDrawable for LineDrawable {
    fn draw_on(&self, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
        if self.fail {
            return Err(Failure::with_message(
                "LineEvaluationFailure",
                "line: expected available geometry",
            ));
        }
        display.set_color(PaletteColor::new(PaletteKind::Rouge));
        if self.spatial {
            display.draw_3d(Point3::new(0., 0., 0.), Point3::new(10., 0., 0.));
        } else {
            display.draw(Point2::new(0., 0.), Point2::new(10., 0.));
        }
        Ok(())
    }
    fn is_3d(&self) -> bool {
        self.spatial
    }
    fn bounds(&self) -> [f64; 4] {
        self.bounds.get()
    }
    fn set_bounds(&self, bounds: [f64; 4]) {
        self.bounds.set(bounds);
    }
    fn visible(&self) -> bool {
        self.visible.get()
    }
    fn set_visible(&self, visible: bool) {
        self.visible.set(visible);
    }
}
