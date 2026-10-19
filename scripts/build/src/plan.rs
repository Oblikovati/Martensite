//! Turns a GitHub Actions trigger into the pipeline to run. The workflow
//! holds no logic of its own: `ci plan` emits the matrix and switches, and
//! `ci run` re-derives the same plan from the same trigger.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use semver::Version;

use crate::error::{BuildError, BuildResult, io_error};
use crate::targets::{PRIMARY_TARGET, RELEASE_TARGETS, ReleaseTarget};
use crate::version::{parse_release_tag, parse_version_argument};

/// The trigger fields GitHub exposes as environment variables.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CiTrigger {
    /// `GITHUB_EVENT_NAME`: `pull_request`, `push` or `workflow_dispatch`.
    pub event: String,
    /// `GITHUB_REF`, e.g. `refs/tags/v1.4.0`.
    pub git_ref: String,
    /// `GITHUB_BASE_REF`: the branch a pull request merges into.
    pub base_ref: String,
    /// `MARTENSITE_CI_MODE`: the `mode` input of a manual dispatch.
    pub dispatch_mode: String,
    /// `MARTENSITE_CI_VERSION`: the `version` input of a manual dispatch.
    pub dispatch_version: String,
}

impl CiTrigger {
    /// Reads the trigger through `lookup`, normally `std::env::var`.
    ///
    /// ```ignore
    /// let trigger = CiTrigger::from_lookup(|key| std::env::var(key).ok());
    /// ```
    pub fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Self {
        let read = |key: &str| lookup(key).unwrap_or_default();
        Self {
            event: read("GITHUB_EVENT_NAME"),
            git_ref: read("GITHUB_REF"),
            base_ref: read("GITHUB_BASE_REF"),
            dispatch_mode: read("MARTENSITE_CI_MODE"),
            dispatch_version: read("MARTENSITE_CI_VERSION"),
        }
    }
}

/// The pipeline a trigger selects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CiPlan {
    /// Validate changes against `base` on the primary target.
    PullRequest {
        /// The remote-tracking ref to diff against, e.g. `origin/main`.
        base: String,
    },
    /// Build, test and package `version` on every release target.
    Release {
        /// The version being released.
        version: Version,
        /// Create the GitHub release afterwards (tag pushes only).
        publish: bool,
    },
}

/// Chooses the pipeline for a trigger.
pub fn plan_for(trigger: &CiTrigger) -> BuildResult<CiPlan> {
    match trigger.event.as_str() {
        "pull_request" => pull_request_plan(&trigger.base_ref),
        "push" => tag_push_plan(&trigger.git_ref),
        "workflow_dispatch" => dispatch_plan(&trigger.dispatch_mode, &trigger.dispatch_version),
        other => Err(BuildError::Usage(format!(
            "unsupported CI event {other:?}; expected pull_request, push of a v* tag, or workflow_dispatch"
        ))),
    }
}

fn pull_request_plan(base_ref: &str) -> BuildResult<CiPlan> {
    if base_ref.is_empty() {
        return Err(BuildError::Usage(
            "pull_request event without GITHUB_BASE_REF; expected a branch".into(),
        ));
    }
    Ok(CiPlan::PullRequest {
        base: format!("origin/{base_ref}"),
    })
}

fn tag_push_plan(git_ref: &str) -> BuildResult<CiPlan> {
    let tag = git_ref.strip_prefix("refs/tags/").ok_or_else(|| {
        BuildError::Usage(format!(
            "push of {git_ref:?} is not a release; expected refs/tags/vX.Y.Z"
        ))
    })?;
    Ok(CiPlan::Release {
        version: parse_release_tag(tag)?,
        publish: true,
    })
}

/// A manual run is a rehearsal: it never publishes, so the full matrix can
/// run on a release candidate before the tag exists.
fn dispatch_plan(mode: &str, version: &str) -> BuildResult<CiPlan> {
    match mode {
        "pr" => Ok(CiPlan::PullRequest {
            base: "origin/main".into(),
        }),
        "release" => Ok(CiPlan::Release {
            version: parse_version_argument(version)?,
            publish: false,
        }),
        other => Err(BuildError::Usage(format!(
            "dispatch mode {other:?}; expected \"pr\" or \"release\""
        ))),
    }
}

/// The `key=value` outputs the workflow's later jobs read.
pub fn plan_outputs(plan: &CiPlan) -> Vec<(&'static str, String)> {
    let (mode, version, publish, targets) = match plan {
        CiPlan::PullRequest { .. } => ("pr", String::new(), false, vec![PRIMARY_TARGET]),
        CiPlan::Release { version, publish } => (
            "release",
            version.to_string(),
            *publish,
            RELEASE_TARGETS.to_vec(),
        ),
    };
    vec![
        ("mode", mode.to_owned()),
        ("version", version),
        ("publish", publish.to_string()),
        ("matrix", matrix_json(&targets)),
    ]
}

fn matrix_json(targets: &[ReleaseTarget]) -> String {
    let include: Vec<_> = targets
        .iter()
        .map(|target| serde_json::json!({"target": target.triple, "runner": target.runner}))
        .collect();
    serde_json::json!({ "include": include }).to_string()
}

/// Appends outputs to the `GITHUB_OUTPUT` file, or prints them locally.
pub fn write_outputs(outputs: &[(&str, String)], github_output: Option<&Path>) -> BuildResult<()> {
    let lines: String = outputs
        .iter()
        .map(|(key, value)| format!("{key}={value}\n"))
        .collect();
    let Some(path) = github_output else {
        print!("{lines}");
        return Ok(());
    };
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .map_err(|cause| io_error(path, cause))?;
    file.write_all(lines.as_bytes())
        .map_err(|cause| io_error(path, cause))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trigger(event: &str, git_ref: &str, base_ref: &str) -> CiTrigger {
        CiTrigger {
            event: event.into(),
            git_ref: git_ref.into(),
            base_ref: base_ref.into(),
            ..CiTrigger::default()
        }
    }

    #[test]
    fn pull_request_diffs_against_its_base_on_one_target() {
        let plan = plan_for(&trigger("pull_request", "refs/pull/7/merge", "main")).unwrap();
        assert_eq!(
            plan,
            CiPlan::PullRequest {
                base: "origin/main".into()
            }
        );
        let outputs = plan_outputs(&plan);
        assert_eq!(outputs[0], ("mode", "pr".to_owned()));
        assert!(outputs[3].1.contains("ubuntu-24.04") && !outputs[3].1.contains("windows"));
    }

    fn assert_release_matrix(plan: &CiPlan) {
        let matrix: serde_json::Value = serde_json::from_str(&plan_outputs(plan)[3].1).unwrap();
        assert_eq!(
            matrix,
            serde_json::json!({"include": [
                {"target": "x86_64-unknown-linux-gnu", "runner": "ubuntu-24.04"},
                {"target": "x86_64-apple-darwin", "runner": "macos-15-intel"},
                {"target": "aarch64-apple-darwin", "runner": "macos-15"},
                {"target": "x86_64-pc-windows-msvc", "runner": "windows-2025"}
            ]})
        );
    }

    #[test]
    fn tag_push_publishes_on_every_target() {
        let plan = plan_for(&trigger("push", "refs/tags/v0.3.0", "")).unwrap();
        assert_eq!(
            plan,
            CiPlan::Release {
                version: Version::new(0, 3, 0),
                publish: true
            }
        );
        let matrix = &plan_outputs(&plan)[3].1;
        assert_eq!(matrix.matches("\"target\"").count(), RELEASE_TARGETS.len());
        assert_release_matrix(&plan);
    }

    #[test]
    fn branch_push_and_bad_tags_are_refused() {
        assert!(plan_for(&trigger("push", "refs/heads/main", "")).is_err());
        assert!(plan_for(&trigger("push", "refs/tags/latest", "")).is_err());
        assert!(plan_for(&trigger("schedule", "", "")).is_err());
        assert!(plan_for(&trigger("pull_request", "", "")).is_err());
    }

    #[test]
    fn dispatch_rehearses_a_release_without_publishing() {
        let trigger = CiTrigger {
            event: "workflow_dispatch".into(),
            dispatch_mode: "release".into(),
            dispatch_version: "0.3.0-rc.1".into(),
            ..CiTrigger::default()
        };
        let plan = plan_for(&trigger).unwrap();
        assert_eq!(plan_outputs(&plan)[2], ("publish", "false".to_owned()));
        assert_release_matrix(&plan);
        let bad = CiTrigger {
            dispatch_mode: "deploy".into(),
            ..trigger
        };
        assert!(plan_for(&bad).is_err());
    }

    #[test]
    fn trigger_is_read_from_the_lookup() {
        let lookup = |key: &str| (key == "GITHUB_BASE_REF").then(|| "main".to_owned());
        assert_eq!(CiTrigger::from_lookup(lookup).base_ref, "main");
    }

    #[test]
    fn outputs_are_appended_to_the_github_output_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("output");
        std::fs::write(&path, "earlier=1\n").unwrap();
        write_outputs(&[("mode", "pr".into())], Some(&path)).unwrap();
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            "earlier=1\nmode=pr\n"
        );
    }
}
