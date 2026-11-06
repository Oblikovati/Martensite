//! Exact same-platform factory snapshots precede dependency relocation.
mod sphere_contract;
use sphere_contract::{capture, cases, compare_or_record, worker};

#[test]
fn sphere_factory_matches_owned_baseline() {
    for case in cases() {
        compare_or_record(case, &capture(case));
    }
}

#[test]
fn sphere_snapshot_worker() {
    worker();
}
