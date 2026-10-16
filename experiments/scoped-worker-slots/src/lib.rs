//! Throwaway #87 feasibility experiment, not a production worker-pool replacement.
//! Run `cargo test --manifest-path experiments/scoped-worker-slots/Cargo.toml`.
#![cfg(test)]
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::thread::JoinHandle;

mod concurrent_launches;
mod workload;

struct Slot {
    pool: Option<ThreadPool>,
    threads: Vec<JoinHandle<()>>,
}

impl Slot {
    fn new() -> Result<Self, rayon::ThreadPoolBuildError> {
        let mut threads = Vec::new();
        let pool = ThreadPoolBuilder::new()
            .num_threads(1)
            .spawn_handler(|worker| {
                threads.push(std::thread::Builder::new().spawn(|| worker.run())?);
                Ok(())
            })
            .build()?;
        Ok(Self {
            pool: Some(pool),
            threads,
        })
    }
}

impl Drop for Slot {
    fn drop(&mut self) {
        drop(self.pool.take());
        for thread in self.threads.drain(..) {
            let _ = thread.join();
        }
    }
}

fn dispatch(slots: &[Slot], index: usize, body: &(dyn Fn(usize) + Sync)) {
    let Some((slot, rest)) = slots.split_first() else {
        body(index);
        return;
    };
    if let Some(pool) = &slot.pool {
        pool.in_place_scope(|scope| {
            scope.spawn(move |_| body(index));
            dispatch(rest, index + 1, body);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{Slot, dispatch};
    use std::{collections::BTreeMap, sync::Mutex, thread::ThreadId};

    fn identities(slots: &[Slot]) -> BTreeMap<usize, ThreadId> {
        let seen = Mutex::new(BTreeMap::new());
        dispatch(slots, 0, &|index| {
            seen.lock()
                .unwrap()
                .insert(index, std::thread::current().id());
        });
        seen.into_inner().unwrap()
    }

    #[test]
    fn borrowed_jobs_keep_worker_identity_and_calling_thread() {
        let slots = [Slot::new().unwrap(), Slot::new().unwrap()];
        let first = identities(&slots);
        assert_eq!(first.len(), 3);
        assert_eq!(first[&2], std::thread::current().id());
        assert_ne!(first[&0], first[&1]);
        assert_ne!(first[&0], first[&2]);
        assert_ne!(first[&1], first[&2]);
        for _ in 0..100 {
            assert_eq!(identities(&slots), first);
        }
    }

    #[test]
    fn jobs_borrow_mutable_elements_without_static_bound() {
        let slots = [Slot::new().unwrap(), Slot::new().unwrap()];
        let mut values = vec![0; 100];
        let remaining = Mutex::new(values.iter_mut());
        dispatch(&slots, 0, &|_| {
            while let Some(value) = remaining.lock().unwrap().next() {
                *value += 1;
            }
        });
        assert_eq!(values, vec![1; 100]);
    }

    #[test]
    fn a_worker_panic_waits_for_other_borrowed_jobs() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let slots = [Slot::new().unwrap(), Slot::new().unwrap()];
        let finished = AtomicUsize::new(0);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            dispatch(&slots, 0, &|index| {
                assert_ne!(index, 0, "injected worker panic");
                finished.fetch_add(1, Ordering::SeqCst);
            });
        }));
        assert!(result.is_err());
        assert_eq!(finished.load(Ordering::SeqCst), 2);
    }
}
