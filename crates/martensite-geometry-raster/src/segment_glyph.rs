//! Typed finite segments retain endpoints without interpreter-owned screen state.
use crate::state::DrawableState;
use martensite_raster_core::{PaletteColor, RasterDisplay, drawable::RasterDrawable};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Point2, Point3};
use std::cell::Cell;

/// Thread-confined finite segment; `SpatialSegment::new(first, last, color)`.
#[derive(Debug)]
pub struct SegmentGlyph<P: Copy> {
    state: DrawableState,
    first: Cell<P>,
    last: Cell<P>,
    color: PaletteColor,
}
/// Spatial finite segment; `SpatialSegment::new(first, last, color)`.
pub type SpatialSegment = SegmentGlyph<Point3>;
/// Planar finite segment; `PlanarSegment::new(first, last, color)`.
pub type PlanarSegment = SegmentGlyph<Point2>;
impl<P: Copy> SegmentGlyph<P> {
    /// Retains typed endpoints; `SpatialSegment::new(first, last, color)`.
    pub fn new(first: P, last: P, color: PaletteColor) -> Self {
        Self {
            state: DrawableState::default(),
            first: Cell::new(first),
            last: Cell::new(last),
            color,
        }
    }
    /// Reads the first endpoint; `let point = segment.first();`.
    pub fn first(&self) -> P {
        self.first.get()
    }
    /// Replaces the first endpoint; `segment.set_first(point)`.
    pub fn set_first(&self, point: P) {
        self.first.set(point);
    }
    /// Reads the last endpoint; `let point = segment.last();`.
    pub fn last(&self) -> P {
        self.last.get()
    }
    /// Replaces the last endpoint; `segment.set_last(point)`.
    pub fn set_last(&self, point: P) {
        self.last.set(point);
    }
}
// Keep the dimension-specific draw calls with one shared screen-state contract.
macro_rules! segment_raster_contract {
    ($point:ty, $spatial:expr, $draw:ident) => {
        impl RasterDrawable for SegmentGlyph<$point> {
            fn draw_on(&self, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
                display.set_color(self.color);
                display.$draw(self.first(), self.last());
                Ok(())
            }
            fn is_3d(&self) -> bool {
                $spatial
            }
            fn bounds(&self) -> [f64; 4] {
                self.state.bounds.get()
            }
            fn set_bounds(&self, bounds: [f64; 4]) {
                self.state.bounds.set(bounds);
            }
            fn visible(&self) -> bool {
                self.state.visible.get()
            }
            fn set_visible(&self, visible: bool) {
                self.state.visible.set(visible);
            }
        }
    };
}
segment_raster_contract!(Point3, true, draw_3d);
segment_raster_contract!(Point2, false, draw);
