//! The parametric spline curve of IGES (type 112) as a B-spline curve.
use crate::iges_data::iges_live;
use crate::iges_geom::IgesGeomSplineCurve;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomBSplineCurve;
use martensite_tkmath::gp::Point3;
use martensite_tkmath::plib;

const WHERE: &str = "IgesConvGeom::spline_curve_from_iges";

/// A status and the curve when one is made: 0 done, 1 not guaranteed C0, 3 control points not made,
/// 5 no segment.
pub type SplineCurveConversion = (i32, Handle<GeomBSplineCurve>);

/// The middle of two poles that miss each other.
pub(super) fn middle_point(first: Point3, second: Point3) -> Point3 {
    Point3::new(
        (first.x() + second.x()) / 2.0,
        (first.y() + second.y()) / 2.0,
        (first.z() + second.z()) / 2.0,
    )
}

/// Interior knots at the degree, the first and the last at the degree plus one.
pub(super) fn end_clamped_multiplicities(nb_knots: usize, degree: i32) -> Vec<i32> {
    let mut multiplicities = vec![degree; nb_knots];
    multiplicities[0] = degree + 1;
    multiplicities[nb_knots - 1] = degree + 1;
    multiplicities
}

/// The polynomial of one segment, scaled from `0..delta` to `0..1`, as Bezier poles.
fn segment_poles(
    spline: &IgesGeomSplineCurve,
    segment: i32,
    delta: f64,
    degree: i32,
) -> Result<Vec<Point3>, Failure> {
    let (mut ax, mut bx, mut cx, mut dx) = (0.0, 0.0, 0.0, 0.0);
    spline.x_coord_polynomial(segment, &mut ax, &mut bx, &mut cx, &mut dx)?;
    let (mut ay, mut by, mut cy, mut dy) = (0.0, 0.0, 0.0, 0.0);
    spline.y_coord_polynomial(segment, &mut ay, &mut by, &mut cy, &mut dy)?;
    let (mut az, mut bz, mut cz, mut dz) = (0.0, 0.0, 0.0, 0.0);
    spline.z_coord_polynomial(segment, &mut az, &mut bz, &mut cz, &mut dz)?;
    if spline.nb_dimensions() == 2 {
        (bz, cz, dz) = (0.0, 0.0, 0.0);
    }
    let (delta2, delta3) = (delta * delta, delta * delta * delta);
    let scaled = [
        Point3::new(ax, ay, az),
        Point3::new(bx * delta, by * delta, bz * delta),
        Point3::new(cx * delta2, cy * delta2, cz * delta2),
        Point3::new(dx * delta3, dy * delta3, dz * delta3),
    ];
    let count = degree as usize + 1;
    let coefficients = Array1::from_shared(&scaled[..count], 0);
    let mut poles = Array1::with_bounds(0, degree).map_err(|error| error.into_failure(WHERE))?;
    plib::coefficients_poles(&coefficients, None, &mut poles, None)?;
    Ok(poles.as_slice().to_vec())
}

/// The poles of the segments, joined at their middle where they miss each other by `epsgeom`;
/// the flag is true when a join was not C0.
fn joined_poles(
    spline: &IgesGeomSplineCurve,
    breaks: &[f64],
    degree: i32,
    epsgeom: f64,
) -> Result<(Vec<Point3>, bool), Failure> {
    let mut poles: Vec<Point3> = Vec::new();
    let mut not_c0 = false;
    for (segment, pair) in (1..).zip(breaks.windows(2)) {
        let bezier = segment_poles(spline, segment, pair[1] - pair[0], degree)?;
        match poles.last_mut() {
            None => poles.push(bezier[0]),
            Some(last) if !last.is_equal(&bezier[0], epsgeom) => {
                not_c0 = true;
                *last = middle_point(*last, bezier[0]);
            }
            Some(_) => {}
        }
        poles.extend_from_slice(&bezier[1..]);
    }
    Ok((poles, not_c0))
}

/// Converts a parametric spline curve; `epsgeom` is how far the ends of two segments may miss each
/// other. The coefficient tolerance plays no part. The status is 0 when done, 1 when the result is
/// not guaranteed C0, 3 for an error in the control points and 5 without a segment, the last two
/// with no curve. A spline of more than cubic type is taken as cubic.
/// Example: `let (status, curve) = spline_curve_from_iges(&spline, 1e-6, 1e-4)?;`.
///
/// # Errors
/// A failure of the entity or of the construction of the curve.
pub fn spline_curve_from_iges(
    spline: &Handle<IgesGeomSplineCurve>,
    _epscoef: f64,
    epsgeom: f64,
) -> Result<SplineCurveConversion, Failure> {
    let live = iges_live(spline, WHERE)?;
    let degree = live.spline_type().min(3);
    let nb_segments = live.nb_segments()?;
    if nb_segments < 1 {
        return Ok((5, Handle::default()));
    }
    let breaks = (1..=nb_segments + 1)
        .map(|rank| live.break_point(rank))
        .collect::<Result<Vec<f64>, Failure>>()?;
    let (poles, not_c0) = joined_poles(live, &breaks, degree, epsgeom)?;
    if poles.len() as i32 != nb_segments * degree + 1 {
        return Ok((3, Handle::default()));
    }
    let multiplicities = end_clamped_multiplicities(breaks.len(), degree);
    let curve = GeomBSplineCurve::new(
        &Array1::from_shared(&poles, 1),
        &Array1::from_shared(&breaks, 1),
        &Array1::from_shared(&multiplicities, 1),
        degree,
        false,
    )?;
    Ok((i32::from(not_c0), Handle::new(curve)))
}
