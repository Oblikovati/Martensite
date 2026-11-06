//! The real leak check behind the `bug7287` scripts, whose own check on the first samples is
//! unstable on the reference build too: past its warm-up, the operation they repeat holds the heap
//! flat. A worker runs alone in a process of its own, because the counter is the whole process's,
//! and with stdout left alone, because captured output is itself heap.
use crate::support::{
    commands::make_box, heap_trend::heap_bytes, raster_session::RasterSession, session::Session,
    two_step::TwoStepBoolean,
};
use martensite_tkbo::bop_algo::Operation;
use std::process::{Child, Command, Stdio};

/// Long-run measurements reach their plateau within 1,000 iterations; 600 is past all but the
/// last fractions of a byte per iteration of it, which the bound below allows.
const WARM_UP: usize = 600;
/// Enough for a leak of one 16-byte block every other iteration to reach the bound.
const MEASURED: usize = 800;
/// The slope that rounds to 0 bytes per iteration.
const FLAT: f64 = 0.5;
const WORKER: &str = "heap_plateau::plateau_worker";
const OPERATION_ENV: &str = "MARTENSITE_HEAP_OPERATION";
const NO_TCACHE: &str = "glibc.malloc.tcache_count=0";

/// The least-squares slope of `values`; `slope(&[1., 3., 5.]) == 2.`.
fn slope(values: &[f64]) -> f64 {
    let count = values.len() as f64;
    let mean_x = (count - 1.) / 2.;
    let mean_y = values.iter().sum::<f64>() / count;
    let (mut above, mut below) = (0., 0.);
    for (index, value) in values.iter().enumerate() {
        let x = index as f64 - mean_x;
        above += x * (value - mean_y);
        below += x * x;
    }
    above / below
}

fn require_flat(values: &[f64], what: &str) {
    let measured = slope(values);
    assert!(
        measured.abs() < FLAT,
        "{what}: {measured} bytes per iteration over {} iterations, from {} to {}; expected 0",
        values.len(),
        values[0],
        values[values.len() - 1]
    );
}

fn operation_named(name: &str) -> Operation {
    match name {
        "tuc" => Operation::Cut21,
        "fuse" => Operation::Fuse,
        other => panic!("{OPERATION_ENV}={other:?}: expected tuc, fuse or leak"),
    }
}

/// The loop of `bug7287_3` and `bug7287_5`: `bop b1 b2` and the result command, with the
/// heap read after each once warm-up is over.
fn heap_after_each_cycle(kind: Operation) -> Vec<f64> {
    let mut session = RasterSession::new("heap_plateau");
    let s: &mut dyn Session = &mut session;
    make_box(s, "b1", [10., 10., 10.], [100., 100., 100.]);
    make_box(s, "b2", [0., 0., 0.], [50., 50., 50.]);
    let mut two_step = TwoStepBoolean::default();
    let mut samples = Vec::with_capacity(MEASURED);
    for iteration in 0..WARM_UP + MEASURED {
        two_step.bop(s, "b1", "b2");
        two_step.result(s, "r", kind);
        if iteration >= WARM_UP {
            samples.push(heap_bytes());
        }
    }
    samples
}

/// A deliberate leak of one block per iteration, for the control below.
fn heap_with_a_leak() -> Vec<f64> {
    let mut samples = Vec::with_capacity(MEASURED);
    for _ in 0..MEASURED {
        Box::leak(vec![0_u8; 64].into_boxed_slice());
        samples.push(heap_bytes());
    }
    samples
}

/// What the tests below start; `--ignored` alone never selects it without the environment.
#[test]
#[ignore = "started by the tests of this module"]
fn plateau_worker() {
    let Ok(name) = std::env::var(OPERATION_ENV) else {
        return;
    };
    let samples = if name == "leak" {
        heap_with_a_leak()
    } else {
        heap_after_each_cycle(operation_named(&name))
    };
    require_flat(&samples, &name);
}

fn started_worker(operation: &str, tunables: Option<&str>) -> Child {
    let mut command = Command::new(std::env::current_exe().unwrap());
    command.args([
        "--ignored",
        "--exact",
        WORKER,
        "--nocapture",
        "--test-threads=1",
    ]);
    command.env(OPERATION_ENV, operation);
    if let Some(value) = tunables {
        command.env("GLIBC_TUNABLES", value);
    }
    command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|cause| panic!("worker {operation}: {cause}"))
}

fn finished_worker(worker: Child, operation: &str) -> (bool, String) {
    let output = worker
        .wait_with_output()
        .unwrap_or_else(|cause| panic!("worker {operation}: {cause}"));
    let text = String::from_utf8_lossy(&output.stderr).into_owned();
    (output.status.success(), text)
}

fn require_plateaus(workers: &[(&str, Option<&str>)]) {
    let started: Vec<(Child, &str)> = workers
        .iter()
        .map(|(operation, tunables)| (started_worker(operation, *tunables), *operation))
        .collect();
    for (worker, operation) in started {
        let (passed, text) = finished_worker(worker, operation);
        assert!(passed, "worker {operation}: {text}");
    }
}

#[test]
fn the_repeated_cut_holds_the_heap_flat_past_warm_up() {
    require_plateaus(&[("tuc", None), ("tuc", Some(NO_TCACHE))]);
}

#[test]
fn the_repeated_fuse_holds_the_heap_flat_past_warm_up() {
    require_plateaus(&[("fuse", None), ("fuse", Some(NO_TCACHE))]);
}

/// Cases running side by side stay in processes of their own: none sees the others' heap.
#[test]
fn workers_side_by_side_each_keep_a_flat_heap() {
    let tunables = Some(NO_TCACHE);
    require_plateaus(&[("tuc", tunables), ("fuse", tunables), ("tuc", tunables)]);
}

#[test]
fn a_real_leak_is_refused() {
    let (passed, text) = finished_worker(started_worker("leak", None), "leak");
    assert!(!passed, "the leak control passed: {text}");
    assert!(text.contains("bytes per iteration"), "{text}");
}

#[test]
fn the_slope_of_a_line_and_of_a_constant() {
    assert_eq!(slope(&[1., 3., 5.]), 2.);
    assert_eq!(slope(&[7., 7., 7., 7.]), 0.);
}
