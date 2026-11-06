//! StepToGeom: STEP geometric entities to Geom and Geom2d objects. Every member returns a null
//! handle when the entity cannot be converted, and a `Failure` where the code
//! raises: a null dereference on a malformed entity or a refused Geom constructor. Lengths are
//! multiplied by the length factor of the transfer; 2D geometry is read unscaled.
mod bounded_curves;
mod bounded_surfaces;
mod bspline_curves;
mod bspline_surfaces;
mod conics;
mod curves;
mod placements;
mod surfaces;
mod transformations;
mod trimmed_curves;
mod ypr_rotation;

/// `StepToGeom`: the static conversion members.
///
/// ```
/// use martensite_tkdestep::{step_data::StepDataFactors, step_to_geom::StepToGeom};
/// use martensite_tkdestep::step_geom::StepGeomCartesianPoint;
/// use martensite_tkernel::{standard::Handle, tcollection::HAsciiString};
/// use martensite_tkg3d::geom::GeomPoint;
/// let point = StepGeomCartesianPoint::new();
/// point.init_3d(&Handle::new(HAsciiString::new("")), 1.0, 2.0, 3.0);
/// let mut factors = StepDataFactors::new();
/// factors.initialize_factors(10.0, 1.0, 1.0);
/// let made = StepToGeom::make_cartesian_point(&Handle::new(point), &factors).unwrap();
/// assert_eq!(made.get().unwrap().x(), 10.0);
/// ```
pub struct StepToGeom;
