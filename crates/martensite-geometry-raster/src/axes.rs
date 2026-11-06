//! Existing labelled axis algorithms over explicit frames and display settings.
use crate::state::DrawableState;
use martensite_raster_core::{PaletteColor, RasterDisplay, drawable::RasterDrawable};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{
    Axis3, Axis22d, Direction3, Point2, Point3, StandardDirection3, Vector2, Vector3,
};
/// Draws fixed screen-size labelled axes; `draw_spatial_axes(&mut display, axes, color, size)?`.
pub fn draw_spatial_axes(
    display: &mut RasterDisplay<'_>,
    axes: Axis3,
    color: PaletteColor,
    size: i32,
) -> Result<(), Failure> {
    display.set_draw_bounds(false);
    display.set_color(color);
    let length = size as f64 / display.zoom();
    let origin = axes.location();
    let directions = [
        (axes.x_direction(), "X"),
        (axes.y_direction(), "Y"),
        (axes.direction(), "Z"),
    ];
    draw_spatial_frame(display, (origin, length), directions);
    display.set_draw_bounds(true);
    Ok(())
}
/// Draws fixed screen-size labelled axes; `draw_planar_axes(&mut display, axes, color, size)?`.
pub fn draw_planar_axes(
    display: &mut RasterDisplay<'_>,
    axes: Axis22d,
    color: PaletteColor,
    size: i32,
) -> Result<(), Failure> {
    display.set_draw_bounds(false);
    display.set_color(color);
    let length = size as f64 / display.zoom();
    let origin = axes.location();
    for (direction, label) in [(axes.x_direction(), "X"), (axes.y_direction(), "Y")] {
        let tip = origin.translated(Vector2::from(direction) * length);
        display.draw(origin, tip);
        display.draw_string(tip, label);
    }
    display.set_draw_bounds(true);
    Ok(())
}

enum AxisFrame {
    Spatial(Axis3),
    Planar(Axis22d),
}
/// Thread-confined labelled frame drawable; `AxisGlyph::spatial(color, 20)`.
pub struct AxisGlyph {
    frame: AxisFrame,
    color: PaletteColor,
    size: i32,
    state: DrawableState,
}
impl AxisGlyph {
    /// Creates standard spatial axes; `let axes = AxisGlyph::spatial(color, 20);`.
    pub fn spatial(color: PaletteColor, size: i32) -> Self {
        let frame = Axis3::with_standard_frame(
            Point3::new(0., 0., 0.),
            StandardDirection3::Z,
            StandardDirection3::X,
        );
        Self {
            frame: AxisFrame::Spatial(frame),
            color,
            size,
            state: DrawableState::default(),
        }
    }
    /// Creates labelled spatial axes on `frame`, as a drawn principal frame of inertia is;
    /// `let axes = AxisGlyph::spatial_on(frame, color, 30);`.
    pub fn spatial_on(frame: Axis3, color: PaletteColor, size: i32) -> Self {
        Self {
            frame: AxisFrame::Spatial(frame),
            color,
            size,
            state: DrawableState::default(),
        }
    }
    /// Creates standard planar axes; `let axes = AxisGlyph::planar(color, 20);`.
    pub fn planar(color: PaletteColor, size: i32) -> Self {
        let mut frame = Axis22d::default();
        frame.set_location(Point2::new(0., 0.));
        Self {
            frame: AxisFrame::Planar(frame),
            color,
            size,
            state: DrawableState::default(),
        }
    }
}
impl RasterDrawable for AxisGlyph {
    fn draw_on(&self, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
        match self.frame {
            AxisFrame::Spatial(frame) => draw_spatial_axes(display, frame, self.color, self.size),
            AxisFrame::Planar(frame) => draw_planar_axes(display, frame, self.color, self.size),
        }
    }
    fn is_3d(&self) -> bool {
        matches!(self.frame, AxisFrame::Spatial(_))
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

fn draw_spatial_frame(
    display: &mut RasterDisplay<'_>,
    (origin, length): (Point3, f64),
    directions: [(Direction3, &str); 3],
) {
    for (direction, label) in directions {
        let tip = origin.translated(Vector3::from(direction) * length);
        display.draw_3d(origin, tip);
        display.draw_string_3d(tip, label);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_raster_core::PaletteKind;

    #[test]
    fn spatial_axes_on_a_frame_are_three_dimensional() {
        let frame = Axis3::with_standard_frame(
            Point3::new(1., 2., 3.),
            StandardDirection3::X,
            StandardDirection3::Y,
        );
        let axes = AxisGlyph::spatial_on(frame, PaletteColor::new(PaletteKind::Orange), 30);
        assert!(axes.is_3d());
        assert!(
            matches!(axes.frame, AxisFrame::Spatial(f) if f.location() == Point3::new(1., 2., 3.))
        );
    }
}
