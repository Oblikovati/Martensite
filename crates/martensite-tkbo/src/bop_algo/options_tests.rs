//! The worker bound applies only in parallel mode and travels with the options (#496).
use super::{Options, Parallelism};
use std::num::NonZeroUsize;

#[test]
fn parallel_options_without_a_bound_use_one_worker_per_task() {
    let mut options = Options::new();
    options.set_run_parallel(false);
    assert_eq!(options.parallelism(), Parallelism::SERIAL);
    options.set_run_parallel(true);
    assert_eq!(options.parallelism(), Parallelism::UNLIMITED);
}

#[test]
fn a_worker_bound_applies_only_in_parallel_mode_and_survives_cloning() {
    let two = NonZeroUsize::new(2).unwrap();
    let mut options = Options::new();
    options.limit_workers(two);
    options.set_run_parallel(false);
    assert_eq!(options.parallelism(), Parallelism::SERIAL);
    options.set_run_parallel(true);
    assert_eq!(options.parallelism(), Parallelism::limited(two));
    assert_eq!(options.clone().parallelism(), Parallelism::limited(two));
    options.clear();
    assert_eq!(options.parallelism(), Parallelism::limited(two));
}
