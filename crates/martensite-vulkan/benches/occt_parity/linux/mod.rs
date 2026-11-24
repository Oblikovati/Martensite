//! The cases, their timing protocol and the output line shared with `occt/main.cpp`.
mod ais;
mod frames;
mod geometry;
#[path = "../../../examples/support/native_window.rs"]
mod native_window;
mod stage;
mod transfer;
mod work;

use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::Graphic3dGraphicDriver as _;
use martensite_vulkan::{
    Graphic3dRenderDriver, RenderBehaviorConfiguration, RenderMeshVertexBehavior, VulkanOptions,
};
use std::cell::RefCell;
use std::time::Instant;
use work::TimedWork;

/// The shared driver every case draws with; startup cases make their own.
pub struct Bench {
    /// One Vulkan device for the process, as one OpenGL driver serves the C++ twins.
    pub driver: Handle<Graphic3dRenderDriver>,
    /// The device work of the current case's timed windows.
    work: RefCell<TimedWork>,
}

impl Bench {
    /// Times one window of `ops` operations run by `window`; nanoseconds per operation.
    /// The window ends when the GPU has finished its frames, as the C++ twin's `glFinish`.
    /// The device work of the window counts towards the case's work column.
    /// Example: `bench.timed(ops, || for frame in 0..ops { stage.redraw_orbit(frame) })`.
    pub fn timed(&self, ops: usize, window: impl FnOnce()) -> f64 {
        let before = self.driver.device_work();
        let start = Instant::now();
        window();
        self.finish_frames();
        let nanos = start.elapsed().as_nanos() as f64 / ops as f64;
        let work = self.driver.device_work().since(&before);
        self.work.borrow_mut().add(work, ops);
        nanos
    }
    /// Waits for the frames on the GPU; a late fault fails the case.
    pub fn finish_frames(&self) {
        if let Err(error) = self.driver.finish_frames() {
            panic!("frames failed on the GPU: {error:?}; expected finished frames");
        }
    }
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
/// Example: `timed_runs(bench, mode, mode.frames, |frame| stage.redraw_orbit(frame))`.
pub fn timed_runs(bench: &Bench, mode: &Mode, ops: usize, mut op: impl FnMut(usize)) -> Vec<f64> {
    (0..mode.repeats)
        .map(|repeat| {
            bench.timed(ops, || {
                for index in 0..ops {
                    op(repeat * ops + index);
                }
            })
        })
        .collect()
}

/// Times `mode.repeats` single operations, each on a new state that `build` makes before
/// the timed window; one untimed build and operation run first, as the warm-up.
/// Example: `timed_rebuilds(bench, mode, || box_grid(&stage, GRID_10K), remove_all)`.
pub fn timed_rebuilds<State>(
    bench: &Bench,
    mode: &Mode,
    mut build: impl FnMut() -> State,
    mut op: impl FnMut(State),
) -> Vec<f64> {
    for _ in 0..mode.warmup.min(1) {
        op(build());
    }
    (0..mode.repeats)
        .map(|_| {
            let state = build();
            // The build's own GPU work must not end inside the timed window.
            bench.finish_frames();
            bench.timed(1, || op(state))
        })
        .collect()
}

/// Runs a case and prints `name<TAB>median ns per op<TAB>checksum<TAB>work`.
fn report(bench: &Bench, case: &Case, mode: &Mode) {
    let mut measured = (case.run)(bench, mode);
    measured.nanos.sort_by(f64::total_cmp);
    let median = measured.nanos[measured.nanos.len() / 2];
    let work = bench.work.take().column();
    println!("{}\t{median:.3}\t{}\t{work}", case.name, measured.checksum);
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
    let quantized = std::env::var_os("MARTENSITE_594_QUANTIZED_MESH").is_some();
    let meshlets = std::env::var_os("MARTENSITE_594_MESHLETS").is_some();
    let driver = if quantized || meshlets {
        Graphic3dRenderDriver::with_render_behavior_configuration(
            options,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            RenderBehaviorConfiguration::default()
                .with_mesh_vertices(if quantized {
                    RenderMeshVertexBehavior::Quantized16OctV2
                } else {
                    Default::default()
                })
                .with_meshlet_draws(meshlets),
        )
    } else {
        Graphic3dRenderDriver::new(options)
    }
    .unwrap_or_else(|error| panic!("Vulkan driver: {error:?}; expected a device"));
    driver.set_vertical_sync(false);
    driver
}

/// Builds the shared driver and reports every selected case.
pub fn run() {
    let mode = Mode::from_args();
    let bench = Bench {
        driver: new_driver(),
        work: RefCell::default(),
    };
    for case in cases().iter().filter(|case| mode.selects(case)) {
        report(&bench, case, &mode);
    }
}
