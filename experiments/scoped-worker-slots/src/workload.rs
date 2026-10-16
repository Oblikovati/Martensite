//! Identical accepted range workload for the existing launcher and scoped-slot candidate.
use super::{Slot, dispatch};
use martensite_tkernel::osd::{Launcher, ThreadPool};
use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};

fn scoped_range(slots: &[Slot], completed: &AtomicUsize) {
    let next = AtomicI32::new(0);
    dispatch(slots, 0, &|_| {
        while next.fetch_add(1, Ordering::SeqCst) < 64 {
            completed.fetch_add(1, Ordering::SeqCst);
        }
    });
}

fn measured(mut launch: impl FnMut(&AtomicUsize)) -> u128 {
    let completed = AtomicUsize::new(0);
    launch(&completed);
    completed.store(0, Ordering::SeqCst);
    let started = std::time::Instant::now();
    for _ in 0..1_000 {
        launch(&completed);
    }
    let elapsed = started.elapsed().as_nanos();
    assert_eq!(completed.load(Ordering::SeqCst), 64_000);
    elapsed
}

#[test]
#[ignore = "manual release comparison, not a timing acceptance assertion"]
fn compare_persistent_range_workload() {
    let pool = ThreadPool::new(3);
    let launcher = Launcher::new(&pool, 3);
    let slots = [Slot::new().unwrap(), Slot::new().unwrap()];
    for _ in 0..3 {
        let baseline = measured(|completed| {
            launcher
                .perform(0, 64, |_, _| {
                    completed.fetch_add(1, Ordering::SeqCst);
                })
                .unwrap();
        });
        let candidate = measured(|completed| scoped_range(&slots, completed));
        println!(
            "range_workload participants=3 launches=1000 items=64000 baseline_ns={baseline} candidate_ns={candidate}"
        );
    }
}
