#[test]
fn plain_test_runs_are_not_case_workers() {
    assert!(
        !super::inside_case_worker(),
        "expected no {} in a plain run",
        super::CASE_CHILD_MARKER
    );
}
