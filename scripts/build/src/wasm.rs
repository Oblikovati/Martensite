//! Multi-threaded `wasm32-unknown-unknown` (`+atomics`) build check and time-API guard (#636).
//!
//! Reads the pinned nightly toolchain from `scripts/build/wasm-toolchain.toml` so the workspace
//! root `/rust-toolchain.toml` remains on stable Rust.
use std::fs;
use std::path::Path;

use crate::context::BuildContext;
use crate::error::{BuildError, BuildResult, io_error};
use crate::git::ChangedFile;
use crate::process::CommandSpec;

/// Target triple for browser multi-threaded WebAssembly.
pub const WASM_TARGET: &str = "wasm32-unknown-unknown";
/// Required LLVM target features for `SharedArrayBuffer` Web Worker threads.
pub const WASM_RUSTFLAGS: &str = "-C target-feature=+atomics,+bulk-memory,+mutable-globals";

/// Native-only workspace crates excluded from `wasm32-unknown-unknown` builds.
pub const NATIVE_ONLY_CRATES: &[&str] = &[
    "martensite-vulkan",
    "martensite-boolean-cases",
    "martensite-process-policy",
    "martensite-topology-test-support",
    "martensite-build",
];

/// Relative path to the isolated nightly toolchain pin used by `wasm-check`.
pub const WASM_TOOLCHAIN_PATH: &str = "scripts/build/wasm-toolchain.toml";

/// Read the pinned nightly channel from `scripts/build/wasm-toolchain.toml`.
pub fn pinned_wasm_channel(root: &Path) -> BuildResult<String> {
    let path = root.join(WASM_TOOLCHAIN_PATH);
    let text = fs::read_to_string(&path).map_err(|err| io_error(&path, err))?;
    parse_channel(&text).ok_or_else(|| BuildError::UnexpectedOutput {
        origin: WASM_TOOLCHAIN_PATH.to_owned(),
        detail: "missing `channel = \"...\"` entry; expected pinned nightly channel".to_owned(),
    })
}

fn parse_channel(toml: &str) -> Option<String> {
    for line in toml.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("channel") {
            let after_eq = rest.trim_start().strip_prefix('=')?.trim();
            let unquoted = after_eq.trim_matches('"').trim_matches('\'');
            if !unquoted.is_empty() {
                return Some(unquoted.to_owned());
            }
        }
    }
    None
}

/// Run `cargo +<pinned> check` (or `clippy`) across all WASM-eligible workspace crates (#636).
pub fn run_wasm_check(context: &BuildContext<'_>, clippy: bool) -> BuildResult<()> {
    let channel = pinned_wasm_channel(context.root())?;
    let subcommand = if clippy { "clippy" } else { "check" };
    let mut args = vec![
        format!("+{channel}"),
        subcommand.to_owned(),
        "--locked".to_owned(),
        "--target".to_owned(),
        WASM_TARGET.to_owned(),
        "-Z".to_owned(),
        "build-std=panic_abort,std".to_owned(),
        "--workspace".to_owned(),
    ];
    for excluded in NATIVE_ONLY_CRATES {
        args.push("--exclude".to_owned());
        args.push((*excluded).to_owned());
    }
    let spec = CommandSpec::new("cargo", context.root())
        .args(args)
        .environment("RUSTFLAGS", Some(WASM_RUSTFLAGS))
        .environment("CARGO_ENCODED_RUSTFLAGS", None);
    context
        .reporter
        .step("Check wasm32 +atomics workspace", || {
            context.runner.run(&spec)
        })
}

/// Reject new direct `std::time::Instant::now` or `std::time::SystemTime::now` calls in changed
/// non-test WASM-eligible crate sources (those panic on `wasm32-unknown-unknown`, #631, #636).
pub fn check_wasm_time_guard(root: &Path, changed: &[ChangedFile]) -> BuildResult<()> {
    let mut violations = Vec::new();
    for file in changed {
        if file.deleted || !is_time_guarded_source(&file.path) {
            continue;
        }
        let full = root.join(&file.path);
        if !full.is_file() {
            continue;
        }
        let source = fs::read_to_string(&full).map_err(|err| io_error(&full, err))?;
        let mut in_cfg_test = false;
        for (idx, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed == "#[cfg(test)]" {
                in_cfg_test = true;
                continue;
            }
            if in_cfg_test || trimmed.starts_with("//") {
                continue;
            }
            if trimmed.contains("std::time::Instant::now")
                || trimmed.contains("std::time::SystemTime::now")
            {
                violations.push(format!("{}:{}: {}", file.path, idx + 1, trimmed));
            }
        }
    }
    if violations.is_empty() {
        return Ok(());
    }
    Err(BuildError::UnexpectedOutput {
        origin: "wasm-time-guard (#631, #636)".to_owned(),
        detail: format!(
            "direct std::time::Instant::now / SystemTime::now panics on wasm32; use martensite_tkernel::osd::{{MonotonicInstant, unix_timestamp}}:\n{}",
            violations.join("\n")
        ),
    })
}

fn is_time_guarded_source(path: &str) -> bool {
    if !path.starts_with("crates/")
        || !path.contains("/src/")
        || !path.ends_with(".rs")
        || path.ends_with("_tests.rs")
        || path.ends_with("/tests.rs")
        || path.contains("/tests/")
    {
        return false;
    }
    if path.starts_with("crates/martensite-tkernel/src/osd/timing/")
        || path == "crates/martensite-tkernel/src/osd/process/native.rs"
    {
        return false;
    }
    for native_crate in NATIVE_ONLY_CRATES {
        if path.starts_with(&format!("crates/{native_crate}/")) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::tests::context_at;
    use crate::fake_runner::RecordingRunner;

    #[test]
    fn parses_pinned_wasm_toolchain_channel() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join(WASM_TOOLCHAIN_PATH);
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(&file, "[toolchain]\nchannel = \"nightly-2026-09-25\"\n").unwrap();
        assert_eq!(
            pinned_wasm_channel(dir.path()).unwrap(),
            "nightly-2026-09-25"
        );
    }

    #[test]
    fn wasm_check_excludes_native_only_crates_and_sets_atomics() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join(WASM_TOOLCHAIN_PATH);
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(&file, "[toolchain]\nchannel = \"nightly-2026-09-25\"\n").unwrap();
        let runner = RecordingRunner::default().with_stdout(
            "git rev-parse --show-toplevel",
            dir.path().to_str().unwrap(),
        );
        run_wasm_check(&context_at(&runner, dir.path()), false).unwrap();
        let cmds = runner.commands();
        assert!(cmds.iter().any(|c| {
            c.starts_with(
                "cargo +nightly-2026-09-25 check --locked --target wasm32-unknown-unknown",
            ) && c.contains("--exclude martensite-vulkan")
        }));
    }

    #[test]
    fn wasm_time_guard_rejects_std_time_now_in_kernel_source() {
        let dir = tempfile::tempdir().unwrap();
        let path = "crates/martensite-tkfillet/src/timer.rs";
        let full = dir.path().join(path);
        fs::create_dir_all(full.parent().unwrap()).unwrap();
        fs::write(&full, "pub fn t() { let _ = std::time::Instant::now(); }\n").unwrap();
        let changed = vec![ChangedFile {
            path: path.into(),
            deleted: false,
            untracked: false,
        }];
        let err = check_wasm_time_guard(dir.path(), &changed).unwrap_err();
        assert!(err.to_string().contains("MonotonicInstant"));
    }

    #[test]
    fn wasm_time_guard_allows_timing_module_and_cfg_test() {
        let dir = tempfile::tempdir().unwrap();
        let allowed = "crates/martensite-tkernel/src/osd/timing/monotonic_clock.rs";
        let full = dir.path().join(allowed);
        fs::create_dir_all(full.parent().unwrap()).unwrap();
        fs::write(&full, "pub fn t() { let _ = std::time::Instant::now(); }\n").unwrap();
        let changed = vec![ChangedFile {
            path: allowed.into(),
            deleted: false,
            untracked: false,
        }];
        check_wasm_time_guard(dir.path(), &changed).unwrap();
    }
}
