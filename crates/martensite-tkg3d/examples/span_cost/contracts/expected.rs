//! Independent quotient derivatives and piecewise-linear periodic values precede any cache change.
use super::curves::DIAMOND;
const ORDERS: [usize; 10] = [0, 0, 1, 0, 1, 2, 0, 1, 2, 3];

/// Differentiate each linear rational segment analytically; `rational_orders(0.25)`.
pub(super) fn rational_orders(parameter: f64) -> [f64; 4] {
    let first = parameter < 0.5;
    let denominator = if first {
        1. + 2. * parameter
    } else {
        3. - 2. * parameter
    };
    let value = if first {
        4. * parameter / denominator
    } else {
        2. / denominator
    };
    let second_sign = if first { -1. } else { 1. };
    [
        value,
        4. / denominator.powi(2),
        second_sign * 16. / denominator.powi(3),
        96. / denominator.powi(4),
    ]
}

/// Preserve the last-boundary left derivative and periodic extension; `diamond_orders(4.)`.
pub(super) fn diamond_orders(parameter: f64) -> [[f64; 2]; 4] {
    let normalized = if parameter == 4. {
        4.
    } else {
        parameter.rem_euclid(4.)
    };
    let index = (normalized.floor() as usize).min(3);
    let fraction = normalized - index as f64;
    let (first, next) = (DIAMOND[index], DIAMOND[(index + 1) % 4]);
    let value = std::array::from_fn(|axis| first[axis] + fraction * (next[axis] - first[axis]));
    let derivative = std::array::from_fn(|axis| next[axis] - first[axis]);
    [value, derivative, [0.; 2], [0.; 2]]
}

/// Flatten the repeated D0..D3 point and derivative rows; `repeated_orders(values)`.
pub(super) fn repeated_orders<const D: usize>(orders: [[f64; D]; 4]) -> Vec<f64> {
    let mut values: Vec<f64> = Vec::with_capacity(ORDERS.len() * D);
    for order in ORDERS {
        values.extend_from_slice(&orders[order]);
    }
    values
}

/// Check all coordinates before publishing their unchanged bits; `checked_bits(label,u,actual,expected)`.
pub(super) fn checked_bits(label: &str, parameter: f64, actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (index, (observed, expected)) in actual.iter().zip(expected).enumerate() {
        assert!(
            (observed - expected).abs() <= 1e-11,
            "{label} u{parameter} field{index}: got{observed}, expected{expected} within1e-11"
        );
    }
    let bits: Vec<u64> = actual.iter().map(|value| value.to_bits()).collect();
    println!("{{\"owned_case\":\"{label}\",\"parameter\":{parameter},\"bits\":{bits:?}}}");
}
