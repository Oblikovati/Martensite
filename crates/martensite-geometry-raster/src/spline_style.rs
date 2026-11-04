//

//! The poles and knots of the spline drawables: their display settings, the control
//! polygon and the `FindPole`/`FindKnot` search.
use crate::curve_properties::PlotPoint;
use martensite_raster_core::{
    MarkerShape as DrawMarkerShape, PaletteColor as DrawColor, PaletteKind as DrawColorKind,
    RasterDisplay as DrawDisplay,
};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Point2, Point3};
use std::cell::Cell;

/// Thread-confined Cell state: clones own independent flags and colors.
/// The control polygon settings (`drawPoles`, `polesLook`).
///
/// ```
/// use martensite_raster_core::{PaletteColor as DrawColor, PaletteKind as DrawColorKind};
/// use martensite_geometry_raster::PolesStyle;
/// let poles = PolesStyle::new(true, DrawColor::new(DrawColorKind::Rouge));
/// poles.clear();
/// assert!(!poles.shown());
/// ```
#[derive(Debug, Clone)]
pub struct PolesStyle {
    shown: Cell<bool>,
    color: Cell<DrawColor>,
}

impl PolesStyle {
    /// Poles shown or not, in `color`.
    /// Builds independent retained options; `PolesStyle::new(true, color)`.
    pub fn new(shown: bool, color: DrawColor) -> Self {
        Self {
            shown: Cell::new(shown),
            color: Cell::new(color),
        }
    }

    /// The default of the one-argument constructors: shown, in red.
    /// Creates the shown default; `let options = Self::default_shown();`.
    pub fn default_shown() -> Self {
        Self::new(true, DrawColor::new(DrawColorKind::Rouge))
    }

    /// `ShowPoles`.
    /// Enables the caller visibility flag; `options.show()`.
    pub fn show(&self) {
        self.shown.set(true);
    }

    /// `ClearPoles`.
    /// Clears the caller visibility flag; `options.clear()`.
    pub fn clear(&self) {
        self.shown.set(false);
    }

    /// Whether the control polygon is drawn (`drawPoles`).
    /// Reads caller visibility; `let shown = options.shown();`.
    pub fn shown(&self) -> bool {
        self.shown.get()
    }

    /// `SetPolesColor`.
    /// Changes retained color; `options.set_color(color)`.
    pub fn set_color(&self, color: DrawColor) {
        self.color.set(color);
    }

    /// `PolesColor`.
    /// Reads retained color; `let color = options.color();`.
    pub fn color(&self) -> DrawColor {
        self.color.get()
    }
}

/// Thread-confined Cell state: clones own independent flags and colors.
/// The knot marker settings (`drawKnots`, `knotsForm`, `knotsLook`, `knotsDim`).
///
/// ```
/// use martensite_raster_core::{PaletteColor as DrawColor, PaletteKind as DrawColorKind, MarkerShape as DrawMarkerShape};
/// use martensite_geometry_raster::KnotsStyle;
/// let knots = KnotsStyle::default_shown();
/// assert_eq!((knots.shape(), knots.size()), (DrawMarkerShape::Losange, 5));
/// ```
#[derive(Debug, Clone)]
pub struct KnotsStyle {
    shown: Cell<bool>,
    shape: Cell<DrawMarkerShape>,
    color: Cell<DrawColor>,
    size: i32,
}

impl KnotsStyle {
    /// Knots shown or not, as `shape` markers of `size` pixels in `color`.
    /// Builds independent retained options; `KnotsStyle::new(true, marker, color, 5)`.
    pub fn new(shown: bool, shape: DrawMarkerShape, color: DrawColor, size: i32) -> Self {
        Self {
            shown: Cell::new(shown),
            shape: Cell::new(shape),
            color: Cell::new(color),
            size,
        }
    }

    /// The default of the one-argument constructors: shown, violet diamonds of 5
    /// pixels.
    /// Creates the shown default; `let options = Self::default_shown();`.
    pub fn default_shown() -> Self {
        Self::new(
            true,
            DrawMarkerShape::Losange,
            DrawColor::new(DrawColorKind::Violet),
            5,
        )
    }

    /// `ShowKnots`.
    /// Enables the caller visibility flag; `options.show()`.
    pub fn show(&self) {
        self.shown.set(true);
    }

    /// `ClearKnots`.
    /// Clears the caller visibility flag; `options.clear()`.
    pub fn clear(&self) {
        self.shown.set(false);
    }

    /// Whether the knots are marked (`drawKnots`).
    /// Reads caller visibility; `let shown = options.shown();`.
    pub fn shown(&self) -> bool {
        self.shown.get()
    }

    /// `SetKnotsShape`.
    /// Changes marker shape; `knots.set_shape(marker)`.
    pub fn set_shape(&self, shape: DrawMarkerShape) {
        self.shape.set(shape);
    }

    /// `KnotsShape`.
    /// Reads marker shape; `let marker = knots.shape();`.
    pub fn shape(&self) -> DrawMarkerShape {
        self.shape.get()
    }

    /// `SetKnotsColor`.
    /// Changes retained color; `options.set_color(color)`.
    pub fn set_color(&self, color: DrawColor) {
        self.color.set(color);
    }

    /// `KnotsColor`.
    /// Reads retained color; `let color = options.color();`.
    pub fn color(&self) -> DrawColor {
        self.color.get()
    }

    /// `knotsDim`: the marker size in pixels.
    /// Reads marker pixel extent; `let extent = knots.size();`.
    pub fn size(&self) -> i32 {
        self.size
    }

    /// Marks `points` in the knot color, shape and size.
    /// Caller chooses visibility before this call; `knots.mark(&mut display, points)?;`.
    pub fn mark<P: PlotPoint>(
        &self,
        display: &mut DrawDisplay<'_>,
        points: impl IntoIterator<Item = Result<P, Failure>>,
    ) -> Result<(), Failure> {
        display.set_color(self.color());
        for point in points {
            point?.mark_on(display, self.shape(), self.size);
        }
        Ok(())
    }
}

/// The control polygon: a move to the first pole and lines to the others, in `color`.
/// Usage: `draw_polygon(&mut display, color, &poles);`.
pub fn draw_polygon<P: PlotPoint>(display: &mut DrawDisplay<'_>, color: DrawColor, poles: &[P]) {
    display.set_color(color);
    let Some((first, rest)) = poles.split_first() else {
        return;
    };
    first.move_on(display);
    for pole in rest {
        pole.draw_on(display);
    }
}

/// The pole net of a surface: each V column of poles along U (closed when the surface
/// is U-periodic), then each U row along V (closed when V-periodic), in `color`.
/// Usage: `draw_pole_net(&mut display, color, counts, periodic, pole)?;`.
pub fn draw_pole_net(
    display: &mut DrawDisplay<'_>,
    color: DrawColor,
    (nb_u, nb_v): (i32, i32),
    (u_periodic, v_periodic): (bool, bool),
    pole: impl Fn(i32, i32) -> Result<Point3, Failure>,
) -> Result<(), Failure> {
    display.set_color(color);
    draw_net_columns(display, (nb_u, nb_v), u_periodic, &pole)?;
    draw_net_rows(display, (nb_u, nb_v), v_periodic, &pole)
}
fn draw_net_columns(
    display: &mut DrawDisplay<'_>,
    (nb_u, nb_v): (i32, i32),
    u_periodic: bool,
    pole: &impl Fn(i32, i32) -> Result<Point3, Failure>,
) -> Result<(), Failure> {
    for j in 1..=nb_v {
        display.move_to_3d(pole(1, j)?);
        for i in 2..=nb_u {
            display.draw_to_3d(pole(i, j)?);
        }
        if u_periodic {
            display.draw_to_3d(pole(1, j)?);
        }
    }
    Ok(())
}
fn draw_net_rows(
    display: &mut DrawDisplay<'_>,
    (nb_u, nb_v): (i32, i32),
    v_periodic: bool,
    pole: &impl Fn(i32, i32) -> Result<Point3, Failure>,
) -> Result<(), Failure> {
    for i in 1..=nb_u {
        display.move_to_3d(pole(i, 1)?);
        for j in 2..=nb_v {
            display.draw_to_3d(pole(i, j)?);
        }
        if v_periodic {
            display.draw_to_3d(pole(i, 1)?);
        }
    }
    Ok(())
}

/// The `FindPole` loop of the surfaces: the first pole after `(u_index, v_index)` in
/// U-then-V order whose projection is within `precision` of `target`; `(0, 0)` when
/// none is. Both indices advance first, so the search resumes at the next V column.
/// Usage: `find_net_index(&display, target, resume, counts, pole)?;`.
pub fn find_net_index(
    display: &DrawDisplay<'_>,
    (target, precision): (Point2, f64),
    (u_index, v_index): (i32, i32),
    (nb_u, nb_v): (i32, i32),
    pole: impl Fn(i32, i32) -> Result<Point3, Failure>,
) -> Result<(i32, i32), Failure> {
    let mut u = u_index + 1;
    for v in v_index + 1..=nb_v {
        if let Some(found) = find_column_index(display, (target, precision), (u, nb_u), v, &pole)? {
            return Ok((found, v));
        }
        u = 1;
    }
    Ok((0, 0))
}
fn find_column_index(
    display: &DrawDisplay<'_>,
    (target, precision): (Point2, f64),
    (mut u, nb_u): (i32, i32),
    v: i32,
    pole: &impl Fn(i32, i32) -> Result<Point3, Failure>,
) -> Result<Option<i32>, Failure> {
    while u <= nb_u {
        if display.project(pole(u, v)?).distance(target) <= precision {
            return Ok(Some(u));
        }
        u += 1;
    }
    Ok(None)
}

/// The search loop of `FindPole` and `FindKnot`: the first index after `index`, up to
/// `count`, whose point projects within `precision` of `target`; 0 when none does.
///
/// ```text
/// FindPole: target (X / zoom, Y / zoom), precision XPrec / zoom
/// FindKnot: target (X, Y), precision Prec (the native code does not scale them)
/// ```
/// Usage: `find_index(&display, target, resume, point)?;`.
pub fn find_index<P: PlotPoint>(
    display: &DrawDisplay<'_>,
    (target, precision): (Point2, f64),
    (index, count): (i32, i32),
    point_at: impl Fn(i32) -> Result<P, Failure>,
) -> Result<i32, Failure> {
    for candidate in index + 1..=count {
        if point_at(candidate)?.project_on(display).distance(target) <= precision {
            return Ok(candidate);
        }
    }
    Ok(0)
}

/// The `FindPole` target and precision: the pick point and tolerance in model units.
/// Usage: `let target = scaled_target(&display, (x, y), precision);`.
pub fn scaled_target(
    display: &DrawDisplay<'_>,
    (x, y): (f64, f64),
    precision: f64,
) -> (Point2, f64) {
    let zoom = display.zoom();
    (Point2::new(x / zoom, y / zoom), precision / zoom)
}
