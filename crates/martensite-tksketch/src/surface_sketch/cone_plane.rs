//! Use an axial reference outside the section for conditioned cone volume integration.
//! The centroid route stalls on the half-turn control (#424); this route preserves
//! the requested convergence and returned error, without retrying or changing the body.
use super::{admission::finite, cone_admission::AdmittedCone, cone_math, failure::*};
use martensite_tkmath::gp::{Direction3, Plane3, Point3};

pub(super) fn reference(admitted: &AdmittedCone) -> Checked<Plane3> {
    let support = admitted.sector.support;
    let offset = reference_offset(admitted);
    let xyz = std::array::from_fn(|i| support.apex[i] + offset * support.axis[i]);
    finite("integration reference plane", &xyz)?;
    let [x, y, z] = xyz;
    let [a, b, c] = support.axis;
    let normal = Direction3::new(a, b, c).map_err(|e| native("integration reference normal", e))?;
    Plane3::new(Point3::new(x, y, z), normal).map_err(|e| native("integration reference plane", e))
}
fn reference_offset(admitted: &AdmittedCone) -> f64 {
    let section = [(0, 0), (0, 1), (1, 0), (1, 1)]
        .map(|(l, t)| cone_math::meridian(admitted, admitted.sector.slant[l], admitted.depths[t]));
    let minimum = section
        .into_iter()
        .map(|point| point[1])
        .fold(f64::INFINITY, f64::min);
    let radius = section.into_iter().map(|point| point[0]).fold(0., f64::max);
    let margin = radius
        .max(admitted.sector.slant[1] - admitted.sector.slant[0])
        .max(admitted.depths[1] - admitted.depths[0]);
    minimum - margin
}
pub(super) fn reference_coordinates(admitted: &AdmittedCone) -> [f64; 3] {
    match admitted.sweep.reference_plane {
        Some(plane) => {
            let point = plane.location();
            [point.x(), point.y(), point.z()]
        }
        None => [0.; 3],
    }
}
