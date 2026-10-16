//! Reviewed target-specific compiler counts complement the lexical ratchet.
use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::parser::SummaryEntry;
use crate::error::{BuildError, BuildResult, io_error};

#[derive(Deserialize)]
struct Baseline {
    target: String,
    compiler: String,
    summary: Vec<SummaryEntry>,
}

pub(super) fn verify(
    root: &Path,
    triple: &str,
    compiler: &str,
    actual: &[SummaryEntry],
) -> BuildResult<()> {
    let path = baseline_path(root, triple)?;
    let bytes = std::fs::read(&path).map_err(|cause| io_error(&path, cause))?;
    let baseline: Baseline = serde_json::from_slice(&bytes)
        .map_err(|error| super::output_error(format!("{}: {error}", path.display())))?;
    compare(&baseline, triple, compiler, actual)
}

fn baseline_path(root: &Path, triple: &str) -> BuildResult<PathBuf> {
    if triple.is_empty()
        || !triple
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
    {
        return Err(super::output_error(format!(
            "target {triple:?}; expected a Rust target triple"
        )));
    }
    Ok(root
        .join("docs/features/safe-toolkits/compiler-baselines")
        .join(format!("{triple}.json")))
}

fn compare(
    baseline: &Baseline,
    triple: &str,
    compiler: &str,
    actual: &[SummaryEntry],
) -> BuildResult<()> {
    let compiler = compiler.lines().next().unwrap_or_default();
    if baseline.target != triple || baseline.compiler != compiler {
        return Err(BuildError::LintFailed(format!(
            "compiler inventory got {triple} / {compiler:?}; expected {} / {:?}",
            baseline.target, baseline.compiler
        )));
    }
    if baseline.summary != actual {
        return Err(BuildError::LintFailed(format!(
            "compiler inventory changed for {triple}; review target/unsafe-inventory/latest.json and update the baseline only for approved changes"
        )));
    }
    Ok(())
}

#[cfg(test)]
pub(crate) fn install_empty_fixture(root: &Path) {
    let path = baseline_path(root, "test-target").unwrap();
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(
        path,
        r#"{"target":"test-target","compiler":"host: test-target","summary":[]}"#,
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(count: usize) -> SummaryEntry {
        serde_json::from_value(serde_json::json!({"location":{"package":"workspace:p@1",
            "target":"p","target_kinds":["lib"],"source":"src/lib.rs",
            "diagnostic":"usage of an unsafe block","expansion_depth":0},
            "emitted_primary_spans":count}))
        .unwrap()
    }

    #[test]
    fn changed_counts_and_removed_groups_require_baseline_review() {
        let baseline = Baseline {
            target: "target".into(),
            compiler: "rustc pinned".into(),
            summary: vec![entry(2)],
        };
        assert!(
            compare(
                &baseline,
                "target",
                "rustc pinned\nhost: machine",
                &[entry(2)]
            )
            .is_ok()
        );
        assert!(compare(&baseline, "target", "rustc pinned", &[entry(3)]).is_err());
        assert!(compare(&baseline, "target", "rustc pinned", &[entry(1)]).is_err());
        assert!(compare(&baseline, "target", "rustc pinned", &[]).is_err());
        assert!(compare(&baseline, "other-target", "rustc pinned", &[entry(2)]).is_err());
        assert!(compare(&baseline, "target", "rustc other", &[entry(2)]).is_err());
    }

    #[test]
    fn missing_baseline_fails_and_target_cannot_escape_baseline_directory() {
        let directory = tempfile::tempdir().unwrap();
        assert!(verify(directory.path(), "missing", "rustc pinned", &[]).is_err());
        assert!(baseline_path(directory.path(), "../../escape").is_err());
        install_empty_fixture(directory.path());
        verify(directory.path(), "test-target", "host: test-target", &[]).unwrap();
    }
}
