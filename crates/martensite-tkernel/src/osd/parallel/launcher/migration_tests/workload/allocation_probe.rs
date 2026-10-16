//! Manual differential allocation probe; compare warmed runs with external profiling.
use super::{AtomicUsize, Launcher, Ordering, ThreadPool};

fn probe_count(name: &str, fallback: usize) -> usize {
    std::env::var(name)
        .map(|value| {
            value
                .parse()
                .unwrap_or_else(|_| panic!("{name}={value:?}; expected an unsigned probe count"))
        })
        .unwrap_or(fallback)
}

#[test]
#[ignore = "external allocation profiling, not a performance acceptance assertion"]
fn warmed_dispatch_allocation_probe() {
    let participants = probe_count("MARTENSITE_PROBE_PARTICIPANTS", 2);
    let launches = probe_count("MARTENSITE_PROBE_LAUNCHES", 1);
    let pool = ThreadPool::new(i32::try_from(participants).unwrap());
    let launcher = Launcher::new(&pool, i32::try_from(participants).unwrap());
    launcher.perform(0, 1, |_, _| {}).unwrap();
    let completed = AtomicUsize::new(0);
    for _ in 0..launches {
        launcher
            .perform(0, 1, |_, _| {
                completed.fetch_add(1, Ordering::SeqCst);
            })
            .unwrap();
    }
    assert_eq!(completed.load(Ordering::SeqCst), launches);
}
