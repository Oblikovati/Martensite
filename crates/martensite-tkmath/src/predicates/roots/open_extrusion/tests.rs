//! Independent finite-domain constructions and complete refusal accounting.
use super::*;
mod fixtures;
mod refusal;
mod replay;
mod represented_profiles;
fn run(input: OpenQuadraticExtrusionInput) -> OpenExtrusionReport {
    certify_open_quadratic_extrusion(input, OpenExtrusionPolicy::bounded(), || false)
}
#[test]
fn proper_parabola_certifies_the_complete_parameter_rectangle() {
    let report = run(fixtures::parabola());
    assert!(report.certificate().is_some(), "{:?}", report.completion());
    assert_eq!(report.work().scheduled_obligations, 1);
    assert_eq!(report.work().finished_obligations, 1);
    assert_eq!(report.work().root_subdivisions, 0);
    assert_eq!(
        report.certificate().unwrap().input().curve_interval(),
        [0., 1.]
    );
}
#[test]
fn tiny_real_endpoint_gap_is_not_snapped_or_cyclically_identified() {
    let input = fixtures::triangle(f64::from_bits(1f64.to_bits() - 1));
    let last_bits = input.poles().last().unwrap()[0].to_bits();
    let report = run(input);
    assert!(report.certificate().is_some(), "{:?}", report.completion());
    assert_eq!(report.work().finished_obligations, 6);
    assert_eq!(
        report.input().poles().last().unwrap()[0].to_bits(),
        last_bits
    );
    assert_ne!(last_bits, report.input().poles()[0][0].to_bits());
}
#[test]
fn coincident_endpoints_do_not_receive_a_cyclic_contact_permission() {
    let input = fixtures::triangle(1.);
    assert_closed_counterpart_certifies(&input);
    let report = run(input);
    assert!(matches!(
        report.completion(),
        OpenExtrusionCompletion::Incomplete(OpenExtrusionCause::UnseparatedPair {
            first: 0,
            second: 2
        })
    ));
    assert!(report.certificate().is_none());
    assert_eq!(report.work().scheduled_obligations, 6);
    assert_eq!(report.work().finished_obligations, 4);
}
fn assert_closed_counterpart_certifies(input: &OpenQuadraticExtrusionInput) {
    use super::super::closed_extrusion::*;
    let report = certify_closed_quadratic_extrusion(
        fixtures::as_closed(input),
        ClosedExtrusionPolicy::bounded(),
        || false,
    );
    assert!(report.certificate().is_some(), "{:?}", report.completion());
}

#[test]
fn source_arrays_and_unnormalized_direction_remain_owned() {
    let mut source = vec![[0., 0., -0.], [1., 1., 0.], [2., 0., 0.]];
    let input = fixtures::directed(source.clone(), [-0., 0., 2.]);
    source[0] = [99.; 3];
    let report = run(input);
    assert_eq!(report.input().poles()[0][2].to_bits(), (-0f64).to_bits());
    assert_eq!(report.input().weights(), &[1.; 3]);
    assert_eq!(report.input().knots(), &[0., 1.]);
    assert_eq!(report.input().multiplicities(), &[3, 3]);
    assert_eq!(report.input().direction()[0].to_bits(), (-0f64).to_bits());
    assert_eq!(report.input().direction()[2], 2.);
    assert_eq!(report.input().extrusion_interval(), [-1., 1.]);
    assert!(report.certificate().is_some());
}
#[test]
fn published_types_are_send_and_sync() {
    fn both<T: Send + Sync>() {}
    both::<OpenQuadraticExtrusionInput>();
    both::<OpenExtrusionInputError>();
    both::<OpenExtrusionPolicy>();
    both::<OpenExtrusionCompletion>();
    both::<OpenExtrusionCause>();
    both::<OpenExtrusionProjectionContact>();
    both::<OpenExtrusionWork>();
    both::<OpenExtrusionStage>();
    both::<OpenExtrusionReport>();
    both::<OpenExtrusionCertificate<'static>>();
}

#[test]
fn finite_planar_profiles_keep_their_real_endpoint_gaps() {
    for input in [
        represented_profiles::offset_profile(),
        represented_profiles::origin_profile(),
    ] {
        let original = input.poles().to_vec();
        assert_ne!(
            original.first().unwrap()[0].to_bits(),
            original.last().unwrap()[0].to_bits()
        );
        let report = run(input);
        assert!(report.certificate().is_some(), "{:?}", report.completion());
        assert_eq!(report.work().scheduled_obligations, 136);
        assert_eq!(report.work().finished_obligations, 136);
        assert_eq!(
            coordinate_bits(report.input().poles()),
            coordinate_bits(&original)
        );
    }
}

fn coordinate_bits(points: &[[f64; 3]]) -> Vec<[u64; 3]> {
    points.iter().map(|point| point.map(f64::to_bits)).collect()
}
