//! B-spline surfaces with knots, rational or not.
use super::bspline_curves::{logical_of, shared_copy};
use super::placements::{GeomToStepMakeCartesianPoint, live_geometry};
use super::{empty_step_name, geom_to_step_class};
use crate::step_data::{StepDataFactors, StepDataLogical};
use crate::step_geom::{
    StepGeomBSplineSurfaceForm, StepGeomBSplineSurfaceWithKnots,
    StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface, StepGeomCartesianPoint,
    StepGeomKnotType,
};
use martensite_tkernel::ncollection::{HArray1, HArray2, array2_failure};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomBSplineSurface, GeomSurface};
use martensite_tkmath::geom_abs::BSplKnotDistribution;

/// The fields shared by both kinds of surface, computed once.
struct SurfaceFields {
    u_degree: i32,
    v_degree: i32,
    poles: Handle<HArray2<Handle<StepGeomCartesianPoint>>>,
    u_closed: StepDataLogical,
    v_closed: StepDataLogical,
    u_multiplicities: Handle<HArray1<i32>>,
    v_multiplicities: Handle<HArray1<i32>>,
    u_knots: Handle<HArray1<f64>>,
    v_knots: Handle<HArray1<f64>>,
    knot_type: StepGeomKnotType,
}

/// The STEP knot type of a surface: a specific type needs both directions to agree.
fn surface_knot_type(u: BSplKnotDistribution, v: BSplKnotDistribution) -> StepGeomKnotType {
    match (u, v) {
        (BSplKnotDistribution::Uniform, BSplKnotDistribution::Uniform) => {
            StepGeomKnotType::KtUniformKnots
        }
        (BSplKnotDistribution::QuasiUniform, BSplKnotDistribution::QuasiUniform) => {
            StepGeomKnotType::KtQuasiUniformKnots
        }
        (BSplKnotDistribution::PiecewiseBezier, BSplKnotDistribution::PiecewiseBezier) => {
            StepGeomKnotType::KtPiecewiseBezierKnots
        }
        _ => StepGeomKnotType::KtUnspecified,
    }
}

impl SurfaceFields {
    fn shared_poles(
        spline: &GeomBSplineSurface,
        factor: f64,
    ) -> Result<Handle<HArray2<Handle<StepGeomCartesianPoint>>>, Failure> {
        let poles = spline.poles();
        let grid = HArray2::<Handle<StepGeomCartesianPoint>>::with_bounds(
            1,
            spline.nb_u_poles(),
            1,
            spline.nb_v_poles(),
        )
        .map_err(array2_failure)?;
        for row in poles.lower_row()..=poles.upper_row() {
            for column in poles.lower_col()..=poles.upper_col() {
                let pole = poles.value(row, column).map_err(array2_failure)?;
                let point = GeomToStepMakeCartesianPoint::from_pnt(pole, factor).value()?;
                grid.change_array2()
                    .set_value(row, column, point)
                    .map_err(array2_failure)?;
            }
        }
        Ok(Handle::new(grid))
    }
    fn new(spline: &GeomBSplineSurface, factors: &StepDataFactors) -> Result<Self, Failure> {
        Ok(Self {
            u_degree: spline.u_degree(),
            v_degree: spline.v_degree(),
            poles: Self::shared_poles(spline, factors.length_factor())?,
            u_closed: logical_of(spline.is_u_closed()?),
            v_closed: logical_of(spline.is_v_closed()?),
            u_multiplicities: shared_copy(&spline.u_multiplicities()),
            v_multiplicities: shared_copy(&spline.v_multiplicities()),
            u_knots: shared_copy(&spline.u_knots()),
            v_knots: shared_copy(&spline.v_knots()),
            knot_type: surface_knot_type(
                spline.u_knot_distribution(),
                spline.v_knot_distribution(),
            ),
        })
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeBSplineSurfaceWithKnots`: a non rational spline surface.
    GeomToStepMakeBSplineSurfaceWithKnots,
    "GeomToStepMakeBSplineSurfaceWithKnots",
    StepGeomBSplineSurfaceWithKnots
);

impl GeomToStepMakeBSplineSurfaceWithKnots {
    /// `GeomToStepMakeBSplineSurfaceWithKnots(GeomBSplineSurface, factors)`.
    pub fn from_geom(
        spline: &Handle<GeomBSplineSurface>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let live = live_geometry(spline, "GeomToStepMakeBSplineSurfaceWithKnots")?;
        let fields = SurfaceFields::new(live, factors)?;
        let surface = StepGeomBSplineSurfaceWithKnots::new();
        surface.init(
            &empty_step_name(),
            fields.u_degree,
            fields.v_degree,
            &fields.poles,
            StepGeomBSplineSurfaceForm::BssfUnspecified,
            fields.u_closed,
            fields.v_closed,
            StepDataLogical::LFalse,
            &fields.u_multiplicities,
            &fields.v_multiplicities,
            &fields.u_knots,
            &fields.v_knots,
            fields.knot_type,
        );
        Ok(Self::done_with(Handle::new(surface)))
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeBSplineSurfaceWithKnotsAndRationalBSplineSurface`: a rational surface.
    GeomToStepMakeBSplineSurfaceWithKnotsAndRationalBSplineSurface,
    "GeomToStepMakeBSplineSurfaceWithKnotsAndRationalBSplineSurface",
    StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface
);

impl GeomToStepMakeBSplineSurfaceWithKnotsAndRationalBSplineSurface {
    /// The weights of a spline; a non rational direction has unit weights, as `weights_array`.
    fn shared_weights(spline: &GeomBSplineSurface) -> Handle<HArray2<f64>> {
        spline.weights_array(|weights| Handle::new(HArray2::from_owned(weights.clone())))
    }
    /// `GeomToStepMakeBSplineSurfaceWithKnotsAndRationalBSplineSurface(GeomBSplineSurface, factors)`.
    pub fn from_geom(
        spline: &Handle<GeomBSplineSurface>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let live = live_geometry(
            spline,
            "GeomToStepMakeBSplineSurfaceWithKnotsAndRationalBSplineSurface",
        )?;
        let fields = SurfaceFields::new(live, factors)?;
        let surface = StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::new();
        surface.init_fields(
            &empty_step_name(),
            fields.u_degree,
            fields.v_degree,
            &fields.poles,
            StepGeomBSplineSurfaceForm::BssfUnspecified,
            fields.u_closed,
            fields.v_closed,
            StepDataLogical::LFalse,
            &fields.u_multiplicities,
            &fields.v_multiplicities,
            &fields.u_knots,
            &fields.v_knots,
            fields.knot_type,
            &Self::shared_weights(live),
        );
        Ok(Self::done_with(Handle::new(surface)))
    }
}
