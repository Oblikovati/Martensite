//! Mutable point markers retain dimension and style without interpreter state.
use crate::state::DrawableState;
use martensite_raster_core::{MarkerShape, PaletteColor, RasterDisplay, drawable::RasterDrawable};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Point2, Point3};
use std::cell::Cell;

/// The `Size` default of `RasterDisplay::DrawMarker`, in pixels.
const MARKER_SIZE: i32 = 5;

/// Thread-confined retained point marker; `PointGlyph::new(point, marker, color)`.
#[derive(Debug)]
pub struct PointGlyph {
    state: DrawableState,
    point: Cell<Point3>,
    spatial: Cell<bool>,
    marker: Cell<MarkerShape>,
    color: Cell<PaletteColor>,
}
impl PointGlyph {
    /// Creates a spatial marker; `PointGlyph::new(point, marker, color)`.
    pub fn new(point: Point3, marker: MarkerShape, color: PaletteColor) -> Self {
        Self {
            state: DrawableState::default(),
            point: Cell::new(point),
            spatial: Cell::new(true),
            marker: Cell::new(marker),
            color: Cell::new(color),
        }
    }
    /// Creates a planar marker; `PointGlyph::new_2d(point, marker, color)`.
    pub fn new_2d(point: Point2, marker: MarkerShape, color: PaletteColor) -> Self {
        let glyph = Self::new(Point3::new(point.x(), point.y(), 0.), marker, color);
        glyph.spatial.set(false);
        glyph
    }
    /// Reads spatial coordinates; `let point = glyph.point();`.
    pub fn point(&self) -> Point3 {
        self.point.get()
    }
    /// Replaces coordinates and selects spatial drawing; `glyph.set_point(point)`.
    pub fn set_point(&self, point: Point3) {
        self.point.set(point);
        self.spatial.set(true);
    }
    /// Reads planar coordinates; `let point = glyph.point_2d();`.
    pub fn point_2d(&self) -> Point2 {
        let point = self.point.get();
        Point2::new(point.x(), point.y())
    }
    /// Replaces coordinates and selects planar drawing; `glyph.set_point_2d(point)`.
    pub fn set_point_2d(&self, point: Point2) {
        self.point.set(Point3::new(point.x(), point.y(), 0.));
        self.spatial.set(false);
    }
    /// Reads marker color; `let color = glyph.color();`.
    pub fn color(&self) -> PaletteColor {
        self.color.get()
    }
    /// Changes marker color; `glyph.set_color(color)`.
    pub fn set_color(&self, color: PaletteColor) {
        self.color.set(color);
    }
    /// Reads marker shape; `let marker = glyph.shape();`.
    pub fn shape(&self) -> MarkerShape {
        self.marker.get()
    }
    /// Changes marker shape; `glyph.set_shape(marker)`.
    pub fn set_shape(&self, marker: MarkerShape) {
        self.marker.set(marker);
    }
}
impl RasterDrawable for PointGlyph {
    fn draw_on(&self, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
        display.set_color(self.color());
        if self.spatial.get() {
            display.draw_marker_3d(self.point(), self.shape(), MARKER_SIZE);
        } else {
            display.draw_marker(self.point_2d(), self.shape(), MARKER_SIZE);
        }
        Ok(())
    }
    fn is_3d(&self) -> bool {
        self.spatial.get()
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
