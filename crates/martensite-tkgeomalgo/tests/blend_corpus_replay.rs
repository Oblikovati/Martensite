//! The context family runs in its own executable so existing global flags cannot race other tests.
#[allow(dead_code)]
#[path = "support/geometry_corpus/mod.rs"]
mod geometry_corpus;

use martensite_tkgeomalgo::app_blend::{get_context_approx_with_no_tgt, get_context_spline_approx};
fn assert_initial_context() {
    assert!(!get_context_approx_with_no_tgt());
    assert!(!get_context_spline_approx());
}
fn replay_blend_request(request: &str, expected: &str) -> usize {
    let mut fields = request.split_whitespace();
    let family = fields.next().unwrap();
    assert!(matches!(family, "ab_context" | "ab_appsurf"));
    let values: Vec<f64> = fields.map(|field| field.parse().unwrap()).collect();
    let actual = geometry_corpus::run_values(family, &values).expect("owned blend family");
    assert!(
        geometry_corpus::policy::equivalent(expected, &actual),
        "{request}: {}",
        geometry_corpus::policy::difference(expected, &actual)
    );
    assert_initial_context();
    usize::from(family == "ab_context")
}
fn assert_frozen_blend_bytes(requests: &'static str, answers: &'static str) {
    assert_eq!(
        geometry_corpus::policy::fixture_bytes_hash(std::iter::once(requests)),
        0xbb45135dab2e2101
    );
    assert_eq!(
        geometry_corpus::policy::fixture_bytes_hash(std::iter::once(answers)),
        0x019f1b0c2c24fcbc
    );
}
#[test]
fn complete_blend_families_in_original_order() {
    assert_initial_context();
    let requests = include_str!("fixtures/geometry_corpus/inputs-blend.txt");
    let answers = include_str!("fixtures/geometry_corpus/reference-blend.txt");
    assert_frozen_blend_bytes(requests, answers);
    assert_eq!(requests.lines().count(), 115);
    assert_eq!(answers.lines().count(), 115);
    let mut counts = [0; 2];
    for (request, expected) in requests.lines().zip(answers.split_inclusive('\n')) {
        counts[replay_blend_request(request, expected)] += 1;
    }
    assert_eq!(counts, [111, 4]);
}
