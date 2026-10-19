//! Curve evaluation through a descriptor, with the parameter map applied to the
//! derivatives.
use crate::geom::errors;
use crate::geom::{GeomCurve, GeomCurveD1, GeomCurveD2, GeomCurveD3};
use crate::geom_eval::rep_curve_desc::{
    RepCurveDescDerivBounded, RepCurveDescFull, RepCurveDescMapped,
};
use crate::geom_eval::{Map1d, RepCurveDesc};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d_eval::RepCurveKind;
use martensite_tkmath::gp::{Point3, Vector3};

/// Derivative of order `n` through the descriptor, scaled by `Scale^n` computed with
/// `std::pow` as natively; `None` for `n < 1` (`TryEvalCurveDN`);
/// `try_eval_curve_dn(&desc, 0.5, 2)?`.
// Unlike Geom2dEvalRepUtils, which multiplies the scale N times, the 3D
// TryEvalCurveDN scales by pow(Scale, N), so this member is not shared.
pub fn try_eval_curve_dn(
    desc: &Handle<dyn RepCurveDesc>,
    u: f64,
    n: i32,
) -> Result<Option<Vector3>, Failure> {
    let Ok(order) = usize::try_from(n) else {
        return Ok(None);
    };
    if order < 1 {
        return Ok(None);
    }
    try_eval(
        desc,
        order,
        u,
        |curve, u| curve.eval_dn(u, n),
        |vector, map| vector * map.scale.powf(f64::from(n)),
    )
}

martensite_tkg2d::define_rep_curve_evaluation!(
    dyn GeomCurve,
    Point3,
    GeomCurveD1,
    GeomCurveD2,
    GeomCurveD3,
    "GeomEval_RepUtils"
);
