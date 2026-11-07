//! A successful rehearsal binds immutable artifact identities to one version and attempt.
use super::{Artifact, Job, RECEIPT, Run, api, archive, refused, run_matches, trigger_version};
use super::{request::ValidationRequest, scope::ScopeBinding};
use crate::{
    context::BuildContext,
    error::{BuildResult, io_error},
    plan::CiPlan,
    targets::RELEASE_TARGETS,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, fs, path::Path};

#[derive(Deserialize, Serialize)]
struct Receipt {
    schema: u32,
    version: String,
    commit: String,
    run: u64,
    attempt: u64,
    artifacts: Vec<Artifact>,
    scope: ScopeBinding,
}

pub(super) fn artifact_name(version: &semver::Version, attempt: u64) -> String {
    format!("release-validation-v{version}-attempt-{attempt}")
}

pub(super) fn required_jobs(
    context: &BuildContext<'_>,
    version: &semver::Version,
) -> BuildResult<BTreeSet<String>> {
    let plan = CiPlan::Release {
        version: version.clone(),
        publish: false,
    };
    let outputs = crate::ci_tests::matrix_outputs(context, &plan)?;
    job_names(&outputs)
}

fn job_names(outputs: &[(&str, String)]) -> BuildResult<BTreeSet<String>> {
    let matrix = outputs
        .iter()
        .find(|(key, _)| *key == "test-matrix")
        .ok_or_else(|| refused("missing test matrix"))?;
    let value: serde_json::Value = serde_json::from_str(&matrix.1).map_err(refused)?;
    let mut names = base_job_names();
    for row in value["include"]
        .as_array()
        .ok_or_else(|| refused("invalid test matrix"))?
    {
        names.insert(shard_job_name(row)?);
    }
    add_conditional_jobs(&mut names, outputs)?;
    Ok(names)
}

fn add_conditional_jobs(
    names: &mut BTreeSet<String>,
    outputs: &[(&str, String)],
) -> BuildResult<()> {
    let sanitizer = outputs.iter().find(|(key, _)| *key == "has-sanitizer");
    match sanitizer.map(|(_, value)| value.as_str()) {
        Some("true") => {
            names.insert("thread sanitizer (x86_64-unknown-linux-gnu)".into());
        }
        Some("false") => {}
        _ => return Err(refused("missing or invalid sanitizer selection")),
    }
    if outputs
        .iter()
        .any(|(key, value)| *key == "renderer-admission" && value == "true")
    {
        names.insert("renderer-arm64".into());
    }
    Ok(())
}

fn base_job_names() -> BTreeSet<String> {
    let mut names = BTreeSet::from(["plan".into(), "release".into()]);
    names.extend(
        RELEASE_TARGETS
            .iter()
            .map(|target| format!("checks ({})", target.triple)),
    );
    names
}

fn shard_job_name(row: &serde_json::Value) -> BuildResult<String> {
    let target = row["target"]
        .as_str()
        .ok_or_else(|| refused("invalid matrix target"))?;
    let shard = row["shard"]
        .as_u64()
        .ok_or_else(|| refused("invalid matrix shard"))?;
    Ok(format!("release tests ({target}, shard {shard})"))
}

pub(super) fn jobs_complete(jobs: &[Job], required: &BTreeSet<String>, sealing: bool) -> bool {
    required.iter().all(|name| {
        let matches: Vec<_> = jobs.iter().filter(|job| &job.name == name).collect();
        matches.len() == 1
            && ((sealing && name == "release" && matches[0].status == "in_progress")
                || (matches[0].status == "completed"
                    && matches[0].conclusion.as_deref() == Some("success")))
    })
}

/// Records only a release rehearsal whose required predecessor jobs passed.
/// Example: `seal(&context, Path::new("validation"))?`.
pub(crate) fn seal(context: &BuildContext<'_>, directory: &Path) -> BuildResult<()> {
    let request = ValidationRequest::rehearsal(context, trigger_version(false)?)?;
    let id = std::env::var("GITHUB_RUN_ID")
        .unwrap_or_default()
        .parse()
        .map_err(refused)?;
    let run = api::run(context, &request.repo, id)?;
    require_rehearsal_origin(&run, &request)?;
    require_jobs(context, &request, &run, true)?;
    let artifacts = distribution(&api::artifacts(context, &request.repo, id)?)?;
    write_receipt(directory, sealed_receipt(request, &run, artifacts))
}

fn sealed_receipt(request: ValidationRequest, run: &Run, artifacts: Vec<Artifact>) -> Receipt {
    Receipt {
        schema: 1,
        version: request.version.to_string(),
        commit: request.head,
        run: run.id,
        attempt: run.run_attempt,
        artifacts,
        scope: request.scope,
    }
}

fn require_rehearsal_origin(run: &Run, request: &ValidationRequest) -> BuildResult<()> {
    if std::env::var("GITHUB_RUN_ATTEMPT").ok() != Some(run.run_attempt.to_string())
        || std::env::var("GITHUB_SHA").ok() != Some(request.head.clone())
        || run.head_sha != request.head
        || run.event != "workflow_dispatch"
        || run.path != super::WORKFLOW
        || run.repository.full_name != request.repo
        || run.head_repository.full_name != request.repo
    {
        return Err(refused("rehearsal origin, event or attempt mismatch"));
    }
    Ok(())
}

fn require_jobs(
    context: &BuildContext<'_>,
    request: &ValidationRequest,
    run: &Run,
    sealing: bool,
) -> BuildResult<()> {
    if !jobs_complete(
        &api::jobs(context, &request.repo, run)?,
        &request.jobs,
        sealing,
    ) {
        return Err(refused("incomplete release matrix"));
    }
    Ok(())
}

fn write_receipt(directory: &Path, receipt: Receipt) -> BuildResult<()> {
    fs::create_dir_all(directory).map_err(|error| io_error(directory, error))?;
    let path = directory.join(RECEIPT);
    fs::write(&path, serde_json::to_vec(&receipt).map_err(refused)?)
        .map_err(|error| io_error(&path, error))
}

fn distribution(artifacts: &[Artifact]) -> BuildResult<Vec<Artifact>> {
    let mut dist: Vec<_> = artifacts
        .iter()
        .filter(|a| a.name.starts_with("dist-"))
        .cloned()
        .collect();
    dist.sort_by_key(|artifact| artifact.id);
    if !complete_distribution(&dist) {
        return Err(refused(
            "incomplete, duplicate or expired distribution artifacts",
        ));
    }
    Ok(dist)
}

fn complete_distribution(dist: &[Artifact]) -> bool {
    let names: BTreeSet<_> = dist.iter().map(|artifact| &artifact.name).collect();
    !dist.is_empty()
        && names.len() == dist.len()
        && dist
            .iter()
            .any(|artifact| artifact.name == "dist-x86_64-unknown-linux-gnu")
        && dist.iter().all(valid_distribution_artifact)
}

fn valid_distribution_artifact(artifact: &Artifact) -> bool {
    !artifact.expired
        && artifact
            .digest
            .strip_prefix("sha256:")
            .is_some_and(|digest| {
                digest.len() == 64 && digest.bytes().all(|byte| byte.is_ascii_hexdigit())
            })
        && RELEASE_TARGETS
            .iter()
            .any(|target| artifact.name == format!("dist-{}", target.triple))
}

pub(super) fn validate(
    context: &BuildContext<'_>,
    request: &ValidationRequest,
    run: &Run,
    artifacts: &[Artifact],
) -> BuildResult<Vec<Artifact>> {
    require_jobs(context, request, run, false)?;
    let receipt = read_receipt(context, request, run, artifacts)?;
    receipt.scope.require_match(&request.scope)?;
    check_receipt(receipt, run, &request.version, artifacts)
}

fn read_receipt(
    context: &BuildContext<'_>,
    request: &ValidationRequest,
    run: &Run,
    artifacts: &[Artifact],
) -> BuildResult<Receipt> {
    let name = artifact_name(&request.version, run.run_attempt);
    let matches: Vec<_> = artifacts
        .iter()
        .filter(|artifact| artifact.name == name)
        .collect();
    if matches.len() != 1 {
        return Err(refused("missing or duplicate rehearsal receipt"));
    }
    let files = archive::files(context, &request.repo, matches[0])?;
    if files.len() != 1 {
        return Err(refused("receipt artifact must contain exactly one file"));
    }
    serde_json::from_slice(
        files
            .get(RECEIPT)
            .ok_or_else(|| refused("missing receipt payload"))?,
    )
    .map_err(refused)
}

fn check_receipt(
    receipt: Receipt,
    run: &Run,
    version: &semver::Version,
    artifacts: &[Artifact],
) -> BuildResult<Vec<Artifact>> {
    if receipt.schema != 1
        || receipt.version != version.to_string()
        || receipt.commit != run.head_sha
        || receipt.run != run.id
        || receipt.attempt != run.run_attempt
    {
        return Err(refused("receipt version, SHA or attempt mismatch"));
    }
    let current = distribution(artifacts)?;
    if receipt.artifacts != current {
        return Err(refused(
            "distribution artifact identities changed after validation",
        ));
    }
    Ok(current)
}

/// Revalidates and restores a rehearsal's exact distribution, without publication.
/// Example: `restore(&context, 123, Path::new("dist"))?`.
pub(crate) fn restore(context: &BuildContext<'_>, id: u64, directory: &Path) -> BuildResult<()> {
    let request = ValidationRequest::tag(context, trigger_version(true)?)?;
    restore_request(context, &request, id, directory)
}

pub(super) fn restore_request(
    context: &BuildContext<'_>,
    request: &ValidationRequest,
    id: u64,
    directory: &Path,
) -> BuildResult<()> {
    let run = api::run(context, &request.repo, id)?;
    if !run_matches(&run, &request.repo, &request.head) {
        return Err(refused("run provenance changed"));
    }
    let artifacts = api::artifacts(context, &request.repo, id)?;
    let verified = validate(context, request, &run, &artifacts)?;
    restore_files(context, &request.repo, &verified, directory)
}

fn restore_files(
    context: &BuildContext<'_>,
    repo: &str,
    artifacts: &[Artifact],
    directory: &Path,
) -> BuildResult<()> {
    fs::create_dir(directory).map_err(|error| io_error(directory, error))?;
    for artifact in artifacts {
        for (name, bytes) in archive::files(context, repo, artifact)? {
            let path = directory.join(name);
            if path.exists() {
                return Err(refused("duplicate distribution payload"));
            }
            fs::write(&path, bytes).map_err(|error| io_error(&path, error))?;
        }
    }
    crate::publish::verified_assets(directory)?;
    Ok(())
}

#[cfg(test)]
mod tests;
