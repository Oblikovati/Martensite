//! Bounded concurrent-borrow stress for the experiment, not the existing raw-job pool.
use super::{Slot, dispatch};
use std::sync::{
    Barrier,
    atomic::{AtomicUsize, Ordering},
};

fn repeated_borrowed_launches(slots: &[Slot], start: &Barrier) {
    let completed = AtomicUsize::new(0);
    start.wait();
    for _ in 0..1_000 {
        dispatch(slots, 0, &|_| {
            completed.fetch_add(1, Ordering::SeqCst);
        });
    }
    assert_eq!(completed.load(Ordering::SeqCst), 3_000);
}

#[test]
fn simultaneous_callers_keep_their_borrowed_jobs_separate() {
    let slots = [Slot::new().unwrap(), Slot::new().unwrap()];
    let start = Barrier::new(4);
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| repeated_borrowed_launches(&slots, &start));
        }
    });
}
