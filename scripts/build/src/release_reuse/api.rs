//! Read-only GitHub API calls through the ordinary command seam.
use super::{Artifact, Job, Run, refused};
use crate::{context::BuildContext, error::BuildResult, process::CommandSpec};
use serde::de::DeserializeOwned;

fn json<T: DeserializeOwned>(context: &BuildContext<'_>, endpoint: &str) -> BuildResult<T> {
    let output = context
        .runner
        .capture(&CommandSpec::new("gh", context.root()).args(["api", endpoint]))?;
    serde_json::from_str(&output).map_err(|error| refused(format!("API {endpoint}: {error}")))
}

fn pages<T: DeserializeOwned>(
    context: &BuildContext<'_>,
    endpoint: &str,
    field: &str,
) -> BuildResult<Vec<T>> {
    let mut rows = Vec::new();
    let separator = if endpoint.contains('?') { "&" } else { "?" };
    for page in 1..=100 {
        let value: serde_json::Value = json(
            context,
            &format!("{endpoint}{separator}per_page=100&page={page}"),
        )?;
        let batch: Vec<T> = serde_json::from_value(value[field].clone()).map_err(refused)?;
        let complete = batch.len() < 100;
        rows.extend(batch);
        if complete {
            return Ok(rows);
        }
    }
    Err(refused("API pagination exceeded 100 pages"))
}

pub(super) fn runs(context: &BuildContext<'_>, repo: &str, head: &str) -> BuildResult<Vec<Run>> {
    pages(
        context,
        &format!(
            "repos/{repo}/actions/workflows/martensite.yml/runs?event=workflow_dispatch&head_sha={head}&status=success"
        ),
        "workflow_runs",
    )
}

pub(super) fn run(context: &BuildContext<'_>, repo: &str, id: u64) -> BuildResult<Run> {
    json(context, &format!("repos/{repo}/actions/runs/{id}"))
}

pub(super) fn artifacts(
    context: &BuildContext<'_>,
    repo: &str,
    id: u64,
) -> BuildResult<Vec<Artifact>> {
    pages(
        context,
        &format!("repos/{repo}/actions/runs/{id}/artifacts"),
        "artifacts",
    )
}

pub(super) fn jobs(context: &BuildContext<'_>, repo: &str, run: &Run) -> BuildResult<Vec<Job>> {
    pages(
        context,
        &format!(
            "repos/{repo}/actions/runs/{}/attempts/{}/jobs?filter=all",
            run.id, run.run_attempt
        ),
        "jobs",
    )
}

pub(super) fn zip(
    context: &BuildContext<'_>,
    repo: &str,
    artifact: &Artifact,
) -> BuildResult<Vec<u8>> {
    let endpoint = format!("repos/{repo}/actions/artifacts/{}/zip", artifact.id);
    let limit = usize::try_from(artifact.size_in_bytes).map_err(refused)?;
    let output = context.runner.capture_bounded(
        &CommandSpec::new("gh", context.root()).args(["api", &endpoint]),
        limit,
    )?;
    if output.code != 0 {
        return Err(refused(format!(
            "artifact {} download exit {}",
            artifact.id, output.code
        )));
    }
    Ok(output.stdout)
}

pub(super) fn git(context: &BuildContext<'_>, revision: &str) -> BuildResult<String> {
    context
        .runner
        .capture(&CommandSpec::new("git", context.root()).args(["rev-parse", "--verify", revision]))
        .map(|value| value.trim().to_owned())
}
