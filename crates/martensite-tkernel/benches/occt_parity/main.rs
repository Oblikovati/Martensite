//! Paired performance cases for the public `tkernel` API (#583).
//!
//! Every case here has a C++ twin of the same name in `occt/`, which does the same work
//! against OCCT's `TKernel` and must report the same checksum. `cargo xtask occt-parity`
//! builds both programs, runs them on pinned CPUs and compares the medians.
//!
//! `cargo bench -p martensite-tkernel --bench occt_parity` measures (Cargo passes `--bench`);
//! without `--bench` every case runs once at a small size, as a smoke test.
//! `--filter <text>` keeps the cases whose name contains the text.
use std::hint::black_box;
use std::time::Instant;

mod collections;
mod objects;
mod services;
mod text;

/// One timed run: the measured nanoseconds, the operation count and the work checksum.
pub struct Sample {
    nanos: f64,
    ops: usize,
    checksum: i64,
}

impl Sample {
    /// Closes a timed section that started at `start`; `Sample::since(start, n, sum)`.
    pub fn since(start: Instant, ops: usize, checksum: i64) -> Self {
        Self {
            nanos: start.elapsed().as_nanos() as f64,
            ops,
            checksum,
        }
    }
}

/// A named case; `run` receives the item count `n` and does its work once.
#[derive(Clone, Copy)]
pub struct Case {
    /// The name shared with the C++ twin; `"datamap_bind"`.
    pub name: &'static str,
    /// The work of one timed run for `n` items.
    pub run: fn(usize) -> Sample,
}

struct Mode {
    items: usize,
    repeats: usize,
    filter: Option<String>,
}

impl Mode {
    fn from_args() -> Self {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let measure = args.iter().any(|arg| arg == "--bench");
        let filter = args
            .iter()
            .position(|arg| arg == "--filter")
            .and_then(|at| args.get(at + 1).cloned());
        let (items, repeats) = if measure { (1_000_000, 7) } else { (1_000, 1) };
        Self {
            items,
            repeats,
            filter,
        }
    }

    fn selects(&self, case: &Case) -> bool {
        self.filter
            .as_deref()
            .is_none_or(|text| case.name.contains(text))
    }
}

/// Runs a case `repeats` times and prints `name<TAB>median ns per op<TAB>checksum`.
fn report(case: &Case, mode: &Mode) {
    let mut times = Vec::with_capacity(mode.repeats);
    let mut checksum = 0;
    for _ in 0..mode.repeats {
        let sample = black_box((case.run)(mode.items));
        times.push(sample.nanos / sample.ops as f64);
        checksum = sample.checksum;
    }
    times.sort_by(f64::total_cmp);
    println!("{}\t{:.3}\t{}", case.name, times[times.len() / 2], checksum);
}

fn cases() -> Vec<Case> {
    [
        collections::CASES,
        objects::CASES,
        text::CASES,
        services::CASES,
    ]
    .concat()
}

fn main() {
    let mode = Mode::from_args();
    for case in cases().iter().filter(|case| mode.selects(case)) {
        report(case, &mode);
    }
}
