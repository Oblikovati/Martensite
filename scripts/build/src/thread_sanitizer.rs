//! Required Linux checks for cache ownership and reference-aware capture and group ownership (#309, #323, #324).
use crate::context::BuildContext;
use crate::error::BuildResult;
use crate::process::CommandSpec;
use std::path::Path;

const NIGHTLY: &str = "+nightly-2026-09-25";
const TARGET: &str = "x86_64-unknown-linux-gnu";
const SMOKE: &str = "scripts/build/tests/fixtures/thread_sanitizer_startup/Cargo.toml";

use crate::thread_sanitizer_selection::Control;

/// Refuse an unusable runtime before executing affected ownership controls.
/// Example: `run_selected(&context, &[Control::Capture])?`.
pub(crate) fn run_selected(context: &BuildContext<'_>, selected: &[Control]) -> BuildResult<()> {
    if selected.is_empty() {
        return Ok(());
    }
    context
        .runner
        .run(&CommandSpec::new("rustc", context.root()).args([NIGHTLY, "-Vv"]))?;
    context.reporter.step("ThreadSanitizer startup smoke", || {
        context.runner.run(&startup(context.root()))
    })?;
    run_controls(context, selected)
}

fn run_controls(context: &BuildContext<'_>, selected: &[Control]) -> BuildResult<()> {
    let both_caches = selected.contains(&Control::Cache2d) && selected.contains(&Control::Cache3d);
    for control in selected {
        if both_caches && *control == Control::Cache3d {
            continue;
        }
        let command = selected_command(context.root(), *control, both_caches);
        context
            .reporter
            .step(&format!("ThreadSanitizer {control:?}"), || {
                context.runner.run(&command)
            })?;
    }
    Ok(())
}

fn selected_command(root: &Path, control: Control, both_caches: bool) -> CommandSpec {
    if both_caches && control == Control::Cache2d {
        return cache_controls(root, &[Control::Cache2d.owner(), Control::Cache3d.owner()]);
    }
    control_command(root, control)
}

fn control_command(root: &Path, control: Control) -> CommandSpec {
    match control {
        Control::Cache2d | Control::Cache3d => cache_controls(root, &[control.owner()]),
        Control::Capture => capture_controls(root),
        Control::Groups => group_controls(root),
        Control::FaceInput => face_input_controls(root),
        Control::FaceCertificate => face_certificate_controls(root),
        Control::OpenExtrusion => open_extrusion_controls(root),
        Control::VertexNeighborhood => vertex_neighborhood_controls(root),
        Control::SurfaceSketch => surface_sketch_controls(root),
        Control::FontTextCapture => font_text_capture_controls(root),
    }
}

fn surface_sketch_controls(root: &Path) -> CommandSpec {
    // Independent operation contexts must preserve published value metadata (#414, #416).
    sanitized_cargo(root, "test").args([
        "--no-fail-fast",
        "--package",
        "martensite-tksketch",
        "--test",
        "surface_sketch",
        "--",
        "replay::parallel_operations_match_sequential_value_metadata",
        "--exact",
        "--test-threads=1",
        "--show-output",
    ])
}

fn open_extrusion_controls(root: &Path) -> CommandSpec {
    // Pure owned reports are also read from independent worker threads (#213).
    sanitized_cargo(root, "test").args([
        "--no-fail-fast",
        "--package",
        "martensite-tkmath",
        "--lib",
        "predicates::roots::open_extrusion",
        "--",
        "--test-threads=1",
        "--show-output",
    ])
}

fn vertex_neighborhood_controls(root: &Path) -> CommandSpec {
    // The owned contact reports retain complete shared-input evidence (#213).
    sanitized_cargo(root, "test").args([
        "--no-fail-fast",
        "--package",
        "martensite-tkmath",
        "--lib",
        "predicates::roots::sketch_incidence::vertex_neighborhood",
        "--",
        "--test-threads=1",
        "--show-output",
    ])
}

fn face_input_controls(root: &Path) -> CommandSpec {
    // Coherent input publication has its own real writer and alias controls (#213).
    sanitized_cargo(root, "test").args([
        "--no-fail-fast",
        "--package",
        "martensite-tkbrep",
        "--lib",
        "brep::face_input_capture",
        "--",
        "--test-threads=1",
        "--show-output",
    ])
}

fn face_certificate_controls(root: &Path) -> CommandSpec {
    // Owned face validation publishes results shared by independent readers (#213).
    sanitized_cargo(root, "test").args([
        "--no-fail-fast",
        "--package",
        "martensite-tktopalgo",
        "--test",
        "toolkit",
        "--",
        "closed_face_validation",
        "--test-threads=1",
        "--show-output",
    ])
}

fn capture_controls(root: &Path) -> CommandSpec {
    // Live capture remains an independent failure boundary before group ownership (#324).
    service_controls(root, &["capture_buffer_reference_checks"])
}

fn group_controls(root: &Path) -> CommandSpec {
    // Both registered module prefixes select the same fourteen normal controls (#324).
    service_controls(root, &["graphic3d_group", "graphic3d_stable_group_nodes"])
}

fn service_controls(root: &Path, filters: &[&str]) -> CommandSpec {
    sanitized_cargo(root, "test")
        .args([
            "--no-fail-fast",
            "--package",
            "martensite-tkservice",
            "--test",
            "toolkit",
        ])
        .arg("--")
        .args(filters.iter().copied())
        .args(["--test-threads=1", "--show-output"])
}

fn startup(root: &Path) -> CommandSpec {
    let command = sanitized_cargo(root, "run");
    let manifest = root.join(SMOKE);
    command
        .arg("--manifest-path")
        .arg(manifest.to_string_lossy())
}

fn cache_controls(root: &Path, packages: &[&str]) -> CommandSpec {
    let mut command = sanitized_cargo(root, "test").arg("--no-fail-fast");
    for package in packages {
        command = command.args(["--package", package]);
    }
    command.args([
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

fn font_text_capture_controls(root: &Path) -> CommandSpec {
    // Private memory-font instances and immutable published snapshots have distinct stress controls (#425).
    sanitized_cargo(root, "test").args([
        "--no-fail-fast",
        "--package",
        "martensite-tkv3d",
        "--lib",
        "brep_font::text_capture::tests::stress",
        "--",
        "--test-threads=1",
        "--show-output",
    ])
}
