//! `cargo xtask occt-parity`: runs a toolkit's paired OCCT and Rust performance cases on
//! the same CPUs and fails when a Rust case is slower than its OCCT twin (#583).
//!
//! The Rust cases are a `harness = false` bench target of the toolkit; the OCCT twins are
//! C++ files next to it, compiled here against a local OCCT install. OCCT is not a CI
//! dependency, so this is a local gate.
use std::path::{Path, PathBuf};

use crate::context::BuildContext;
use crate::error::{BuildError, BuildResult, io_error};
use crate::process::CommandSpec;

mod compare;
use compare::{CaseComparison, SideRuns, compare, parse_lines, render_table, verdict};

/// A toolkit with paired cases.
#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum ParityToolkit {
    /// `martensite-tkernel` against OCCT `TKernel`.
    Tkernel,
    /// `martensite-vulkan` against OCCT `TKOpenGl`, both driven through `V3d`; needs an
    /// X11 display, a GPU and an OCCT install with the Visualization module.
    Vulkan,
}

/// Where a toolkit's two case programs come from.
struct ToolkitCases {
    name: &'static str,
    package: &'static str,
    bench: &'static str,
    cpp_dir: &'static str,
    libraries: &'static [&'static str],
}

impl ParityToolkit {
    fn cases(self) -> ToolkitCases {
        match self {
            Self::Tkernel => ToolkitCases {
                name: "tkernel",
                package: "martensite-tkernel",
                bench: "occt_parity",
                cpp_dir: "crates/martensite-tkernel/benches/occt_parity/occt",
                libraries: &["TKernel"],
            },
            Self::Vulkan => ToolkitCases {
                name: "vulkan",
                package: "martensite-vulkan",
                bench: "occt_parity",
                cpp_dir: "crates/martensite-vulkan/benches/occt_parity/occt",
                libraries: &VULKAN_TWIN_LIBRARIES,
            },
        }
    }
}

/// The libraries the `TKOpenGl` twins link: the OCCT toolkits under `TKV3d` and `TKOpenGl`,
/// Xlib for the unmanaged window and GLX for the swap interval (see `occt/stage.cpp`).
const VULKAN_TWIN_LIBRARIES: [&str; 17] = [
    "TKernel",
    "TKMath",
    "TKG2d",
    "TKG3d",
    "TKGeomBase",
    "TKGeomAlgo",
    "TKBRep",
    "TKTopAlgo",
    "TKPrim",
    "TKMesh",
    "TKHLR",
    "TKShHealing",
    "TKService",
    "TKV3d",
    "TKOpenGl",
    "X11",
    "GL",
];

/// One parity run; `ParityRequest { toolkit: ParityToolkit::Tkernel, .. }`.
pub struct ParityRequest {
    /// The toolkit whose cases run.
    pub toolkit: ParityToolkit,
    /// The OCCT install prefix, holding `include/opencascade` and `lib`.
    pub occt_prefix: PathBuf,
    /// The C++ compiler command.
    pub compiler: String,
    /// A `taskset` CPU list that both programs run on, e.g. `8-15`.
    pub cpus: Option<String>,
    /// How many times each program runs, alternating.
    pub rounds: usize,
    /// Keeps the cases whose name contains this text.
    pub filter: Option<String>,
    /// Frames (operations) of one timed run, for toolkits whose cases draw frames.
    pub frames: Option<usize>,
    /// The relative difference still counted as parity, e.g. `0.05`.
    pub tolerance: f64,
}

/// Builds, measures and compares; `run_occt_parity(&context, &request)?`.
pub fn run_occt_parity(context: &BuildContext<'_>, request: &ParityRequest) -> BuildResult<()> {
    let cases = request.toolkit.cases();
    let reporter = &context.reporter;
    let rust = reporter.step("Build the Rust cases", || build_rust_cases(context, &cases))?;
    let occt = reporter.step("Build the OCCT cases", || {
        build_occt_cases(context, request, &cases)
    })?;
    let (occt_runs, rust_runs) =
        reporter.step("Measure", || measure(context, request, &occt, &rust))?;
    let comparisons = compare(&occt_runs, &rust_runs, request.tolerance)?;
    println!("{}", render_table(&comparisons));
    let report = write_report(context, request, &cases, &comparisons)?;
    reporter.note(&format!("report: {}", report.display()));
    verdict(&comparisons)
}

fn build_rust_cases(context: &BuildContext<'_>, cases: &ToolkitCases) -> BuildResult<PathBuf> {
    let spec = CommandSpec::new("cargo", context.root()).args([
        "bench",
        "--locked",
        "--no-run",
        "--package",
        cases.package,
        "--bench",
        cases.bench,
        "--message-format=json",
    ]);
    bench_executable(&context.runner.capture(&spec)?, cases.bench)
}

/// The executable of bench target `bench` in Cargo's JSON messages.
fn bench_executable(messages: &str, bench: &str) -> BuildResult<PathBuf> {
    messages
        .lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .find(|message| is_bench_artifact(message, bench))
        .and_then(|message| message["executable"].as_str().map(PathBuf::from))
        .ok_or_else(|| BuildError::UnexpectedOutput {
            origin: "cargo bench --no-run".into(),
            detail: format!("no executable for bench target {bench}; expected a compiler-artifact"),
        })
}

fn is_bench_artifact(message: &serde_json::Value, bench: &str) -> bool {
    let target = &message["target"];
    message["reason"] == "compiler-artifact"
        && target["name"] == bench
        && target["kind"]
            .as_array()
            .is_some_and(|kinds| kinds.iter().any(|kind| kind == "bench"))
}

fn build_occt_cases(
    context: &BuildContext<'_>,
    request: &ParityRequest,
    cases: &ToolkitCases,
) -> BuildResult<PathBuf> {
    let output = output_dir(context.root(), cases);
    std::fs::create_dir_all(&output).map_err(|cause| io_error(&output, cause))?;
    let executable = output.join("occt_parity");
    let sources = cpp_sources(&context.root().join(cases.cpp_dir))?;
    let spec = compile_command(context.root(), request, cases, &sources, &executable);
    context.runner.run(&spec)?;
    Ok(executable)
}

fn output_dir(root: &Path, cases: &ToolkitCases) -> PathBuf {
    root.join("target").join("occt-parity").join(cases.name)
}

/// The `.cpp` files of `dir` in name order, so the command line is reproducible.
fn cpp_sources(dir: &Path) -> BuildResult<Vec<PathBuf>> {
    let entries = std::fs::read_dir(dir).map_err(|cause| io_error(dir, cause))?;
    let mut sources = Vec::new();
    for entry in entries {
        let path = entry.map_err(|cause| io_error(dir, cause))?.path();
        if path.extension().is_some_and(|extension| extension == "cpp") {
            sources.push(path);
        }
    }
    sources.sort();
    Ok(sources)
}

fn compile_command(
    root: &Path,
    request: &ParityRequest,
    cases: &ToolkitCases,
    sources: &[PathBuf],
    executable: &Path,
) -> CommandSpec {
    let prefix = request.occt_prefix.display();
    let mut spec = CommandSpec::new(&request.compiler, root)
        .args(["-O3", "-DNDEBUG", "-std=c++17"])
        .arg(format!("-I{prefix}/include/opencascade"))
        .args(sources.iter().map(|source| source.display().to_string()))
        .arg(format!("-L{prefix}/lib"))
        // OCCT's libraries carry no run path of their own. A RUNPATH would only serve the
        // program's direct dependencies, so TKV3d could not find TKGeomAlgo; an RPATH also
        // serves the libraries the program loads.
        .arg("-Wl,--disable-new-dtags")
        .arg(format!("-Wl,-rpath,{prefix}/lib"));
    for library in cases.libraries {
        spec = spec.arg(format!("-l{library}"));
    }
    spec.arg("-o").arg(executable.display().to_string())
}

/// Runs both programs `rounds` times, OCCT first in each round.
fn measure(
    context: &BuildContext<'_>,
    request: &ParityRequest,
    occt: &Path,
    rust: &Path,
) -> BuildResult<(SideRuns, SideRuns)> {
    let (mut occt_runs, mut rust_runs) = (SideRuns::default(), SideRuns::default());
    for _ in 0..request.rounds {
        let occt_out = context
            .runner
            .capture(&case_command(context.root(), request, occt))?;
        occt_runs.add_round("OCCT", parse_lines("OCCT", &occt_out)?)?;
        let rust_out = context
            .runner
            .capture(&case_command(context.root(), request, rust))?;
        rust_runs.add_round("Rust", parse_lines("Rust", &rust_out)?)?;
    }
    Ok((occt_runs, rust_runs))
}

fn case_command(root: &Path, request: &ParityRequest, program: &Path) -> CommandSpec {
    let program = program.display().to_string();
    let spec = match &request.cpus {
        Some(cpus) => {
            CommandSpec::new("taskset", root).args(["-c", cpus.as_str(), program.as_str()])
        }
        None => CommandSpec::new(&program, root),
    };
    let spec = spec.arg("--bench");
    let spec = match &request.filter {
        Some(filter) => spec.args(["--filter", filter.as_str()]),
        None => spec,
    };
    match request.frames {
        Some(frames) => spec.args(["--frames".to_owned(), frames.to_string()]),
        None => spec,
    }
}

fn write_report(
    context: &BuildContext<'_>,
    request: &ParityRequest,
    cases: &ToolkitCases,
    comparisons: &[CaseComparison],
) -> BuildResult<PathBuf> {
    let path = output_dir(context.root(), cases).join("report.json");
    let report = serde_json::json!({
        "toolkit": cases.name,
        "occt_prefix": request.occt_prefix,
        "cpus": request.cpus,
        "rounds": request.rounds,
        "tolerance": request.tolerance,
        "frames": request.frames,
        "cases": comparisons,
    });
    let text =
        serde_json::to_string_pretty(&report).map_err(|cause| BuildError::UnexpectedOutput {
            origin: "parity report".into(),
            detail: format!("{cause}; expected serializable comparisons"),
        })?;
    std::fs::write(&path, text + "\n").map_err(|cause| io_error(&path, cause))?;
    Ok(path)
}

#[cfg(test)]
#[path = "occt_parity_tests.rs"]
mod tests;
