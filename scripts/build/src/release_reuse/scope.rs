//! Binds the selected packages and suites, not just the number of matrix jobs.
use super::{api, refused};
use crate::{
    context::BuildContext, error::BuildResult, release_scope::ReleaseScope,
    workspace::CargoWorkspace,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub(super) struct ScopeBinding {
    baseline: Option<String>,
    baseline_commit: Option<String>,
    history_sha256: String,
    packages: Vec<String>,
    tests: Vec<String>,
    sanitizer_controls: Vec<String>,
}

impl ScopeBinding {
    pub(super) fn load(context: &BuildContext<'_>, version: &semver::Version) -> BuildResult<Self> {
        require_plan_pin()?;
        let workspace = CargoWorkspace::load(context.runner, context.root())?;
        let scope = ReleaseScope::load(context, &workspace, version)?;
        Self::from_scope(context, scope)
    }

    fn from_scope(context: &BuildContext<'_>, scope: ReleaseScope) -> BuildResult<Self> {
        let commit = scope
            .baseline
            .as_ref()
            .map(|tag| api::git(context, &format!("{tag}^{{commit}}")))
            .transpose()?;
        let sanitizer_controls = selected_controls(context, &scope)?;
        Ok(Self {
            sanitizer_controls,
            baseline: scope.baseline,
            baseline_commit: commit,
            history_sha256: scope
                .history_sha256
                .ok_or_else(|| refused("missing release history fingerprint"))?,
            packages: scope.packages,
            tests: scope.tests,
        })
    }

    pub(super) fn require_match(&self, expected: &Self) -> BuildResult<()> {
        if self != expected {
            return Err(refused(
                "release baseline, history, package, test or sanitizer scope changed",
            ));
        }
        Ok(())
    }
}

fn selected_controls(context: &BuildContext<'_>, scope: &ReleaseScope) -> BuildResult<Vec<String>> {
    let changed = match &scope.baseline {
        Some(base) => context.git.changed_files(base)?,
        None => Vec::new(),
    };
    let packages = scope.tests.iter().cloned().collect();
    Ok(
        crate::thread_sanitizer_selection::select(&packages, &changed)
            .into_iter()
            .map(|control| control.id().to_owned())
            .collect(),
    )
}

fn require_plan_pin() -> BuildResult<()> {
    let pin = std::env::var("MARTENSITE_RELEASE_HISTORY_SHA256")
        .map_err(|_| refused("missing pinned plan history"))?;
    if pin.is_empty() {
        return Err(refused("empty pinned plan history"));
    }
    Ok(())
}

#[cfg(test)]
pub(super) fn fixture() -> ScopeBinding {
    ScopeBinding {
        baseline: Some("v0.13.0".into()),
        baseline_commit: Some("c".repeat(40)),
        history_sha256: "d".repeat(64),
        packages: vec!["leaf".into()],
        tests: vec!["leaf".into(), "dependent".into()],
        sanitizer_controls: vec!["capture".into()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_shard_count_does_not_authorize_changed_test_membership() {
        let expected = fixture();
        let mut changed = expected.clone();
        changed.tests[1] = "unvalidated-dependent".into();
        assert_eq!(changed.tests.len(), expected.tests.len());
        assert!(changed.require_match(&expected).is_err());
        expected.require_match(&expected).unwrap();
    }

    #[test]
    fn same_job_count_does_not_authorize_changed_sanitizer_controls() {
        let expected = fixture();
        let mut changed = expected.clone();
        changed.sanitizer_controls[0] = "groups".into();
        assert_eq!(
            changed.sanitizer_controls.len(),
            expected.sanitizer_controls.len()
        );
        assert!(changed.require_match(&expected).is_err());
        changed.sanitizer_controls.clear();
        assert!(changed.require_match(&expected).is_err());
    }

    #[test]
    fn changed_baseline_commit_or_publication_history_is_refused() {
        let expected = fixture();
        let mut changed = expected.clone();
        changed.baseline_commit = Some("e".repeat(40));
        assert!(changed.require_match(&expected).is_err());
        changed = expected.clone();
        changed.history_sha256 = "f".repeat(64);
        assert!(changed.require_match(&expected).is_err());
        changed = expected.clone();
        changed.packages.push("new-package".into());
        assert!(changed.require_match(&expected).is_err());
    }
}
