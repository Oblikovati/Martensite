//! The cases, their timing protocol and the output line shared with `occt/main.cpp`.
#[path = "ais.rs"]
mod ais;
#[path = "frames.rs"]
mod frames;
#[path = "geometry.rs"]
mod geometry;
#[path = "../../examples/support/native_window.rs"]
mod native_window;
#[path = "stage.rs"]
mod stage;
#[path = "transfer.rs"]
mod transfer;

use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::Graphic3dGraphicDriver as _;
use martensite_vulkan::{Graphic3dRenderDriver, VulkanOptions};
use std::time::Instant;

/// The shared driver every case draws with; startup cases make their own.
pub struct Bench {
    /// One Vulkan device for the process, as one OpenGL driver serves the C++ twins.
    pub driver: Handle<Graphic3dRenderDriver>,
}

/// The sizes of one program run: `--bench` measures, otherwise a two-frame smoke test.
pub struct Mode {
    /// Frames (or other operations) in one timed run.
    pub frames: usize,
    /// Timed runs per case; the median is reported.
    pub repeats: usize,
    /// Untimed frames before the first timed run.
    pub warmup: usize,
    filter: Option<String>,
}

/// One case's timed runs, in nanoseconds per operation, and its work checksum.
pub struct Measured {
    /// Nanoseconds per operation of every timed run.
    pub nanos: Vec<f64>,
    /// The work checksum; the C++ twin must report the same one.
    pub checksum: i64,
}

/// A named case; `run` builds its scene once and times `mode.repeats` runs.
#[derive(Clone, Copy)]
pub struct Case {
    /// The name shared with the C++ twin; `"frame_mesh_1m"`.
    pub name: &'static str,
    /// Builds, measures and checks the case.
    pub run: fn(&Bench, &Mode) -> Measured,
}

impl Mode {
    fn from_args() -> Self {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let measure = args.iter().any(|arg| arg == "--bench");
        let filter = args
            .iter()
            .position(|arg| arg == "--filter")
            .and_then(|at| args.get(at + 1).cloned());
        let (frames, repeats, warmup) = if measure { (100, 7, 10) } else { (2, 1, 1) };
        let frames = args
            .iter()
            .position(|arg| arg == "--frames")
            .and_then(|at| args.get(at + 1))
            .map_or(frames, |text| frame_count(text));
        Self {
            frames,
            repeats,
            warmup: warmup.min(frames),
            filter,
        }
    }

    fn selects(&self, case: &Case) -> bool {
        self.filter
            .as_deref()
            .is_none_or(|text| case.name.contains(text))
    }

    /// The operations of one timed run, scaled from the frame count; `mode.scaled(4)`.
    pub fn scaled(&self, divisor: usize) -> usize {
        (self.frames / divisor).max(1)
    }
}

/// Parses `--frames <n>`; `frame_count("10") == 10`.
fn frame_count(text: &str) -> usize {
    match text.parse::<usize>() {
        Ok(count) if count > 0 => count,
        _ => panic!("received --frames {text:?}; expected a positive frame count"),
    }
}

/// Times `mode.repeats` runs of `ops` calls of `op`, which receives a running index.
/// Example: `timed_runs(mode, mode.frames, |frame| stage.redraw_orbit(frame))`.
pub fn timed_runs(mode: &Mode, ops: usize, mut op: impl FnMut(usize)) -> Vec<f64> {
    (0..mode.repeats)
        .map(|repeat| {
            let start = Instant::now();
            for index in 0..ops {
                op(repeat * ops + index);
            }
            start.elapsed().as_nanos() as f64 / ops as f64
        })
        .collect()
}

/// Runs a case and prints `name<TAB>median ns per op<TAB>checksum`.
fn report(bench: &Bench, case: &Case, mode: &Mode) {
    let mut measured = (case.run)(bench, mode);
    measured.nanos.sort_by(f64::total_cmp);
    let median = measured.nanos[measured.nanos.len() / 2];
    println!("{}\t{:.3}\t{}", case.name, median, measured.checksum);
}

fn cases() -> Vec<Case> {
    [
        transfer::STARTUP,
        frames::CASES,
        ais::CASES,
        transfer::CASES,
    ]
    .concat()
}

/// The vertical-sync-free driver of one process; `new_driver()`.
pub fn new_driver() -> Handle<Graphic3dRenderDriver> {
    let options = VulkanOptions {
        validation: false,
        device_index: None,
    };
    let driver = Graphic3dRenderDriver::new(options)
        .unwrap_or_else(|error| panic!("Vulkan driver: {error:?}; expected a device"));
    driver.set_vertical_sync(false);
    driver
}

/// Builds the shared driver and reports every selected case.
pub fn run() {
    let mode = Mode::from_args();
    let bench = Bench {
        driver: new_driver(),
    };
    for case in cases().iter().filter(|case| mode.selects(case)) {
        report(&bench, case, &mode);
    }
}
