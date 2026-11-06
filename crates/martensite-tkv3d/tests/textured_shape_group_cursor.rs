//! Actual protected group mutations replay independent Native values against the unchanged helper.
#[allow(dead_code)] // Existing shared state fixture also serves file/pixmap dispatch tests.
#[path = "support/textured_shape_state_fixture.rs"]
mod state_fixture;
#[path = "support/textured_cursor/mod.rs"]
mod textured_cursor;
const NATIVE: &str = include_str!("fixtures/ais_textured_shape/native-group-cursor-live.jsonl");

fn assert_cursor_case(name: &'static str) {
    let needle = format!("\"case\":\"{name}\"");
    let expected: Vec<&str> = NATIVE
        .lines()
        .filter(|line| line.contains(&needle))
        .collect();
    let actual = textured_cursor::replay_cursor_case(name);
    assert_eq!(
        actual, expected,
        "actual production callback case={name}; expected retained Native full record sequence"
    );
}
#[test]
fn textured_cursor_unchanged_groups_preserve_native_borrowed_counts() {
    // The baseline distinguishes traversal ownership from mutation effects.
    assert_cursor_case("unchanged");
}
#[test]
fn textured_cursor_remove_earlier_keeps_actual_successor() {
    // B survives removal of earlier A, so the actual C callback remains required.
    assert_cursor_case("remove-earlier");
}
#[test]
fn textured_cursor_remove_next_omits_removed_callback() {
    // Removing C while B is current is a valid stable-node mutation.
    assert_cursor_case("remove-next");
}
#[test]
fn textured_cursor_append_visits_new_group_with_no_external_owner() {
    // D has only its presentation owner, matching the independently observed input.
    assert_cursor_case("append");
}
#[test]
fn textured_cursor_current_field_rebind_retargets_borrowed_argument() {
    // B's existing argument and later C must both observe the actual changed field.
    assert_cursor_case("rebind-current-aspect");
}
#[test]
fn textured_cursor_duplicate_nodes_visit_the_same_group_twice() {
    // Two sequence nodes retain B: both callbacks run, in node order.
    assert_cursor_case("duplicate-active-unchanged");
}
#[test]
fn textured_cursor_duplicate_nodes_survive_removing_an_earlier_group() {
    // Removing A during the first B callback keeps the second B node and C.
    assert_cursor_case("duplicate-active-remove-earlier");
}
#[test]
fn textured_cursor_shading_field_rebind_keeps_copied_current_aspect() {
    // The drawer field is not the protected myAspect copy made before traversal.
    assert_cursor_case("rebind-shading-aspect");
}
