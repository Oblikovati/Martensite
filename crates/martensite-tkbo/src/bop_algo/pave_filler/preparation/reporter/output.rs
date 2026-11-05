//! Emit fixed semantic records for external deep comparison with the frozen Native corpus.
use super::super::super::owned_progress_fixture::NativeScopeFrame;
use super::journal::{FrontEvent, FrontJournal};
use super::state::FrontState;

/// Emit actual values only; `emit_front_record(name, false, 0, journal, state)`.
pub(super) fn emit_front_record(
    name: &str,
    parallel: bool,
    target: usize,
    journal: &FrontJournal,
    state: FrontState,
) {
    let polls = journal.polls.lock().unwrap();
    let events = events_json(journal);
    super::reference::assert_front_reference(name, parallel, target, state, &polls, &events);
    println!(
        "FRONT_NATIVE_RECORD {{\"profile\":\"{name}\",\"parallel\":{},\"target\":{target},\"state\":{state:?},\"polls\":{polls:?},\"events\":[{}]}}",
        u8::from(parallel),
        events
    );
}

fn events_json(journal: &FrontJournal) -> String {
    let events: Vec<String> = journal
        .events
        .lock()
        .unwrap()
        .iter()
        .map(event_json)
        .collect();
    events.join(",")
}

fn event_json(event: &FrontEvent) -> String {
    let frames: Vec<String> = event.1.iter().map(frame_json).collect();
    format!("[{},[{}]]", event.0, frames.join(","))
}

fn frame_json(frame: &NativeScopeFrame) -> String {
    let name = scope_name_json(frame.name.as_deref());
    format!(
        "[{name},{},{},{},{},{}]",
        frame.maximum,
        frame.value,
        frame.portion,
        u8::from(frame.infinite),
        u8::from(frame.active)
    )
}

fn scope_name_json(name: Option<&[u8]>) -> String {
    let Some(bytes) = name else {
        return "null".to_owned();
    };
    assert!(
        bytes.is_ascii() && !bytes.contains(&b'"') && !bytes.contains(&b'\\'),
        "scope name bytes {bytes:?}, expected pinned plain ASCII without quotes/backslashes"
    );
    format!("\"{}\"", std::str::from_utf8(bytes).unwrap())
}
