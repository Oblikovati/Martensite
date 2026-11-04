//! Exact ordered oriented-box edges over a value-owned box snapshot.
use martensite_raster_core::{PaletteColor, RasterDisplay};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::{
    bnd::OrientedBoundingBox,
    gp::{Point3, Xyz},
};

/// Immutable rendering inputs; later mutation of a copied source box is independent.
/// Example: `let rendering = BoxRendering::new(box_value, color);`.
#[derive(Debug, Clone, Copy)]
pub struct BoxRendering {
    obb: OrientedBoundingBox,
    color: PaletteColor,
}
impl BoxRendering {
    /// Retain the defining values by copy; `BoxRendering::new(obb, color)`.
    pub fn new(obb: OrientedBoundingBox, color: PaletteColor) -> Self {
        Self { obb, color }
    }

    /// Borrow the owned box values; `rendering.obb().is_void()`.
    pub fn obb(&self) -> &OrientedBoundingBox {
        &self.obb
    }

    /// Convert box-frame coordinates; `rendering.to_wcs(1., 2., 3.)`.
    pub fn to_wcs(&self, x: f64, y: f64, z: f64) -> Point3 {
        let obb = &self.obb;
        Point3::from(
            obb.center() + x * obb.x_direction() + y * obb.y_direction() + z * obb.z_direction(),
        )
    }

    /// `point` shifted by `shift` along a box axis.
    fn moved(point: Point3, shift: f64, axis: Xyz) -> Point3 {
        Point3::from(point.xyz() + shift * axis)
    }

    /// Translate along box X; `rendering.move_x(2., point)`.
    pub fn move_x(&self, shift: f64, point: Point3) -> Point3 {
        Self::moved(point, shift, self.obb.x_direction())
    }

    /// Translate along box Y; `rendering.move_y(2., point)`.
    pub fn move_y(&self, shift: f64, point: Point3) -> Point3 {
        Self::moved(point, shift, self.obb.y_direction())
    }

    /// Translate along box Z; `rendering.move_z(2., point)`.
    pub fn move_z(&self, shift: f64, point: Point3) -> Point3 {
        Self::moved(point, shift, self.obb.z_direction())
    }
}

impl BoxRendering {
    /// Draw bottom/top rectangles then uprights; a void box leaves state unchanged.
    /// Example: `rendering.draw_on(&mut display)?`.
    pub fn draw_on(&self, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
        if self.obb.is_void() {
            return Ok(());
        }
        display.set_color(self.color);
        let (hx, hy, hz) = (
            self.obb.x_h_size(),
            self.obb.y_h_size(),
            self.obb.z_h_size(),
        );
        self.draw_rectangles(display, hx, hy, hz);
        self.draw_uprights(display, hx, hy, hz);
        Ok(())
    }
}

impl BoxRendering {
    fn draw_rectangles(&self, display: &mut RasterDisplay<'_>, hx: f64, hy: f64, hz: f64) {
        let mut point = self.to_wcs(-hx, -hy, -hz);
        display.move_to_3d(point);
        for _ in 0..2 {
            // The rectangle ends where it started; the next one starts at the top face.
            self.draw_rectangle(display, point, hx, hy);
            point = self.to_wcs(-hx, -hy, hz);
            display.move_to_3d(point);
        }
    }
    fn draw_uprights(&self, display: &mut RasterDisplay<'_>, hx: f64, hy: f64, hz: f64) {
        for (x, y) in [(-hx, -hy), (hx, -hy), (hx, hy), (-hx, hy)] {
            let bottom = self.to_wcs(x, y, -hz);
            display.move_to_3d(bottom);
            display.draw_to_3d(self.move_z(2.0 * hz, bottom));
        }
    }
    /// Draws the rectangle of the box face through `point` (X, Y, -X, -Y).
    fn draw_rectangle(&self, display: &mut RasterDisplay<'_>, mut point: Point3, hx: f64, hy: f64) {
        point = self.move_x(2.0 * hx, point);
        display.draw_to_3d(point);
        point = self.move_y(2.0 * hy, point);
        display.draw_to_3d(point);
        point = self.move_x(-2.0 * hx, point);
        display.draw_to_3d(point);
        point = self.move_y(-2.0 * hy, point);
        display.draw_to_3d(point);
    }
}
