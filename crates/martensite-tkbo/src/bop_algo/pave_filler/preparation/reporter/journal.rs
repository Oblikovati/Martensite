//! The existing SDK serializes Show; queries capture live state before their one-shot request.
use super::super::super::owned_progress_fixture::{NativeScopeFrame, native_scope_frames};
use super::state::{FrontState, ReceiverStateSource};
use martensite_tkernel::message::{ProgressIndicator, ProgressIndicatorHooks, ProgressScope};
use std::sync::{Arc, Mutex};

pub(super) type FrontEvent = (u64, Vec<NativeScopeFrame>);
#[derive(Default)]
pub(super) struct FrontJournal {
    pub polls: Mutex<Vec<FrontState>>,
    pub events: Mutex<Vec<FrontEvent>>,
}

struct FrontRecordedHooks {
    source: ReceiverStateSource,
    journal: Arc<FrontJournal>,
    target: usize,
}

impl ProgressIndicatorHooks for FrontRecordedHooks {
    fn show(&self, scope: &ProgressScope<'_>, position: f64, _: bool) {
        let event = (position.to_bits(), native_scope_frames(scope));
        self.journal.events.lock().unwrap().push(event);
    }

    fn user_break(&self) -> bool {
        let snapshot = self.source.read();
        let mut polls = self.journal.polls.lock().unwrap();
        polls.push(snapshot);
        polls.len() == self.target
    }
}

/// Own observations without retaining the builder or ranges; `front_journal(source, 1)`.
pub(super) fn front_journal(
    source: ReceiverStateSource,
    target: usize,
) -> (ProgressIndicator, Arc<FrontJournal>) {
    let journal = Arc::new(FrontJournal::default());
    let hooks = FrontRecordedHooks {
        source,
        journal: journal.clone(),
        target,
    };
    (ProgressIndicator::new(Box::new(hooks)), journal)
}
