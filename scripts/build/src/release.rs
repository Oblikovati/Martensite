//! Release gates and artifacts for one target. Selected tests run locally or
//! in the required CI matrix. This pipeline never publishes; `publish` does.

use std::fs;
use std::path::{Path, PathBuf};

use semver::Version;

use crate::archive::{BundleEntry, archive_member_paths, file_name, write_checksum, write_tar_gz};
use crate::cargo::{BuildTarget, CargoTasks, PackageSelection, ShippedArtifact};
use crate::changelog::release_notes;
use crate::context::{BUILD_TOOL_PACKAGE, BuildContext};
use crate::error::{BuildError, BuildResult, io_error};
use crate::release_scope::ReleaseScope;
use crate::version::{check_bump, previous_release};
use crate::workspace::{CargoWorkspace, WorkspacePackage};

/// Repository documents shipped inside every binary archive. Official binaries
/// are offered under the Apache license; the source stays GPL (#176).
const BUNDLED_DOCUMENTS: [&str; 3] = ["LICENSE-APACHE", "README.md", "CHANGELOG.md"];

/// The license text every released source package must carry (#176). Cargo
/// copies it into each `.crate` from the workspace `license-file`.
const SOURCE_LICENSE: &str = "LICENSE-GPL";

/// What to release and where to put the packages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseRequest {
    /// The version being released.
    pub version: Version,
    /// The target triple to build and test for.
    pub triple: String,
    /// Also produce the platform-independent `.crate` source packages.
    pub source_packages: bool,
    /// Directory that receives archives and checksums; recreated each run.
    pub dist_dir: PathBuf,
}

/// The default package directory for a version: `target/dist/v<version>`.
///
/// ```ignore
/// let dist = default_dist_dir(context.root(), &version);
/// ```
pub fn default_dist_dir(root: &Path, version: &Version) -> PathBuf {
    root.join("target").join("dist").join(format!("v{version}"))
}

/// Runs the whole release pipeline and returns the files it packaged.
pub fn run_release(
    context: &BuildContext<'_>,
    request: &ReleaseRequest,
) -> BuildResult<Vec<PathBuf>> {
    run_release_checks(context, request, true)
}

/// CI may delegate tests to its required matrix while retaining every other gate.
/// Example: `run_release_checks(context, request, false)` prepares CI artifacts.
pub(crate) fn run_release_checks(
    context: &BuildContext<'_>,
    request: &ReleaseRequest,
    run_tests: bool,
) -> BuildResult<Vec<PathBuf>> {
    let workspace = CargoWorkspace::load(context.runner, context.root())?;
    let reporter = &context.reporter;
    reporter.step("Validate release version", || {
        validate_version(context, &workspace, &request.version)
    })?;
    let scope = ReleaseScope::load(context, &workspace, &request.version)?;
    let cargo = CargoTasks::new(context.runner, context.root());
    let target = BuildTarget::Triple(request.triple.clone());
    verify_workspace(context, &cargo, &target, &scope.tests, run_tests)?;
    let shipped = PackageSelection::WorkspaceExcept(vec![BUILD_TOOL_PACKAGE.to_owned()]);
    let artifacts = reporter.step("Build release", || cargo.release_build(&shipped, &target))?;
    reporter.step("Package release", || {
        package_release(context, request, &workspace, &scope, &artifacts)
    })
}

/// Release test scope is computed from the previous release, never the PR base.
fn verify_workspace(
    context: &BuildContext<'_>,
    cargo: &CargoTasks<'_>,
    target: &BuildTarget,
    tests: &[String],
    run_tests: bool,
) -> BuildResult<()> {
    let reporter = &context.reporter;
    reporter.step("Check formatting", || cargo.format_check_all())?;
    reporter.step("Clippy workspace", || {
        cargo.clippy(&PackageSelection::Workspace, target)
    })?;
    if !run_tests || tests.is_empty() {
        return Ok(());
    }
    reporter.step("Test changed and affected packages", || {
        cargo.test(&PackageSelection::Packages(tests.to_vec()), target)
    })
}

fn releasable(workspace: &CargoWorkspace) -> Vec<&WorkspacePackage> {
    workspace
        .packages
        .iter()
        .filter(|package| package.name != BUILD_TOOL_PACKAGE)
        .collect()
}

fn validate_version(
    context: &BuildContext<'_>,
    workspace: &CargoWorkspace,
    version: &Version,
) -> BuildResult<()> {
    if releasable(workspace).is_empty() {
        return Err(BuildError::ReleaseRule(
            "the workspace has no releasable package".into(),
        ));
    }
    let changelog_path = context.root().join("CHANGELOG.md");
    let changelog =
        fs::read_to_string(&changelog_path).map_err(|cause| io_error(&changelog_path, cause))?;
    let notes = release_notes(&changelog, &version.to_string())?;
    let previous = previous_release(&context.git.release_tags()?, version);
    check_bump(previous.as_ref(), version, &notes)?;
    let baseline = previous.map_or("no earlier release".to_owned(), |previous| {
        format!("previous release {previous}")
    });
    context
        .reporter
        .note(&format!("Releasing {version} ({baseline})"));
    Ok(())
}

fn package_release(
    context: &BuildContext<'_>,
    request: &ReleaseRequest,
    workspace: &CargoWorkspace,
    scope: &ReleaseScope,
    artifacts: &[ShippedArtifact],
) -> BuildResult<Vec<PathBuf>> {
    recreate_dir(&request.dist_dir)?;
    let mut assets = package_binaries(context, request, artifacts)?;
    if request.source_packages {
        assets.extend(package_sources(
            context,
            request,
            workspace,
            &scope.packages,
        )?);
        assets.extend(crate::release_manifest::write_package_manifest(
            request, workspace, scope,
        )?);
    }
    for asset in &assets {
        context
            .reporter
            .note(&format!("Packaged {}", asset.display()));
    }
    Ok(assets)
}

fn recreate_dir(dir: &Path) -> BuildResult<()> {
    if dir.exists() {
        fs::remove_dir_all(dir).map_err(|cause| io_error(dir, cause))?;
    }
    fs::create_dir_all(dir).map_err(|cause| io_error(dir, cause))
}

/// Archives executables and native libraries; a library-only workspace has
/// none, and its consumers use the source packages instead.
fn package_binaries(
    context: &BuildContext<'_>,
    request: &ReleaseRequest,
    artifacts: &[ShippedArtifact],
) -> BuildResult<Vec<PathBuf>> {
    if artifacts.is_empty() {
        context
            .reporter
            .note("No executable or native library to archive for this target.");
        return Ok(Vec::new());
    }
    let top_dir = format!("martensite-{}-{}", request.version, request.triple);
    let archive = request.dist_dir.join(format!("{top_dir}.tar.gz"));
    let mut entries = artifact_entries(artifacts)?;
    entries.extend(document_entries(context.root()));
    write_tar_gz(&archive, &top_dir, &entries)?;
    let checksum = write_checksum(&archive)?;
    Ok(vec![archive, checksum])
}

fn artifact_entries(artifacts: &[ShippedArtifact]) -> BuildResult<Vec<BundleEntry>> {
    let entry = |artifact: &ShippedArtifact| -> BuildResult<BundleEntry> {
        let folder = if artifact.executable { "bin" } else { "lib" };
        Ok(BundleEntry {
            source: artifact.path.clone(),
            archive_path: format!("{folder}/{}", file_name(&artifact.path)?),
            executable: artifact.executable,
        })
    };
    artifacts.iter().map(entry).collect()
}

fn document_entries(root: &Path) -> Vec<BundleEntry> {
    let entry = |name: &str| BundleEntry {
        source: root.join(name),
        archive_path: name.to_owned(),
        executable: false,
    };
    BUNDLED_DOCUMENTS.iter().map(|name| entry(name)).collect()
}

/// `cargo package` verifies each crate builds from its packaged sources alone.
fn package_sources(
    context: &BuildContext<'_>,
    request: &ReleaseRequest,
    workspace: &CargoWorkspace,
    changed: &[String],
) -> BuildResult<Vec<PathBuf>> {
    if changed.is_empty() {
        return Ok(Vec::new());
    }
    let names: Vec<String> = releasable(workspace)
        .iter()
        .map(|package| package.name.clone())
        .collect();
    CargoTasks::new(context.runner, context.root())
        .package_sources(&PackageSelection::Packages(names.clone()))?;
    let mut assets = Vec::new();
    for package in releasable(workspace)
        .into_iter()
        .filter(|p| changed.contains(&p.name))
    {
        assets.extend(ship_crate(context.root(), request, package)?);
    }
    Ok(assets)
}

/// Copies `target/package/<name>-<version>.crate` into the dist directory.
fn ship_crate(
    root: &Path,
    request: &ReleaseRequest,
    package: &WorkspacePackage,
) -> BuildResult<[PathBuf; 2]> {
    let package_dir = format!("{}-{}", package.name, package.version);
    let crate_file = format!("{package_dir}.crate");
    let packaged = root.join("target").join("package").join(&crate_file);
    require_source_license(&packaged, &package_dir)?;
    let shipped = request.dist_dir.join(&crate_file);
    fs::copy(&packaged, &shipped).map_err(|cause| io_error(&packaged, cause))?;
    let checksum = write_checksum(&shipped)?;
    Ok([shipped, checksum])
}

/// Refuses a `.crate` whose top directory `package_dir` lacks the GPL text.
fn require_source_license(crate_file: &Path, package_dir: &str) -> BuildResult<()> {
    let expected = format!("{package_dir}/{SOURCE_LICENSE}");
    if archive_member_paths(crate_file)?.contains(&expected) {
        return Ok(());
    }
    Err(BuildError::ReleaseRule(format!(
        "{} has no {expected}; expected the crate to set `license-file.workspace = true`",
        crate_file.display()
    )))
}

#[cfg(test)]
#[path = "release_tests.rs"]
pub(crate) mod tests;
