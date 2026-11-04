//! Retained spatial curves share the existing engine without an interpreter context.
use super::CurveControls;
use crate::{CurveLook, CurveSampling, CurveStyle, SpatialCurveRendering};
use martensite_raster_core::{RasterDisplay, drawable::RasterDrawable};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomCurve;
/// Mutable controls and screen membership require a single rendering thread at a time.
/// Geometry remains live and shared through its retained handle.
/// ```compile_fail
/// use martensite_geometry_raster::SpatialCurveDrawable;
/// fn shared<T: Sync>() {}
/// shared::<SpatialCurveDrawable>();
/// ```
pub struct SpatialCurveDrawable {
    rendering: SpatialCurveRendering,
    controls: CurveControls,
}
impl SpatialCurveDrawable {
    /// Creates explicit retained controls; `SpatialCurveDrawable::with_look(curve, look, settings)`.
    pub fn with_look(
        curve: Handle<dyn GeomCurve>,
        look: CurveLook,
        (discret, deflection, mode): (i32, f64, i32),
    ) -> Self {
        Self {
            rendering: SpatialCurveRendering::new(curve),
            controls: CurveControls::new(look, (discret, deflection, mode)),
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
    pub fn curve(&self) -> Handle<dyn GeomCurve> {
        self.rendering.curve()
    }
}
impl RasterDrawable for SpatialCurveDrawable {
    fn draw_on(&self, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
        self.rendering
            .draw_on(&self.controls.sampling, &self.controls.style, display)
    }
    fn is_3d(&self) -> bool {
        true
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
