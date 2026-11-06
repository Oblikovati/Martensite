//! Bounded surfaces: Bezier, uniform and quasi-uniform surfaces become B-spline surfaces.
use super::StepToGeom;
use super::bounded_curves::GeneratedKnots;
use crate::step_data::StepDataFactors;
use crate::step_geom::{
    StepGeomBSplineSurface, StepGeomBSplineSurfaceWithKnots,
    StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface, StepGeomBezierSurface,
    StepGeomBoundedSurface, StepGeomKnotType, StepGeomQuasiUniformSurface,
    StepGeomQuasiUniformSurfaceAndRationalBSplineSurface, StepGeomRectangularTrimmedSurface,
    StepGeomUniformSurface, StepGeomUniformSurfaceAndRationalBSplineSurface,
};
use martensite_tkernel::ncollection::HArray2;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomBoundedSurface;

/// A non rational B-spline surface with the fields of a descriptor and generated knots.
fn with_knots_from(
    source: &StepGeomBSplineSurface,
    u: &GeneratedKnots,
    v: &GeneratedKnots,
) -> Handle<StepGeomBSplineSurface> {
    let spline = StepGeomBSplineSurfaceWithKnots::new();
    spline.set_u_degree(source.u_degree());
    spline.set_v_degree(source.v_degree());
    spline.set_control_points_list(&source.control_points_list());
    spline.set_surface_form(source.surface_form());
    spline.set_u_closed(source.u_closed());
    spline.set_v_closed(source.v_closed());
    spline.set_self_intersect(source.self_intersect());
    spline.set_u_multiplicities(&u.multiplicities);
    spline.set_v_multiplicities(&v.multiplicities);
    spline.set_u_knots(&u.knots);
    spline.set_v_knots(&v.knots);
    Handle::new(spline).cast()
}

/// A rational B-spline surface with the fields of a descriptor and generated knots.
fn rational_from(
    source: &StepGeomBSplineSurface,
    weights: &Handle<HArray2<f64>>,
    u: &GeneratedKnots,
    v: &GeneratedKnots,
) -> Handle<StepGeomBSplineSurface> {
    let spline = StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::new();
    spline.init_fields(
        &source.name(),
        source.u_degree(),
        source.v_degree(),
        &source.control_points_list(),
        source.surface_form(),
        source.u_closed(),
        source.v_closed(),
        source.self_intersect(),
        &u.multiplicities,
        &v.multiplicities,
        &u.knots,
        &v.knots,
        StepGeomKnotType::KtUnspecified,
        weights,
    );
    Handle::new(spline).cast()
}

/// The generated knots of both directions of a uniform descriptor.
fn uniform_knots(source: &StepGeomBSplineSurface) -> (GeneratedKnots, GeneratedKnots) {
    (
        GeneratedKnots::uniform(source.nb_control_points_list_i() + source.u_degree() + 1),
        GeneratedKnots::uniform(source.nb_control_points_list_j() + source.v_degree() + 1),
    )
}

/// The generated knots of both directions of a quasi-uniform descriptor.
fn quasi_uniform_knots(source: &StepGeomBSplineSurface) -> (GeneratedKnots, GeneratedKnots) {
    (
        GeneratedKnots::quasi_uniform(
            source.nb_control_points_list_i() - source.u_degree() + 1,
            source.u_degree(),
        ),
        GeneratedKnots::quasi_uniform(
            source.nb_control_points_list_j() - source.v_degree() + 1,
            source.v_degree(),
        ),
    )
}

/// The B-spline form of a Bezier, uniform or quasi-uniform surface (they are transformed before
/// being mapped); a null handle when `ss` is none of these.
fn descriptor_as_bspline(
    ss: &Handle<StepGeomBoundedSurface>,
) -> Result<Handle<StepGeomBSplineSurface>, Failure> {
    let base = ss.cast::<StepGeomBSplineSurface>();
    let Some(source) = base.get() else {
        return Ok(Handle::default());
    };
    if ss.cast::<StepGeomBezierSurface>().is_some() {
        let u = GeneratedKnots::bezier(source.u_degree());
        let v = GeneratedKnots::bezier(source.v_degree());
        return Ok(with_knots_from(source, &u, &v));
    }
    if ss.cast::<StepGeomUniformSurface>().is_some() {
        let (u, v) = uniform_knots(source);
        return Ok(with_knots_from(source, &u, &v));
    }
    if ss.cast::<StepGeomQuasiUniformSurface>().is_some() {
        let (u, v) = quasi_uniform_knots(source);
        return Ok(with_knots_from(source, &u, &v));
    }
    if let Some(rational) = ss
        .cast::<StepGeomUniformSurfaceAndRationalBSplineSurface>()
        .get()
    {
        let (u, v) = uniform_knots(source);
        return Ok(rational_from(source, &rational.weights_data()?, &u, &v));
    }
    if let Some(rational) = ss
        .cast::<StepGeomQuasiUniformSurfaceAndRationalBSplineSurface>()
        .get()
    {
        let (u, v) = quasi_uniform_knots(source);
        return Ok(rational_from(source, &rational.weights_data()?, &u, &v));
    }
    Ok(Handle::default())
}

impl StepToGeom {
    /// `make_bounded_surface`: null for another bounded surface.
    pub fn make_bounded_surface(
        ss: &Handle<StepGeomBoundedSurface>,
        factors: &StepDataFactors,
    ) -> Result<Handle<dyn GeomBoundedSurface>, Failure> {
        if ss
            .cast::<StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface>()
            .is_some()
            || ss.cast::<StepGeomBSplineSurfaceWithKnots>().is_some()
        {
            return Ok(Self::make_bspline_surface(&ss.cast(), factors)?.cast());
        }
        let trimmed = ss.cast::<StepGeomRectangularTrimmedSurface>();
        if trimmed.is_some() {
            return Ok(Self::make_rectangular_trimmed_surface(&trimmed, factors)?.cast());
        }
        let spline = descriptor_as_bspline(ss)?;
        if spline.is_some() {
            return Ok(Self::make_bspline_surface(&spline, factors)?.cast());
        }
        Ok(Handle::default())
    }
}
