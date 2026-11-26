// SPDX-License-Identifier: GPL-2.0-only
//! Exactly interpolate the three original stations at 0, 1/2 and 1.
use super::{
    CubicBernsteinFailure, CubicChordDistortion, CubicFitNumericError as Error, ExactRatio as R,
    chord_distortion, exact_point, l1, nearest_binary64, squared_speed, upper,
};

/// The actual rounded quadratic and certified errors from its three-station affine fit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuadraticFitMaterialization {
    /// The three retained Bernstein controls. The two endpoint stations retain their bits.
    pub poles: [[f64; 3]; 3],
    /// Outward world L1 pole materialization bound, valid across the span.
    pub pole_rounding_upper: f64,
    /// Outward world L1 error of the actual curve at the literal middle authored station.
    pub midpoint_miss_upper: f64,
}

/// Compute one exact affine quadratic at the fixed parameters 0, 1/2 and 1.
/// Example: `materialize_fit_quadratic(stations)?.poles`.
pub fn materialize_fit_quadratic(
    stations: [[f64; 3]; 3],
) -> Result<QuadraticFitMaterialization, Error> {
    let [first, middle, last] = exact_three(stations)?;
    let half = R::from_finite(0.5);
    let exact: [R; 3] = std::array::from_fn(|axis| {
        let twice_middle = middle[axis].add(&middle[axis]);
        twice_middle.subtract(&first[axis].add(&last[axis]).multiply(&half))
    });
    let rounded = round(exact.each_ref(), 1)?;
    let residual = l1(std::array::from_fn::<_, 3, _>(|axis| {
        exact[axis].subtract(&R::from_finite(rounded[axis]))
    }));
    // The endpoint controls are exact and the middle Bernstein basis is 1/2 at t=1/2.
    Ok(QuadraticFitMaterialization {
        poles: [stations[0], rounded, stations[2]],
        pole_rounding_upper: upper(&residual)?,
        midpoint_miss_upper: upper(&residual.multiply(&half))?,
    })
}

/// Read the actual unweighted quadratic at one finite normalized parameter.
/// Example: `evaluate_fit_quadratic(poles, 0.5)?`.
pub fn evaluate_fit_quadratic(poles: [[f64; 3]; 3], t: f64) -> Result<[f64; 3], Error> {
    let exact = exact_three(poles)?;
    if !(0. ..=1.).contains(&t) {
        return Err(Error::Parameter(CubicBernsteinFailure::Parameter(t)));
    }
    if t == 0. || t == 1. {
        return Ok(poles[usize::from(t == 1.) * 2]);
    }
    let t = R::from_finite(t);
    let s = R::from_finite(1.).subtract(&t);
    let w = [
        s.multiply(&s),
        s.multiply(&t).multiply(&R::from_finite(2.)),
        t.multiply(&t),
    ];
    let evaluated: [R; 3] = std::array::from_fn(|i| {
        w[0].multiply(&exact[0][i])
            .add(&w[1].multiply(&exact[1][i]))
            .add(&w[2].multiply(&exact[2][i]))
    });
    round(evaluated.each_ref(), 0)
}

/// Bound the two adjacent chord speeds at the three fixed authored parameters.
/// Example: `quadratic_fit_chord_distortion(stations)?.ratio_upper`.
pub fn quadratic_fit_chord_distortion(
    stations: [[f64; 3]; 3],
) -> Result<CubicChordDistortion, Error> {
    let stations = exact_three(stations)?;
    let parameters = [0., 0.5, 1.].map(R::from_finite);
    let [a, b] = [0, 1].map(|i| squared_speed(&stations, &parameters, i));
    chord_distortion(&[a?, b?])
}

fn exact_three(stations: [[f64; 3]; 3]) -> Result<[[R; 3]; 3], Error> {
    let [a, b, c] = [0, 1, 2].map(|i| exact_point(stations[i], i));
    Ok([a?, b?, c?])
}

fn round(coordinates: [&R; 3], pole: usize) -> Result<[f64; 3], Error> {
    let [a, b, c] = [0, 1, 2]
        .map(|axis| nearest_binary64(coordinates[axis]).ok_or(Error::PoleRange { pole, axis }));
    Ok([a?, b?, c?])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn midpoint_interpolation_owns_two_literal_endpoints_and_a_derived_control() {
        let stations = [[-3., 2.75, -0.], [0., 1.25, 0.], [3., 2.75, 0.]];
        let pair = materialize_fit_quadratic(stations).unwrap();
        assert_eq!(pair.poles[1], [0., -0.25, 0.]);
        assert_eq!(pair.poles[0][2].to_bits(), (-0_f64).to_bits());
        assert_eq!(pair.pole_rounding_upper, 0.);
        assert_eq!(pair.midpoint_miss_upper, 0.);
        assert_eq!(
            evaluate_fit_quadratic(pair.poles, 0.5).unwrap(),
            stations[1]
        );
        assert_eq!(
            quadratic_fit_chord_distortion(stations)
                .unwrap()
                .ratio_upper,
            1.
        );
    }

    #[test]
    fn one_thin_midpoint_bit_retains_its_actual_rounding_residual() {
        let stations = [
            [0., 0., 0.],
            [1., 1.0_f64.next_up(), 0.],
            [2., f64::EPSILON, 0.],
        ];
        let result = materialize_fit_quadratic(stations).unwrap();
        assert!(result.pole_rounding_upper > 0.);
        assert_eq!(result.midpoint_miss_upper, result.pole_rounding_upper * 0.5);
    }
}
