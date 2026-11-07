//! Cone-specific coordinates and closed-form scalars, independent of native construction.
use super::{cone_admission::AdmittedCone, cone_request::*, frame, request::NormalSense};
use martensite_cmath::{cos, sin};

pub(super) fn sign(support: ConeSupport) -> f64 {
    match support.normal {
        NormalSense::Outward => 1.,
        NormalSense::Inward => -1.,
    }
}
pub(super) fn meridian(admitted: &AdmittedCone, slant: f64, depth: f64) -> [f64; 2] {
    let normal = sign(admitted.sector.support);
    [
        slant * admitted.sine + normal * depth * admitted.cosine,
        slant * admitted.cosine - normal * depth * admitted.sine,
    ]
}
pub(super) fn point(admitted: &AdmittedCone, angle: f64, slant: f64, depth: f64) -> [f64; 3] {
    let support = admitted.sector.support;
    let [radius, height] = meridian(admitted, slant, depth);
    let tangent = frame::cross(support.axis, support.radial);
    std::array::from_fn(|i| {
        support.apex[i]
            + radius * (cos(angle) * support.radial[i] + sin(angle) * tangent[i])
            + height * support.axis[i]
    })
}
pub(super) fn corner(admitted: &AdmittedCone, corner: usize, depth: usize) -> [f64; 3] {
    let angle = admitted.start
        + if corner == 1 || corner == 2 {
            admitted.sweep.angle
        } else {
            0.
        };
    let slant = admitted.sector.slant[usize::from(corner >= 2)];
    point(admitted, angle, slant, admitted.depths[depth])
}
pub(super) fn section_points(admitted: &AdmittedCone) -> [[f64; 3]; 4] {
    let first = admitted.sweep.first_depth;
    [(0, first), (0, 1 - first), (1, 1 - first), (1, first)].map(|(slant, depth)| {
        point(
            admitted,
            admitted.start,
            admitted.sector.slant[slant],
            admitted.depths[depth],
        )
    })
}
pub(super) fn scalars(admitted: &AdmittedCone) -> [f64; 2] {
    let [l0, l1] = admitted.sector.slant;
    let [t0, t1] = admitted.depths;
    let dl = l1 - l0;
    let dt = t1 - t0;
    let middle = meridian(admitted, l0 + dl / 2., t0 + dt / 2.)[0];
    let angle = admitted.sweep.angle;
    [
        angle * dl * dt * middle,
        2. * angle * middle * (dl + dt) + 2. * dl * dt,
    ]
}
pub(super) fn depth_apex(admitted: &AdmittedCone, depth: usize) -> [f64; 3] {
    let support = admitted.sector.support;
    let offset = -sign(support) * admitted.depths[depth] / admitted.sine;
    std::array::from_fn(|i| support.apex[i] + offset * support.axis[i])
}
pub(super) fn slant_apex(admitted: &AdmittedCone, slant: usize) -> [f64; 3] {
    let support = admitted.sector.support;
    let offset = admitted.sector.slant[slant] / admitted.cosine;
    std::array::from_fn(|i| support.apex[i] + offset * support.axis[i])
}
