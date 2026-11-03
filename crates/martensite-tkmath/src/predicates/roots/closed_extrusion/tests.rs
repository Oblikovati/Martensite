//! Independent simple loops, actual owned support storage, and refusal controls.
use super::*;
#[path = "../../../../tests/support/closed_quadratic_extrusion_inputs.rs"]
mod inputs;

fn diamond() -> ClosedQuadraticExtrusionInput {
    let poles = vec![
        [1., 0., 0.],
        [1., 1., 0.],
        [0., 1., 0.],
        [-1., 1., 0.],
        [-1., 0., 0.],
        [-1., -1., 0.],
        [0., -1., 0.],
        [1., -1., 0.],
        [1., 0., 0.],
    ];
    capture(poles, vec![0., 1., 2., 3., 4.], vec![3, 2, 2, 2, 3])
}
fn capture(
    poles: Vec<[f64; 3]>,
    knots: Vec<f64>,
    mults: Vec<u32>,
) -> ClosedQuadraticExtrusionInput {
    let weights = vec![1.; poles.len()];
    ClosedQuadraticExtrusionInput::capture(poles, weights, knots, mults, [0., 0., 1.], [-1., 1.])
        .unwrap()
}
fn run(input: ClosedQuadraticExtrusionInput) -> ClosedExtrusionReport {
    certify_closed_quadratic_extrusion(input, ClosedExtrusionPolicy::bounded(), || false)
}

#[test]
fn independently_disjoint_quadrants_certify_both_windings() {
    let input = diamond();
    let first = run(input.clone());
    let mut reversed = input;
    reversed.poles.reverse();
    assert_eq!(first.completion(), &ClosedExtrusionCompletion::Certified);
    assert_eq!(
        run(reversed).completion(),
        &ClosedExtrusionCompletion::Certified
    );
    assert_eq!(first.work().scheduled_obligations, 10);
    assert_eq!(first.work().finished_obligations, 10);
    assert!(first.certificate().is_some());
}

#[test]
fn oblique_projection_and_tiny_exact_coordinates_do_not_round_signs() {
    let mut input = diamond();
    input.direction = [1., 2., 3.];
    let oblique = run(input);
    assert!(oblique.certificate().is_some());
    let mut tiny = diamond();
    for point in &mut tiny.poles {
        for value in point {
            *value *= 2f64.powi(-1000);
        }
    }
    assert!(run(tiny).certificate().is_some());
}

#[test]
fn known_bow_tie_crossing_stays_unproved() {
    let report = run(bow_tie());
    assert!(matches!(
        report.completion(),
        ClosedExtrusionCompletion::Incomplete(ClosedExtrusionCause::UnseparatedPair { .. })
    ));
    assert!(report.certificate().is_none());
    assert!(report.work().finished_obligations < report.work().scheduled_obligations);
}

fn bow_tie() -> ClosedQuadraticExtrusionInput {
    let vertices = [
        [-1., -1., 0.],
        [1., 1., 0.],
        [-1., 1., 0.],
        [1., -1., 0.],
        [-1., -1., 0.],
    ];
    let mut poles = vec![vertices[0]];
    for pair in vertices.windows(2) {
        poles.push(std::array::from_fn(|i| (pair[0][i] + pair[1][i]) / 2.));
        poles.push(pair[1]);
    }
    capture(poles, vec![0., 1., 2., 3., 4.], vec![3, 2, 2, 2, 3])
}

#[test]
fn a_closed_quadratic_line_backtracks_and_constant_projection_collapses() {
    let input = capture(
        vec![[0.; 3], [1., 0., 0.], [0.; 3]],
        vec![0., 1.],
        vec![3, 3],
    );
    assert!(matches!(
        run(input).completion(),
        ClosedExtrusionCompletion::NotExcluded(
            ClosedExtrusionProjectionContact::BacktrackingSpan { span: 0 }
        )
    ));
    let input = capture(vec![[0.; 3]; 3], vec![0., 1.], vec![3, 3]);
    assert!(matches!(
        run(input).completion(),
        ClosedExtrusionCompletion::NotExcluded(ClosedExtrusionProjectionContact::CollapsedSpan {
            span: 0
        })
    ));
}

#[test]
fn global_budget_and_cancellation_never_publish_partial_coverage() {
    let mut policy = ClosedExtrusionPolicy::bounded();
    policy.max_proof_steps = 60;
    let report = certify_closed_quadratic_extrusion(diamond(), policy, || false);
    assert!(matches!(
        report.completion(),
        ClosedExtrusionCompletion::Incomplete(ClosedExtrusionCause::Limit {
            kind: "proof steps",
            ..
        })
    ));
    assert_eq!(report.work().started_proof_steps, 60);
    assert!(report.work().started_obligations > 5);
    assert!(report.certificate().is_none());
}

#[test]
fn cancellation_preserves_partial_work_without_a_certificate() {
    let mut polls = 0;
    let cancelled =
        certify_closed_quadratic_extrusion(diamond(), ClosedExtrusionPolicy::bounded(), || {
            polls += 1;
            polls == 20
        });
    assert_eq!(
        cancelled.completion(),
        &ClosedExtrusionCompletion::Incomplete(ClosedExtrusionCause::Cancelled)
    );
    assert_eq!(cancelled.work().started_proof_steps, 19);
    assert!(cancelled.certificate().is_none());
}

#[test]
fn one_bit_open_seam_and_nonfinite_coordinates_are_invalid() {
    let input = diamond();
    let mut poles = input.poles.clone();
    poles.last_mut().unwrap()[0] = f64::from_bits(1f64.to_bits() + 1);
    let result = ClosedQuadraticExtrusionInput::capture(
        poles,
        input.weights.clone(),
        input.knots.clone(),
        input.multiplicities.clone(),
        input.direction,
        input.interval,
    );
    assert!(matches!(
        result,
        Err(ClosedExtrusionInputError::OpenEndpoints { .. })
    ));
}

#[test]
fn nonfinite_coordinates_are_invalid() {
    let mut input = diamond();
    input.poles[1][0] = f64::NAN;
    assert!(matches!(
        super::validation::input(&input),
        Err(ClosedExtrusionInputError::Numeric { .. })
    ));
}

#[test]
fn nonconstant_weights_and_requested_caps_remain_incomplete() {
    let mut input = diamond();
    input.weights[1] = 2.;
    assert_eq!(
        run(input).completion(),
        &ClosedExtrusionCompletion::Incomplete(ClosedExtrusionCause::NonConstantWeights)
    );
    let mut policy = ClosedExtrusionPolicy::bounded();
    policy.max_coefficient_bits = 0;
    assert!(matches!(
        certify_closed_quadratic_extrusion(diamond(), policy, || false).completion(),
        ClosedExtrusionCompletion::Incomplete(ClosedExtrusionCause::Limit {
            kind: "exact coefficient bits",
            ..
        })
    ));
}

#[test]
fn owned_report_is_isolated_and_all_published_types_are_send_sync() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<ClosedQuadraticExtrusionInput>();
    send_sync::<ClosedExtrusionReport>();
    send_sync::<ClosedExtrusionCertificate<'static>>();
    let mut original = diamond();
    let captured = original.clone();
    let report = run(captured);
    original.poles[0][0] = 99.;
    assert_eq!(report.input().poles()[0][0], 1.);
    assert_eq!(report.work().root_subdivisions, 0);
}

#[test]
fn owned_numeric_sources_cover_every_declared_obligation() {
    let mut completed = 0;
    for fixture in fixtures() {
        let input = fixture_input(fixture);
        assert_eq!(
            input.curve_interval().map(f64::to_bits),
            fixture.curve_domain
        );
        let report = run(input);
        assert_eq!(report.completion(), &ClosedExtrusionCompletion::Certified);
        assert_eq!(
            report.work().started_obligations,
            report.work().scheduled_obligations
        );
        completed += report.work().finished_obligations;
    }
    assert_eq!(completed, 614);
}

fn fixtures() -> [inputs::ClosedQuadraticFixture; 5] {
    use inputs::*;
    [
        CLOSED_QUADRATIC_34_POLES,
        CLOSED_QUADRATIC_13_POLES,
        CLOSED_QUADRATIC_13_POLES_SECOND,
        CLOSED_QUADRATIC_33_POLES,
        CLOSED_QUADRATIC_11_POLES,
    ]
}

fn fixture_input(fixture: inputs::ClosedQuadraticFixture) -> ClosedQuadraticExtrusionInput {
    let poles = fixture
        .poles
        .iter()
        .map(|p| p.map(f64::from_bits))
        .collect();
    let weights = fixture_reals(fixture.weights);
    let knots = fixture.knots.iter().copied().map(f64::from_bits).collect();
    ClosedQuadraticExtrusionInput::capture(
        poles,
        weights,
        knots,
        fixture.multiplicities.to_vec(),
        fixture.direction.map(f64::from_bits),
        fixture.extrusion_domain.map(f64::from_bits),
    )
    .unwrap()
}

fn fixture_reals(values: &[u64]) -> Vec<f64> {
    values.iter().copied().map(f64::from_bits).collect()
}
