//! Progress and parallel cases; twins in `occt/services.cpp`.
use super::{Case, Sample};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::osd::Parallel;
use std::hint::black_box;
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::Instant;

pub const CASES: &[Case] = &[
    Case {
        name: "progress_scopes",
        run: progress_scopes,
    },
    Case {
        name: "parallel_for_launch64",
        run: parallel_for_launch64,
    },
    Case {
        name: "parallel_for_items",
        run: parallel_for_items,
    },
];

/// One outer scope of `n` steps on a null range, with one inner scope per step.
fn progress_scopes(n: usize) -> Sample {
    let start = Instant::now();
    let mut steps = 0i64;
    let range = ProgressRange::default();
    let outer = ProgressScope::with_literal(&range, "outer", n as f64, false);
    for _ in 0..n {
        let inner_range = outer.next(1.0);
        let inner = ProgressScope::with_literal(&inner_range, "inner", 1.0, false);
        black_box(inner.next(1.0));
        steps += 1;
    }
    drop(outer);
    Sample::since(start, n, steps)
}

/// Launch cost: `n / 500` parallel loops of 64 items each.
fn parallel_for_launch64(n: usize) -> Sample {
    let launches = (n / 500).max(1);
    let sum = AtomicI64::new(0);
    let start = Instant::now();
    for _ in 0..launches {
        let add = |i: i32| {
            sum.fetch_add(i64::from(i), Ordering::Relaxed);
        };
        Parallel::for_range(0, 64, add, false).unwrap_or_else(|failure| panic!("{failure:?}"));
    }
    Sample::since(start, launches, sum.load(Ordering::Relaxed))
}

/// Per-item cost of one parallel loop over `n` items.
fn parallel_for_items(n: usize) -> Sample {
    let values: Vec<AtomicI64> = (0..n).map(|_| AtomicI64::new(1)).collect();
    let start = Instant::now();
    let update = |i: i32| {
        let cell = &values[i as usize];
        cell.store(cell.load(Ordering::Relaxed) * 2 + 1, Ordering::Relaxed);
    };
    Parallel::for_range(0, n as i32, update, false).unwrap_or_else(|failure| panic!("{failure:?}"));
    let sample = Sample::since(start, n, 0);
    let sum = values
        .iter()
        .map(|value| value.load(Ordering::Relaxed))
        .sum();
    Sample {
        checksum: sum,
        ..sample
    }
}
