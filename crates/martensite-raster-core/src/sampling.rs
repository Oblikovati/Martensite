//! Shared interval bounds and orientation arrows retain exact sampling arithmetic.
use crate::RasterDisplay;
use martensite_tkernel::{ncollection::Array1, standard::Failure};
use martensite_tkmath::gp::{self, Direction2, Point2, Vector2};

/// Allocates one-based continuity-interval limits before a caller fills them.
/// For example, `let values = interval_bounds(1, |_| Ok(()))?;`.
pub fn interval_bounds(
    nb_intervals: i32,
    fill: impl FnOnce(&mut Array1<'static, f64>) -> Result<(), Failure>,
) -> Result<Array1<'static, f64>, Failure> {
    let mut bounds: Array1<'static, f64> = Array1::with_bounds(1, nb_intervals + 1)
        .map_err(|error| error.into_failure("NCollection_Array1"))?;
    fill(&mut bounds)?;
    Ok(bounds)
}

/// Reads a one-based interval limit with a typed bounds failure; `bound(&limits, 1)?`.
pub fn bound(bounds: &Array1<'_, f64>, index: i32) -> Result<f64, Failure> {
    bounds
        .value(index)
        .copied()
        .map_err(|error| error.into_failure("NCollection_Array1::Value"))
}

/// Draws an orientation arrow; `draw_orientation_arrow(&mut display, tip, direction)?`.
/// The orientation arrow at `tip` along `direction` is screen-sized, 20 by 10 pixels,
/// drawn without growing the bounds (`RasterBounds = false`); nothing when
/// `direction` is shorter than `gp::Resolution`.
pub fn draw_orientation_arrow(
    display: &mut RasterDisplay<'_>,
    tip: Point2,
    direction: Vector2,
) -> Result<(), Failure> {
    display.set_draw_bounds(false);
    let drawn = draw_direction_arrow(display, tip, direction);
    display.set_draw_bounds(true);
    drawn
}

/// Draws a direction arrow; `draw_direction_arrow(&mut display, tip, direction)?`.
/// The arrow of [`draw_orientation_arrow`] with the bounds left as they are (the
/// topology-command edge arrows).
pub fn draw_direction_arrow(
    display: &mut RasterDisplay<'_>,
    tip: Point2,
    direction: Vector2,
) -> Result<(), Failure> {
    // `if (v.Magnitude() > gp::Resolution())`: nothing for NaN either.
    let long_enough = direction.magnitude() > gp::resolution();
    if !long_enough {
        return Ok(());
    }
    let length = 20.0 / display.zoom();
    let half_width = 10.0 / display.zoom();
    let d = Direction2::from_vector(direction)?;
    draw_arrow_strokes(display, tip, (length, half_width, d));
    Ok(())
}

fn draw_arrow_strokes(
    display: &mut RasterDisplay<'_>,
    tip: Point2,
    (length, half_width, d): (f64, f64, Direction2),
) {
    let (x, y) = (tip.x(), tip.y());
    display.move_to(Point2::new(
        x - length * d.x() - half_width * d.y(),
        y - length * d.y() + half_width * d.x(),
    ));
    display.draw_to(tip);
    display.draw_to(Point2::new(
        x - length * d.x() + half_width * d.y(),
        y - length * d.y() - half_width * d.x(),
    ));
}
