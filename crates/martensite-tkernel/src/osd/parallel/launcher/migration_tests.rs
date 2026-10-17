//! Persistent-pool compatibility fixtures before removing borrowed-job erasure (#87).
use super::{Launcher, ThreadPool};
use std::{collections::BTreeMap, sync::Mutex, thread::ThreadId};

mod workload;

fn participant_ids(launcher: &Launcher<'_>) -> BTreeMap<i32, ThreadId> {
    let participants = Mutex::new(BTreeMap::new());
    launcher
        .perform_job(&|index| {
            participants
                .lock()
                .unwrap()
                .insert(index, std::thread::current().id());
        })
        .unwrap();
    participants.into_inner().unwrap()
}

#[test]
fn repeated_jobs_keep_worker_threads_and_caller_index() {
    let pool = ThreadPool::new(3);
    let launcher = Launcher::new(&pool, 3);
    let first = participant_ids(&launcher);
    assert_eq!(first.len(), 3);
    assert_eq!(first[&2], std::thread::current().id());
    assert_ne!(first[&0], first[&1]);
    assert_ne!(first[&0], first[&2]);
    assert_ne!(first[&1], first[&2]);
    for _ in 0..20 {
        assert_eq!(participant_ids(&launcher), first);
    }
}

#[test]
fn reservations_force_busy_fallback_and_release_disables_jobs() {
    let pool = ThreadPool::new(3);
    let mut reserved = Launcher::new(&pool, 3);
    let busy = Launcher::new(&pool, 3);
    assert_eq!(busy.nb_threads(), 1);
    assert_eq!(participant_ids(&busy)[&0], std::thread::current().id());
    reserved.release();
    assert_eq!(reserved.nb_threads(), 0);
    assert!(participant_ids(&reserved).is_empty());
    let available = Launcher::new(&pool, 3);
    assert_eq!(available.nb_threads(), 3);
    assert_eq!(participant_ids(&available).len(), 3);
}
