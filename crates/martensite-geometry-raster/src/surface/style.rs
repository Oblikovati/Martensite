// Surface display colors/counts from GeometrySurface.
//! Actual surface control cells, shared by generic and specialized rendering.
use martensite_raster_core::PaletteColor;
use std::cell::Cell;
/// The colors and iso counts of a surface drawable (the arguments of the second
/// `GeometrySurface` constructor besides the discretisation).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceLook {
    /// `Nu`: the number of U isos (its absolute value is kept).
    pub nb_u_isos: i32,
    /// `Nv`: the number of V isos.
    pub nb_v_isos: i32,
    /// `BoundsColor`.
    pub bounds_color: PaletteColor,
    /// `IsosColor`.
    pub isos_color: PaletteColor,
}

/// Thread-confined controls own each cell once; `SurfaceStyle::new(look)`.
/// ```compile_fail
/// use martensite_geometry_raster::SurfaceStyle;
/// fn shared<T: Sync>() {}
/// shared::<SurfaceStyle>();
/// ```
pub struct SurfaceStyle {
    bounds_look: Cell<PaletteColor>,
    isos_look: Cell<PaletteColor>,
    nb_u_isos: Cell<i32>,
    nb_v_isos: Cell<i32>,
    knots_isos: Cell<bool>,
}
impl SurfaceStyle {
    /// Creates controls from explicit values; `SurfaceStyle::new(look)`.
    pub fn new(look: SurfaceLook) -> Self {
        Self::with_values(
            (look.nb_u_isos, look.nb_v_isos),
            (look.bounds_color, look.isos_color),
        )
    }
    /// Creates the actual cells; `SurfaceStyle::with_values((1,1),(bounds,isos))`.
    pub fn with_values(
        (nb_u, nb_v): (i32, i32),
        (bounds, isos): (PaletteColor, PaletteColor),
    ) -> Self {
        Self {
            bounds_look: Cell::new(bounds),
            isos_look: Cell::new(isos),
            nb_u_isos: Cell::new(nb_u.abs()),
            nb_v_isos: Cell::new(nb_v.abs()),
            knots_isos: Cell::new(false),
        }
    }
    /// Retains current color/count or iso-switch semantics; `let color = style.bounds_color();`.
    pub fn bounds_color(&self) -> PaletteColor {
        self.bounds_look.get()
    }
    /// Retains current color/count or iso-switch semantics; `style.set_bounds_color(color);`.
    pub fn set_bounds_color(&self, color: PaletteColor) {
        self.bounds_look.set(color);
    }
    /// Retains current color/count or iso-switch semantics; `let color = style.isos_color();`.
    pub fn isos_color(&self) -> PaletteColor {
        self.isos_look.get()
    }
    /// Retains current color/count or iso-switch semantics; `style.set_isos_color(color);`.
    pub fn set_isos_color(&self, color: PaletteColor) {
        self.isos_look.set(color);
    }
    /// Retains current color/count or iso-switch semantics; `let counts = style.nb_isos();`.
    pub fn nb_isos(&self) -> (i32, i32) {
        (self.nb_u_isos.get(), self.nb_v_isos.get())
    }
    /// Retains current color/count or iso-switch semantics; `style.show_isos(-2,3);`.
    // Changing the uniform counts also exits specialized knot-iso selection.
    pub fn show_isos(&self, nb_u: i32, nb_v: i32) {
        self.knots_isos.set(false);
        self.nb_u_isos.set(nb_u.abs());
        self.nb_v_isos.set(nb_v.abs());
    }
    /// Retains current color/count or iso-switch semantics; `style.clear_isos();`.
    // Clear both modes together so a specialized renderer cannot retain knot isos.
    pub fn clear_isos(&self) {
        self.knots_isos.set(false);
        self.nb_u_isos.set(0);
        self.nb_v_isos.set(0);
    }
    /// Retains current color/count or iso-switch semantics; `let enabled = style.knots_isos();`.
    pub fn knots_isos(&self) -> bool {
        self.knots_isos.get()
    }
    /// Retains current color/count or iso-switch semantics; `style.set_knots_isos((2,3));`.
    pub fn set_knots_isos(&self, (nb_u, nb_v): (i32, i32)) {
        self.knots_isos.set(true);
        self.nb_u_isos.set(nb_u);
        self.nb_v_isos.set(nb_v);
    }
    /// Retains current color/count or iso-switch semantics; `let profile = style.look();`.
    pub fn look(&self) -> SurfaceLook {
        let (nb_u_isos, nb_v_isos) = self.nb_isos();
        SurfaceLook {
            nb_u_isos,
            nb_v_isos,
            bounds_color: self.bounds_color(),
            isos_color: self.isos_color(),
        }
    }
}
