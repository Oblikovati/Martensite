//! Admit exactly representable endpoints of native affine pcurves.
use crate::predicates::{NonFiniteCoordinate, exact_coordinate, roots::DyadicBound};

/// Return an exact binary64 result for `origin + direction * parameter` when representable.
/// Example: `assert_eq!(exact_affine_coordinate(1., 0.5, 4.)?, Some(3.));`.
pub fn exact_affine_coordinate(
    origin: f64,
    direction: f64,
    parameter: f64,
) -> Result<Option<f64>, NonFiniteCoordinate> {
    let origin = exact_coordinate(origin, 0, 0)?;
    let direction = exact_coordinate(direction, 1, 0)?;
    let parameter = exact_coordinate(parameter, 2, 0)?;
    let exact = DyadicBound::new((origin << 1074_usize) + direction * parameter, -2148);
    Ok(exact
        .binary64_bounds()
        .and_then(|[lo, hi]| (lo == hi).then_some(lo)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_line_endpoint_distinguishes_underflow_and_rounding() {
        assert_eq!(exact_affine_coordinate(2., 0.25, 2.).unwrap(), Some(2.5));
        assert_eq!(
            exact_affine_coordinate(0., f64::from_bits(1), 0.5).unwrap(),
            None
        );
        assert_eq!(
            exact_affine_coordinate(1., f64::from_bits(1), 1.).unwrap(),
            None
        );
        assert_eq!(exact_affine_coordinate(-1., 0.25, 4.).unwrap(), Some(0.));
    }
}
