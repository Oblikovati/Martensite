//! Raising the continuity of B-spline curves and surfaces by removing knots.
use martensite_tkernel::standard::Failure;
use martensite_tkg2d::geom2d::Geom2dBSplineCurve;
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomBSplineSurface};

/// One parametric direction of a spline: its degree, interior knots and knot removal.
trait KnotDirection {
    fn degree(&self) -> i32;
    fn first_index(&self) -> Result<i32, Failure>;
    fn last_index(&self) -> Result<i32, Failure>;
    fn multiplicity(&self, index: i32) -> Result<i32, Failure>;
    fn remove(&self, index: i32, mult: i32, tolerance: f64) -> Result<bool, Failure>;
}

macro_rules! curve_direction {
    ($curve:ty) => {
        impl KnotDirection for $curve {
            fn degree(&self) -> i32 {
                <$curve>::degree(self)
            }
            fn first_index(&self) -> Result<i32, Failure> {
                self.first_u_knot_index()
            }
            fn last_index(&self) -> Result<i32, Failure> {
                self.last_u_knot_index()
            }
            fn multiplicity(&self, index: i32) -> Result<i32, Failure> {
                <$curve>::multiplicity(self, index)
            }
            fn remove(&self, index: i32, mult: i32, tolerance: f64) -> Result<bool, Failure> {
                self.remove_knot(index, mult, tolerance)
            }
        }
    };
}
curve_direction!(GeomBSplineCurve);
curve_direction!(Geom2dBSplineCurve);

/// The U direction of a surface.
struct SurfaceU<'a>(&'a GeomBSplineSurface);
/// The V direction of a surface.
struct SurfaceV<'a>(&'a GeomBSplineSurface);

impl KnotDirection for SurfaceU<'_> {
    fn degree(&self) -> i32 {
        self.0.u_degree()
    }
    fn first_index(&self) -> Result<i32, Failure> {
        self.0.first_u_knot_index()
    }
    fn last_index(&self) -> Result<i32, Failure> {
        self.0.last_u_knot_index()
    }
    fn multiplicity(&self, index: i32) -> Result<i32, Failure> {
        self.0.u_multiplicity(index)
    }
    fn remove(&self, index: i32, mult: i32, tolerance: f64) -> Result<bool, Failure> {
        self.0.remove_u_knot(index, mult, tolerance)
    }
}

impl KnotDirection for SurfaceV<'_> {
    fn degree(&self) -> i32 {
        self.0.v_degree()
    }
    fn first_index(&self) -> Result<i32, Failure> {
        self.0.first_v_knot_index()
    }
    fn last_index(&self) -> Result<i32, Failure> {
        self.0.last_v_knot_index()
    }
    fn multiplicity(&self, index: i32) -> Result<i32, Failure> {
        self.0.v_multiplicity(index)
    }
    fn remove(&self, index: i32, mult: i32, tolerance: f64) -> Result<bool, Failure> {
        self.0.remove_v_knot(index, mult, tolerance)
    }
}

/// What the removal passes established; a knot that once refused stays recorded.
struct ReachedContinuity {
    c1: bool,
    c2: bool,
}

impl ReachedContinuity {
    fn status(&self, continuity: i32) -> i32 {
        if !self.c1 {
            return 0;
        }
        if continuity >= 2 && !self.c2 {
            return 1;
        }
        continuity
    }
}

/// Tries one knot towards C2 then C1, or towards C1 alone; true when a knot was removed.
fn raise_knot(
    direction: &impl KnotDirection,
    index: i32,
    (tolerance, continuity): (f64, i32),
    reached: &mut ReachedContinuity,
) -> Result<bool, Failure> {
    let degree = direction.degree();
    if continuity >= 2 {
        if direction.remove(index, degree - 2, tolerance)? {
            return Ok(true);
        }
        reached.c2 = false;
    }
    let removed = direction.remove(index, degree - 1, tolerance)?;
    reached.c1 &= removed;
    Ok(removed)
}

/// One pass over the interior knots; the last index is read again because removal lowers it.
fn raise_pass(
    direction: &impl KnotDirection,
    request: (f64, i32),
    reached: &mut ReachedContinuity,
) -> Result<bool, Failure> {
    let mut modified = false;
    let mut index = direction.first_index()? + 1;
    while index < direction.last_index()? {
        if direction.degree() - direction.multiplicity(index)? < request.1 {
            modified |= raise_knot(direction, index, request, reached)?;
        }
        index += 1;
    }
    Ok(modified)
}

/// Repeats the passes until one removes nothing.
fn raise_direction(
    direction: &impl KnotDirection,
    request: (f64, i32),
    reached: &mut ReachedContinuity,
) -> Result<(), Failure> {
    while raise_pass(direction, request, reached)? {}
    Ok(())
}

fn raise_curve(
    curve: &impl KnotDirection,
    tolerance: f64,
    continuity: i32,
) -> Result<i32, Failure> {
    if continuity < 1 {
        return Ok(continuity);
    }
    let mut reached = ReachedContinuity { c1: true, c2: true };
    raise_direction(curve, (tolerance, continuity), &mut reached)?;
    Ok(reached.status(continuity))
}

/// Tries to raise a curve to `continuity` (1 or 2, more counts as 2) within `tolerance` by
/// removing knots; returns the continuity reached on all knots, every knot that can be raised is.
/// Example: `increase_curve_continuity(&curve, 1e-7, 2)?`.
pub fn increase_curve_continuity(
    curve: &GeomBSplineCurve,
    tolerance: f64,
    continuity: i32,
) -> Result<i32, Failure> {
    raise_curve(curve, tolerance, continuity)
}

/// The same as [`increase_curve_continuity`] for a curve in the plane.
/// Example: `increase_curve2d_continuity(&curve, 1e-7, 1)?`.
pub fn increase_curve2d_continuity(
    curve: &Geom2dBSplineCurve,
    tolerance: f64,
    continuity: i32,
) -> Result<i32, Failure> {
    raise_curve(curve, tolerance, continuity)
}

/// The same as [`increase_curve_continuity`] for a surface, in U then in V.
/// Example: `increase_surface_continuity(&surface, 1e-7, 2)?`.
pub fn increase_surface_continuity(
    surface: &GeomBSplineSurface,
    tolerance: f64,
    continuity: i32,
) -> Result<i32, Failure> {
    if continuity < 1 {
        return Ok(continuity);
    }
    let mut reached = ReachedContinuity { c1: true, c2: true };
    let request = (tolerance, continuity);
    raise_direction(&SurfaceU(surface), request, &mut reached)?;
    raise_direction(&SurfaceV(surface), request, &mut reached)?;
    Ok(reached.status(continuity))
}
