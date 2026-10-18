//! Lifecycle fixtures for the scoped replacement (#87, #106).
use super::{JoinedThreads, ScopedWorker, Worker, lock};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

#[test]
fn prepared_workers_use_inline_storage_through_thirty_two_workers() {
    for count in [0, 1, 32, 33] {
        let workers: Vec<_> = (0..count).map(|index| Worker::new(index, false)).collect();
        let (ready, failures) = super::prepare_workers(&workers);
        assert!(failures.is_empty());
        assert_eq!(ready.len(), count as usize);
        assert_eq!(ready.spilled(), count > 32);
        drop(ready);
        for worker in workers {
            worker.shut_down();
        }
    }
}

struct RefusedThreadStart;

#[test]
fn caller_only_dispatch_keeps_identity_borrows_and_sticky_failure() {
    let worker = Worker::new(7, true);
    let caller = std::thread::current().id();
    let calls = std::sync::atomic::AtomicUsize::new(0);
    let body = |index| {
        assert_eq!(index, 7);
        assert_eq!(std::thread::current().id(), caller);
        calls.fetch_add(1, Ordering::SeqCst);
    };
    assert!(super::dispatch(&[Arc::clone(&worker)], &body).is_empty());
    let failed = super::dispatch(&[Arc::clone(&worker)], &|_| panic!("caller failure"));
    assert_eq!(failed.len(), 1);
    let retained = super::dispatch(&[Arc::clone(&worker)], &body);
    assert_eq!(retained.len(), 1);
    assert_eq!(retained[0].message(), failed[0].message());
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    assert!(lock(&worker.executor).is_none());
}

impl RefusedThreadStart {
    fn create() -> Result<ScopedWorker, rayon::ThreadPoolBuildError> {
        rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .spawn_handler(|_| Err(std::io::Error::other("injected thread start refusal")))
            .build()
            .map(|pool| ScopedWorker {
                pool,
                _joined_threads: JoinedThreads(Vec::new()),
            })
    }
}

#[test]
fn failed_start_is_reported_and_the_next_launch_retries() {
    let worker = Worker::new(0, false);
    let failure = worker
        .executor_with(RefusedThreadStart::create)
        .err()
        .unwrap();
    assert_eq!(failure.exception_type(), "Standard_ProgramError");
    assert!(
        failure
            .message()
            .starts_with("Error: unable to start a pool thread (")
    );
    assert!(failure.message().contains("injected thread start refusal"));
    assert!(lock(&worker.executor).is_none());
    let executor = worker.executor().unwrap();
    assert_eq!(executor.pool.current_num_threads(), 1);
    drop(executor);
    worker.shut_down();
    assert!(lock(&worker.executor).is_none());
}

struct ThreadExit(Arc<AtomicBool>);

impl Drop for ThreadExit {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

thread_local! {
    static EXIT: std::cell::RefCell<Option<ThreadExit>> = const { std::cell::RefCell::new(None) };
}

#[test]
fn shutdown_waits_for_the_physical_worker_to_exit() {
    let worker = Worker::new(0, false);
    assert!(lock(&worker.executor).is_none(), "creation remains lazy");
    let exited = Arc::new(AtomicBool::new(false));
    let executor = worker.executor().unwrap();
    executor.pool.install(|| {
        EXIT.with(|slot| *slot.borrow_mut() = Some(ThreadExit(Arc::clone(&exited))));
    });
    assert!(!exited.load(Ordering::SeqCst));
    drop(executor);
    worker.shut_down();
    assert!(
        exited.load(Ordering::SeqCst),
        "join includes TLS destruction"
    );
}

#[test]
fn unused_worker_shutdown_does_not_start_a_thread() {
    let worker = Worker::new(0, false);
    worker.shut_down();
    assert!(lock(&worker.executor).is_none());
}
