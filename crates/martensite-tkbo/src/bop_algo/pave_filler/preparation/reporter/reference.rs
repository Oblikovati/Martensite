//! Exact retained state/poll comparisons and serial callback ancestry bits.
use super::state::FrontState;

/// Compare all state/poll snapshots and the deterministic serial trace.
/// Example: `assert_front_reference("init-empty", false, 1, state, polls, events)`.
pub(super) fn assert_front_reference(
    name: &str,
    parallel: bool,
    target: usize,
    state: FrontState,
    polls: &[FrontState],
    events: &str,
) {
    let key = format!("{name}|{}|{target}|", u8::from(parallel));
    let actual = format!("{state:?}|{polls:?}").replace(' ', "");
    let expected = reference_row(include_str!("states.expected"), &key);
    assert_eq!(actual, expected, "front state/polls {key}");
    if !parallel {
        let expected = reference_row(include_str!("events.expected"), &key);
        assert_eq!(format!("[{events}]"), expected, "front serial trace {key}");
    }
}

fn reference_row<'a>(fixture: &'a str, key: &str) -> &'a str {
    let mut matches = fixture.lines().filter_map(|line| line.strip_prefix(key));
    let row = matches
        .next()
        .unwrap_or_else(|| panic!("front identity {key:?}; expected one frozen observation"));
    assert!(
        matches.next().is_none(),
        "duplicate front identity {key:?}; expected one frozen observation"
    );
    row
}
