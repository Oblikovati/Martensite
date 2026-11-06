//! First genuine modelling-fixture operation: analytic boxes, real direct/prepared builders.
#[path = "common_operation/mod.rs"]
mod common_operation;
#[path = "modeling_fixture_box_cut/mod.rs"]
mod modeling_fixture_box_cut;

#[test]
fn direct_and_prepared_box_cut_preserve_closed_form_geometry_and_real_history() {
    modeling_fixture_box_cut::direct_and_prepared();
}

#[test]
fn explicit_non_destructive_box_cut_preserves_input_geometry() {
    modeling_fixture_box_cut::non_destructive();
}

#[test]
fn failed_non_destructive_cut_reports_failure_and_preserves_inputs() {
    modeling_fixture_box_cut::failed_non_destructive();
}
