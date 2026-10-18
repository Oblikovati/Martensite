//! Closed-form Hermite columns and immutable quadrature weights form the expected polynomial.
use super::ROOTS8;
const C0_WEIGHTS: [[f64; 4]; 2] = [
    [
        0.33935089976056215,
        0.2198555068838821,
        0.07866106707857388,
        0.007629750644884637,
    ],
    [
        0.16469460868253652,
        0.3056932664759789,
        0.16580033229089916,
        0.019384817026729877,
    ],
];
const C1_WEIGHTS: [f64; 4] = [
    0.37569368093723404,
    0.18231189064285402,
    0.032921979398829085,
    0.0006804273310444661,
];
const HERMITE: [[f64; 4]; 4] = [
    [0.5, -0.75, 0., 0.25],
    [0.5, 0.75, 0., -0.25],
    [0.25, -0.25, -0.25, 0.25],
    [-0.25, -0.25, 0.25, 0.25],
];
fn horner(coefficients: &[f64; 4], parameter: f64) -> f64 {
    coefficients[..3]
        .iter()
        .rev()
        .fold(coefficients[3], |sum, coefficient| {
            sum * parameter + coefficient
        })
}
pub(super) fn c0_curve(shared: bool) -> [f64; 4] {
    let (left, right) = (if shared { 1. } else { -1. }, 1.);
    let (mut even, mut odd) = (0., 0.);
    for (index, root) in ROOTS8[4..].iter().enumerate() {
        let negative = left * (0.5 + 0.5 * root) + right * (0.5 - 0.5 * root);
        let positive = left * (0.5 - 0.5 * root) + right * (0.5 + 0.5 * root);
        even += ((0. - negative) - positive) * C0_WEIGHTS[0][index];
        odd += (((root - (-root)) + negative) - positive) * C0_WEIGHTS[1][index];
    }
    if !shared {
        assert!(even.abs() + odd.abs() * 3. < 1e-6);
        even = 0.;
        odd = 0.;
    }
    let constant = (0. + left * 0.5) + right * 0.5;
    let linear = (0. + left * (-0.5)) + right * 0.5;
    [
        (0. + constant) + even * 0.9682458365518543,
        (0. + linear) + odd * 2.5617376914898995,
        0. + even * (-0.9682458365518543),
        0. + odd * (-2.5617376914898995),
    ]
}
fn c1_free_coefficient(shared: bool) -> f64 {
    let constraints = [1., 1., if shared { 2. } else { -2. }, 2.];
    let mut projection = 0.;
    for (root, weight) in ROOTS8[4..].iter().zip(C1_WEIGHTS) {
        let mut folded = root * root + (-root) * (-root);
        for order in [0, 2] {
            for sample in [-root, *root] {
                folded -= constraints[order] * horner(&HERMITE[order], sample)
                    + constraints[order + 1] * horner(&HERMITE[order + 1], sample);
            }
        }
        projection += folded * weight;
    }
    projection
}
pub(super) fn c1_curve_and_errors(shared: bool) -> ([f64; 5], [f64; 2]) {
    let mut free = c1_free_coefficient(shared);
    let bound = free.abs() * 1.109264959331178;
    let errors = if shared {
        [0., 0.]
    } else {
        [(bound * bound).sqrt(), ((free * free) / 2.).sqrt()]
    };
    if !shared {
        assert!(bound < 1e-6);
        free = 0.;
    }
    let constraints = if shared {
        [1., -1., 0., 1.]
    } else {
        [0., 0., 1., 0.]
    };
    (
        [
            (0. + constraints[0]) + free * 1.109264959331178,
            0. + constraints[1],
            (0. + constraints[2]) + free * (-2.218529918662356),
            0. + constraints[3],
            0. + free * 1.109264959331178,
        ],
        errors,
    )
}
