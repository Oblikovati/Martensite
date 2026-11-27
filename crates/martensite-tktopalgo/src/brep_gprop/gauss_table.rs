//! The Gauss points and weights of one order, zero-based, as every integrator of the
//! package reads them.
use martensite_tkernel::standard::Failure;
use martensite_tkmath::math::{self, Vector};

/// The points and the weights of one Gauss rule, zero-based.
pub(super) type GaussRule = (Vec<f64>, Vec<f64>);

/// The `order` Gauss points and weights of `math::GaussPoints` and `math::GaussWeights`
/// (positive points first, then their opposites), zero-based; a failure for a
/// negative order (the native `math_Vector(1, Order)` bounds).
pub(super) fn gauss_points_and_weights(order: i32) -> Result<GaussRule, Failure> {
    if order < 1 {
        let _ = Vector::with_bounds(1, order)?;
    }
    let len = order as usize;
    let mut points = vec![0.0_f64; len];
    let mut weights = vec![0.0_f64; len];
    {
        let mut p_vec = Vector::from_buffer(&mut points, 1, order)?;
        let mut w_vec = Vector::from_buffer(&mut weights, 1, order)?;
        math::gauss_points(order, &mut p_vec);
        math::gauss_weights(order, &mut w_vec);
    }
    Ok((points, weights))
}
