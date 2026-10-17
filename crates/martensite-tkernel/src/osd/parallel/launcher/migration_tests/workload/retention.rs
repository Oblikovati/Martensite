//! Identical pool teardown workload for external heap-retention measurements.
use super::{AtomicUsize, Launcher, Ordering, ThreadPool};

fn shutdown_cycles(cycles: usize) {
    let completed = AtomicUsize::new(0);
    for _ in 0..cycles {
        let pool = ThreadPool::new(8);
        let launcher = Launcher::new(&pool, 8);
        launcher
            .perform(0, 64, |_, _| {
                completed.fetch_add(1, Ordering::SeqCst);
            })
            .unwrap();
    }
    assert_eq!(completed.load(Ordering::SeqCst), cycles * 64);
}

#[test]
#[ignore = "external memory profiling control"]
fn one_pool_shutdown() {
    shutdown_cycles(1);
}

#[test]
#[ignore = "external memory profiling repeated teardown"]
fn hundred_pool_shutdowns() {
    shutdown_cycles(100);
}

#[test]
#[ignore = "external memory profiling extended teardown"]
fn thousand_pool_shutdowns() {
    shutdown_cycles(1_000);
}
