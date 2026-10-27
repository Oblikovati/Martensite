//! Required Linux checks for cache ownership and reference-aware capture (#309, #323).
use crate::context::BuildContext;
use crate::error::BuildResult;
use crate::process::CommandSpec;
use std::path::Path;

const NIGHTLY: &str = "+nightly-2026-09-25";
const TARGET: &str = "x86_64-unknown-linux-gnu";
const SMOKE: &str = "scripts/build/tests/fixtures/thread_sanitizer_startup/Cargo.toml";

/// Refuse an unusable runtime before checking cache and live capture ownership.
/// Example: `run(&context)?` executes the same pipeline as `cargo xtask ci thread-sanitizer`.
pub fn run(context: &BuildContext<'_>) -> BuildResult<()> {
    let root = context.root();
    context
        .runner
        .run(&CommandSpec::new("rustc", root).args([NIGHTLY, "-Vv"]))?;
    context.reporter.step("ThreadSanitizer startup smoke", || {
        context.runner.run(&startup(root))
    })?;
    run_ownership_controls(context)
}

fn run_ownership_controls(context: &BuildContext<'_>) -> BuildResult<()> {
    context
        .reporter
        .step("ThreadSanitizer cache ownership controls", || {
            context.runner.run(&controls(context.root()))
        })?;
    context
        .reporter
        .step("ThreadSanitizer live capture controls", || {
            context.runner.run(&capture_controls(context.root()))
        })
}
fn capture_controls(root: &Path) -> CommandSpec {
    sanitized_cargo(root, "test").args([
        "--no-fail-fast",
        "--package",
        "martensite-tkservice",
        "--test",
        "toolkit",
        "capture_buffer_reference_checks",
        "--",
        "--test-threads=1",
        "--show-output",
    ])
}

fn startup(root: &Path) -> CommandSpec {
    let command = sanitized_cargo(root, "run");
    let manifest = root.join(SMOKE);
    command
        .arg("--manifest-path")
        .arg(manifest.to_string_lossy())
}

fn controls(root: &Path) -> CommandSpec {
    sanitized_cargo(root, "test").args([
        "--no-fail-fast",
        "--package",
        "martensite-tkg2d",
        "--package",
        "martensite-tkg3d",
        "--lib",
        "cache_inputs_tests",
        "--",
        "--test-threads=1",
        "--show-output",
    ])
}

fn sanitized_cargo(root: &Path, subcommand: &str) -> CommandSpec {
    let command = CommandSpec::new("cargo", root).args([
        NIGHTLY,
        subcommand,
        "--locked",
        "-Zbuild-std",
        "--target",
        TARGET,
    ]);
    sanitizer_environment(command, root)
}

fn sanitizer_environment(command: CommandSpec, root: &Path) -> CommandSpec {
    let target = root.join("target/thread-sanitizer");
    let command = command.environment_os("CARGO_TARGET_DIR", Some(target.as_os_str()));
    sanitizer_flags(command)
        .environment("CARGO_BUILD_JOBS", Some("2"))
        .environment("CARGO_INCREMENTAL", Some("0"))
        .environment("CARGO_PROFILE_DEV_DEBUG", Some("0"))
        .environment("CARGO_PROFILE_TEST_DEBUG", Some("0"))
        .environment("LD_PRELOAD", None)
        .environment("LD_LIBRARY_PATH", None)
}

fn sanitizer_flags(command: CommandSpec) -> CommandSpec {
    let flags = "-Zsanitizer=thread -Cforce-frame-pointers=yes";
    command
        .environment("RUSTFLAGS", Some(flags))
        .environment("CARGO_ENCODED_RUSTFLAGS", None)
        .environment("RUSTC_WRAPPER", None)
        .environment("RUSTC_WORKSPACE_WRAPPER", None)
        .environment("TSAN_OPTIONS", None)
}

#[cfg(test)]
#[path = "thread_sanitizer_tests.rs"]
mod tests;
