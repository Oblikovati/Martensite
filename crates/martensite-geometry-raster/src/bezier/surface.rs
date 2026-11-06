//! Retained Bezier surfaces own one actual generic surface and pole style.
use super::surface_rendering;
use crate::{PolesStyle, SurfaceDrawable, SurfaceLook};
use martensite_raster_core::{PaletteColor, PaletteKind, RasterDisplay, drawable::RasterDrawable};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomBezierSurface, GeomSurface};
/// Rendering cells are Send but not Sync; geometry stays live and shared.
/// ```compile_fail
/// use martensite_geometry_raster::BezierSurfaceDrawable;
/// fn shared<T: Sync>() {}
/// shared::<BezierSurfaceDrawable>();
/// ```
pub struct BezierSurfaceDrawable {
    surface: SurfaceDrawable,
    poles: PolesStyle,
}
impl BezierSurfaceDrawable {
    /// Creates shown red poles, one blue iso each way and yellow bounds; `Self::new(surface)`.
    pub fn new(surface: Handle<GeomBezierSurface>) -> Self {
        let look = SurfaceLook {
            nb_u_isos: 1,
            nb_v_isos: 1,
            bounds_color: PaletteColor::new(PaletteKind::Jaune),
            isos_color: PaletteColor::new(PaletteKind::Bleu),
        };
        Self::with_style(surface, look, PolesStyle::default_shown(), (30, 0.05, 0))
    }
    /// Creates explicit live controls; `Self::with_style(surface,look,poles,(30,0.01,0))`.
    pub fn with_style(
        surface: Handle<GeomBezierSurface>,
        look: SurfaceLook,
        poles: PolesStyle,
        settings: (i32, f64, i32),
    ) -> Self {
        let geometry = surface.cast::<dyn GeomSurface>();
        Self {
            surface: SurfaceDrawable::with_look(geometry, look, settings),
            poles,
        }
    }
    /// Borrows the actual generic part; `surface.surface_part().clear_isos()`.
    pub fn surface_part(&self) -> &SurfaceDrawable {
        &self.surface
    }
    /// Borrows actual pole options; `surface.poles().clear()`.
    pub fn poles(&self) -> &PolesStyle {
        &self.poles
    }
    fn bezier(&self) -> Handle<GeomBezierSurface> {
        self.surface.surface().cast::<GeomBezierSurface>()
    }
    /// Finds first-after-indices poles in pixels; `surface.find_pole((x,y),display,1.,(0,0))?`.
    pub fn find_pole(
        &self,
        point: (f64, f64),
        display: &RasterDisplay<'_>,
        precision: f64,
        indices: (i32, i32),
    ) -> Result<(i32, i32), Failure> {
        let handle = self.bezier();
        surface_rendering::find_pole(&handle, point, display, precision, indices)
    }
}
impl RasterDrawable for BezierSurfaceDrawable {
    fn draw_on(&self, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
        surface_rendering::draw_poles(|| self.bezier(), &self.poles, display)?;
        self.surface.draw_on_with_isos(display, true)
    }
    fn is_3d(&self) -> bool {
        self.surface.is_3d()
    }
    fn bounds(&self) -> [f64; 4] {
        self.surface.bounds()
    }
    fn set_bounds(&self, bounds: [f64; 4]) {
        self.surface.set_bounds(bounds);
    }
    fn visible(&self) -> bool {
        self.surface.visible()
    }
    fn set_visible(&self, visible: bool) {
        self.surface.set_visible(visible);
    }
}
