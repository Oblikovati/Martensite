//! Lifecycle fixtures for the scoped replacement (#87, #106).
use super::{ScopedWorker, Worker, lock};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

struct RefusedThreadStart;

impl RefusedThreadStart {
    fn create() -> Result<ScopedWorker, rayon::ThreadPoolBuildError> {
        rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .spawn_handler(|_| Err(std::io::Error::other("injected thread start refusal")))
            .build()
            .map(|pool| ScopedWorker {
                pool: Some(pool),
                threads: Vec::new(),
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
    assert!(executor.pool.is_some());
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
    executor.pool.as_ref().unwrap().install(|| {
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
