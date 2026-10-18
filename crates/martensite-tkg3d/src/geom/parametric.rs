//! The parameter changes of a transformed surface that several Geom surfaces
//! write with the same retained statements.
use martensite_tkernel::precision;
use martensite_tkmath::gp::{
    Axis2d, Direction2, GeneralTransform2, Transform3, dx2d, dy2d, origin2d,
};

/// `if (!Precision::IsInfinite(P)) P *= std::abs(T.ScaleFactor())`: a length
/// parameter scaled by the similarity ratio; `scaled_parameter(2.0, t)`.
pub fn scaled_parameter(parameter: f64, transform: Transform3) -> f64 {
    if precision::is_infinite(parameter) {
        return parameter;
    }
    parameter * transform.scale_factor().abs()
}

/// `gpGTrsf2d T; T.SetAffinity(gpAx2d(gp::Origin2d(), direction), ratio)`: the
/// affinity that scales the parameter normal to `direction` by `ratio`.
fn affinity(direction: Direction2, ratio: f64) -> GeneralTransform2 {
    let mut result = GeneralTransform2::default();
    result.set_affinity(Axis2d::new(origin2d(), direction), ratio);
    result
}

/// The affinity about the U axis (`gp::DX2d()`), which scales V by `ratio`;
/// `v_affinity(2.0)`.
pub fn v_affinity(ratio: f64) -> GeneralTransform2 {
    affinity(dx2d(), ratio)
}

/// The affinity about the V axis (`gp::DY2d()`), which scales U by `ratio`;
/// `u_affinity(2.0)`.
pub fn u_affinity(ratio: f64) -> GeneralTransform2 {
    affinity(dy2d(), ratio)
}
