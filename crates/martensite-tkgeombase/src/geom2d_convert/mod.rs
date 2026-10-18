//! Geom2dConvert: conversion of Geom2d curves to B-spline and Bezier curves,
//! splitting, concatenation and approximation by arcs and segments.
//!
//! The static members of the retained `Geom2dConvert` class are free functions
//! (`Geom2dConvert::CurveToBSplineCurve` is [`curve_to_bspline_curve`]); each class
//! of the package is a struct. A retained exception is a [`Failure`] with the same
//! exception type; the output reference parameters of the concatenation members stay
//! `&mut` parameters, because the retained members assign them before they can raise.
//!
//! `let bspline = geom2d_convert::curve_to_bspline_curve(&curve, ParameterisationType::TgtThetaOver2)?;`
mod approx_arcs_segments;
mod approx_curve;
mod bspline_to_bezier;
mod c0_to_c1;
mod comp_curve;
mod concat;
mod concat_support;
mod continuity;
mod curve_to_bspline;
mod knot_splitting;
mod ppoint;
mod reparametrize;

pub use approx_arcs_segments::{ApproxArcsSegments, ApproxArcsStatus};
pub use approx_curve::ApproxCurve;
pub use bspline_to_bezier::BSplineCurveToBezierCurve;
pub use c0_to_c1::{
    c0_bspline_to_array_of_c1_bspline_curve, c0_bspline_to_array_of_c1_bspline_curve_with_angular,
    c0_bspline_to_c1_bspline_curve,
};
pub use comp_curve::CompCurveToBSplineCurve;
pub use concat::{ConcatOutputs, concat_c1, concat_c1_with_angular, concat_g1};
pub use curve_to_bspline::curve_to_bspline_curve;
pub use knot_splitting::BSplineCurveKnotSplitting;
pub use knot_splitting::split_knots;
pub use ppoint::PPoint;
pub(crate) use reparametrize::{LawEvaluator, QuadraticLaw, ReparameteriseEvaluator};

use crate::native_failure::failure;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dCurve, Geom2dGeometry};

/// The shared array of B-splines the concatenation members fill
/// (`occ::handle<HArray1<occ::handle<BSplineCurve>>>`).
pub type HArrayOfBSplineCurves = Handle<HArray1<Handle<Geom2dBSplineCurve>>>;

/// `Geom2dConvert::SplitBSplineCurve(C, FromK1, ToK2, SameOrientation = true)`: the arc
/// of `curve` between the knots of indices `from_k1` and `to_k2`, as a new curve.
///
/// A periodic curve keeps its orientation unless `same_orientation` is false; any
/// other curve runs from `from_k1` to `to_k2`. domain failure when the indices
/// are equal, index-range failure when one lies outside
/// `[FirstUKnotIndex, LastUKnotIndex]`;
/// `geom2d_convert::split_bspline_curve_knots(&curve, 1, 3, true)?`.
pub fn split_bspline_curve_knots(
    curve: &Handle<Geom2dBSplineCurve>,
    from_k1: i32,
    to_k2: i32,
    same_orientation: bool,
) -> Result<Handle<Geom2dBSplineCurve>, Failure> {
    let source: &Geom2dBSplineCurve = live_bspline(curve)?;
    let (first, last) = (source.first_u_knot_index()?, source.last_u_knot_index()?);
    if from_k1 == to_k2 {
        return Err(failure(
            "Standard_DomainError",
            &format!(
                "Geom2dConvert::SplitBSplineCurve: FromK1 = ToK2 = {from_k1}; expected distinct knots"
            ),
        ));
    }
    let (first_k, last_k) = (from_k1.min(to_k2), from_k1.max(to_k2));
    if first_k < first || last_k > last {
        return Err(failure(
            "Standard_OutOfRange",
            &format!(
                "Geom2dConvert::SplitBSplineCurve: knots {from_k1}..{to_k2}; expected within {first}..{last}"
            ),
        ));
    }
    let new_curve: Handle<Geom2dBSplineCurve> = copy_bspline(source)?;
    new_curve.segment(
        source.knot(first_k)?,
        source.knot(last_k)?,
        martensite_tkernel::precision::p_confusion(),
    )?;
    orient_split(
        &new_curve,
        source.is_periodic(),
        same_orientation,
        from_k1 > to_k2,
    )?;
    Ok(new_curve)
}

/// `Geom2dConvert::SplitBSplineCurve(C, FromU1, ToU2, ParametricTolerance,
/// SameOrientation = true)`: the arc of `curve` between two parameters.
///
/// The retained member ignores `parametric_tolerance` and checks no bound: the
/// B-spline `Segment` raises for parameters it rejects;
/// `geom2d_convert::split_bspline_curve(&curve, 0.2, 0.8, 1e-9, true)?`.
pub fn split_bspline_curve(
    curve: &Handle<Geom2dBSplineCurve>,
    from_u1: f64,
    to_u2: f64,
    _parametric_tolerance: f64,
    same_orientation: bool,
) -> Result<Handle<Geom2dBSplineCurve>, Failure> {
    let source: &Geom2dBSplineCurve = live_bspline(curve)?;
    let (first_u, last_u) = (from_u1.min(to_u2), from_u1.max(to_u2));
    let new_curve: Handle<Geom2dBSplineCurve> = copy_bspline(source)?;
    new_curve.segment(
        first_u,
        last_u,
        martensite_tkernel::precision::p_confusion(),
    )?;
    // The retained test is `FromU1 > ToU2`, false for NaN.
    orient_split(
        &new_curve,
        source.is_periodic(),
        same_orientation,
        from_u1 > to_u2,
    )?;
    Ok(new_curve)
}

// Reverses a split arc: a periodic curve when the other orientation is asked, any
// other curve when its bounds were given in decreasing order.
fn orient_split(
    curve: &Handle<Geom2dBSplineCurve>,
    periodic: bool,
    same_orientation: bool,
    decreasing: bool,
) -> Result<(), Failure> {
    let reverse: bool = if periodic {
        !same_orientation
    } else {
        decreasing
    };
    if reverse {
        curve.reverse()?;
    }
    Ok(())
}

/// The object of a non-null handle; null-object failure where the retained code
/// dereferences a null handle.
pub(crate) fn live_bspline(
    curve: &Handle<Geom2dBSplineCurve>,
) -> Result<&Geom2dBSplineCurve, Failure> {
    curve.get().ok_or_else(|| {
        failure(
            "Standard_NullObject",
            "Geom2dConvert: null Geom2d_BSplineCurve handle; expected a curve",
        )
    })
}

/// `occ::down_cast<BSplineCurve>(C->Copy())`.
pub(crate) fn copy_bspline(
    curve: &Geom2dBSplineCurve,
) -> Result<Handle<Geom2dBSplineCurve>, Failure> {
    Ok(curve.copy()?.cast::<Geom2dBSplineCurve>())
}

/// `Handle<Geom2dBSplineCurve>` seen as a `Curve`, as the retained implicit
/// up cast.
pub(crate) fn as_curve(curve: &Handle<Geom2dBSplineCurve>) -> Handle<dyn Geom2dCurve> {
    curve.cast::<dyn Geom2dCurve>()
}
