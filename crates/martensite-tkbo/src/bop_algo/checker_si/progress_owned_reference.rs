//! Correct predecessor graph laws remain the reference where the native forward flag is discarded.
use super::fixture::{CheckerCase, canonical_pair};
/// Check the retained input relation and cancellation law; `check_owned_incidence(case, target, actual, native)`.
pub(super) fn check_owned_incidence(case: &CheckerCase, target: usize, actual: &str, native: &str) {
    let prepared = case
        .prepared
        .as_ref()
        .expect("positive incidence fixture: expected retained input pairs and seeded graph");
    let (actual_header, actual_rest) = actual.split_once(';').unwrap();
    let (native_header, _) = native.split_once(';').unwrap();
    assert_eq!(
        actual_header, native_header,
        "native reports and scopes remain applicable"
    );
    assert_eq!(prepared.pairs.len(), if case.kind == 44 { 1 } else { 6 });
    let expected = expected_stored_pairs(&prepared.pairs, target);
    let slot = if case.kind == 44 { 7 } else { 8 };
    let (records, _) = actual_rest.split_once(';').unwrap();
    assert_eq!(records, expected_record_arrays(&expected, slot));
    let mut graph = prepared.graph.clone();
    graph.extend(expected.iter().copied().map(canonical_pair));
    let actual_graph: std::collections::BTreeSet<_> = case
        .checker
        .data_structure()
        .unwrap()
        .interferences()
        .iter()
        .map(|pair| canonical_pair(pair.indices()))
        .collect();
    assert_eq!(
        actual_graph, graph,
        "only successful input relations may be added to the graph"
    );
}
fn expected_stored_pairs(inputs: &[(i32, i32)], target: usize) -> Vec<(i32, i32)> {
    if target == inputs.len() + 1 {
        return Vec::new();
    }
    inputs
        .iter()
        .enumerate()
        .filter(|(ordinal, _)| target != ordinal + 1)
        .map(|(_, pair)| *pair)
        .collect()
}
fn expected_record_arrays(pairs: &[(i32, i32)], slot: usize) -> String {
    let rows: Vec<_> = pairs.iter().map(|(a, b)| format!("[{a},{b}]")).collect();
    let arrays: Vec<_> = (0..10)
        .map(|position| {
            if position == slot {
                format!("[{}]", rows.join(","))
            } else {
                "[]".to_owned()
            }
        })
        .collect();
    format!("[{}]", arrays.join(","))
}
