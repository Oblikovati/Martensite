//! `ShapeCustomSurface`: recognition of analytic (plane, cylinder, cone, sphere, torus)
//! B-spline and Bezier surfaces, and conversion of closed B-spline surfaces to periodic.
mod analytical;
mod periodic;

use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::GeomSurface;

/// `ShapeCustomSurface`: a tool on one surface (`mySurf`) that remembers the deviation
/// (`myGap`) found by the last `ConvertToAnalytical`.
///
/// ```
/// use martensite_tkshhealing::shape_custom::ShapeCustomSurface;
/// let mut tool = ShapeCustomSurface::new();
/// assert!(tool.convert_to_analytical(1e-7, false).unwrap().is_null());
/// assert_eq!(tool.gap(), 0.0);
/// ```
#[derive(Clone, Default)]
pub struct ShapeCustomSurface {
    surface: Handle<dyn GeomSurface>,
    gap: f64,
}

impl ShapeCustomSurface {
    /// no surface, gap 0; `ShapeCustomSurface::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// the tool on `surface`;
    /// `ShapeCustomSurface::with_surface(&surface)`.
    pub fn with_surface(surface: &Handle<dyn GeomSurface>) -> Self {
        let mut tool = Self::new();
        tool.init(surface);
        tool
    }

    /// replaces the surface (the gap is kept); `tool.init(&surface)`.
    pub fn init(&mut self, surface: &Handle<dyn GeomSurface>) {
        self.surface = surface.clone();
    }

    /// the deviation computed by the last `ConvertToAnalytical`; `tool.gap()`.
    pub fn gap(&self) -> f64 {
        self.gap
    }

    /// The current surface (the private `mySurf`, the result after a substitution);
    /// `tool.surface()`.
    pub fn surface(&self) -> Handle<dyn GeomSurface> {
        self.surface.clone()
    }
}
