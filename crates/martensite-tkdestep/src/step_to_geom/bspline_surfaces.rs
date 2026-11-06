//! B-spline surfaces.
use super::StepToGeom;
use super::bspline_curves::{MergedKnots, count_unique_knots, looks_periodic, merge_knots};
use crate::geom_to_step::array1_of;
use crate::step_data::StepDataFactors;
use crate::step_entity::{step_array2_item, step_live};
use crate::step_geom::{
    StepGeomBSplineSurface, StepGeomBSplineSurfaceWithKnots,
    StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface,
};
use martensite_tkernel::ncollection::{Array2, HArray1, array2_failure};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomBSplineSurface, GeomPoint};
use martensite_tkmath::gp::Point3;

/// The poles of a surface; `None` when a point cannot be made.
fn surface_poles(
    source: &StepGeomBSplineSurfaceWithKnots,
    factors: &StepDataFactors,
) -> Result<Option<Array2<'static, Point3>>, Failure> {
    let (nb_u, nb_v) = (
        source.nb_control_points_list_i(),
        source.nb_control_points_list_j(),
    );
    let list = source.control_points_list();
    let mut poles = Array2::<Point3>::with_bounds(1, nb_u, 1, nb_v).map_err(array2_failure)?;
    for u in 1..=nb_u {
        for v in 1..=nb_v {
            let point = step_array2_item(&list, u, v, "StepToGeom::MakeBSplineSurface")?;
            let made = StepToGeom::make_cartesian_point(&point, factors)?;
            let Some(live) = made.get() else {
                return Ok(None);
            };
            poles.set_value(u, v, live.pnt()).map_err(array2_failure)?;
        }
    }
    Ok(Some(poles))
}

/// The merged knots of a direction; the count pass of the code sizes its arrays.
fn direction_knots(
    knots: &Handle<HArray1<f64>>,
    multiplicities: &Handle<HArray1<i32>>,
    nb_knots: i32,
) -> Result<MergedKnots, Failure> {
    count_unique_knots(knots, nb_knots)?;
    merge_knots(knots, multiplicities, nb_knots)
}

/// The weights of a rational surface, 1-based over the poles.
fn surface_weights(
    rational: &StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface,
    nb_u: i32,
    nb_v: i32,
) -> Result<Array2<'static, f64>, Failure> {
    let source = rational.weights_data()?;
    let mut weights = Array2::<f64>::with_bounds(1, nb_u, 1, nb_v).map_err(array2_failure)?;
    for u in 1..=nb_u {
        for v in 1..=nb_v {
            let weight = step_array2_item(&source, u, v, "StepToGeom::MakeBSplineSurface")?;
            weights.set_value(u, v, weight).map_err(array2_failure)?;
        }
    }
    Ok(weights)
}

impl StepToGeom {
    /// `MakeBSplineSurface`: null when a point cannot be made.
    pub fn make_bspline_surface(
        ss: &Handle<StepGeomBSplineSurface>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomBSplineSurface>, Failure> {
        let rational = ss.cast::<StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface>();
        let with_knots: Handle<StepGeomBSplineSurfaceWithKnots> = match rational.get() {
            Some(live) => live.b_spline_surface_with_knots(),
            None => ss.cast(),
        };
        let source = step_live(&with_knots, "StepToGeom::MakeBSplineSurface")?;
        let degrees = (source.u_degree(), source.v_degree());
        let (nb_u, nb_v) = (
            source.nb_control_points_list_i(),
            source.nb_control_points_list_j(),
        );
        let Some(poles) = surface_poles(source, factors)? else {
            return Ok(Handle::default());
        };
        let u = direction_knots(
            &source.u_knots(),
            &source.u_multiplicities(),
            source.nb_u_multiplicities()?,
        )?;
        let v = direction_knots(
            &source.v_knots(),
            &source.v_multiplicities(),
            source.nb_v_multiplicities()?,
        )?;
        let periodic = (
            looks_periodic(&u.multiplicities, nb_u, degrees.0),
            looks_periodic(&v.multiplicities, nb_v, degrees.1),
        );
        let (u_knots, u_mults) = (array1_of(u.knots), array1_of(u.multiplicities));
        let (v_knots, v_mults) = (array1_of(v.knots), array1_of(v.multiplicities));
        let u_vector = (&u_knots, &u_mults);
        let v_vector = (&v_knots, &v_mults);
        let surface = match rational.get() {
            Some(live) => {
                let weights = surface_weights(live, nb_u, nb_v)?;
                GeomBSplineSurface::with_weights(
                    &poles, &weights, u_vector, v_vector, degrees, periodic,
                )?
            }
            None => GeomBSplineSurface::new(&poles, u_vector, v_vector, degrees, periodic)?,
        };
        Ok(Handle::new(surface))
    }
}
