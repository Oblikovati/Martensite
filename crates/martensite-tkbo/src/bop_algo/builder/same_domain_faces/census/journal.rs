//! Capture callbacks without retaining a mutable builder or its coordinator maps.
use crate::bop_algo::pave_filler::owned_progress_fixture::{NativeScopeFrame, native_scope_frames};
use martensite_tkernel::message::{ProgressIndicator, ProgressIndicatorHooks, ProgressScope};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};

type SameDomainEvents = Vec<(u64, Vec<NativeScopeFrame>)>;
#[derive(Default)]
pub(super) struct SameDomainJournal {
    polls: AtomicUsize,
    pub events: Mutex<SameDomainEvents>,
}
impl SameDomainJournal {
    /// Count actual queries; `journal.poll_count()` includes worker and coordinator polls.
    pub(super) fn poll_count(&self) -> usize {
        self.polls.load(Ordering::SeqCst)
    }
}
struct SameDomainHooks {
    journal: Arc<SameDomainJournal>,
    target: usize,
}
impl ProgressIndicatorHooks for SameDomainHooks {
    fn user_break(&self) -> bool {
        let ordinal = self.journal.polls.fetch_add(1, Ordering::SeqCst) + 1;
        self.target != 0 && ordinal == self.target
    }
    fn show(&self, scope: &ProgressScope<'_>, position: f64, _: bool) {
        self.journal
            .events
            .lock()
            .unwrap()
            .push((position.to_bits(), native_scope_frames(scope)));
    }
}
/// Own a single case's actual pulse policy; `indicator(5)` requests only query five.
pub(super) fn indicator(target: usize) -> (ProgressIndicator, Arc<SameDomainJournal>) {
    let journal = Arc::new(SameDomainJournal::default());
    let hooks = SameDomainHooks {
        journal: journal.clone(),
        target,
    };
    (ProgressIndicator::new(Box::new(hooks)), journal)
}
