//! A cancelled empty task isolates native range ownership from geometry work.
use super::*;
use crate::bop_algo::pave_filler::owned_progress_fixture::{NativeScopeFrame, native_scope_frames};
use martensite_tkernel::message::{ProgressIndicator, ProgressIndicatorHooks};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};
static_assertions::assert_impl_all!(WireBlockTask<'static, 'static>: Send);
static_assertions::assert_not_impl_any!(WireBlockTask<'static, 'static>: Sync);
#[derive(Default)]
struct WireScopeJournal {
    queries: AtomicUsize,
    frames: Mutex<Vec<Vec<NativeScopeFrame>>>,
}
struct WireScopeHooks(Arc<WireScopeJournal>);
impl ProgressIndicatorHooks for WireScopeHooks {
    fn user_break(&self) -> bool {
        self.0.queries.fetch_add(1, Ordering::SeqCst);
        true
    }
    fn show(&self, scope: &ProgressScope<'_>, _: f64, _: bool) {
        self.0
            .frames
            .lock()
            .unwrap()
            .push(native_scope_frames(scope));
    }
}
#[test]
fn owned_wire_range_has_one_worker_scope_and_refuses_reexecution() {
    let journal = Arc::new(WireScopeJournal::default());
    let indicator = ProgressIndicator::new(Box::new(WireScopeHooks(journal.clone())));
    let range = indicator.start();
    let scope = ProgressScope::unnamed(&range, 1., false);
    let mut blocks = [ConnexityBlock::default()];
    let mut tasks = wire_tasks(&mut blocks, &scope);
    let context = new_worker_context();
    let observed = ProgressCancellation::default();
    tasks[0]
        .perform(&Face::default(), &context, &observed)
        .unwrap();
    assert!(observed.is_cancelled());
    assert_eq!(journal.queries.load(Ordering::SeqCst), 1);
    let frames = journal.frames.lock().unwrap();
    assert_eq!(frames.iter().map(Vec::len).max(), Some(3));
    assert!(frames.iter().any(|chain| chain.len() == 3));
    drop(frames);
    let failure = tasks[0]
        .perform(&Face::default(), &context, &observed)
        .unwrap_err();
    assert!(failure.message().contains("wire block 0 has no range"));
    assert_eq!(journal.queries.load(Ordering::SeqCst), 1);
}
