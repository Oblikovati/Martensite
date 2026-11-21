//! Worker hints stay within the task count, the operation bound and `i32`.
use super::Parallelism;
use std::num::NonZeroUsize;

fn four() -> Parallelism {
    Parallelism::limited(NonZeroUsize::new(4).unwrap())
}

#[test]
fn unlimited_hint_follows_the_task_count_without_wrapping_to_serial() {
    let cases = [
        (0usize, 0),
        (1, 1),
        (32, 32),
        (i32::MAX as usize, i32::MAX),
        (i32::MAX as usize + 1, i32::MAX),
        (usize::MAX, i32::MAX),
    ];
    for (count, expected) in cases {
        assert_eq!(Parallelism::UNLIMITED.worker_cap(count), expected);
    }
}

#[test]
fn a_limit_bounds_the_hint_but_never_exceeds_the_task_count() {
    let cases = [(0usize, 0), (1, 1), (3, 3), (4, 4), (5, 4), (usize::MAX, 4)];
    for (count, expected) in cases {
        assert_eq!(four().worker_cap(count), expected);
    }
}

#[test]
fn a_single_task_runs_on_the_caller_and_serial_stays_serial() {
    assert_eq!(four().for_tasks(1), Parallelism::SERIAL);
    assert_eq!(four().for_tasks(2), four());
    assert_eq!(Parallelism::UNLIMITED.for_tasks(0), Parallelism::UNLIMITED);
    assert!(!Parallelism::SERIAL.for_tasks(8).enabled());
    assert!(four().enabled() && Parallelism::UNLIMITED.enabled());
}

#[test]
fn the_native_flag_maps_to_serial_or_unlimited() {
    assert_eq!(Parallelism::from_flag(false), Parallelism::SERIAL);
    assert_eq!(Parallelism::from_flag(true), Parallelism::UNLIMITED);
}
