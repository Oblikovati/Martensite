//

//! The look of a drawable curve: color, orientation arrow and curvature radii.
use martensite_raster_core::PaletteColor as DrawColor;
use std::cell::Cell;

/// The display options of a curve beyond the discretisation: the optional arguments
/// of the `GeometryCurve` and `GeometryCurve2d` constructors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurveLook {
    /// `aColor`.
    pub color: DrawColor,
    /// `DispOrigin`: the orientation arrow at the end.
    pub display_origin: bool,
    /// `DispCurvRadius`: the curvature radii.
    pub display_curvature: bool,
    /// `RadiusMax`.
    pub radius_max: f64,
    /// `RatioOfRadius`.
    pub radius_ratio: f64,
}

impl CurveLook {
    /// `color` with the C++ defaults of the other options (origin shown, no radii,
    /// radii capped at 1000, drawn at 10%).
    ///
    /// ```
    /// use martensite_raster_core::{PaletteColor as DrawColor, PaletteKind as DrawColorKind};
    /// use martensite_geometry_raster::CurveLook;
    /// let look = CurveLook::new(DrawColor::new(DrawColorKind::Rouge));
    /// assert!(look.display_origin && !look.display_curvature);
    /// ```
    /// Creates copied display options; `CurveLook::new(color)`.
    pub fn new(color: DrawColor) -> Self {
        Self {
            color,
            display_origin: true,
            display_curvature: false,
            radius_max: 1.0e3,
            radius_ratio: 0.1,
        }
    }
}

/// Thread-confined Cell state: reading shared mutable options is not `Sync`.
/// The mutable look of a drawable curve (the `look`, `disporigin`, `dispcurvradius`,
/// `radiusmax` and `radiusratio` members), shared by the 3D and 2D curves.
///
/// ```
/// use martensite_raster_core::{PaletteColor as DrawColor, PaletteKind as DrawColorKind};
/// use martensite_geometry_raster::{CurveLook, CurveStyle};
/// let style = CurveStyle::new(CurveLook::new(DrawColor::new(DrawColorKind::Vert)));
/// style.show_curvature();
/// assert!(style.curvature_shown());
/// ```
#[derive(Debug)]
pub struct CurveStyle {
    color: Cell<DrawColor>,
    display_origin: Cell<bool>,
    display_curvature: Cell<bool>,
    radius_max: Cell<f64>,
    radius_ratio: Cell<f64>,
}

impl CurveStyle {
    /// The style of a new curve.
    /// Builds independent mutable options; `CurveStyle::new(look)`.
    pub fn new(look: CurveLook) -> Self {
        Self {
            color: Cell::new(look.color),
            display_origin: Cell::new(look.display_origin),
            display_curvature: Cell::new(look.display_curvature),
            radius_max: Cell::new(look.radius_max),
            radius_ratio: Cell::new(look.radius_ratio),
        }
    }

    /// `SetColor`.
    /// Changes curve color; `style.set_color(color)`.
    pub fn set_color(&self, color: DrawColor) {
        self.color.set(color);
    }

    /// `Color`.
    /// Reads curve color; `let color = style.color();`.
    pub fn color(&self) -> DrawColor {
        self.color.get()
    }

    /// `DisplayOrigin()`.
    /// Reads arrow visibility; `let shown = style.display_origin();`.
    pub fn display_origin(&self) -> bool {
        self.display_origin.get()
    }

    /// `DisplayOrigin(bool)`.
    /// Selects arrow visibility; `style.set_display_origin(false)`.
    pub fn set_display_origin(&self, display: bool) {
        self.display_origin.set(display);
    }

    /// `ShowCurvature`.
    /// Enables curvature radii; `style.show_curvature()`.
    pub fn show_curvature(&self) {
        self.display_curvature.set(true);
    }

    /// `ClearCurvature`.
    /// Disables curvature radii; `style.clear_curvature()`.
    pub fn clear_curvature(&self) {
        self.display_curvature.set(false);
    }

    /// Whether the curvature radii are drawn (`dispcurvradius`).
    /// Reads radius visibility; `let shown = style.curvature_shown();`.
    pub fn curvature_shown(&self) -> bool {
        self.display_curvature.get()
    }

    /// `SetRadiusMax`.
    /// Sets the radius cap; `style.set_radius_max(10.)`.
    pub fn set_radius_max(&self, radius: f64) {
        self.radius_max.set(radius);
    }

    /// `RadiusMax`.
    /// Reads the radius cap; `let cap = style.radius_max();`.
    pub fn radius_max(&self) -> f64 {
        self.radius_max.get()
    }

    /// `SetRadiusRatio`.
    /// Sets the drawn fraction; `style.set_radius_ratio(0.2)`.
    pub fn set_radius_ratio(&self, ratio: f64) {
        self.radius_ratio.set(ratio);
    }

    /// `RadiusRatio`.
    /// Reads the drawn fraction; `let fraction = style.radius_ratio();`.
    pub fn radius_ratio(&self) -> f64 {
        self.radius_ratio.get()
    }
}
