//! THROWAWAY #86 measurement harness: one identical release workload on the current
//! `ncollection::List` and on the safe prototype in `list_safe_storage.rs`.
//! The counting global allocator is measurement tooling only (it forwards to `System`); the
//! prototype module keeps its own `#![forbid(unsafe_code)]`.
//!
//! Build (from the repository root):
//!   cargo build --release -p martensite-tkernel
//!   rustc --edition=2024 -C opt-level=3 -L target/release/deps \
//!     --extern martensite_tkernel=$(ls target/release/deps/libmartensite_tkernel-*.rlib) \
//!     experiments/list_safe_storage_bench.rs -o /tmp/list-bench && /tmp/list-bench
#[path = "list_safe_storage.rs"]
#[allow(dead_code)]
mod prototype;

use martensite_tkernel::ncollection::List;
use prototype::{Ledger, SafeList};
use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::time::Instant;

struct Counting;
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
fn grow(bytes: usize) {
    ALLOCATIONS.fetch_add(1, Relaxed);
    let live = LIVE.fetch_add(bytes, Relaxed) + bytes;
    PEAK.fetch_max(live, Relaxed);
}
// SAFETY: every call forwards unchanged to System; the counters never touch the memory.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        grow(layout.size());
        // SAFETY: forwarded caller contract.
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        LIVE.fetch_sub(layout.size(), Relaxed);
        // SAFETY: forwarded caller contract.
        unsafe { System.dealloc(pointer, layout) }
    }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        LIVE.fetch_sub(layout.size(), Relaxed);
        grow(size);
        // SAFETY: forwarded caller contract.
        unsafe { System.realloc(pointer, layout, size) }
    }
}
#[global_allocator]
static GLOBAL: Counting = Counting;

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
    allocations: usize,
}
impl Probe {
    fn new() -> Self {
        PEAK.store(LIVE.load(Relaxed), Relaxed);
        let allocations = ALLOCATIONS.load(Relaxed);
        Self {
            sample: [(0, 0); 6],
            phase: 0,
            started: Instant::now(),
            allocations,
        }
    }
    fn mark(&mut self) {
        let allocations = ALLOCATIONS.load(Relaxed);
        let elapsed = self.started.elapsed().as_nanos();
        self.sample[self.phase] = (elapsed, allocations - self.allocations);
        (self.phase, self.allocations, self.started) =
            (self.phase + 1, allocations, Instant::now());
    }
}

macro_rules! workload {
    ($name:ident, $make:expr) => {
        fn $name() -> (Sample, usize) {
            let base = LIVE.load(Relaxed);
            let mut probe = Probe::new();
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
            (probe.sample, PEAK.load(Relaxed) - base)
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
