//! This object only delegates sampling and owns its independent viewer bookkeeping.
use super::{Geometry, SamplingDriver};
use martensite_raster_core::{PaletteColor, PaletteKind, RasterDisplay, drawable::RasterDrawable};
use martensite_tkernel::standard::Failure;
use std::cell::Cell;
pub struct SamplingProbe<S> {
    sampler: S,
    geometry: Geometry,
    bounds: Cell<[f64; 4]>,
    visible: Cell<bool>,
}
impl<S: SamplingDriver> SamplingProbe<S> {
    pub fn new(geometry: Geometry, mode: i32) -> Self {
        Self {
            sampler: S::configured(mode),
            geometry,
            bounds: Cell::new([0.; 4]),
            visible: Cell::new(false),
        }
    }
}
impl<S: SamplingDriver> RasterDrawable for SamplingProbe<S> {
    fn draw_on(&self, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
        display.set_color(PaletteColor::new(PaletteKind::Rouge));
        self.sampler.draw_geometry(&self.geometry, display)
    }
    fn is_3d(&self) -> bool {
        self.geometry.spatial()
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
