// SPDX-License-Identifier: GPL-2.0-only
use super::{CubicBernsteinFailure, ExactCubicBernstein3};

#[test]
fn once_rounded_rows_choose_even_adjacent_binary64_in_both_signs() {
    let tiny = f64::from_bits(1);
    let curve = ExactCubicBernstein3::new([
        [0., -0., f64::MAX],
        [0., -0., f64::MAX],
        [tiny, -tiny, f64::MAX],
        [tiny, -tiny, f64::MAX],
    ])
    .unwrap();
    let jet = curve.jet(0.5).unwrap();
    assert_eq!(
        jet.position.map(f64::to_bits),
        [0., -0., f64::MAX].map(f64::to_bits)
    );
    assert_eq!(jet.first, [2. * tiny, -2. * tiny, 0.]);
    assert_eq!(jet.second, [0.; 3]);

    let odd = 1_f64.next_up();
    let even = odd.next_up();
    let normals = ExactCubicBernstein3::new([
        [1., odd, -odd],
        [1., odd, -odd],
        [odd, even, -even],
        [odd, even, -even],
    ])
    .unwrap();
    assert_eq!(normals.position(0.5).unwrap(), [1., even, -even]);
}

#[test]
fn smallest_parameter_retains_effect_and_unrequested_overflow_stays_independent() {
    let far = f64::from_bits(2046_u64 << 52);
    let t = f64::from_bits(1);
    let poles = [[0.; 3], [far, -far, 0.], [0.; 3], [1., 0., 0.]];
    let curve = ExactCubicBernstein3::new(poles).unwrap();
    let product = (far * t) * 3.;
    assert_eq!(curve.position(t).unwrap(), [product, -product, 0.]);
    assert_eq!(
        curve.first_derivative(t),
        Err(CubicBernsteinFailure::Unrepresentable {
            order: 1,
            axis: 0,
            negative: false
        })
    );
    assert_eq!(
        curve.jet(t),
        Err(CubicBernsteinFailure::Unrepresentable {
            order: 1,
            axis: 0,
            negative: false
        })
    );
}

#[test]
fn three_jet_rows_match_independent_integer_bernstein_weights() {
    let coefficients = [[-7, 11, 5], [2, -4, 0], [-3, 6, 7], [9, -1, -2]];
    let curve = ExactCubicBernstein3::new(coefficients.map(|pole| pole.map(f64::from))).unwrap();
    for right in 0..=16 {
        let left = 16 - right;
        let squared = [left * left, left * right, right * right];
        let jet = curve.jet(f64::from(right) / 16.).unwrap();
        for axis in 0..3 {
            let [a, b, c, d] = coefficients.map(|pole| pole[axis]);
            let numerator = a * squared[0] * left
                + 3 * b * squared[1] * left
                + 3 * c * squared[1] * right
                + d * squared[2] * right;
            let speed =
                3 * ((b - a) * squared[0] + 2 * (c - b) * squared[1] + (d - c) * squared[2]);
            let acceleration = 6 * ((a - 2 * b + c) * left + (b - 2 * c + d) * right);
            assert_eq!(jet.position[axis], f64::from(numerator) / 4096.);
            assert_eq!(jet.first[axis], f64::from(speed) / 256.);
            assert_eq!(jet.second[axis], f64::from(acceleration) / 16.);
        }
    }
}

#[test]
fn cubic_derivative_beyond_max_can_still_round_to_a_finite_value() {
    let unit = f64::from_bits((1023_u64 + 968) << 52);
    let high = 6_004_799_503_160_660_f64 * (4. * unit);
    let curve = ExactCubicBernstein3::new([
        [-3. * unit, 3. * unit, 0.],
        [high, -high, 0.],
        [0.; 3],
        [1., 1., 1.],
    ])
    .unwrap();
    // The first derivative is ±(MAX + unit), one quarter of the way to the tie.
    assert_eq!(
        curve.first_derivative(0.).unwrap(),
        [f64::MAX, -f64::MAX, 0.]
    );
    let tie =
        ExactCubicBernstein3::new([[0.; 3], [high.next_up(), 0., 0.], [0.; 3], [1.; 3]]).unwrap();
    assert_eq!(
        tie.first_derivative(0.),
        Err(CubicBernsteinFailure::Unrepresentable {
            order: 1,
            axis: 0,
            negative: false
        })
    );
}

#[test]
fn exact_capture_and_parameter_bounds_refuse_invalid_input() {
    let mut poles = [[0.; 3]; 4];
    poles[2][1] = f64::INFINITY;
    let invalid = ExactCubicBernstein3::new(poles).unwrap_err();
    assert_eq!(
        (invalid.point, invalid.axis, invalid.value),
        (2, 1, f64::INFINITY)
    );
    let curve = ExactCubicBernstein3::new([[0.; 3]; 4]).unwrap();
    assert!(
        matches!(curve.position(f64::NAN), Err(CubicBernsteinFailure::Parameter(x)) if x.is_nan())
    );
    assert_eq!(
        curve.first_derivative(1_f64.next_up()),
        Err(CubicBernsteinFailure::Parameter(1_f64.next_up()))
    );
    assert_eq!(
        curve.jet(-f64::from_bits(1)),
        Err(CubicBernsteinFailure::Parameter(-f64::from_bits(1)))
    );
}
