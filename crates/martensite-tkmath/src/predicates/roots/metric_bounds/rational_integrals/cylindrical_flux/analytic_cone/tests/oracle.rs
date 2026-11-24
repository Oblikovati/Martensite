// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use num_bigint::BigInt;

// Independent alternating series at exact alpha=1/2: the adjacent partial sums
// bound sin/cos without using the production reduction, recurrence or grid arithmetic.
fn trig_bounds(parity: usize) -> [ExactRatio; 2] {
    let mut sum = ExactRatio::from_finite(0.);
    let mut lower = sum.clone();
    for term in 0..=16 {
        let power = 2 * term + parity;
        let factorial = (1..=power).fold(BigInt::from(1), |product, n| product * n);
        let value = ExactRatio::new(BigInt::from(1), factorial << power);
        sum = if term % 2 == 0 {
            sum.add(&value)
        } else {
            sum.subtract(&value)
        };
        if term == 15 {
            lower = sum.clone();
        }
    }
    [lower, sum]
}

#[test]
fn actual_nonclosed_cap_encloses_independent_exact_taylor_oracle() {
    let mut spans = [span(&ZERO, &ZERO, None)];
    spans[0].cap_pcurves[0] = &[[1., 0.], [0., 1.]];
    let proof = certify_analytic_conical_boundary_flux(query(&spans), || false).unwrap();
    let expected = nonclosed_cap_bounds();
    assert!(ExactRatio::from_finite(proof.enclosure()[0]) <= expected[0]);
    assert!(ExactRatio::from_finite(proof.enclosure()[1]) >= expected[1]);
}

fn nonclosed_cap_bounds() -> [ExactRatio; 2] {
    let sine = trig_bounds(1);
    let cosine = trig_bounds(0);
    let two = ExactRatio::from_finite(2.);
    let third = ExactRatio::new(1.into(), 3.into());
    let eighteenth = ExactRatio::new(1.into(), 18.into());
    let lower = cosine[0].multiply(&two).subtract(&sine[1]);
    let upper = cosine[1].multiply(&two).subtract(&sine[0]);
    [
        lower.multiply(&third.add(&sine[0].multiply(&eighteenth))),
        upper.multiply(&third.add(&sine[1].multiply(&eighteenth))),
    ]
}
