//! A torus with a major radius below its minor radius, written as a surface of revolution:
//! %pdn 30 Nov 98: TestRally 9 issue on r1001_ec.stp: toruses with major_radius < minor are
//! re-coded as degenerate; abv 30.11.99: changed to produce SurfaceOfRevolution instead of
//! DegenerateToroidalSurface.
use crate::geom_to_step::GeomToStepMakeSurface;
use crate::step_data::StepDataFactors;
use crate::step_geom::{StepGeomSurface, StepGeomToroidalSurface};
use martensite_tkbrep::topods::Face;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{
    GeomCircle, GeomCurve, GeomElementarySurface, GeomOffsetSurface, GeomSurface,
    GeomSurfaceOfRevolution, GeomToroidalSurface,
};
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkmath::gp::{Axis2, Point3};
use martensite_tkshhealing::shape_algo::ShapeAlgo;
use martensite_tkshhealing::shape_construct::ShapeConstructApproxSettings;

/// The revolution standing for the torus: its basis circle, a B-spline when the face does not go
/// all the way round, to avoid a self-intersecting surface of revolution (necessary e.g. for CATIA).
fn revolution_of(
    torus: &GeomToroidalSurface,
    face: &Face,
) -> Result<Handle<GeomSurfaceOfRevolution>, Failure> {
    let position = torus.position();
    let direction = position.direction();
    let x_direction = position.x_direction();
    let (_, _, v_first, v_last) = ShapeAlgo::algo_container().get_face_uv_bounds(face)?;
    let location = position.location();
    let radius = torus.major_radius();
    let origin = Point3::new(
        location.x() + x_direction.x() * radius,
        location.y() + x_direction.y() * radius,
        location.z() + x_direction.z() * radius,
    );
    let axis2 = Axis2::with_x_direction(origin, x_direction.crossed(direction)?, x_direction)?;
    let circle: Handle<dyn GeomCurve> =
        Handle::new(GeomCircle::new(axis2, torus.minor_radius())?).cast();
    let mut basis = circle;
    if v_last - v_first - 2.0 * std::f64::consts::PI < -precision::p_confusion() {
        let settings = ShapeConstructApproxSettings {
            tol3d: precision::approximation(),
            continuity: Continuity::C1,
            max_segments: 100,
            max_degree: 9,
        };
        basis = ShapeAlgo::algo_container()
            .convert_curve_to_bspline(&basis, v_first, v_last, settings)?
            .cast();
    }
    let mut axis = position.axis();
    if !position.direct()? {
        axis.reverse();
    }
    GeomSurfaceOfRevolution::new(&basis, axis).map(Handle::new)
}

/// The entity for the surface of `face`: the given `made`, or the revolution for a degenerate
/// torus. If the surface is offset it is necessary to check the base surface.
pub(super) fn degenerate_torus_as_revolution(
    surface: &Handle<dyn GeomSurface>,
    made: Handle<StepGeomSurface>,
    face: &Face,
    factors: &StepDataFactors,
) -> Result<Handle<StepGeomSurface>, Failure> {
    let offset = surface.cast::<GeomOffsetSurface>();
    let torus: Handle<GeomToroidalSurface> = match offset.get() {
        Some(live) => live.basis_surface().cast(),
        None if made.cast::<StepGeomToroidalSurface>().is_some() => surface.cast(),
        None => return Ok(made),
    };
    let Some(live) = torus.get() else {
        return Ok(made);
    };
    if live.major_radius() >= live.minor_radius() {
        return Ok(made);
    }
    let revolution = revolution_of(live, face)?;
    if let Some(live_offset) = offset.get() {
        live_offset.set_basis_surface(&revolution.cast(), false)?;
        return GeomToStepMakeSurface::from_geom(&offset.cast(), factors)?.value();
    }
    GeomToStepMakeSurface::from_geom(&revolution.cast(), factors)?.value()
}
