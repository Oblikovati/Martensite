//! THROWAWAY #86 measurement harness: one identical release workload on the current
//! `ncollection::List` and on the safe prototype in `list_safe_storage.rs`.
//! Allocation instrumentation uses the established `alloc_tracker` dependency.
//!
//! Build (from the repository root):
//!   cargo run --release --manifest-path experiments/list_safe_storage/Cargo.toml
#![forbid(unsafe_code)]
#[path = "list_safe_storage.rs"]
#[allow(dead_code)]
mod prototype;

use alloc_tracker::{Allocator, Operation, Session, ThreadSpan};
use martensite_tkernel::ncollection::List;
use prototype::{Ledger, SafeList};
use std::alloc::System;
use std::hint::black_box;
use std::time::Instant;

#[global_allocator]
static GLOBAL: Allocator<System> = Allocator::system();

const N: u64 = 100_000;
const PHASES: [&str; 6] = [
    "build",
    "traverse10",
    "iter_mut",
    "splice",
    "retain_half",
    "drop",
];
type Sample = [(u128, usize); 6];

struct Probe {
    sample: Sample,
    phase: usize,
    started: Instant,
    session: Session,
    operations: [Operation; 6],
    span: Option<ThreadSpan>,
}
impl Probe {
    fn new() -> Self {
        let session = Session::new().no_stdout().no_file();
        let operations = PHASES.map(|label| session.operation(label));
        Self {
            sample: [(0, 0); 6],
            phase: 0,
            started: Instant::now(),
            session,
            operations,
            span: None,
        }
    }
    fn start(&mut self) {
        self.span = Some(self.operations[0].measure_thread().iterations(1));
        self.started = Instant::now();
    }
    fn mark(&mut self) {
        let elapsed = self.started.elapsed().as_nanos();
        drop(self.span.take());
        self.sample[self.phase].0 = elapsed;
        self.phase += 1;
        self.span = self
            .operations
            .get(self.phase)
            .map(|op| op.measure_thread().iterations(1));
        self.started = Instant::now();
    }
    fn finish(mut self) -> Sample {
        for (name, measurement) in self.session.to_report().operations() {
            if let Some(phase) = PHASES.iter().position(|label| *label == name) {
                self.sample[phase].1 = measurement.total_allocations_count() as usize;
            }
        }
        self.sample
    }
}

macro_rules! workload {
    ($name:ident, $make:expr) => {
        fn $name() -> (Sample, usize) {
            let mut probe = Probe::new();
            let peak_session = Session::new().no_stdout().no_file();
            let peak_operation = peak_session.operation("whole");
            let peak_span = peak_operation.measure_thread().iterations(1);
            probe.start();
            let mut list = $make;
            (0..N).for_each(|v| _ = list.append(v));
            probe.mark();
            (0..10).for_each(|_| _ = black_box(list.iter().sum::<u64>()));
            probe.mark();
            list.iter_mut().for_each(|v| *v += 1);
            probe.mark();
            let mut second = $make;
            (0..N / 2).for_each(|v| _ = second.append(v));
            list.append_list(&mut second);
            probe.mark();
            list.retain(|v| v % 2 == 0);
            black_box(list.iter().count());
            probe.mark();
            drop((list, second));
            probe.mark();
            drop(peak_span);
            let peak = peak_session
                .to_report()
                .operations()
                .filter_map(|(_, op)| op.peak_outstanding_bytes())
                .fold(0.0, f64::max);
            (probe.finish(), peak as usize)
        }
    };
}
workload!(current, List::<u64>::default());
workload!(safe, {
    thread_local!(static LEDGER: std::sync::Arc<Ledger> = Ledger::new(false));
    SafeList::<u64>::with_ledger(LEDGER.with(std::sync::Arc::clone))
});

fn report(name: &str, runs: Vec<(Sample, usize)>) {
    for (phase, label) in PHASES.iter().enumerate() {
        let mut times: Vec<u128> = runs.iter().map(|run| run.0[phase].0).collect();
        times.sort_unstable();
        let allocations = runs[0].0[phase].1;
        println!(
            "{name} {label}: median_us={} allocations={allocations}",
            times[times.len() / 2] / 1000
        );
    }
    let total: Vec<u128> = runs
        .iter()
        .map(|run| run.0.iter().map(|p| p.0).sum())
        .collect();
    println!(
        "{name} total_median_us={} peak_bytes={}",
        median(total) / 1000,
        runs[0].1
    );
}
fn median(mut values: Vec<u128>) -> u128 {
    values.sort_unstable();
    values[values.len() / 2]
}
fn main() {
    let runs = 9;
    let (mut a, mut b) = (Vec::new(), Vec::new());
    for _ in 0..runs {
        a.push(current());
        b.push(safe());
    }
    report("current", a);
    report("safe", b);
}

#[test]
fn phase_counts_exclude_setup_and_peak_does_not_sum_disjoint_allocations() {
    let mut probe = Probe::new();
    let session = Session::new().no_stdout().no_file();
    let operation = session.operation("whole");
    let whole = operation.measure_thread().iterations(1);
    probe.start();
    for _ in 0..6 {
        drop(black_box(vec![7u64; 100]));
        probe.mark();
    }
    drop(whole);
    for (_, measured) in session.to_report().operations() {
        assert_eq!(measured.total_allocations_count(), 6);
        assert_eq!(measured.peak_outstanding_bytes(), Some(800.0));
    }
    assert!(probe.finish().iter().all(|(_, count)| *count == 1));
}
