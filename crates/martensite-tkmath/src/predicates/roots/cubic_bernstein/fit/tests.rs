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

#[test]
fn dyadic_fit_values_keep_the_prior_exact_ratio_rounding() {
    use super::{CubicFitEnd, nearest_binary64, upper};

    let rows = [[0.25, 0.25, 0.25], [0.125, 0.25, 0.25]];
    let operator = CubicFitOperator::new(rows).unwrap();
    let tiny = f64::from_bits(1);
    let far = (1_u64 << 60) as f64;
    for stations in [
        [[-0., 0., 1.], [1., 0., -1.], [-2., 1., 0.], [2., -0., 0.]],
        [
            [far, -far, -0.],
            [far + 1024., 1., tiny],
            [far + 2304., 2., 0.],
            [far + 3072., 4., -tiny],
        ],
        [
            [f64::MAX, 0., -0.],
            [-f64::MAX, 0., 0.],
            [f64::MAX, tiny, -tiny],
            [0., 0., 0.],
        ],
    ] {
        let material = operator.materialize(stations).unwrap();
        let values = stations.map(|point| point.map(ExactRatio::from_finite));
        let mut max_error = ExactRatio::from_finite(0.);
        for (k, row) in rows.into_iter().enumerate() {
            let pole = [0, 1, 2].map(|axis| ratio_affine(&values, row, axis));
            let stored = pole
                .each_ref()
                .map(|value| nearest_binary64(value).unwrap());
            assert_eq!(
                material.poles[k + 1].map(f64::to_bits),
                stored.map(f64::to_bits)
            );
            let axis_error =
                [0, 1, 2].map(|axis| pole[axis].subtract(&ExactRatio::from_finite(stored[axis])));
            max_error = max_error.max(super::l1(axis_error));
            let (end, index) = if k == 0 {
                (CubicFitEnd::Start, 0)
            } else {
                (CubicFitEnd::End, 3)
            };
            let expected = [0, 1, 2].map(|axis| {
                let derivative = pole[axis]
                    .subtract(&values[index][axis])
                    .multiply(&ExactRatio::from_finite(3.));
                nearest_binary64(&derivative).ok_or(CubicFitNumericError::Unbounded)
            });
            let [x, y, z] = expected;
            let expected = x.and_then(|x| y.and_then(|y| z.map(|z| [x, y, z])));
            assert_eq!(operator.endpoint_tangent(end).velocity(stations), expected);
        }
        assert_eq!(material.pole_rounding_upper, upper(&max_error).unwrap());
        assert_eq!(
            material.poles[0].map(f64::to_bits),
            stations[0].map(f64::to_bits)
        );
        assert_eq!(
            material.poles[3].map(f64::to_bits),
            stations[3].map(f64::to_bits)
        );
    }
}

fn ratio_affine(stations: &[[ExactRatio; 3]; 4], row: [f64; 3], axis: usize) -> ExactRatio {
    let origin = &stations[0][axis];
    row.into_iter()
        .zip(&stations[1..])
        .fold(origin.clone(), |sum, (weight, station)| {
            sum.add(&ExactRatio::from_finite(weight).multiply(&station[axis].subtract(origin)))
        })
}
