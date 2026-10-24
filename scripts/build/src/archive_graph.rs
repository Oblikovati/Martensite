//! Exact package archives retain their original dependency contracts across releases (#279).
mod manifest;

use crate::{
    archive,
    context::BUILD_TOOL_PACKAGE,
    error::{BuildError, BuildResult},
    publication_plan::{PublicationPlan, PublicationReason},
    published_history::archive_manifest,
    release::ReleaseRequest,
    workspace::CargoWorkspace,
};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
/// Requirement evidence from exact archives; it makes no archive-build claim.
pub(crate) struct ArchiveGraph {
    pub compatibility: String,
    pub validation: String,
    pub archives: BTreeMap<String, ArchiveIdentity>,
    pub incompatible_requirements: Vec<Mismatch>,
}

#[derive(Debug, Serialize)]
/// Published or newly shipped archive bytes used in requirement validation.
pub(crate) struct ArchiveIdentity {
    pub version: String,
    pub release: String,
    pub sha256: String,
}

#[derive(Debug, Serialize)]
pub(super) struct Mismatch {
    package: String,
    dependency: String,
    requirement: String,
    selected_version: String,
}

/// Reports actual archive compatibility; e.g. `let graph = inspect(request, workspace, plan)?;`.
pub(crate) fn inspect(
    request: &ReleaseRequest,
    workspace: &CargoWorkspace,
    plan: &PublicationPlan,
) -> BuildResult<ArchiveGraph> {
    plan.require_sealed()?;
    let input = Inspection {
        request,
        plan,
        selected: selected_versions(workspace),
    };
    let mut graph = empty_graph();
    for package in packages(workspace) {
        let (identity, mismatches) = input.package(package)?;
        graph.archives.insert(package.name.clone(), identity);
        graph.incompatible_requirements.extend(mismatches);
    }
    Ok(graph.complete())
}

impl ArchiveGraph {
    fn complete(mut self) -> Self {
        if !self.incompatible_requirements.is_empty() {
            self.compatibility = "incompatible".into();
        }
        self
    }
}

fn packages(
    workspace: &CargoWorkspace,
) -> impl Iterator<Item = &crate::workspace::WorkspacePackage> {
    workspace
        .packages
        .iter()
        .filter(|p| p.name != BUILD_TOOL_PACKAGE)
}

fn selected_versions(workspace: &CargoWorkspace) -> BTreeMap<String, semver::Version> {
    workspace
        .packages
        .iter()
        .filter(|p| p.name != BUILD_TOOL_PACKAGE)
        .map(|p| (p.name.clone(), p.version.clone()))
        .collect()
}

struct Inspection<'a> {
    request: &'a ReleaseRequest,
    plan: &'a PublicationPlan,
    selected: BTreeMap<String, semver::Version>,
}

impl Inspection<'_> {
    fn package(
        &self,
        package: &crate::workspace::WorkspacePackage,
    ) -> BuildResult<(ArchiveIdentity, Vec<Mismatch>)> {
        let path = self
            .plan
            .archive_path(&package.name, &self.request.dist_dir)?;
        archive::verify_checksum(&path)?;
        let text = archive_manifest(&path, &package.name, &package.version)?;
        let sha256 = archive::sha256_hex(&path)?;
        let release = selected_origin(self.plan, &package.name, &sha256, self.request)?;
        let mismatches = manifest::mismatches(&package.name, &text, &self.selected)?;
        let identity = ArchiveIdentity {
            version: package.version.to_string(),
            release,
            sha256,
        };
        Ok((identity, mismatches))
    }
}

fn selected_origin(
    plan: &PublicationPlan,
    name: &str,
    digest: &str,
    request: &ReleaseRequest,
) -> BuildResult<String> {
    let selected = plan
        .packages
        .get(name)
        .ok_or_else(|| invalid_plan(name, "a selected package"))?;
    if selected.archive_sha256.as_deref() != Some(digest) {
        return Err(invalid_plan(
            digest,
            "the sealed publication archive digest",
        ));
    }
    publication_release(selected, request)
}

fn publication_release(
    selected: &crate::publication_plan::PackagePublication,
    request: &ReleaseRequest,
) -> BuildResult<String> {
    if selected.reason != PublicationReason::HistoricalReuse {
        return Ok(format!("v{}", request.version));
    }
    selected
        .origins
        .first()
        .map(|origin| origin.release_tag.clone())
        .ok_or_else(|| invalid_plan(&selected.name, "verified historical release provenance"))
}

fn invalid_plan(value: &str, expected: &str) -> BuildError {
    BuildError::ReleaseRule(format!(
        "archive publication {value:?}; expected {expected}"
    ))
}

fn empty_graph() -> ArchiveGraph {
    ArchiveGraph {
        compatibility: "requirements-compatible".into(),
        validation: "internal-version-requirements-from-checksummed-normalized-manifests; not an archive consumer build".into(),
        archives: BTreeMap::new(),
        incompatible_requirements: Vec::new(),
    }
}

#[cfg(test)]
mod fixture;
#[cfg(test)]
mod tests;
