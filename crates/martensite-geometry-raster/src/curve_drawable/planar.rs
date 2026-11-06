//! Retained planar curves share the existing engine without an interpreter context.
use super::CurveControls;
use crate::{CurveLook, CurveSampling, CurveStyle, PlanarCurveRendering};
use martensite_raster_core::{RasterDisplay, drawable::RasterDrawable};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dCurve;
/// Mutable controls and screen membership require a single rendering thread at a time.
/// Geometry remains live and shared through its retained handle.
/// ```compile_fail
/// use martensite_geometry_raster::PlanarCurveDrawable;
/// fn shared<T: Sync>() {}
/// shared::<PlanarCurveDrawable>();
/// ```
pub struct PlanarCurveDrawable {
    rendering: PlanarCurveRendering,
    controls: CurveControls,
}
impl PlanarCurveDrawable {
    /// Creates explicit retained controls; `PlanarCurveDrawable::with_look(curve, look, 30)`.
    pub fn with_look(curve: Handle<dyn Geom2dCurve>, look: CurveLook, discret: i32) -> Self {
        Self {
            rendering: PlanarCurveRendering::new(curve),
            controls: CurveControls::new(look, (discret, 0.01, 0)),
        }
    }
    /// Borrows the live sampling cells; `curve.sampling().set_draw_mode(1)`.
    pub fn sampling(&self) -> &CurveSampling {
        &self.controls.sampling
    }
    /// Borrows the live style cells; `curve.style().show_curvature()`.
    pub fn style(&self) -> &CurveStyle {
        &self.controls.style
    }
    /// Shares the retained geometry; `let geometry = curve.curve();`.
    pub fn curve(&self) -> Handle<dyn Geom2dCurve> {
        self.rendering.curve()
    }
    /// Replaces the live geometry selection; `curve.set_curve(replacement)`.
    pub fn set_curve(&self, curve: Handle<dyn Geom2dCurve>) {
        self.rendering.set_curve(curve);
    }
}
impl RasterDrawable for PlanarCurveDrawable {
    fn draw_on(&self, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
        self.rendering
            .draw_on(&self.controls.sampling, &self.controls.style, display)
    }
    fn is_3d(&self) -> bool {
        false
    }
    fn bounds(&self) -> [f64; 4] {
        self.controls.state.bounds.get()
    }
    fn set_bounds(&self, bounds: [f64; 4]) {
        self.controls.state.bounds.set(bounds);
    }
    fn visible(&self) -> bool {
        self.controls.state.visible.get()
    }
    fn set_visible(&self, visible: bool) {
        self.controls.state.visible.set(visible);
    }
}
