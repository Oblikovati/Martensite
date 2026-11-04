//! Invalid storage, projected contacts and unfinished proof are distinct outcomes.
use super::*;
#[test]
fn known_crossing_never_becomes_a_support_certificate() {
    let report = run(fixtures::crossing());
    assert!(matches!(
        report.completion(),
        OpenExtrusionCompletion::Incomplete(OpenExtrusionCause::UnseparatedPair { .. })
    ));
    assert!(report.certificate().is_none());
}
#[test]
fn collapsed_and_backtracking_spans_retain_their_projected_contact() {
    let collapsed = fixtures::capture(vec![[0.; 3]; 3], vec![0., 1.], vec![3, 3]);
    let backtracking = fixtures::capture(
        vec![[0., 0., 0.], [2., 0., 0.], [1., 0., 0.]],
        vec![0., 1.],
        vec![3, 3],
    );
    for input in [collapsed, backtracking] {
        let report = run(input);
        assert!(matches!(
            report.completion(),
            OpenExtrusionCompletion::NotExcluded(_)
        ));
        assert!(report.certificate().is_none());
    }
}
#[test]
fn nonpositive_storage_and_nonconstant_positive_weights_are_not_conflated() {
    let invalid = fixtures::weighted(vec![-1., 1., 1.]).unwrap_err();
    assert!(matches!(
        invalid.cause(),
        super::super::super::closed_extrusion::ClosedExtrusionInputError::Numeric {
            field: "weights",
            index: 0,
            ..
        }
    ));
    assert!(invalid.to_string().contains("positive weights"));
    let report = run(fixtures::weighted(vec![1., 2., 1.]).unwrap());
    assert_eq!(
        report.completion(),
        &OpenExtrusionCompletion::Incomplete(OpenExtrusionCause::NonConstantWeights)
    );
    assert!(report.certificate().is_none());
}
#[test]
fn every_finite_work_allowance_can_refuse_without_partial_publication() {
    for kind in ["spans", "obligations", "steps", "bits"] {
        let policy = exhausted(kind);
        let report = certify_open_quadratic_extrusion(
            fixtures::triangle(f64::from_bits(1f64.to_bits() - 1)),
            policy,
            || false,
        );
        assert!(
            matches!(
                report.completion(),
                OpenExtrusionCompletion::Incomplete(OpenExtrusionCause::Limit { .. })
            ),
            "{kind}: {:?}",
            report.completion()
        );
        assert!(report.certificate().is_none());
        assert_eq!(report.policy(), policy);
    }
}
fn exhausted(kind: &str) -> OpenExtrusionPolicy {
    let mut policy = OpenExtrusionPolicy::bounded();
    match kind {
        "spans" => policy.max_spans = 0,
        "obligations" => policy.max_obligations = 0,
        "steps" => policy.max_proof_steps = 0,
        "bits" => policy.max_coefficient_bits = 1,
        other => panic!("limit {other:?}; expected spans, obligations, steps or bits"),
    }
    policy
}
#[test]
fn cancellation_and_unknown_behavior_preserve_owned_inputs() {
    let input = fixtures::parabola();
    let report =
        certify_open_quadratic_extrusion(input.clone(), OpenExtrusionPolicy::bounded(), || true);
    assert_eq!(
        report.completion(),
        &OpenExtrusionCompletion::Incomplete(OpenExtrusionCause::Cancelled)
    );
    assert_eq!(report.input().poles(), input.poles());
    assert!(report.certificate().is_none());
    let mut policy = OpenExtrusionPolicy::bounded();
    policy.version = 99;
    let report = certify_open_quadratic_extrusion(input, policy, || false);
    assert_eq!(
        report.completion(),
        &OpenExtrusionCompletion::Incomplete(OpenExtrusionCause::PolicyVersion(99))
    );
    assert!(report.certificate().is_none());
}

#[test]
fn invalid_storage_and_nonfinite_or_empty_domains_refuse_admission() {
    let empty = OpenQuadraticExtrusionInput::capture(
        vec![],
        vec![],
        vec![],
        vec![],
        [0., 0., 1.],
        [-1., 1.],
    );
    assert!(empty.is_err());
    assert!(fixtures::weighted(vec![f64::NAN; 3]).is_err());
    for (direction, interval) in [
        ([0.; 3], [-1., 1.]),
        ([0., 0., f64::INFINITY], [-1., 1.]),
        ([0., 0., 1.], [1., 1.]),
        ([0., 0., 1.], [0., f64::INFINITY]),
    ] {
        let result = invalid_domain(direction, interval);
        assert!(
            result.is_err(),
            "direction={direction:?}, interval={interval:?}"
        );
    }
}
fn invalid_domain(
    direction: [f64; 3],
    interval: [f64; 2],
) -> Result<OpenQuadraticExtrusionInput, OpenExtrusionInputError> {
    OpenQuadraticExtrusionInput::capture(
        vec![[0., 0., 0.], [1., 1., 0.], [2., 0., 0.]],
        vec![1.; 3],
        vec![0., 1.],
        vec![3, 3],
        direction,
        interval,
    )
}
