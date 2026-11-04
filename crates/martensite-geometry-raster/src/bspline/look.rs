//! Plain constructor values carry no source-session dependency.
use crate::{KnotsStyle, PolesStyle};
use martensite_raster_core::{MarkerShape, PaletteColor, PaletteKind};
/// Independent constructor values; `let look = SplineCurveLook::default();`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SplineCurveLook {
    /// Color for the sampled curve.
    pub curve_color: PaletteColor,
    /// Color for control polygon segments.
    pub poles_color: PaletteColor,
    /// Color for knot markers.
    pub knots_color: PaletteColor,
    /// Marker used at each selected knot.
    pub knots_shape: MarkerShape,
    /// Marker size in pixels.
    pub knots_size: i32,
    /// Whether the control polygon is initially shown.
    pub show_poles: bool,
    /// Whether knot markers are initially shown.
    pub show_knots: bool,
}
impl Default for SplineCurveLook {
    fn default() -> Self {
        Self {
            curve_color: PaletteColor::new(PaletteKind::Vert),
            poles_color: PaletteColor::new(PaletteKind::Rouge),
            knots_color: PaletteColor::new(PaletteKind::Violet),
            knots_shape: MarkerShape::Losange,
            knots_size: 5,
            show_poles: true,
            show_knots: true,
        }
    }
}
impl SplineCurveLook {
    pub(super) fn poles(&self) -> PolesStyle {
        PolesStyle::new(self.show_poles, self.poles_color)
    }
    pub(super) fn knots(&self) -> KnotsStyle {
        KnotsStyle::new(
            self.show_knots,
            self.knots_shape,
            self.knots_color,
            self.knots_size,
        )
    }
}
