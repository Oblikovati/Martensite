// SPDX-License-Identifier: GPL-2.0-only
use super::{
    CubicFitNumericError, CubicFitOperator, ExactCubicBernstein3, ExactRatio, complete_weights,
    cubic_chord_distortion,
};

#[test]
fn dyadic_complement_matches_general_ratio_on_mixed_scales_and_cancellation() {
    for values in [
        [0., -0., 0.],
        [0.25, -0.5, 0.125],
        [f64::MAX, -f64::MAX, f64::from_bits(1)],
        [-f64::from_bits(1), f64::MIN_POSITIVE, 0.75],
    ] {
        for order in 0..3 {
            let weights = values.map(ExactRatio::from_finite);
            let basis_sum = ExactRatio::from_finite(f64::from(order == 0));
            let expected = weights
                .iter()
                .fold(basis_sum, |sum, weight| sum.subtract(weight));
            let [first, _, _, _] = complete_weights(weights, order);
            assert_eq!(first, expected);
        }
    }
}

#[test]
fn exact_affine_origin_and_complement_keep_large_translated_stations() {
    let operator = CubicFitOperator::new([[0.5, 0.25, 0.25], [0.125, 0.5, 0.125]]).unwrap();
    let base = f64::from_bits((1023_u64 + 60) << 52);
    let stations = [0., 256., 512., 1024.].map(|dx| [base + dx, base, -0.]);
    let material = operator.materialize(stations).unwrap();
    let poles = material.poles;
    assert_eq!(material.pole_rounding_upper, 96.);
    assert_eq!(
        poles.map(|pole| pole[0]),
        [base, base + 512., base + 512., base + 1024.]
    );
    assert_eq!(poles.map(|pole| pole[1]), [base; 4]);
    assert_eq!(poles[0][2].to_bits(), (-0_f64).to_bits());
    assert_eq!(poles[3][2].to_bits(), (-0_f64).to_bits());
    assert_eq!(operator.weight_jet(0.).unwrap()[0], [1., 0., 0., 0.]);
    assert_eq!(operator.weight_jet(0.).unwrap()[1], [-3., 1.5, 0.75, 0.75]);
    assert_eq!(operator.weight_jet(1.).unwrap()[0], [0., 0., 0., 1.]);
    assert_eq!(operator.gain_upper().unwrap(), 1.);
}

#[test]
fn exact_station_miss_detects_motion_hidden_by_rounded_d0() {
    let base = f64::from_bits((1023_u64 + 60) << 52);
    let target = [base, -base, 0.];
    let last = [base + 256., -base - 256., 0.];
    let curve = ExactCubicBernstein3::new([target, target, target, last]).unwrap();
    assert_eq!(curve.position(0.5).unwrap(), target);
    assert_eq!(curve.position_miss_upper(0.5, target).unwrap(), 64.);
}

#[test]
fn chord_spread_respects_exact_parameter_intervals_and_zero_chords() {
    let values = [0., 1., 3., 5.].map(|x| [x, 0., 0.]);
    let parameters = [0., 0.25, 0.75, 1.];
    let result = cubic_chord_distortion(values, parameters).unwrap();
    assert_eq!(result.ratio_upper, 2.);
    assert_eq!((result.slowest, result.fastest), (0, 2));
    assert_eq!(
        cubic_chord_distortion(values, [0., 0.25, 0.25, 1.]),
        Err(CubicFitNumericError::StationOrder(1))
    );
    let mut duplicate = values;
    duplicate[1] = duplicate[0];
    assert_eq!(
        cubic_chord_distortion(duplicate, parameters),
        Err(CubicFitNumericError::ZeroChord(0))
    );
}
