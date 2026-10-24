//! Offline child processes use private dependency and build caches (#279).
use crate::{
    error::{BuildError, BuildResult},
    process::{CommandRunner, CommandSpec},
};
use std::path::Path;

pub(super) fn output(runner: &dyn CommandRunner, spec: &CommandSpec) -> BuildResult<Vec<u8>> {
    let output = runner.capture_output(spec)?;
    if output.code != 0 {
        return Err(BuildError::ReleaseRule(format!(
            "{} exited {}: {}; expected successful SDK command",
            spec.display(),
            output.code,
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(output.stdout)
}

pub(super) fn isolated(root: &Path, home: &Path, target: &Path) -> CommandSpec {
    let mut spec = CommandSpec::new("cargo", root)
        .environment("CARGO_HOME", home.to_str())
        .environment("CARGO_TARGET_DIR", target.to_str())
        .environment("CARGO_NET_OFFLINE", Some("true"));
    for (name, _) in std::env::vars_os().filter(|(name, _)| removed(&name.to_string_lossy())) {
        spec = spec.environment(&name.to_string_lossy(), None);
    }
    spec
}

fn removed(name: &str) -> bool {
    matches!(
        name,
        "RUSTC"
            | "RUSTDOC"
            | "CARGO"
            | "RUSTFLAGS"
            | "CARGO_ENCODED_RUSTFLAGS"
            | "RUSTC_WRAPPER"
            | "RUSTC_WORKSPACE_WRAPPER"
            | "MARTENSITE_TEST_DATA_DIR"
    ) || name.starts_with("CARGO_SOURCE_")
        || name.starts_with("CARGO_REGISTRIES_")
        || name.starts_with("CARGO_BUILD_")
        || name.starts_with("CARGO_TARGET_") && name != "CARGO_TARGET_DIR"
}
