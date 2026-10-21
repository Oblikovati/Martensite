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
    let mut points: Vector<'static> = Vector::with_bounds(1, order)?;
    let mut weights: Vector<'static> = Vector::with_bounds(1, order)?;
    math::gauss_points(order, &mut points);
    math::gauss_weights(order, &mut weights);
    let points: Vec<f64> = (1..=order).map(|i| points[i]).collect();
    let weights: Vec<f64> = (1..=order).map(|i| weights[i]).collect();
    Ok((points, weights))
}
