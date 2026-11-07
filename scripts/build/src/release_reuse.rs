//! Exact-commit rehearsal evidence for tag publication (#275).
mod api;
mod archive;
mod receipt;
mod request;
mod scope;
#[cfg(test)]
mod tests;

use crate::context::BuildContext;
use crate::error::{BuildError, BuildResult};
use crate::plan::{CiPlan, CiTrigger, plan_for};
use serde::{Deserialize, Serialize};

const WORKFLOW: &str = ".github/workflows/martensite.yml";
const RECEIPT: &str = "release-validation.json";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
struct Artifact {
    id: u64,
    name: String,
    digest: String,
    size_in_bytes: u64,
    expired: bool,
}

#[derive(Debug, Deserialize)]
struct Run {
    id: u64,
    run_attempt: u64,
    head_sha: String,
    path: String,
    event: String,
    status: String,
    conclusion: Option<String>,
    repository: Repository,
    head_repository: Repository,
}

#[derive(Debug, Deserialize)]
struct Repository {
    full_name: String,
}

#[derive(Debug, Deserialize)]
struct Job {
    name: String,
    status: String,
    conclusion: Option<String>,
}

fn refused(detail: impl std::fmt::Display) -> BuildError {
    BuildError::ReleaseRule(format!(
        "release reuse: {detail}; expected matching successful rehearsal evidence"
    ))
}

fn trigger_version(publish: bool) -> BuildResult<semver::Version> {
    let trigger = CiTrigger::from_lookup(|key| std::env::var(key).ok());
    match plan_for(&trigger)? {
        CiPlan::Release {
            version,
            publish: actual,
        } if actual == publish => Ok(version),
        other => Err(refused(format!("trigger {other:?}"))),
    }
}

fn repository() -> BuildResult<String> {
    let value = std::env::var("GITHUB_REPOSITORY").unwrap_or_default();
    let pieces: Vec<_> = value.split('/').collect();
    if pieces.len() != 2
        || pieces.iter().any(|p| {
            p.is_empty()
                || !p
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || b"-_.".contains(&c))
        })
    {
        return Err(refused(format!("repository {value:?}")));
    }
    Ok(value)
}

fn exact_head(context: &BuildContext<'_>, version: &semver::Version) -> BuildResult<String> {
    let head = api::git(context, "HEAD^{commit}")?;
    let tag = api::git(context, &format!("refs/tags/v{version}^{{commit}}"))?;
    if head != tag || std::env::var("GITHUB_SHA").ok().as_deref() != Some(&head) {
        return Err(refused("tag, checkout and event SHA disagree"));
    }
    Ok(head)
}

/// Finds a validated rehearsal without changing tags or publishing assets.
/// Example: `discover(&context)?` writes the `reuse-run` workflow output.
pub(crate) fn discover(context: &BuildContext<'_>) -> BuildResult<()> {
    let trigger = CiTrigger::from_lookup(|key| std::env::var(key).ok());
    let mut selected = String::new();
    if let CiPlan::Release {
        version,
        publish: true,
    } = plan_for(&trigger)?
    {
        selected = discover_tag(context, &version)?.map_or_else(String::new, |id| id.to_string());
    }
    let output = std::env::var("GITHUB_OUTPUT").map_err(|_| refused("missing GITHUB_OUTPUT"))?;
    crate::plan::write_outputs(
        &[("reuse-run", selected)],
        Some(std::path::Path::new(&output)),
    )
}

fn discover_tag(context: &BuildContext<'_>, version: &semver::Version) -> BuildResult<Option<u64>> {
    let request = request::ValidationRequest::tag(context, version.clone())?;
    find_rehearsal(context, &request)
}

fn find_rehearsal(
    context: &BuildContext<'_>,
    request: &request::ValidationRequest,
) -> BuildResult<Option<u64>> {
    for run in api::runs(context, &request.repo, &request.head)? {
        if !run_matches(&run, &request.repo, &request.head) {
            continue;
        }
        let artifacts = api::artifacts(context, &request.repo, run.id)?;
        let name = receipt::artifact_name(&request.version, run.run_attempt);
        if !artifacts.iter().any(|a| a.name == name && !a.expired) {
            continue;
        }
        match receipt::validate(context, request, &run, &artifacts) {
            Ok(_) => return Ok(Some(run.id)),
            Err(error) => context.reporter.note(&format!(
                "Rehearsal {} refused: {error}; checking other evidence before full validation",
                run.id
            )),
        }
    }
    Ok(None)
}

fn run_matches(run: &Run, repo: &str, head: &str) -> bool {
    run.id > 0
        && run.run_attempt > 0
        && run.head_sha == head
        && run.path == WORKFLOW
        && run.event == "workflow_dispatch"
        && run.status == "completed"
        && run.conclusion.as_deref() == Some("success")
        && run.repository.full_name == repo
        && run.head_repository.full_name == repo
}

pub(crate) use receipt::{restore, seal};

#[cfg(all(test, target_os = "linux"))]
mod archive_tests;

#[cfg(all(test, target_os = "linux"))]
mod orchestration_tests;
