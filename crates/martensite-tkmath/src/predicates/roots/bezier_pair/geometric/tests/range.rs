// SPDX-License-Identifier: GPL-2.0-only
use super::*;
#[test]
fn finite_underflow_second_intersection_never_becomes_endpoint_only() {
    let h = f64::from_bits(423_u64 << 52);
    let p = [0., 0.];
    let v = [h, h / 2.];
    let query = pair(&[p, v, v, [h, h]], &[p, p, p, [1., 0.75]]);
    // Ay-.75Ax=(h/4)t(t²+3t-3) changes sign at interior dyadic witnesses.
    let polynomial = |t: f64| t * t + 3. * t - 3.;
    assert!(polynomial(0.5) < 0.);
    assert!(polynomial(1.) > 0.);
    let report = query.certify_geometric(policy(), || false);
    assert_eq!(
        report.completion(),
        BezierPairGeometricCompletion::Inconclusive
    );
    underflow_cross(p, h, v);
}
#[test]
fn exact_positive_hairpins_survive_extreme_scaling_and_zero_signs() {
    for scale in [
        f64::from_bits(423_u64 << 52),
        1.,
        f64::from_bits(2045_u64 << 52),
    ] {
        let report = hairpin(scale).certify_geometric(policy(), || false);
        assert_eq!(
            report.complete().unwrap().kind(),
            BezierPairGeometricKind::EndpointSector
        );
    }
}
#[test]
fn differing_origin_determinants_never_round_subtractions_or_overflow() {
    let m = f64::from_bits(2046_u64 << 52);
    let a = Vector {
        from: [m, m],
        to: [-m, m],
    };
    let b = Vector {
        from: [m, -m],
        to: [m, m],
    };
    assert_eq!(
        predicates::cross(&a, &b).unwrap(),
        OrientationSign::Negative
    );
    common_origin_extreme(m);
}

fn underflow_cross(p: Point, h: f64, v: Point) {
    let a = Vector {
        from: p,
        to: [h, h],
    };
    let b = Vector { from: p, to: v };
    assert_eq!(
        predicates::cross(&a, &b).unwrap(),
        OrientationSign::Negative
    );
}

fn common_origin_extreme(m: f64) {
    let p = [m, m];
    let a = Vector {
        from: p,
        to: [-m, m],
    };
    let b = Vector {
        from: p,
        to: [m, -m],
    };
    assert_eq!(
        predicates::cross(&a, &b).unwrap(),
        OrientationSign::Positive
    );
}
