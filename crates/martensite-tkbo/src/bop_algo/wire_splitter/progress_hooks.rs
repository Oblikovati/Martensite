//! A bounded real-task rendezvous proves executor overlap rather than synthetic update counts.
use martensite_tkernel::message::{ProgressIndicator, ProgressIndicatorHooks, ProgressScope};
use std::{
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    thread::ThreadId,
    time::Duration,
};

#[derive(Default)]
pub(super) struct WorkerRendezvous {
    threads: Mutex<Vec<ThreadId>>,
    changed: Condvar,
}

impl WorkerRendezvous {
    fn arrive(&self) {
        let current = std::thread::current().id();
        let mut threads = self.threads.lock().unwrap();
        if !threads.contains(&current) {
            threads.push(current);
        }
        self.changed.notify_all();
        let (threads, _) = self
            .changed
            .wait_timeout_while(threads, Duration::from_secs(10), |seen| seen.len() < 2)
            .unwrap();
        assert!(
            threads.len() >= 2,
            "actual task executor used {} threads; expected two overlapping workers",
            threads.len()
        );
    }

    /// Read actual participating threads; `rendezvous.count()` is independent of progress percentage.
    pub(super) fn count(&self) -> usize {
        let threads = self.threads.lock().unwrap();
        threads.len()
    }
}

struct ActualWireProgress {
    polls: AtomicUsize,
    cancel_at: Option<usize>,
    rendezvous: Option<Arc<WorkerRendezvous>>,
}

impl ProgressIndicatorHooks for ActualWireProgress {
    fn show(&self, _: &ProgressScope<'_>, position: f64, _: bool) {
        // Percentage is deliberately not used to establish operation completion.
        std::hint::black_box(position);
    }

    fn user_break(&self) -> bool {
        let poll = self.polls.fetch_add(1, Ordering::AcqRel) + 1;
        if let Some(rendezvous) = self.rendezvous.as_ref().filter(|_| poll >= 4) {
            // Perform plus the two outer-block polls precede these actual task polls.
            rendezvous.arrive();
        }
        self.cancel_at == Some(poll)
    }
}

/// Observe real task polls and optionally rendezvous; `wire_indicator(Some(4), None)` cancels one task poll.
pub(super) fn wire_indicator(
    cancel_at: Option<usize>,
    rendezvous: Option<Arc<WorkerRendezvous>>,
) -> ProgressIndicator {
    let hooks = ActualWireProgress {
        polls: AtomicUsize::new(0),
        cancel_at,
        rendezvous,
    };
    ProgressIndicator::new(Box::new(hooks))
}
