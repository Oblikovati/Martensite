// SPDX-License-Identifier: GPL-2.0-only
use super::*;
mod norm;
#[test]
fn constant_actual_meridian_shift_contains_independent_unit_speed_distance() {
    with_fixture(|mut pair, source, boundary, domain| {
        pair.pcurves = [&[[0., 0.001], [0., 0.001]]; 2];
        pair.spatial_limits = [0.01; 2];
        let proof =
            certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || false).unwrap();
        for trace in proof.into_traces() {
            assert!(trace.residual_bounds()[2] >= 0.001);
            assert!(trace.residual_bounds()[2] < 0.002);
        }
    });
}
#[test]
fn full_quadratic_and_cubic_uv_have_certified_nonaffine_bounds() {
    with_fixture(|pair, source, boundary, domain| {
        for uv in [
            &[[0., 0.], [1e-7, 2e-7], [0., 0.]][..],
            &[[0., 0.], [1e-7, 2e-7], [-1e-7, -2e-7], [0., 0.]][..],
        ] {
            check_full_uv(pair, source, boundary, domain, uv);
        }
    });
}
fn check_full_uv<'a>(
    mut pair: ConicalTrimPairQuery<'a>,
    source: ConicalBoundaryQuery<'a>,
    boundary: &ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
    uv: &'a [[f64; 2]],
) {
    pair.pcurves = [uv; 2];
    let proof =
        certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || false).unwrap();
    assert!(proof.matches_source(pair));
    assert!(
        proof
            .into_traces()
            .iter()
            .all(|trace| trace.residual_bounds()[2] < 1e-4)
    );
}
#[test]
fn actual_carrier_radius_and_origin_are_measured_not_assumed_identical() {
    with_fixture(|mut pair, source, boundary, domain| {
        let original = pair.carriers[1];
        pair.carriers[1].radius += 1e-6;
        pair.carriers[1].axial_origin -= 2e-6;
        let radial = (pair.carriers[1].radius - original.radius).abs();
        let axial = (pair.carriers[1].axial_origin - original.axial_origin).abs();
        let proof =
            certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || false).unwrap();
        let traces = proof.into_traces();
        // The constant UV lies on +X: actual radial/axial deltas are independent oracles.
        // The previous factor-two assertion pinned an overestimate rather than geometry.
        let [xy, z, _] = traces[1].residual_bounds();
        assert!(xy >= radial && xy < 1.5 * radial);
        assert!(z >= axial && z < axial + 1e-12);
    });
}
#[test]
fn actual_nonpositive_native_radius_refuses_before_publication() {
    with_fixture(|mut pair, source, boundary, domain| {
        pair.pcurves[0] = &[[0., -1000.], [0., -1000.]];
        let failure = certify_conical_trim_pair_in_chart(pair, source, boundary, domain, || false)
            .unwrap_err();
        assert_eq!(failure.depth, Some(0));
        assert!(matches!(
            failure.cause.as_ref(),
            ConicalChartTrimPairFailure::Trace(ConicalTrimError {
                cause: CylindricalProfileFailure::DirectionUnresolved { .. },
                ..
            })
        ));
    });
}
#[test]
fn actual_nonzero_weight_delta_reuses_the_measured_storage_bound() {
    with_fixture(|_, mut source, _, domain| {
        let weights = &[1., 1.000000000000001];
        source.edges[0].stored_weights = weights;
        source.edges[1].stored_weights = weights;
        let boundary =
            certify_conical_boundary_in_chart(source, domain, fixture::options(), || false)
                .unwrap();
        assert!(
            boundary
                .storage()
                .iter()
                .all(|bound| bound.compare_f64(0.).unwrap().is_gt())
        );
        let pair = pair(source);
        let proof =
            certify_conical_trim_pair_in_chart(pair, source, &boundary, domain, || false).unwrap();
        assert!(proof.matches_source(pair));
        assert!(proof.into_traces()[0].residual_bounds()[2] < 1e-4);
    });
}

#[test]
fn actual_half_angle_fit_has_the_independent_endpoint_angle_bound() {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(sector_api::policy(fixture::options()), &mut never);
    let zero = arithmetic.scalar(0.).unwrap();
    let bound = bounds::phase::bound(
        std::slice::from_ref(&zero),
        &[0., 0.125],
        &zero,
        &mut arithmetic,
    )
    .unwrap();
    let upper = arithmetic.upper_f64(bound.upper).unwrap();
    let endpoint = 2. * martensite_cmath::atan(0.125);
    assert!(upper >= endpoint);
    assert!(upper <= 0.251);
}
