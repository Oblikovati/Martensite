//! Complete AppPar fitting families with immutable original requests and retained observations.
const CHUNKS: &[(&str, &str)] = &[
    (
        include_str!("fixtures/multi_curve_fit_corpus/inputs-00.txt"),
        include_str!("fixtures/multi_curve_fit_corpus/expected-00.txt"),
    ),
    (
        include_str!("fixtures/multi_curve_fit_corpus/inputs-01.txt"),
        include_str!("fixtures/multi_curve_fit_corpus/expected-01.txt"),
    ),
    (
        include_str!("fixtures/multi_curve_fit_corpus/inputs-02.txt"),
        include_str!("fixtures/multi_curve_fit_corpus/expected-02.txt"),
    ),
    (
        include_str!("fixtures/multi_curve_fit_corpus/inputs-03.txt"),
        include_str!("fixtures/multi_curve_fit_corpus/expected-03.txt"),
    ),
    (
        include_str!("fixtures/multi_curve_fit_corpus/inputs-04.txt"),
        include_str!("fixtures/multi_curve_fit_corpus/expected-04.txt"),
    ),
    (
        include_str!("fixtures/multi_curve_fit_corpus/inputs-05.txt"),
        include_str!("fixtures/multi_curve_fit_corpus/expected-05.txt"),
    ),
    (
        include_str!("fixtures/multi_curve_fit_corpus/inputs-06.txt"),
        include_str!("fixtures/multi_curve_fit_corpus/expected-06.txt"),
    ),
    (
        include_str!("fixtures/multi_curve_fit_corpus/inputs-07.txt"),
        include_str!("fixtures/multi_curve_fit_corpus/expected-07.txt"),
    ),
    (
        include_str!("fixtures/multi_curve_fit_corpus/inputs-08.txt"),
        include_str!("fixtures/multi_curve_fit_corpus/expected-08.txt"),
    ),
    (
        include_str!("fixtures/multi_curve_fit_corpus/inputs-09.txt"),
        include_str!("fixtures/multi_curve_fit_corpus/expected-09.txt"),
    ),
];
#[test]
fn fixture_bytes_and_all_rows_are_preserved() {
    assert_eq!(
        CHUNKS.iter().map(|(r, _)| r.lines().count()).sum::<usize>(),
        1882
    );
    assert_eq!(
        CHUNKS.iter().map(|(_, r)| r.lines().count()).sum::<usize>(),
        1882
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(r, _)| *r)),
        13996401873499903697
    );
    assert_eq!(
        super::fixture_hash(CHUNKS.iter().map(|(_, r)| *r)),
        3511032665925612143
    );
}
#[test]
fn apc_ls_complete_family() {
    super::replay_family(CHUNKS, "apc_ls", 1584);
}
#[test]
fn apc_resol_complete_family() {
    super::replay_family(CHUNKS, "apc_resol", 60);
}
#[test]
fn apc_function_complete_family() {
    super::replay_family(CHUNKS, "apc_function", 50);
}
#[test]
fn apc_bspfunction_complete_family() {
    super::replay_family(CHUNKS, "apc_bspfunction", 32);
}
#[test]
fn apc_gradient_complete_family() {
    super::replay_family(CHUNKS, "apc_gradient", 60);
}
#[test]
fn apc_bspgradient_complete_family() {
    super::replay_family(CHUNKS, "apc_bspgradient", 96);
}
#[path = "support/multi_curve_fit_controls.rs"]
mod independent_controls;
