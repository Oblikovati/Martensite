//! The cargo workspace as the pipelines see it: packages, their directories,
//! their in-workspace dependencies and versions, read from `cargo metadata`.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::{BuildError, BuildResult};
use crate::process::{CommandRunner, CommandSpec};

/// One workspace member.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspacePackage {
    /// The cargo package name, as accepted by `--package`.
    pub name: String,
    /// The version declared in the manifest.
    pub version: semver::Version,
    /// The package directory relative to the workspace root, `/`-separated.
    pub dir: String,
    /// Names of dependencies that are path dependencies (workspace siblings).
    pub path_dependencies: Vec<String>,
}

/// All members of the workspace rooted at the repository root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CargoWorkspace {
    /// Members sorted by name.
    pub packages: Vec<WorkspacePackage>,
}

#[derive(Deserialize)]
struct MetadataDocument {
    workspace_root: PathBuf,
    packages: Vec<MetadataPackage>,
}

#[derive(Deserialize)]
struct MetadataPackage {
    name: String,
    version: semver::Version,
    manifest_path: PathBuf,
    dependencies: Vec<MetadataDependency>,
}

#[derive(Deserialize)]
struct MetadataDependency {
    name: String,
    path: Option<PathBuf>,
}

impl CargoWorkspace {
    /// Reads the workspace with `cargo metadata --no-deps`.
    ///
    /// ```ignore
    /// let workspace = CargoWorkspace::load(&ProcessRunner, root)?;
    /// ```
    pub fn load(runner: &dyn CommandRunner, root: &Path) -> BuildResult<Self> {
        let spec = CommandSpec::new("cargo", root).args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--locked",
        ]);
        Self::parse(&runner.capture(&spec)?)
    }

    /// Parses `cargo metadata --format-version 1 --no-deps` JSON.
    pub fn parse(json: &str) -> BuildResult<Self> {
        let document: MetadataDocument =
            serde_json::from_str(json).map_err(|error| BuildError::UnexpectedOutput {
                origin: "cargo metadata".into(),
                detail: format!("{error}; expected format-version 1 JSON"),
            })?;
        let root = document.workspace_root;
        let mut packages: Vec<_> = document
            .packages
            .into_iter()
            .map(|package| member(&root, package))
            .collect::<BuildResult<_>>()?;
        packages.sort_by(|left, right| left.name.cmp(&right.name));
        Ok(Self { packages })
    }

    /// All package names, sorted.
    pub fn names(&self) -> Vec<String> {
        self.packages
            .iter()
            .map(|package| package.name.clone())
            .collect()
    }
}

fn member(root: &Path, package: MetadataPackage) -> BuildResult<WorkspacePackage> {
    let dir = package_dir(root, &package.manifest_path)?;
    let path_dependencies = package
        .dependencies
        .into_iter()
        .filter(|dependency| dependency.path.is_some())
        .map(|dependency| dependency.name)
        .collect();
    Ok(WorkspacePackage {
        name: package.name,
        version: package.version,
        dir,
        path_dependencies,
    })
}

/// The manifest's directory relative to the root, with `/` separators so it
/// compares directly with paths printed by git on every platform.
fn package_dir(root: &Path, manifest: &Path) -> BuildResult<String> {
    let relative = manifest
        .parent()
        .and_then(|dir| dir.strip_prefix(root).ok())
        .ok_or_else(|| BuildError::UnexpectedOutput {
            origin: "cargo metadata".into(),
            detail: format!(
                "manifest {} is not under workspace root {}; expected a workspace member",
                manifest.display(),
                root.display()
            ),
        })?;
    let parts: Vec<_> = relative
        .components()
        .map(|part| part.as_os_str().to_string_lossy())
        .collect();
    Ok(parts.join("/"))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// A two-crate workspace: `martensite-kernel` depends on `martensite-math`.
    pub(crate) fn sample_metadata() -> String {
        metadata_at("/repo", "0.3.0")
    }

    /// The same two crates, rooted at `root` and both at `version`.
    pub(crate) fn metadata_at(root: &str, version: &str) -> String {
        serde_json::json!({
            "workspace_root": root,
            "packages": [
                {"name": "martensite-kernel", "version": version,
                 "manifest_path": format!("{root}/crates/martensite-kernel/Cargo.toml"),
                 "dependencies": [
                    {"name": "martensite-math", "path": format!("{root}/crates/martensite-math")},
                    {"name": "serde", "path": null}]},
                {"name": "martensite-math", "version": version,
                 "manifest_path": format!("{root}/crates/martensite-math/Cargo.toml"),
                 "dependencies": []}
            ]
        })
        .to_string()
    }

    #[test]
    fn parse_keeps_relative_dirs_and_path_dependencies_only() {
        let workspace = CargoWorkspace::parse(&sample_metadata()).unwrap();
        let kernel = &workspace.packages[0];
        assert_eq!(kernel.dir, "crates/martensite-kernel");
        assert_eq!(kernel.path_dependencies, ["martensite-math"]);
        assert_eq!(kernel.version, semver::Version::new(0, 3, 0));
    }

    #[test]
    fn parse_rejects_non_metadata_json() {
        let error = CargoWorkspace::parse("{}").unwrap_err();
        assert!(error.to_string().contains("expected format-version 1 JSON"));
    }

    #[test]
    fn manifest_outside_root_is_rejected_with_both_paths() {
        let error =
            package_dir(Path::new("/repo"), Path::new("/elsewhere/Cargo.toml")).unwrap_err();
        assert!(error.to_string().contains("/elsewhere/Cargo.toml"));
    }

    #[test]
    fn names_are_sorted() {
        let workspace = CargoWorkspace::parse(&sample_metadata()).unwrap();
        assert_eq!(workspace.names(), ["martensite-kernel", "martensite-math"]);
    }
}
