//! The parametric spline surface of IGES (type 114) as a B-spline surface.
use super::spline_curve::{end_clamped_multiplicities, middle_point};
use crate::iges_data::{IgesDataToolLocation, iges_array1_item, iges_live};
use crate::iges_geom::IgesGeomSplineSurface;
use martensite_tkernel::ncollection::{Array1, Array2, HArray1, array2_failure};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomBSplineSurface;
use martensite_tkmath::gp::{GeneralTransform3, Point3, Transform3};
use martensite_tkmath::plib;

const WHERE: &str = "IgesConvGeom::spline_surface_from_iges";
/// How far a position may be from a similarity to apply to the poles.
const SIMILARITY_EPSILON: f64 = 1.0e-4;

/// A status and the surface when one is made; see [`spline_surface_from_iges`].
pub type SplineSurfaceConversion = (i32, Handle<GeomBSplineSurface>);

/// The patch being joined to the poles already placed.
struct JoinedPoles {
    poles: Array2<'static, Point3>,
    degree: i32,
    tolerance: f64,
    matched: bool,
}

impl JoinedPoles {
    fn value(&self, row: i32, column: i32) -> Result<Point3, Failure> {
        self.poles
            .value(row, column)
            .copied()
            .map_err(array2_failure)
    }
    fn set(&mut self, row: i32, column: i32, pole: Point3) -> Result<(), Failure> {
        self.poles
            .set_value(row, column, pole)
            .map_err(array2_failure)
    }
    /// Puts a pole on one that a previous patch placed, at their middle when they miss each other.
    fn merge(&mut self, row: i32, column: i32, pole: Point3) -> Result<(), Failure> {
        let present = self.value(row, column)?;
        if pole.is_equal(&present, self.tolerance) {
            return self.set(row, column, pole);
        }
        self.matched = false;
        self.set(row, column, middle_point(pole, present))
    }
    /// Places one patch: the shared column first, then the shared row, then the free poles.
    fn place(
        &mut self,
        bezier: &Array2<'_, Point3>,
        u_segment: i32,
        v_segment: i32,
    ) -> Result<(), Failure> {
        let row_base = (u_segment - 1) * self.degree;
        let column_base = (v_segment - 1) * self.degree;
        let count = self.degree + 1;
        let pole_at =
            |row: i32, column: i32| bezier.value(row, column).copied().map_err(array2_failure);
        if v_segment > 1 {
            for row in 1..=count {
                self.merge(row_base + row, column_base + 1, pole_at(row, 1)?)?;
            }
        }
        if u_segment > 1 {
            for column in 1..=count {
                self.merge(row_base + 1, column_base + column, pole_at(1, column)?)?;
            }
        }
        let first_row = if u_segment > 1 { 2 } else { 1 };
        let first_column = if v_segment > 1 { 2 } else { 1 };
        for row in first_row..=count {
            for column in first_column..=count {
                self.set(row_base + row, column_base + column, pole_at(row, column)?)?;
            }
        }
        Ok(())
    }
}

/// The polynomial of a patch, scaled to the unit square, as Bezier poles.
fn patch_poles(
    spline: &IgesGeomSplineSurface,
    (u_segment, v_segment): (i32, i32),
    (delta_u, delta_v): (f64, f64),
    degree: i32,
) -> Result<Array2<'static, Point3>, Failure> {
    let x: Handle<HArray1<f64>> = spline.x_polynomial(u_segment, v_segment)?;
    let y: Handle<HArray1<f64>> = spline.y_polynomial(u_segment, v_segment)?;
    let z: Handle<HArray1<f64>> = spline.z_polynomial(u_segment, v_segment)?;
    let count = degree + 1;
    let mut coefficients = Array2::with_bounds(1, count, 1, count).map_err(array2_failure)?;
    let mut scale_u = 1.0;
    for row in 1..=count {
        let mut scale = scale_u;
        for column in 1..=count {
            let rank = row + 4 * (column - 1);
            let coefficient = Point3::new(
                iges_array1_item(&x, rank, WHERE)? * scale,
                iges_array1_item(&y, rank, WHERE)? * scale,
                iges_array1_item(&z, rank, WHERE)? * scale,
            );
            coefficients
                .set_value(row, column, coefficient)
                .map_err(array2_failure)?;
            scale *= delta_v;
        }
        scale_u *= delta_u;
    }
    let mut poles = Array2::with_bounds(1, count, 1, count).map_err(array2_failure)?;
    plib::coefficients_poles_grid(&coefficients, None, &mut poles, None)?;
    Ok(poles)
}

fn break_points(
    count: i32,
    point: impl Fn(i32) -> Result<f64, Failure>,
) -> Result<Vec<f64>, Failure> {
    (1..=count + 1).map(point).collect()
}

/// The patches in the order that decides the middle poles: the first, along U, along V, the rest.
fn patch_order(nb_u: i32, nb_v: i32) -> impl Iterator<Item = (i32, i32)> {
    let along_u = (1..=nb_u).map(|u_segment| (u_segment, 1));
    let along_v = (2..=nb_v).map(|v_segment| (1, v_segment));
    let rest = (2..=nb_v)
        .flat_map(move |v_segment| (2..=nb_u).map(move |u_segment| (u_segment, v_segment)));
    along_u.chain(along_v).chain(rest)
}

fn apply_position(
    spline: &IgesGeomSplineSurface,
    poles: &mut Array2<'static, Point3>,
) -> Result<(), Failure> {
    if !spline.has_transf() {
        return Ok(());
    }
    let location: GeneralTransform3 = spline.compound_location()?;
    let mut similarity = Transform3::default();
    if !IgesDataToolLocation::convert_location(SIMILARITY_EPSILON, &location, &mut similarity, 1.0)?
    {
        return Ok(());
    }
    for pole in poles.as_mut_slice() {
        *pole = similarity.apply(*pole);
    }
    Ok(())
}

/// Converts a parametric spline surface; `epsgeom` is how far two patches may miss each other, the
/// coefficient tolerance plays no part. The status is 5 with a segment missing in U or V, and no
/// surface. Otherwise it is 1 when all patches joined within `epsgeom`, and 0 when some were moved
/// to the middle. A boundary type of more than cubic is taken as cubic. The position of the entity
/// applies when it is a similarity.
/// Example: `let (status, surface) = spline_surface_from_iges(&spline, 1e-6, 1e-4)?;`.
///
/// # Errors
/// A failure of the entity or of the construction of the surface.
pub fn spline_surface_from_iges(
    spline: &Handle<IgesGeomSplineSurface>,
    _epscoef: f64,
    epsgeom: f64,
) -> Result<SplineSurfaceConversion, Failure> {
    let live = iges_live(spline, WHERE)?;
    let degree = live.boundary_type().min(3);
    let (nb_u, nb_v) = (live.nb_u_segments()?, live.nb_v_segments()?);
    if nb_u < 1 || nb_v < 1 {
        return Ok((5, Handle::default()));
    }
    let u_knots = break_points(nb_u, |rank| live.u_break_point(rank))?;
    let v_knots = break_points(nb_v, |rank| live.v_break_point(rank))?;
    let poles =
        Array2::with_bounds(1, nb_u * degree + 1, 1, nb_v * degree + 1).map_err(array2_failure)?;
    let mut joined = JoinedPoles {
        poles,
        degree,
        tolerance: epsgeom,
        matched: true,
    };
    for (u_segment, v_segment) in patch_order(nb_u, nb_v) {
        let deltas = (
            u_knots[u_segment as usize] - u_knots[u_segment as usize - 1],
            v_knots[v_segment as usize] - v_knots[v_segment as usize - 1],
        );
        let bezier = patch_poles(live, (u_segment, v_segment), deltas, degree)?;
        joined.place(&bezier, u_segment, v_segment)?;
    }
    apply_position(live, &mut joined.poles)?;
    let u_mults = end_clamped_multiplicities(u_knots.len(), degree);
    let v_mults = end_clamped_multiplicities(v_knots.len(), degree);
    let surface = GeomBSplineSurface::new(
        &joined.poles,
        (
            &Array1::from_shared(&u_knots, 1),
            &Array1::from_shared(&u_mults, 1),
        ),
        (
            &Array1::from_shared(&v_knots, 1),
            &Array1::from_shared(&v_mults, 1),
        ),
        (degree, degree),
        (false, false),
    )?;
    Ok((i32::from(joined.matched), Handle::new(surface)))
}
