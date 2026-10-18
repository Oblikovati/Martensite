//! Complete original AppDef numeric families with reviewed captured expectations.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/fit_constraints_corpus/inputs-00.txt"),
        include_str!("fixtures/fit_constraints_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/fit_constraints_corpus/inputs-01.txt"),
        include_str!("fixtures/fit_constraints_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/fit_constraints_corpus/inputs-02.txt"),
        include_str!("fixtures/fit_constraints_corpus/expected-02.txt"),
    ),
    (
        include_str!("fixtures/fit_constraints_corpus/inputs-03.txt"),
        include_str!("fixtures/fit_constraints_corpus/expected-03.txt"),
    ),
];
#[test]
fn fixture_bytes_and_all_rows_are_preserved() {
    assert_eq!(
        CHUNKS.iter().map(|(r, _)| r.lines().count()).sum::<usize>(),
        317
    );
    assert_eq!(
        CHUNKS.iter().map(|(_, r)| r.lines().count()).sum::<usize>(),
        317
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(r, _)| *r)),
        17699540774014611260
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, r)| *r)),
        8874465279718063828
    );
}
#[test]
fn ad_compute_complete_family() {
    super::replay_family(CHUNKS, "ad_compute", 36);
}
#[test]
fn ad_criteria_complete_family() {
    super::replay_family(CHUNKS, "ad_criteria", 38);
}
#[test]
fn ad_var_complete_family() {
    super::replay_family(CHUNKS, "ad_var", 243);
}

#[path = "support/fit_constraints_controls.rs"]
mod independent_controls;

#[test]
fn changed_expected_value_is_rejected_by_the_existing_policy() {
    let request = CHUNKS[0].0.lines().next().unwrap();
    assert!(request.starts_with("ad_compute "));
    let expected = CHUNKS[0].1.lines().next().unwrap();
    let (_, tail) = expected.split_once(' ').unwrap();
    let corrupted = format!("31337 {tail}\n");
    let result = std::panic::catch_unwind(|| {
        super::construction_corpus::replay::checked_request(request, &corrupted, "ad_compute");
    });
    assert!(result.is_err());
}

#[test]
fn missing_expected_row_is_rejected_before_request_execution() {
    let result = std::panic::catch_unwind(|| {
        super::replay_family(&[(CHUNKS[0].0, "")], "ad_compute", 36);
    });
    assert!(result.is_err());
}
