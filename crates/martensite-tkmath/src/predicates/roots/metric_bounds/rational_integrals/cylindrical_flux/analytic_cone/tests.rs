// SPDX-License-Identifier: GPL-2.0-only
use super::*;
const UV: [[f64; 2]; 2] = [[0., 0.], [0., 0.]];
const A: [[f64; 3]; 2] = [[0., 0., 1.], [1., 0., 1.]];
const B: [[f64; 3]; 2] = [[0., 1., 1.], [1., 1., 1.]];
const ZERO: [[f64; 3]; 2] = [[0.; 3]; 2];
fn span<'a>(
    a: &'a [[f64; 3]],
    b: &'a [[f64; 3]],
    weights: Option<&'a [f64]>,
) -> AnalyticConicalFluxSpan<'a> {
    AnalyticConicalFluxSpan {
        boundaries: [
            MetricBezier { poles: a, weights },
            MetricBezier { poles: b, weights },
        ],
        cap_pcurves: [&UV, &UV],
        side_reversed: false,
        cap_reversed: [false; 2],
    }
}
fn query<'a>(spans: &'a [AnalyticConicalFluxSpan<'a>]) -> AnalyticConicalFluxQuery<'a> {
    AnalyticConicalFluxQuery {
        spans,
        options: Default::default(),
        arithmetic: Default::default(),
        carriers: [ConicalTrimCarrier {
            radius: 2.,
            axial_origin: 1.,
            semi_angle: 0.5,
        }; 2],
    }
}
fn contains(proof: &CertifiedAnalyticConicalFlux, expected: f64) {
    assert!(
        proof.enclosure()[0] <= expected && proof.enclosure()[1] >= expected,
        "{:?} excludes {expected}",
        proof.enclosure()
    );
}

#[test]
fn planar_side_uses_shared_ruled_integrator_and_preserves_receipts() {
    let spans = [span(&A, &B, None)];
    let input = query(&spans);
    let proof = certify_analytic_conical_boundary_flux(input, || false).unwrap();
    contains(&proof, 1. / 3.);
    assert_eq!(proof.integration_subintervals(), 0);
    assert!(proof.arithmetic_operations() > 0);
    assert_eq!(proof.requested_options(), input.options);
    assert_eq!(proof.effective_options(), input.options);
    assert_eq!(proof.requested_arithmetic(), input.arithmetic);
    assert_eq!(proof.effective_arithmetic(), input.arithmetic);
    assert_eq!(
        proof.evidence(),
        RationalIntegralEvidence::ErrorBoundedIntegration
    );
    assert!(proof.matches_source(input));
}

#[test]
fn nonclosed_cap_keeps_radial_fan_moments_instead_of_changing_green_gauge() {
    // U=1-t,V=t: m0=1/2 and m1=1/6, despite the missing closing fan edges.
    let mut spans = [span(&ZERO, &ZERO, None)];
    spans[0].cap_pcurves[0] = &[[1., 0.], [0., 1.]];
    let proof = certify_analytic_conical_boundary_flux(query(&spans), || false).unwrap();
    let (k, h) = martensite_cmath::sincos(0.5);
    let offset = 2. * h - k;
    let expected = offset * (1. / 3. + k / 18.);
    assert!((proof.representative() - expected).abs() < 1e-14);
    assert!(proof.absolute_error_bound() < 1e-9);
}

#[test]
fn closed_rectangle_cap_matches_independent_density_integral() {
    let edges = [
        [[0., 0.], [1., 0.]],
        [[1., 0.], [1., 2.]],
        [[1., 2.], [0., 2.]],
        [[0., 2.], [0., 0.]],
    ];
    let spans: Vec<_> = edges
        .iter()
        .map(|edge| {
            let mut occurrence = span(&ZERO, &ZERO, None);
            occurrence.cap_pcurves[1] = edge;
            occurrence
        })
        .collect();
    let proof = certify_analytic_conical_boundary_flux(query(&spans), || false).unwrap();
    let (k, h) = martensite_cmath::sincos(0.5);
    let expected = (2. * h - k) * (4. + 2. * k) / 3.;
    assert!((proof.representative() - expected).abs() < 1e-14);
}

#[test]
fn actual_cap_reversal_and_side_reversal_negate_the_whole_flux() {
    let mut spans = [span(&A, &B, None)];
    spans[0].cap_pcurves[0] = &[[1., 0.], [0., 1.]];
    let positive = certify_analytic_conical_boundary_flux(query(&spans), || false).unwrap();
    spans[0].side_reversed = true;
    spans[0].cap_reversed = [true; 2];
    let negative = certify_analytic_conical_boundary_flux(query(&spans), || false).unwrap();
    assert_eq!(
        negative.enclosure(),
        [-positive.enclosure()[1], -positive.enclosure()[0]]
    );
}

#[test]
fn rational_side_and_aggregate_cells_use_existing_work_limits() {
    let spans = [span(&A, &B, Some(&[1., 2.])); 2];
    let complete = certify_analytic_conical_boundary_flux(query(&spans), || false).unwrap();
    assert!(complete.integration_subintervals() > 0);
    let mut input = query(&spans);
    input.options.max_total_subintervals = complete.integration_subintervals() as u32 - 1;
    let failure = certify_analytic_conical_boundary_flux(input, || false).unwrap_err();
    assert!(matches!(
        failure.cause,
        AnalyticConicalFluxFailure::Integration { span: Some(1), .. }
    ));
}

mod refusal;
mod source;

mod oracle;

mod accumulation;
