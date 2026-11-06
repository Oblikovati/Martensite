//! Bounded curves: the Bezier, uniform and quasi-uniform curves become B-spline curves.
use super::StepToGeom;
use crate::geom_to_step::shared_array1;
use crate::step_data::StepDataFactors;
use crate::step_geom::{
    StepGeomBSplineCurve, StepGeomBSplineCurveWithKnots,
    StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve, StepGeomBezierCurve,
    StepGeomBoundedCurve, StepGeomKnotType, StepGeomPolyline, StepGeomQuasiUniformCurve,
    StepGeomQuasiUniformCurveAndRationalBSplineCurve, StepGeomTrimmedCurve, StepGeomUniformCurve,
    StepGeomUniformCurveAndRationalBSplineCurve,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dBoundedCurve};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomBoundedCurve};

/// The knots and multiplicities a descriptor without knots is given.
pub(super) struct GeneratedKnots {
    pub(super) multiplicities: Handle<HArray1<i32>>,
    pub(super) knots: Handle<HArray1<f64>>,
}

impl GeneratedKnots {
    fn from_values(multiplicities: Vec<i32>, knots: Vec<f64>) -> Self {
        Self {
            multiplicities: shared_array1(multiplicities),
            knots: shared_array1(knots),
        }
    }
    /// A Bezier descriptor: two knots of full multiplicity.
    pub(super) fn bezier(degree: i32) -> Self {
        Self::from_values(vec![degree + 1, degree + 1], vec![0.0, 1.0])
    }
    /// A uniform descriptor: `count` knots of multiplicity one.
    pub(super) fn uniform(count: i32) -> Self {
        Self::from_values(
            vec![1; count.max(0) as usize],
            (0..count).map(f64::from).collect(),
        )
    }
    /// A quasi-uniform descriptor: as uniform, with full multiplicity at both ends.
    pub(super) fn quasi_uniform(count: i32, degree: i32) -> Self {
        let mut multiplicities = vec![1; count.max(0) as usize];
        if let Some(first) = multiplicities.first_mut() {
            *first = degree + 1;
        }
        if let Some(last) = multiplicities.last_mut() {
            *last = degree + 1;
        }
        Self::from_values(multiplicities, (0..count).map(f64::from).collect())
    }
}

/// True when the guard on the degree of a descriptor refuses it.
fn degree_out_of_range(degree: i32) -> bool {
    degree < 1 || degree > GeomBSplineCurve::max_degree()
}

/// A non rational B-spline curve with the fields of a descriptor and generated knots.
fn with_knots_from(
    source: &StepGeomBSplineCurve,
    knots: &GeneratedKnots,
) -> Handle<StepGeomBSplineCurve> {
    let spline = StepGeomBSplineCurveWithKnots::new();
    spline.set_degree(source.degree());
    spline.set_control_points_list(&source.control_points_list());
    spline.set_curve_form(source.curve_form());
    spline.set_closed_curve(source.closed_curve());
    spline.set_self_intersect(source.self_intersect());
    spline.set_knot_multiplicities(&knots.multiplicities);
    spline.set_knots(&knots.knots);
    Handle::new(spline).cast()
}

/// A rational B-spline curve with the fields of a descriptor and generated knots.
fn rational_from(
    source: &StepGeomBSplineCurve,
    weights: &Handle<HArray1<f64>>,
    knots: &GeneratedKnots,
) -> Handle<StepGeomBSplineCurve> {
    let spline = StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::new();
    spline.init_fields(
        &source.name(),
        source.degree(),
        &source.control_points_list(),
        source.curve_form(),
        source.closed_curve(),
        source.self_intersect(),
        &knots.multiplicities,
        &knots.knots,
        StepGeomKnotType::KtUnspecified,
        weights,
    );
    Handle::new(spline).cast()
}

/// The B-spline form of a Bezier, uniform or quasi-uniform descriptor (they are transformed
/// into a STEP B-spline before being mapped); a null handle when `sc` is none of these or its
/// degree is out of range.
fn descriptor_as_bspline(
    sc: &Handle<StepGeomBoundedCurve>,
) -> Result<Handle<StepGeomBSplineCurve>, Failure> {
    let source = sc.cast::<StepGeomBSplineCurve>();
    let Some(live) = source.get() else {
        return Ok(Handle::default());
    };
    let degree = live.degree();
    let nb_poles = live.nb_control_points_list();
    if sc.cast::<StepGeomBezierCurve>().is_some() {
        return Ok(if degree_out_of_range(degree) {
            Handle::default()
        } else {
            with_knots_from(live, &GeneratedKnots::bezier(degree))
        });
    }
    if sc.cast::<StepGeomUniformCurve>().is_some() {
        return Ok(if degree_out_of_range(degree) {
            Handle::default()
        } else {
            with_knots_from(live, &GeneratedKnots::uniform(nb_poles + degree + 1))
        });
    }
    if sc.cast::<StepGeomQuasiUniformCurve>().is_some() {
        return Ok(if degree_out_of_range(degree) {
            Handle::default()
        } else {
            with_knots_from(
                live,
                &GeneratedKnots::quasi_uniform(nb_poles - degree + 1, degree),
            )
        });
    }
    let uniform = sc.cast::<StepGeomUniformCurveAndRationalBSplineCurve>();
    if let Some(rational) = uniform.get() {
        return Ok(if degree_out_of_range(degree) {
            Handle::default()
        } else {
            rational_from(
                live,
                &rational.weights_data()?,
                &GeneratedKnots::uniform(nb_poles + degree + 1),
            )
        });
    }
    let quasi = sc.cast::<StepGeomQuasiUniformCurveAndRationalBSplineCurve>();
    if let Some(rational) = quasi.get() {
        return Ok(if degree_out_of_range(degree) {
            Handle::default()
        } else {
            rational_from(
                live,
                &rational.weights_data()?,
                &GeneratedKnots::quasi_uniform(nb_poles - degree + 1, degree),
            )
        });
    }
    Ok(Handle::default())
}

impl StepToGeom {
    /// `make_bounded_curve`: null for another bounded curve or a descriptor of a wrong degree.
    pub fn make_bounded_curve(
        sc: &Handle<StepGeomBoundedCurve>,
        factors: &StepDataFactors,
    ) -> Result<Handle<dyn GeomBoundedCurve>, Failure> {
        if sc
            .cast::<StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve>()
            .is_some()
            || sc.cast::<StepGeomBSplineCurveWithKnots>().is_some()
        {
            return Ok(Self::make_bspline_curve(&sc.cast(), factors)?.cast());
        }
        let trimmed = sc.cast::<StepGeomTrimmedCurve>();
        if trimmed.is_some() {
            return Ok(Self::make_trimmed_curve(&trimmed, factors)?.cast());
        }
        let spline = descriptor_as_bspline(sc)?;
        if spline.is_some() {
            return Ok(Self::make_bspline_curve(&spline, factors)?.cast());
        }
        let polyline = sc.cast::<StepGeomPolyline>();
        if polyline.is_some() {
            //:n6 abv 15 Feb 99
            return Ok(Self::make_polyline(&polyline, factors)?.cast());
        }
        Ok(Handle::default())
    }
    /// `MakeBoundedCurve2d`: splines, trimmed curves and polylines; null for another curve.
    pub fn make_bounded_curve_2d(
        sc: &Handle<StepGeomBoundedCurve>,
        factors: &StepDataFactors,
    ) -> Result<Handle<dyn Geom2dBoundedCurve>, Failure> {
        if sc
            .cast::<StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve>()
            .is_some()
            || sc.cast::<StepGeomBSplineCurveWithKnots>().is_some()
        {
            let made: Handle<Geom2dBSplineCurve> =
                Self::make_bspline_curve_2d(&sc.cast(), factors)?;
            return Ok(made.cast());
        }
        let trimmed = sc.cast::<StepGeomTrimmedCurve>();
        if trimmed.is_some() {
            return Ok(Self::make_trimmed_curve_2d(&trimmed, factors)?.cast());
        }
        let polyline = sc.cast::<StepGeomPolyline>();
        if polyline.is_some() {
            //:n6 abv 15 Feb 99
            return Ok(Self::make_polyline_2d(&polyline, factors)?.cast());
        }
        Ok(Handle::default())
    }
}
