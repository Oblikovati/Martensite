//! B-spline curves with knots, rational or not, and the bounded curve dispatch.
use super::placements::{GeomToStepMakeCartesianPoint, live_geometry};
use super::{empty_step_name, geom_to_step_class, shared_array1};
use crate::step_data::{StepDataFactors, StepDataLogical};
use crate::step_geom::{
    StepGeomBSplineCurveForm, StepGeomBSplineCurveWithKnots,
    StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve, StepGeomBoundedCurve,
    StepGeomCartesianPoint, StepGeomKnotType,
};
use martensite_tkernel::ncollection::{Array1, HArray1};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{
    Geom2dBSplineCurve, Geom2dBezierCurve, Geom2dBoundedCurve, Geom2dCurve, Geom2dGeometry,
};
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomBezierCurve, GeomBoundedCurve, GeomCurve, GeomGeometry,
};
use martensite_tkgeombase::{geom_convert, geom2d_convert};
use martensite_tkmath::convert::ParameterisationType;
use martensite_tkmath::geom_abs::BSplKnotDistribution;

/// The fields shared by both kinds of curve, computed once.
struct CurveFields {
    degree: i32,
    poles: Handle<HArray1<Handle<StepGeomCartesianPoint>>>,
    closed: StepDataLogical,
    multiplicities: Handle<HArray1<i32>>,
    knots: Handle<HArray1<f64>>,
    knot_type: StepGeomKnotType,
    weights: Handle<HArray1<f64>>,
}

/// The STEP knot type of a knot distribution of a curve.
pub(super) fn curve_knot_type(distribution: BSplKnotDistribution) -> StepGeomKnotType {
    match distribution {
        BSplKnotDistribution::NonUniform => StepGeomKnotType::KtUnspecified,
        BSplKnotDistribution::Uniform => StepGeomKnotType::KtUniformKnots,
        BSplKnotDistribution::QuasiUniform => StepGeomKnotType::KtQuasiUniformKnots,
        BSplKnotDistribution::PiecewiseBezier => StepGeomKnotType::KtPiecewiseBezierKnots,
    }
}

/// `true` or `false` of a flag.
pub(super) fn logical_of(flag: bool) -> StepDataLogical {
    if flag {
        StepDataLogical::LTrue
    } else {
        StepDataLogical::LFalse
    }
}

/// A shared 1-based copy of a array.
pub(super) fn shared_copy<T: Clone + Default + Send + Sync + 'static>(
    items: &Array1<'_, T>,
) -> Handle<HArray1<T>> {
    shared_array1(items.as_slice().to_vec())
}

/// What the two dimensions of a spline give the shared fields.
trait CurveFieldSource {
    fn cartesian_poles(&self, factor: f64) -> Result<Vec<Handle<StepGeomCartesianPoint>>, Failure>;
    fn fields_without_poles(&self) -> Result<PartialFields, Failure>;
}

/// The fields besides the poles.
struct PartialFields {
    degree: i32,
    closed: bool,
    multiplicities: Array1<'static, i32>,
    knots: Array1<'static, f64>,
    distribution: BSplKnotDistribution,
    weights: Option<Array1<'static, f64>>,
}

macro_rules! curve_field_source {
    ($spline:ty, $from_pole:ident) => {
        impl CurveFieldSource for $spline {
            fn cartesian_poles(
                &self,
                factor: f64,
            ) -> Result<Vec<Handle<StepGeomCartesianPoint>>, Failure> {
                self.poles()
                    .iter()
                    .map(|pole| GeomToStepMakeCartesianPoint::$from_pole(pole, factor).value())
                    .collect()
            }
            fn fields_without_poles(&self) -> Result<PartialFields, Failure> {
                Ok(PartialFields {
                    degree: self.degree(),
                    closed: self.is_closed()?,
                    multiplicities: self.multiplicities(),
                    knots: self.knots(),
                    distribution: self.knot_distribution(),
                    weights: self.weights(),
                })
            }
        }
    };
}
curve_field_source!(GeomBSplineCurve, from_pnt);
curve_field_source!(Geom2dBSplineCurve, from_pnt2d);

impl CurveFields {
    fn new(spline: &dyn CurveFieldSource, factors: &StepDataFactors) -> Result<Self, Failure> {
        let poles = spline.cartesian_poles(factors.length_factor())?;
        let partial = spline.fields_without_poles()?;
        Ok(Self {
            degree: partial.degree,
            poles: shared_array1(poles),
            closed: logical_of(partial.closed),
            multiplicities: shared_copy(&partial.multiplicities),
            knots: shared_copy(&partial.knots),
            knot_type: curve_knot_type(partial.distribution),
            weights: partial
                .weights
                .as_ref()
                .map_or_else(Handle::default, shared_copy),
        })
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeBSplineCurveWithKnots`: a non rational spline.
    GeomToStepMakeBSplineCurveWithKnots,
    "GeomToStepMakeBSplineCurveWithKnots",
    StepGeomBSplineCurveWithKnots
);

impl GeomToStepMakeBSplineCurveWithKnots {
    fn with_fields(fields: &CurveFields) -> Self {
        let curve = StepGeomBSplineCurveWithKnots::new();
        curve.init(
            &empty_step_name(),
            fields.degree,
            &fields.poles,
            StepGeomBSplineCurveForm::BscfUnspecified,
            fields.closed,
            StepDataLogical::LFalse,
            &fields.multiplicities,
            &fields.knots,
            fields.knot_type,
        );
        Self::done_with(Handle::new(curve))
    }
    /// `GeomToStepMakeBSplineCurveWithKnots(GeomBSplineCurve, factors)`.
    pub fn from_geom(
        spline: &Handle<GeomBSplineCurve>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let live = live_geometry(spline, "GeomToStepMakeBSplineCurveWithKnots")?;
        Ok(Self::with_fields(&CurveFields::new(live, factors)?))
    }
    /// `GeomToStepMakeBSplineCurveWithKnots(Geom2dBSplineCurve, factors)`.
    pub fn from_geom2d(
        spline: &Handle<Geom2dBSplineCurve>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let live = live_geometry(spline, "GeomToStepMakeBSplineCurveWithKnots")?;
        Ok(Self::with_fields(&CurveFields::new(live, factors)?))
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeBSplineCurveWithKnotsAndRationalBSplineCurve`: a rational spline.
    GeomToStepMakeBSplineCurveWithKnotsAndRationalBSplineCurve,
    "GeomToStepMakeBSplineCurveWithKnotsAndRationalBSplineCurve",
    StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve
);

impl GeomToStepMakeBSplineCurveWithKnotsAndRationalBSplineCurve {
    fn with_fields(fields: &CurveFields) -> Self {
        let curve = StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::new();
        curve.init_fields(
            &empty_step_name(),
            fields.degree,
            &fields.poles,
            StepGeomBSplineCurveForm::BscfUnspecified,
            fields.closed,
            StepDataLogical::LFalse,
            &fields.multiplicities,
            &fields.knots,
            fields.knot_type,
            &fields.weights,
        );
        Self::done_with(Handle::new(curve))
    }
    /// `GeomToStepMakeBSplineCurveWithKnotsAndRationalBSplineCurve(GeomBSplineCurve, factors)`.
    pub fn from_geom(
        spline: &Handle<GeomBSplineCurve>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let live = live_geometry(
            spline,
            "GeomToStepMakeBSplineCurveWithKnotsAndRationalBSplineCurve",
        )?;
        Ok(Self::with_fields(&CurveFields::new(live, factors)?))
    }
    /// `GeomToStepMakeBSplineCurveWithKnotsAndRationalBSplineCurve(Geom2dBSplineCurve, factors)`.
    pub fn from_geom2d(
        spline: &Handle<Geom2dBSplineCurve>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let live = live_geometry(
            spline,
            "GeomToStepMakeBSplineCurveWithKnotsAndRationalBSplineCurve",
        )?;
        Ok(Self::with_fields(&CurveFields::new(live, factors)?))
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeBoundedCurve`: the dispatch over the splines and the Bezier curves.
    GeomToStepMakeBoundedCurve,
    "GeomToStepMakeBoundedCurve",
    StepGeomBoundedCurve
);

impl GeomToStepMakeBoundedCurve {
    fn from_spline(
        spline: &Handle<GeomBSplineCurve>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let live = live_geometry(spline, "GeomToStepMakeBoundedCurve")?;
        if live.is_rational() {
            let made = GeomToStepMakeBSplineCurveWithKnotsAndRationalBSplineCurve::from_geom(
                spline, factors,
            )?;
            return Ok(Self::done_with(made.value()?.cast()));
        }
        let made = GeomToStepMakeBSplineCurveWithKnots::from_geom(spline, factors)?;
        Ok(Self::done_with(made.value()?.cast()))
    }
    fn from_spline2d(
        spline: &Handle<Geom2dBSplineCurve>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let live = live_geometry(spline, "GeomToStepMakeBoundedCurve")?;
        if live.is_rational() {
            let made = GeomToStepMakeBSplineCurveWithKnotsAndRationalBSplineCurve::from_geom2d(
                spline, factors,
            )?;
            return Ok(Self::done_with(made.value()?.cast()));
        }
        let made = GeomToStepMakeBSplineCurveWithKnots::from_geom2d(spline, factors)?;
        Ok(Self::done_with(made.value()?.cast()))
    }
    /// A periodic spline written unperiodic on a copy; the source is untouched.
    fn unperiodic(spline: &Handle<GeomBSplineCurve>) -> Result<Handle<GeomBSplineCurve>, Failure> {
        let copy = spline.copy()?.cast::<GeomBSplineCurve>();
        live_geometry(&copy, "GeomToStepMakeBoundedCurve")?.set_not_periodic()?;
        Ok(copy)
    }
    fn unperiodic2d(
        spline: &Handle<Geom2dBSplineCurve>,
    ) -> Result<Handle<Geom2dBSplineCurve>, Failure> {
        let copy = spline.copy()?.cast::<Geom2dBSplineCurve>();
        live_geometry(&copy, "GeomToStepMakeBoundedCurve")?.set_not_periodic()?;
        Ok(copy)
    }
    /// `GeomToStepMakeBoundedCurve(GeomBoundedCurve, factors)`; another curve is not done.
    pub fn from_geom(
        curve: &Handle<dyn GeomBoundedCurve>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let spline = curve.cast::<GeomBSplineCurve>();
        if let Some(live) = spline.get() {
            if live.is_periodic() {
                return Self::from_spline(&Self::unperiodic(&spline)?, factors);
            }
            return Self::from_spline(&spline, factors);
        }
        if curve.cast::<GeomBezierCurve>().is_some() {
            return Self::from_spline(
                &geom_convert::curve_to_bspline_curve(
                    &curve.cast(),
                    ParameterisationType::TgtThetaOver2,
                )?,
                factors,
            );
        }
        Ok(Self::not_done())
    }
    /// `GeomToStepMakeBoundedCurve(Geom2dBoundedCurve, factors)`: a 2D Bezier curve is written
    /// without weights even when it is rational.
    pub fn from_geom2d(
        curve: &Handle<dyn Geom2dBoundedCurve>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let spline = curve.cast::<Geom2dBSplineCurve>();
        if let Some(live) = spline.get() {
            if live.is_periodic() {
                return Self::from_spline2d(&Self::unperiodic2d(&spline)?, factors);
            }
            return Self::from_spline2d(&spline, factors);
        }
        if curve.cast::<Geom2dBezierCurve>().is_some() {
            let converted = geom2d_convert::curve_to_bspline_curve(
                &curve.cast(),
                ParameterisationType::TgtThetaOver2,
            )?;
            let made = GeomToStepMakeBSplineCurveWithKnots::from_geom2d(&converted, factors)?;
            return Ok(Self::done_with(made.value()?.cast()));
        }
        Ok(Self::not_done())
    }
}
