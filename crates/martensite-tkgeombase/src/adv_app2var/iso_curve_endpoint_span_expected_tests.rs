//! Independent affine function and C1 Hermite polynomial arithmetic.
use super::*;
pub(super) fn domain(iso: i32) -> [f64; 4] {
    if iso == 1 {
        return [0., 4., 0., 2.];
    }
    [0., 2., 0., 4.]
}
pub(super) fn check_callback(call: &mut Call<'_, '_, '_>, count: usize, iso: i32) {
    let orders = if count == 1 {
        (0, 0)
    } else if iso == 1 {
        (0, 1)
    } else {
        (1, 0)
    };
    assert_eq!(
        (
            call.integer(AdvApp2VarIntegerField::UOrder),
            call.integer(AdvApp2VarIntegerField::VOrder)
        ),
        orders
    );
    check_callback_parameters(call, count);
}
fn check_callback_parameters(call: &mut Call<'_, '_, '_>, count: usize) {
    let parameters = callback_parameters(count);
    assert_eq!(
        call.integer(AdvApp2VarIntegerField::ParameterCount) as usize,
        parameters.len()
    );
    for (index, expected) in parameters.iter().enumerate() {
        assert_eq!(
            call.buffers().parameter(index).to_bits(),
            expected.to_bits()
        );
    }
}
fn callback_parameters(count: usize) -> Vec<f64> {
    if count == 1 {
        return [
            vec![0.],
            fixture::ROOTS.iter().map(|root| root + 1.).collect(),
            vec![2.],
        ]
        .concat();
    }
    vec![if count == 2 { 0. } else { 2. }]
}

fn polynomial(coefficients: [f64; 4], value: f64) -> f64 {
    let mut result = coefficients[3];
    for coefficient in coefficients[..3].iter().rev() {
        result = result * value + coefficient;
    }
    result
}
fn folded_expected() -> ([f64; 5], [f64; 5]) {
    let samples = fixture::ROOTS.map(|root| 0.5 + (root + 1.));
    let mut sums = [0.; 5];
    let mut differences = [0.; 5];
    for pair in 1..5 {
        sums[pair] = samples[3 + pair] + samples[4 - pair];
        differences[pair] = samples[3 + pair] - samples[4 - pair];
    }
    subtract_hermite(&mut sums, &mut differences);
    (sums, differences)
}
fn subtract_hermite(sums: &mut [f64; 5], differences: &mut [f64; 5]) {
    let bases = [
        [0.5, -0.75, 0., 0.25],
        [0.5, 0.75, 0., -0.25],
        [0.25, -0.25, -0.25, 0.25],
        [-0.25, -0.25, 0.25, 0.25],
    ];
    for (order, constraints) in [(0.5, 2.5), (1., 1.)].into_iter().enumerate() {
        let pair = [bases[2 * order], bases[2 * order + 1]];
        subtract_hermite_order(sums, differences, pair, constraints);
    }
}
fn subtract_hermite_order(
    sums: &mut [f64; 5],
    differences: &mut [f64; 5],
    basis: [[f64; 4]; 2],
    constraints: (f64, f64),
) {
    for (side, sign) in [(0, 1.), (1, -1.)] {
        for pair in 1..5 {
            let root = fixture::ROOTS[if side == 0 { 4 - pair } else { 3 + pair }];
            let correction = constraints.0 * polynomial(basis[0], root)
                + constraints.1 * polynomial(basis[1], root);
            sums[pair] -= correction;
            differences[pair] += sign * correction;
        }
    }
}
fn expected_owner(iso: i32) -> Vec<f64> {
    let mut expected = fixture::real_owner();
    expected[16..20].copy_from_slice(&domain(iso));
    expected[64..69].fill(0.);
    let (sums, differences) = folded_expected();
    expected[192..197].copy_from_slice(&sums);
    expected[256..261].copy_from_slice(&differences);
    expected[320..322].copy_from_slice(&[0.5, 1.]);
    expected[384..386].copy_from_slice(&[2.5, 1.]);
    expected[448..450].copy_from_slice(&[0., 2.]);
    expected
}
pub(super) fn check_owner(frame: &mut AdvApp2VarIsoOperationFrame, iso: i32) {
    let expected = expected_owner(iso);
    let mut owner = frame.caller_scalar_curve_slots(0, 1100).unwrap();
    owner.with_curve_values(|values| {
        for (index, (actual, expected)) in values.iter().zip(expected).enumerate() {
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "caller real slot={index}"
            );
        }
    });
}
