// SPDX-License-Identifier: GPL-2.0-only
//! Geometric cardinal admission differs from immutable bit-exact source authority.
use super::*;
#[test]
fn signed_zero_cardinal_axes_admit_but_remain_in_the_issued_source() {
    fixture(|query| {
        let mut caps = query.caps.to_vec();
        caps[0].frame[1][2] = -0.;
        caps[0].frame[2][0] = -0.;
        let signed = ConicalDirectedGammaQuery {
            caps: &caps,
            ..query
        };
        let proof = certify_conical_directed_gamma(signed, || false).unwrap();
        assert!(proof.matches(&signed));
        assert!(!proof.matches(&query));
        let positive = certify_conical_directed_gamma(query, || false).unwrap();
        assert_eq!(proof.gamma(), positive.gamma());
        assert_eq!(proof.operations(), positive.operations());
        assert!(!proof.same_authority(&positive));
    });
}
#[test]
fn nonzero_cardinal_axis_component_refuses_without_polling_or_charging() {
    fixture(|query| {
        let mut caps = query.caps.to_vec();
        caps[0].frame[1][2] = f64::MIN_POSITIVE;
        let changed = ConicalDirectedGammaQuery {
            caps: &caps,
            ..query
        };
        let mut polls = 0;
        let error = certify_conical_directed_gamma(changed, || {
            polls += 1;
            true
        })
        .unwrap_err();
        assert_eq!(polls, 0);
        assert_eq!(error.operations, 0);
        assert_eq!(error.cap, Some(ConicalActualCapId(10)));
        assert_noncardinal(error);
    });
}
fn assert_noncardinal(error: ConicalActualPatchError) {
    if let ConicalActualPatchFailure::Binding {
        field,
        actual,
        expected,
    } = error.cause.as_ref()
    {
        assert_eq!(*field, "cardinal cone frame");
        assert_eq!(actual[5], f64::MIN_POSITIVE.to_bits());
        assert_eq!(expected[5], 0.0_f64.to_bits());
        assert_ne!(actual, expected);
    } else {
        panic!("expected complete cardinal-frame operands: {error:?}");
    }
}
