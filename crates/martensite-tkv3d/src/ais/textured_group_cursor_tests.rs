//! Node-cursor boundaries complement the six native callback regressions.
use super::*;
use martensite_tkservice::null_driver::NullGroup;

fn identity_group() -> Handle<dyn Graphic3dGroup> {
    // Only identity/cursor behavior is inspected, so no presentation operation uses this null parent.
    Handle::new(NullGroup::new(&Handle::default())).cast()
}
fn sequence(groups: &[&Handle<dyn Graphic3dGroup>]) -> GroupSequence {
    groups.iter().map(|group| (*group).clone()).collect()
}
fn address(group: Option<HandleObserver<dyn Graphic3dGroup>>) -> Option<*const ()> {
    group.map(|group| group.object_address())
}
#[test]
fn textured_cursor_empty_sequence_ends_without_failure() {
    let mut cursor = LiveNodeCursor::default();
    assert!(cursor.advance(&GroupSequence::default()).unwrap().is_none());
}
#[test]
fn textured_cursor_removed_earlier_node_keeps_the_successor() {
    let [earlier, active, next] = [identity_group(), identity_group(), identity_group()];
    let mut groups = sequence(&[&earlier, &active, &next]);
    let mut cursor = LiveNodeCursor::default();
    cursor.advance(&groups).unwrap();
    assert_eq!(
        address(cursor.advance(&groups).unwrap()),
        Some(active.object_address())
    );
    groups.remove(1).unwrap();
    assert_eq!(
        address(cursor.advance(&groups).unwrap()),
        Some(next.object_address())
    );
    assert!(cursor.advance(&groups).unwrap().is_none());
}
#[test]
fn textured_cursor_repeated_handle_nodes_are_visited_separately() {
    let [first, repeated] = [identity_group(), identity_group()];
    let groups = sequence(&[&first, &repeated, &repeated]);
    let mut cursor = LiveNodeCursor::default();
    let visited: Vec<_> = std::iter::from_fn(|| cursor.advance(&groups).unwrap())
        .map(|group| group.object_address())
        .collect();
    let expected = [
        first.object_address(),
        repeated.object_address(),
        repeated.object_address(),
    ];
    assert_eq!(visited, expected);
}
#[test]
fn textured_cursor_removed_active_node_returns_typed_failure() {
    let [active, next] = [identity_group(), identity_group()];
    let mut groups = sequence(&[&active, &next]);
    let mut cursor = LiveNodeCursor::default();
    cursor.advance(&groups).unwrap();
    groups.remove(1).unwrap();
    let Err(error) = cursor.advance(&groups) else {
        panic!("advance after removing the active node; expected a typed failure");
    };
    assert!(
        error.to_string().contains("NoSuchObject") || error.to_string().contains("Next"),
        "failure={error}; expected the sequence cursor invalidation"
    );
}
