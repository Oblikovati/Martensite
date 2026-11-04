//! Textured SetColor/SetMaterial/UnsetMaterial walk the live presentation sequence node by
//! node; every case replays the native record sequence of
//! `the recorded textured_presentation_cursor capture` exactly.
#[allow(dead_code)] // The shared state fixture also serves file/pixmap dispatch tests.
#[path = "support/textured_shape_state_fixture.rs"]
mod state_fixture;
#[path = "support/textured_prs_cursor/mod.rs"]
mod textured_prs_cursor;
const NATIVE: &str =
    include_str!("fixtures/ais_textured_shape/native-presentation-cursor-live.jsonl");

fn assert_prs_cursor_case(name: &'static str) {
    for operation in 0..3 {
        let needle = format!("\"case\":\"{name}\",\"operation\":{operation},");
        let expected: Vec<&str> = NATIVE.lines().filter(|l| l.contains(&needle)).collect();
        assert!(
            !expected.is_empty(),
            "native fixture has no case={name} operation={operation}"
        );
        let actual = textured_prs_cursor::replay_prs_cursor_case(name, operation);
        assert_eq!(
            actual, expected,
            "production refresh case={name} operation={operation}; expected the native records"
        );
    }
}
#[test]
fn textured_presentation_walk_unchanged_lends_without_an_owner() {
    // Each mode-3 presentation is visited once; the walk adds no presentation owner.
    assert_prs_cursor_case("unchanged");
}
#[test]
fn textured_presentation_walk_survives_removing_an_earlier_node() {
    // Removing A during B's callback keeps the walk on B's node, so C is still visited.
    assert_prs_cursor_case("remove-earlier");
}
#[test]
fn textured_presentation_walk_skips_a_removed_next_node() {
    assert_prs_cursor_case("remove-next");
}
#[test]
fn textured_presentation_walk_visits_an_appended_node() {
    // D, appended during B's callback with only its node owner, is visited after C.
    assert_prs_cursor_case("append");
}
#[test]
fn textured_presentation_walk_skips_other_modes() {
    assert_prs_cursor_case("earlier-mode0");
}
#[test]
fn textured_presentation_walk_visits_a_repeated_handle_per_node() {
    assert_prs_cursor_case("duplicate-active-unchanged");
}
#[test]
fn textured_presentation_walk_keeps_the_second_node_after_removing_earlier() {
    assert_prs_cursor_case("duplicate-active-remove-earlier");
}
