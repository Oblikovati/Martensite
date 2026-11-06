//! Same serialized valid native graphs exercise the production connection query without an reference runtime.
mod connection_graphs;
#[allow(dead_code)] // Only the canonical IoStream-to-text adapter is needed for input-isolation snapshots.
#[path = "support/geometry_text.rs"]
mod geometry_text;

#[test]
fn valid_connection_graphs_match_independent_native_and_analytic_expectations() {
    let checked = connection_graphs::run();
    assert_eq!(checked, 1428);
}
