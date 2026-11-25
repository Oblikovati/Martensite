// SPDX-License-Identifier: GPL-2.0-only
//! Actual-axis displacement separates geometric error from the old norm overestimate.
use super::*;
#[test]
fn actual_radial_shift_completes_under_the_unchanged_independent_allowance() {
    with_fixture(|mut pair, source, boundary, domain| {
        let delta = 1. / 1024.;
        let original = pair.carriers;
        for carrier in &mut pair.carriers {
            carrier.radius += delta;
        }
        let limit = 1.5 * delta;
        pair.spatial_limits = [limit; 2];
        assert!(2. * delta > limit);
        let proof =
            certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || false).unwrap();
        assert!(proof.matches_source(pair));
        check_actual_axis_delta(proof, pair, original, delta, limit);
    });
}
fn check_actual_axis_delta(
    proof: CertifiedConicalTrimPair,
    pair: ConicalTrimPairQuery<'_>,
    original: [ConicalTrimCarrier; 2],
    delta: f64,
    limit: f64,
) {
    for (depth, trace) in proof.into_traces().into_iter().enumerate() {
        let actual_x_delta = pair.carriers[depth].radius - original[depth].radius;
        assert_eq!(actual_x_delta, delta);
        assert_eq!(
            pair.carriers[depth].axial_origin,
            original[depth].axial_origin
        );
        assert!(trace.residual_bounds()[0] >= actual_x_delta);
        assert!(trace.residual_bounds()[0] < limit);
        assert!(trace.residual_bounds()[2] < limit);
    }
}
