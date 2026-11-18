// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::{
    arithmetic::product,
    coefficients as capture,
    polynomial_integrals::arithmetic::{green_numerator, integrate_unit, native_plane},
};
use num_bigint::BigInt;

pub(in super::super) fn integrate(
    poles: &[[f64; 2]],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<[ExactRatio; 2], PolynomialIntegralFailure> {
    let source = MetricCurve::Bezier(MetricBezier {
        poles,
        weights: None,
    });
    let curve = capture::embedded(source, native_plane(), "analytic cone cap UV", cancelled)?;
    let green = green_numerator(&curve, cancelled)?;
    let square = &curve[3][0] * &curve[3][0];
    let area = integrate_unit(&green, BigInt::from(2) * &square, cancelled)?;
    let weighted = product(&green, &curve[1], cancelled)?;
    let moment = integrate_unit(
        &weighted,
        BigInt::from(3) * square * &curve[3][0],
        cancelled,
    )?;
    Ok([area, moment])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nonclosed_line_has_exact_radial_fan_area_and_meridian_moment() {
        let result = integrate(&[[1., 0.], [0., 1.]], &mut || false).unwrap();
        assert_eq!(result[0], ExactRatio::from_finite(0.5));
        assert_eq!(result[1], ExactRatio::new(1.into(), 6.into()));
    }
    #[test]
    fn quadratic_moments_preserve_native_uv_scale_and_translation() {
        // U=1+t,V=t^2 gives U V'-V U'=2t+t^2.
        let result = integrate(&[[1., 0.], [1.5, 0.], [2., 1.]], &mut || false).unwrap();
        assert_eq!(result[0], ExactRatio::new(2.into(), 3.into()));
        assert_eq!(result[1], ExactRatio::new(7.into(), 30.into()));
    }
}
