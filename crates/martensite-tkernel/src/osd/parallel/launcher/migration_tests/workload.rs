//! Accepted workload shared by baseline and candidate measurements (#87).
use super::{Launcher, ThreadPool};
use std::sync::atomic::{AtomicUsize, Ordering};

fn measured(launcher: &Launcher<'_>) -> u128 {
    let completed = AtomicUsize::new(0);
    let started = std::time::Instant::now();
    for _ in 0..1_000 {
        launcher
            .perform(0, 64, |_, _| {
                completed.fetch_add(1, Ordering::SeqCst);
            })
            .unwrap();
    }
    let elapsed = started.elapsed().as_nanos();
    assert_eq!(completed.load(Ordering::SeqCst), 64_000);
    elapsed
}

#[test]
#[ignore = "manual release measurement, not a timing acceptance assertion"]
fn persistent_range_workload() {
    for participants in [1, 3, 8, 32] {
        let pool = ThreadPool::new(participants);
        let launcher = Launcher::new(&pool, participants);
        launcher.perform(0, 64, |_, _| {}).unwrap();
        for sample in 0..3 {
            let elapsed_ns = measured(&launcher);
            println!(
                "participants={participants} sample={sample} launches=1000 items=64000 elapsed_ns={elapsed_ns}"
            );
        }
    }
}

#[test]
#[ignore = "isolated large-pool stack assessment, not a default CI workload"]
fn persistent_large_pool_workload() {
    let pool = ThreadPool::new(128);
    let launcher = Launcher::new(&pool, 128);
    let elapsed_ns = measured(&launcher);
    println!("participants=128 launches=1000 items=64000 elapsed_ns={elapsed_ns}");
}
