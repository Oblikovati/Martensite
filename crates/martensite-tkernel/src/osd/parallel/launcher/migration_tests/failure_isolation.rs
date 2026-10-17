//! Per-invocation worker failures must not cross concurrent borrowed jobs (#106).
use super::{Launcher, ThreadPool};
use std::sync::{
    Barrier,
    atomic::{AtomicUsize, Ordering},
};

fn failing_launch(launcher: &Launcher<'_>, label: usize) {
    let finished = AtomicUsize::new(0);
    let failure = launcher
        .perform_job(&|index| {
            assert_ne!(index, 0, "worker failure for launch {label}");
            finished.fetch_add(1, Ordering::SeqCst);
        })
        .unwrap_err();
    assert!(
        failure
            .message()
            .contains(&format!("worker failure for launch {label}"))
    );
    assert!(!failure.message().contains("Multiple exceptions"));
    assert_eq!(finished.load(Ordering::SeqCst), 2);
}

#[test]
fn concurrent_worker_failures_belong_to_their_invocation() {
    let pool = ThreadPool::new(3);
    let launcher = Launcher::new(&pool, 3);
    let start = Barrier::new(4);
    std::thread::scope(|scope| {
        for label in 0..4 {
            let (launcher, start) = (&launcher, &start);
            scope.spawn(move || {
                start.wait();
                for _ in 0..100 {
                    failing_launch(launcher, label);
                }
            });
        }
    });
    launcher.perform_job(&|_| {}).unwrap();
}

#[test]
fn nested_worker_failures_wait_and_do_not_poison_outer_jobs() {
    let pool = ThreadPool::new(3);
    let launcher = Launcher::new(&pool, 3);
    launcher
        .perform_job(&|index| failing_launch(&launcher, index as usize))
        .unwrap();
    launcher.perform_job(&|_| {}).unwrap();
}

#[test]
fn balanced_dispatch_preserves_failure_index_order() {
    let pool = ThreadPool::new(8);
    let launcher = Launcher::new(&pool, 8);
    let failure = launcher
        .perform_job(&|index| panic!("participant {index}"))
        .unwrap_err();
    let messages: Vec<String> = (0..8)
        .map(|index| format!("panic: participant {index}"))
        .collect();
    assert_eq!(
        failure.message(),
        format!("Multiple exceptions:\n{}", messages.join("\n"))
    );
}
