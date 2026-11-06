//! B-spline curves: the shared reading of the knots and the poles, in 3D and 2D.
use super::StepToGeom;
use crate::geom_to_step::array1_of;
use crate::step_data::{StepDataFactors, StepDataLogical};
use crate::step_entity::{step_array1_item, step_live};
use crate::step_geom::{
    StepGeomBSplineCurve, StepGeomBSplineCurveWithKnots,
    StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve, StepGeomCartesianPoint,
};
use martensite_tkernel::ncollection::{Array1, HArray1};
use martensite_tkernel::standard::real::{epsilon, real_first};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{
    Geom2dBSplineCurve, Geom2dCartesianPoint, Geom2dCurve, Geom2dPoint,
};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCartesianPoint, GeomCurve, GeomPoint};
use martensite_tkmath::gp::{Point2, Point3};

/// The knots with repeated values merged and their multiplicities added.
pub(super) struct MergedKnots {
    pub(super) knots: Vec<f64>,
    pub(super) multiplicities: Vec<i32>,
}

/// True when `knot` differs from `last` by more than the spacing of reals at `last`.
fn is_new_knot(knot: f64, last: f64) -> bool {
    knot - last > epsilon(last.abs())
}

/// The number of distinct knots, counted from `real_first`.
pub(super) fn count_unique_knots(
    knots: &Handle<HArray1<f64>>,
    nb_knots: i32,
) -> Result<i32, Failure> {
    let mut count = 0;
    let mut last = real_first();
    for rank in 1..=nb_knots {
        let knot = step_array1_item(knots, rank, "StepToGeom knots")?;
        if is_new_knot(knot, last) {
            count += 1;
            last = knot;
        }
    }
    Ok(count)
}

/// Merges the repeated knots; a knot array without a first value raises.
pub(super) fn merge_knots(
    knots: &Handle<HArray1<f64>>,
    multiplicities: &Handle<HArray1<i32>>,
    nb_knots: i32,
) -> Result<MergedKnots, Failure> {
    let mut last = step_array1_item(knots, 1, "StepToGeom knots")?;
    let mut merged = MergedKnots {
        knots: vec![last],
        multiplicities: vec![step_array1_item(
            multiplicities,
            1,
            "StepToGeom multiplicities",
        )?],
    };
    for rank in 2..=nb_knots {
        let knot = step_array1_item(knots, rank, "StepToGeom knots")?;
        let multiplicity = step_array1_item(multiplicities, rank, "StepToGeom multiplicities")?;
        if is_new_knot(knot, last) {
            merged.knots.push(knot);
            merged.multiplicities.push(multiplicity);
            last = knot;
        } else if let Some(current) = merged.multiplicities.last_mut() {
            // Knot not unique, increase multiplicity
            *current += multiplicity;
        }
    }
    Ok(merged)
}

/// Does the descriptor look like a periodic one? Counts of a clamped curve come first.
pub(super) fn looks_periodic(multiplicities: &[i32], nb_poles: i32, degree: i32) -> bool {
    let sum: i32 = multiplicities.iter().sum();
    if sum == nb_poles + degree + 1 {
        return false;
    }
    let first = multiplicities.first().copied().unwrap_or_default();
    let last = multiplicities.last().copied().unwrap_or_default();
    first == last && sum - first == nb_poles
}

/// Multiplicities above `degree + 1` are clamped; the poles they added at either end are dropped.
/// Returns how many poles to drop at the first and the last end.
fn clamp_multiplicities(multiplicities: &mut [i32], degree: i32) -> (i32, i32) {
    let count = multiplicities.len();
    let (mut first_excess, mut last_excess) = (0, 0);
    for (index, value) in multiplicities.iter_mut().enumerate() {
        if *value <= degree + 1 {
            continue;
        }
        if index == 0 {
            first_excess = *value - degree - 1;
        }
        if index + 1 == count {
            last_excess = *value - degree - 1;
        }
        *value = degree + 1;
    }
    (first_excess, last_excess)
}

/// The fields of a curve read once, in order, independent of its dimension.
struct CurveRead {
    with_knots: Handle<StepGeomBSplineCurveWithKnots>,
    rational: Handle<StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve>,
}

impl CurveRead {
    fn new(curve: &Handle<StepGeomBSplineCurve>) -> Result<Self, Failure> {
        let rational = curve.cast::<StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve>();
        if let Some(live) = rational.get() {
            return Ok(Self {
                with_knots: live.b_spline_curve_with_knots(),
                rational,
            });
        }
        Ok(Self {
            with_knots: curve.cast(),
            rational,
        })
    }
    /// The weights of the kept poles, when the curve is rational.
    fn kept_weights(
        &self,
        kept: std::ops::RangeInclusive<i32>,
    ) -> Result<Option<Vec<f64>>, Failure> {
        let Some(live) = self.rational.get() else {
            return Ok(None);
        };
        let weights = live.weights_data()?;
        let values: Result<Vec<f64>, Failure> = kept
            .map(|rank| step_array1_item(&weights, rank, "StepToGeom weights"))
            .collect();
        values.map(Some)
    }
}

/// What a dimension gives the shared curve reading: its point and its curve.
trait BSplineCurveKind: Sized + martensite_tkernel::standard::RuntimeTyped {
    type Pole: Clone + Default + Send + Sync + 'static;
    fn make_pole(
        point: &Handle<StepGeomCartesianPoint>,
        factors: &StepDataFactors,
    ) -> Result<Option<Self::Pole>, Failure>;
    fn build(parts: &CurveParts<'_, Self::Pole>) -> Result<Handle<Self>, Failure>;
    fn force_periodic(curve: &Self) -> Result<(), Failure>;
    fn is_closed_above_degree_one(curve: &Self) -> Result<bool, Failure>;
}

/// The arrays a B-spline curve is built from.
struct CurveParts<'a, P: Send + Sync + 'static> {
    poles: &'a Array1<'static, P>,
    weights: Option<&'a Array1<'static, f64>>,
    knots: &'a Array1<'static, f64>,
    multiplicities: &'a Array1<'static, i32>,
    degree: i32,
    periodic: bool,
}

impl BSplineCurveKind for GeomBSplineCurve {
    type Pole = Point3;
    fn make_pole(
        point: &Handle<StepGeomCartesianPoint>,
        factors: &StepDataFactors,
    ) -> Result<Option<Point3>, Failure> {
        let made: Handle<GeomCartesianPoint> = StepToGeom::make_cartesian_point(point, factors)?;
        Ok(made.get().map(GeomPoint::pnt))
    }
    fn build(parts: &CurveParts<'_, Point3>) -> Result<Handle<Self>, Failure> {
        let curve = match parts.weights {
            Some(weights) => Self::with_weights(
                parts.poles,
                weights,
                parts.knots,
                parts.multiplicities,
                parts.degree,
                parts.periodic,
            )?,
            None => Self::new(
                parts.poles,
                parts.knots,
                parts.multiplicities,
                parts.degree,
                parts.periodic,
            )?,
        };
        Ok(Handle::new(curve))
    }
    fn force_periodic(curve: &Self) -> Result<(), Failure> {
        curve.set_periodic()
    }
    fn is_closed_above_degree_one(curve: &Self) -> Result<bool, Failure> {
        Ok(curve.degree() > 1 && curve.is_closed()?)
    }
}

impl BSplineCurveKind for Geom2dBSplineCurve {
    type Pole = Point2;
    fn make_pole(
        point: &Handle<StepGeomCartesianPoint>,
        factors: &StepDataFactors,
    ) -> Result<Option<Point2>, Failure> {
        let made: Handle<Geom2dCartesianPoint> =
            StepToGeom::make_cartesian_point_2d(point, factors)?;
        Ok(made.get().map(Geom2dPoint::pnt2d))
    }
    fn build(parts: &CurveParts<'_, Point2>) -> Result<Handle<Self>, Failure> {
        let curve = match parts.weights {
            Some(weights) => Self::with_weights(
                parts.poles,
                weights,
                parts.knots,
                parts.multiplicities,
                parts.degree,
                parts.periodic,
            )?,
            None => Self::new(
                parts.poles,
                parts.knots,
                parts.multiplicities,
                parts.degree,
                parts.periodic,
            )?,
        };
        Ok(Handle::new(curve))
    }
    fn force_periodic(curve: &Self) -> Result<(), Failure> {
        curve.set_periodic()
    }
    fn is_closed_above_degree_one(curve: &Self) -> Result<bool, Failure> {
        Ok(curve.degree() > 1 && curve.is_closed()?)
    }
}

/// The poles of the kept range; `None` when a point cannot be made.
fn kept_poles<K: BSplineCurveKind>(
    points: &Handle<HArray1<Handle<StepGeomCartesianPoint>>>,
    kept: std::ops::RangeInclusive<i32>,
    factors: &StepDataFactors,
) -> Result<Option<Vec<K::Pole>>, Failure> {
    let mut poles = Vec::with_capacity(kept.clone().count());
    for rank in kept {
        let point = step_array1_item(points, rank, "StepToGeom control points")?;
        let Some(pole) = K::make_pole(&point, factors)? else {
            return Ok(None);
        };
        poles.push(pole);
    }
    Ok(Some(poles))
}

/// `MakeBSplineCurveCommon`.
fn make_bspline_curve_common<K: BSplineCurveKind>(
    curve: &Handle<StepGeomBSplineCurve>,
    factors: &StepDataFactors,
) -> Result<Handle<K>, Failure>
where
    K: martensite_tkernel::standard::RuntimeTyped,
{
    let read = CurveRead::new(curve)?;
    let source = step_live(&read.with_knots, "StepToGeom::MakeBSplineCurve")?;
    let degree = source.degree();
    let nb_poles = source.nb_control_points_list();
    let nb_knots = source.nb_knot_multiplicities()?;
    let knots = source.knots();
    if count_unique_knots(&knots, nb_knots)? <= 1 {
        return Ok(Handle::default());
    }
    let mut merged = merge_knots(&knots, &source.knot_multiplicities(), nb_knots)?;
    let (first_excess, last_excess) = clamp_multiplicities(&mut merged.multiplicities, degree);
    if nb_poles - first_excess - last_excess <= 0 {
        return Ok(Handle::default());
    }
    let kept = (1 + first_excess)..=(nb_poles - last_excess);
    let Some(poles) = kept_poles::<K>(&source.control_points_list(), kept.clone(), factors)? else {
        return Ok(Handle::default());
    };
    let periodic = looks_periodic(&merged.multiplicities, nb_poles, degree);
    let weights = read.kept_weights(kept)?.map(array1_of);
    let built = K::build(&CurveParts {
        poles: &array1_of(poles),
        weights: weights.as_ref(),
        knots: &array1_of(merged.knots),
        multiplicities: &array1_of(merged.multiplicities),
        degree,
        periodic,
    })?;
    // abv 04.07.00 CAX-IF TRJ4: trj4_k1_top-md-203.stp #716 (face #581):
    // force periodicity on closed curves
    let closed_curve = step_live(curve, "StepToGeom::MakeBSplineCurve")?.closed_curve();
    if let Some(live) = built.get() {
        if closed_curve != StepDataLogical::LFalse && K::is_closed_above_degree_one(live)? {
            K::force_periodic(live)?;
        }
    }
    Ok(built)
}

impl StepToGeom {
    /// `MakeBSplineCurve`: null for a curve without two distinct knots, poles or valid points.
    pub fn make_bspline_curve(
        curve: &Handle<StepGeomBSplineCurve>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomBSplineCurve>, Failure> {
        make_bspline_curve_common::<GeomBSplineCurve>(curve, factors)
    }
    /// `MakeBSplineCurve2d`: as `make_bspline_curve`, on a 2D curve.
    pub fn make_bspline_curve_2d(
        curve: &Handle<StepGeomBSplineCurve>,
        factors: &StepDataFactors,
    ) -> Result<Handle<Geom2dBSplineCurve>, Failure> {
        make_bspline_curve_common::<Geom2dBSplineCurve>(curve, factors)
    }
}
