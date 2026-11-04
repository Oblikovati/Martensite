//! Each fixture drawable owns screen membership independently of helper style state.
use super::{Workload, fixtures};
use martensite_raster_core::{PaletteColor, PaletteKind, RasterDisplay, drawable::RasterDrawable};
use martensite_tkernel::standard::Failure;
use std::cell::Cell;
pub struct HelperProbe {
    workload: Workload,
    bounds: Cell<[f64; 4]>,
    visible: Cell<bool>,
}
impl HelperProbe {
    pub fn new(workload: Workload) -> Self {
        Self {
            workload,
            bounds: Cell::new([0.; 4]),
            visible: Cell::new(false),
        }
    }
}
impl RasterDrawable for HelperProbe {
    fn draw_on(&self, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
        display.set_color(PaletteColor::new(PaletteKind::Rouge));
        fixtures::draw(self.workload, display)
    }
    fn is_3d(&self) -> bool {
        self.workload.spatial()
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
