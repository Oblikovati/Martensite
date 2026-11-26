//! Complete point-distribution families preserve requests, state transitions and all retained fields.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/point_distribution_corpus/inputs-00.txt"),
        include_str!("fixtures/point_distribution_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/point_distribution_corpus/inputs-01.txt"),
        include_str!("fixtures/point_distribution_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/point_distribution_corpus/inputs-02.txt"),
        include_str!("fixtures/point_distribution_corpus/expected-02.txt"),
    ),
    (
        include_str!("fixtures/point_distribution_corpus/inputs-03.txt"),
        include_str!("fixtures/point_distribution_corpus/expected-03.txt"),
    ),
    (
        include_str!("fixtures/point_distribution_corpus/inputs-04.txt"),
        include_str!("fixtures/point_distribution_corpus/expected-04.txt"),
    ),
    (
        include_str!("fixtures/point_distribution_corpus/inputs-05.txt"),
        include_str!("fixtures/point_distribution_corpus/expected-05.txt"),
    ),
    (
        include_str!("fixtures/point_distribution_corpus/inputs-06.txt"),
        include_str!("fixtures/point_distribution_corpus/expected-06.txt"),
    ),
    (
        include_str!("fixtures/point_distribution_corpus/inputs-07.txt"),
        include_str!("fixtures/point_distribution_corpus/expected-07.txt"),
    ),
    (
        include_str!("fixtures/point_distribution_corpus/inputs-08.txt"),
        include_str!("fixtures/point_distribution_corpus/expected-08.txt"),
    ),
];
#[test]
fn gcp_length_complete_family() {
    super::replay_family(CHUNKS, "gcp_length", 192);
}
#[test]
fn cpnts_abscissa_complete_family() {
    super::replay_family(CHUNKS, "cpnts_abscissa", 608);
}
#[test]
fn cpnts_udefl_complete_family() {
    super::replay_family(CHUNKS, "cpnts_udefl", 256);
}
#[test]
fn gcp_abscissa_complete_family() {
    super::replay_family(CHUNKS, "gcp_abscissa", 640);
}
#[test]
fn gcp_uabs_complete_family() {
    super::replay_family(CHUNKS, "gcp_uabs", 32);
}
#[test]
fn gcp_quabs_complete_family() {
    super::replay_family(CHUNKS, "gcp_quabs", 32);
}
#[test]
fn gcp_udefl_complete_family() {
    replay_deflection_family(CHUNKS, "gcp_udefl", 35);
}
#[test]
fn gcp_qudefl_complete_family() {
    replay_deflection_family(CHUNKS, "gcp_qudefl", 35);
}
#[test]
fn gcp_tdefl_complete_family() {
    super::replay_family(CHUNKS, "gcp_tdefl", 32);
}
#[test]
fn gcp_arc_step_complete_family() {
    super::replay_family(CHUNKS, "gcp_arc_step", 252);
}
#[test]
fn gcp_dist_complete_family() {
    super::replay_family(CHUNKS, "gcp_dist", 96);
}
#[test]
fn point_distribution_inputs_and_answers_stay_frozen() {
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(q, _)| *q)),
        0xb966e54e5e1d89ee
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, a)| *a)),
        0x40d3cad86189f8d0
    );
}
fn assert_unit_speed_lengths(request: &[f64]) {
    let output = super::construction_corpus::run_values("gcp_length", request).unwrap();
    let expected: String = [4.0_f64, 4.0, 2.0, 2.0, 4.0, 4.0, 2.0, 2.0]
        .iter()
        .map(|value| format!("{} ", value.to_bits()))
        .collect();
    assert_eq!(output, expected + "\n");
}
#[test]
fn planar_line_lengths_match_independent_parameter_differences() {
    assert_unit_speed_lengths(&[2.0, 1.0, 0.0, 4.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 3.0, 1e-6]);
}
#[test]
fn spatial_line_lengths_match_independent_parameter_differences() {
    assert_unit_speed_lengths(&[
        3.0, 1.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 3.0, 1e-6,
    ]);
}

// #530: `perform_circular` anchors exact endpoints instead of leaving OCCT's accumulated
// `u += angle` final sample drifted from `u2` (which could step outside `[u1, u2]`).
// Keep the frozen OCCT fixture files untouched and verify that non-circular requests match
// bit-for-bit while circular requests differ only by sub-1e-11 endpoint anchoring deltas.
fn is_circular_deflection_request(values: &[f64]) -> bool {
    let trimmed = values.get(1).copied() == Some(1.0);
    let kind_index = if trimmed { 4 } else { 2 };
    match values.get(kind_index).copied() {
        Some(1.0) => true,
        Some(7.0) => values.get(kind_index + 3).copied() == Some(1.0),
        _ => false,
    }
}

fn tokens_match_within_circular_endpoint_anchor(expected: &str, actual: &str) -> Result<(), String> {
    let exp: Vec<&str> = expected.split_whitespace().collect();
    let act: Vec<&str> = actual.split_whitespace().collect();
    if exp.len() != act.len() {
        return Err(format!("token count differs: {} vs {}", exp.len(), act.len()));
    }
    for (idx, (left, right)) in exp.iter().zip(act.iter()).enumerate() {
        if left != right {
            check_anchored_token(idx, left, right)?;
        }
    }
    Ok(())
}

fn check_anchored_token(idx: usize, left: &str, right: &str) -> Result<(), String> {
    let (Ok(l_bits), Ok(r_bits)) = (left.parse::<u64>(), right.parse::<u64>()) else {
        return Err(format!("non-numeric token {idx} differs: {left} vs {right}"));
    };
    let (l_val, r_val) = (f64::from_bits(l_bits), f64::from_bits(r_bits));
    let limit = 1e-11 * l_val.abs().max(r_val.abs()).max(1.0);
    if !(l_val.is_finite() && r_val.is_finite()) || (l_val - r_val).abs() > limit {
        return Err(format!("token {idx} exceeds #530 anchor bound: {l_val:?} vs {r_val:?}"));
    }
    Ok(())
}

fn check_deflection_request(request: &str, expected: &str, family: &str) -> bool {
    let mut fields = request.split_whitespace();
    if fields.next() != Some(family) {
        return false;
    }
    let values: Vec<f64> = fields.map(|field| field.parse().unwrap()).collect();
    let actual = super::construction_corpus::run_values(family, &values).expect("owned family");
    if super::construction_corpus::policy::equivalent(expected, &actual) {
        return true;
    }
    assert!(
        is_circular_deflection_request(&values),
        "{request}: non-circular request diverged: {}",
        super::construction_corpus::policy::difference(expected, &actual)
    );
    tokens_match_within_circular_endpoint_anchor(expected, &actual)
        .unwrap_or_else(|reason| panic!("{request}: {reason}"));
    true
}

fn replay_deflection_family(chunks: &[(&str, &str)], family: &str, expected_count: usize) {
    let mut count = 0;
    for (requests, answers) in chunks {
        assert_eq!(requests.lines().count(), answers.lines().count());
        for (request, expected) in requests.lines().zip(answers.split_inclusive('\n')) {
            count += usize::from(check_deflection_request(request, expected, family));
        }
    }
    assert_eq!(count, expected_count, "Every original {family} request must run");
}
