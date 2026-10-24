//! Exact package archives retain their original dependency contracts across releases (#279).
mod fetch;
mod manifest;

use crate::{
    archive,
    context::{BUILD_TOOL_PACKAGE, BuildContext},
    error::{BuildResult, io_error},
    release::ReleaseRequest,
    release_scope::ReleaseScope,
    workspace::CargoWorkspace,
};
use serde::Serialize;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

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

/// Reports actual archive compatibility; e.g. `let graph = inspect(context, request, workspace, scope)?;`.
pub(crate) fn inspect(
    context: &BuildContext<'_>,
    request: &ReleaseRequest,
    workspace: &CargoWorkspace,
    scope: &ReleaseScope,
) -> BuildResult<ArchiveGraph> {
    let temporary =
        tempfile::tempdir().map_err(|cause| io_error(Path::new("archive compatibility"), cause))?;
    let input = Inspection {
        context,
        request,
        scope,
        directory: temporary.path(),
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

struct Inspection<'a, 'r> {
    context: &'a BuildContext<'r>,
    request: &'a ReleaseRequest,
    scope: &'a ReleaseScope,
    directory: &'a Path,
    selected: BTreeMap<String, semver::Version>,
}

impl Inspection<'_, '_> {
    fn package(
        &self,
        package: &crate::workspace::WorkspacePackage,
    ) -> BuildResult<(ArchiveIdentity, Vec<Mismatch>)> {
        let name = format!("{}-{}.crate", package.name, package.version);
        let (path, release) = archive_path(
            self.context,
            self.request,
            self.scope,
            &name,
            &package.name,
            self.directory,
        )?;
        let text = manifest::read(&path, &package.name, &package.version)?;
        let mismatches = manifest::mismatches(&package.name, &text, &self.selected)?;
        let identity = ArchiveIdentity {
            version: package.version.to_string(),
            release,
            sha256: archive::sha256_hex(&path)?,
        };
        Ok((identity, mismatches))
    }
}

fn empty_graph() -> ArchiveGraph {
    ArchiveGraph {
        compatibility: "requirements-compatible".into(),
        validation: "internal-version-requirements-from-checksummed-normalized-manifests; not an archive consumer build".into(),
        archives: BTreeMap::new(),
        incompatible_requirements: Vec::new(),
    }
}

fn archive_path(
    context: &BuildContext<'_>,
    request: &ReleaseRequest,
    scope: &ReleaseScope,
    archive: &str,
    package: &str,
    directory: &Path,
) -> BuildResult<(PathBuf, String)> {
    if scope.packages.iter().any(|name| name == package) {
        let path = request.dist_dir.join(archive);
        crate::archive::verify_checksum(&path)?;
        return Ok((path, format!("v{}", request.version)));
    }
    fetch::published(context, &request.version, archive, directory)
}

#[cfg(test)]
mod fixture;
#[cfg(test)]
mod tests;
