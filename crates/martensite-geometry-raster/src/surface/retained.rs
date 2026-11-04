//! Retained surface drawing owns mutable controls exactly once.
use super::{SurfaceLook, SurfaceRendering, SurfaceStyle};
use crate::{CurveSampling, state::DrawableState};
use martensite_raster_core::{PaletteColor, PaletteKind, RasterDisplay, drawable::RasterDrawable};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomSurface;
/// Thread-confined rendering cells are Send but not Sync; geometry remains shared live.
/// ```compile_fail
/// use martensite_geometry_raster::SurfaceDrawable;
/// fn shared<T: Sync>() {}
/// shared::<SurfaceDrawable>();
/// ```
pub struct SurfaceDrawable {
    rendering: SurfaceRendering,
    sampling: CurveSampling,
    style: SurfaceStyle,
    state: DrawableState,
}
impl SurfaceDrawable {
    /// Creates one iso each way with yellow bounds and blue isos; `Self::new(surface)`.
    pub fn new(surface: Handle<dyn GeomSurface>) -> Self {
        let look = SurfaceLook {
            nb_u_isos: 1,
            nb_v_isos: 1,
            bounds_color: PaletteColor::new(PaletteKind::Jaune),
            isos_color: PaletteColor::new(PaletteKind::Bleu),
        };
        Self::with_look(surface, look, (16, 0.01, 1))
    }
    /// Creates explicit retained controls; `Self::with_look(surface,look,(30,0.01,0))`.
    pub fn with_look(
        surface: Handle<dyn GeomSurface>,
        look: SurfaceLook,
        (discret, deflection, mode): (i32, f64, i32),
    ) -> Self {
        Self {
            rendering: SurfaceRendering::new(surface),
            sampling: CurveSampling::new(discret, deflection, mode),
            style: SurfaceStyle::new(look),
            state: DrawableState::default(),
        }
    }
    /// Borrows actual sampling controls; `surface.sampling().set_draw_mode(1)`.
    pub fn sampling(&self) -> &CurveSampling {
        &self.sampling
    }
    /// Borrows actual surface controls; `surface.style().clear_isos()`.
    pub fn style(&self) -> &SurfaceStyle {
        &self.style
    }
    /// Shares live geometry; `let geometry = surface.surface();`.
    pub fn surface(&self) -> Handle<dyn GeomSurface> {
        self.rendering.surface()
    }
    /// Uses the actual surface control cells; `surface.bounds_color()`.
    pub fn bounds_color(&self) -> PaletteColor {
        self.style.bounds_color()
    }
    /// Uses the actual surface control cells; `surface.set_bounds_color(color)`.
    pub fn set_bounds_color(&self, color: PaletteColor) {
        self.style.set_bounds_color(color)
    }
    /// Uses the actual surface control cells; `surface.isos_color()`.
    pub fn isos_color(&self) -> PaletteColor {
        self.style.isos_color()
    }
    /// Uses the actual surface control cells; `surface.set_isos_color(color)`.
    pub fn set_isos_color(&self, color: PaletteColor) {
        self.style.set_isos_color(color)
    }
    /// Uses the actual surface control cells; `surface.nb_isos()`.
    pub fn nb_isos(&self) -> (i32, i32) {
        self.style.nb_isos()
    }
    /// Uses the actual surface control cells; `surface.show_isos(nb_u,nb_v)`.
    pub fn show_isos(&self, nb_u: i32, nb_v: i32) {
        self.style.show_isos(nb_u, nb_v)
    }
    /// Uses the actual surface control cells; `surface.clear_isos()`.
    pub fn clear_isos(&self) {
        self.style.clear_isos()
    }
    /// Draws optional isos, finite bounds and orientation; `surface.draw_on_with_isos(display,false)?`.
    pub fn draw_on_with_isos(
        &self,
        display: &mut RasterDisplay<'_>,
        with_isos: bool,
    ) -> Result<(), Failure> {
        self.rendering
            .draw_on(&self.sampling, &self.style, display, with_isos)
    }
}
impl RasterDrawable for SurfaceDrawable {
    fn is_3d(&self) -> bool {
        true
    }

    fn draw_on(&self, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
        self.draw_on_with_isos(display, true)
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
