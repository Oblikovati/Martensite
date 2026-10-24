//! Release packages and tests are selected against the previous public release.

use std::{collections::BTreeMap, fs};

use semver::Version;
use toml::Table;

use crate::context::{BUILD_TOOL_PACKAGE, BuildContext};
use crate::error::{BuildError, BuildResult, io_error};
use crate::git::ChangedFile;
use crate::package_manifest::{PackageManifest, parse_table};
use crate::version::{parse_release_tag, previous_release};
use crate::workspace::CargoWorkspace;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ReleaseScope {
    pub baseline: Option<String>,
    pub tests: Vec<String>,
    pub packages: Vec<String>,
}

impl ReleaseScope {
    pub fn load(
        context: &BuildContext<'_>,
        workspace: &CargoWorkspace,
        version: &Version,
    ) -> BuildResult<Self> {
        let tags = context.git.release_tags()?;
        let baseline = previous_release(&tags, version).map(|v| format!("v{v}"));
        let Some(reference) = baseline.as_deref() else {
            return Ok(initial_scope(workspace));
        };
        let previous = published_manifests(context, &tags, version)?;
        let changed = context.git.changed_files(reference)?;
        let packages = selected_packages(context, workspace, &previous, &changed)?;
        let tests = crate::release_impact::release_tests(
            context, workspace, reference, &changed, &packages,
        )?;
        Ok(Self {
            baseline,
            tests,
            packages,
        })
    }
}

fn selected_packages(
    context: &BuildContext<'_>,
    workspace: &CargoWorkspace,
    previous: &BTreeMap<String, PackageManifest>,
    changed: &[ChangedFile],
) -> BuildResult<Vec<String>> {
    let inherited = workspace_fields(&read_manifest(context, "Cargo.toml")?)?;
    let mut packages = Vec::new();
    for package in workspace
        .packages
        .iter()
        .filter(|p| p.name != BUILD_TOOL_PACKAGE)
    {
        let manifest = current_manifest(context, &package.dir, &inherited)?;
        let old = previous.get(&manifest.name);
        let source_changed = package_changed(&manifest, old, &package.dir, changed);
        check_package_version(&manifest, old, source_changed)?;
        if source_changed {
            packages.push(package.name.clone());
        }
    }
    Ok(packages)
}

fn current_manifest(
    context: &BuildContext<'_>,
    directory: &str,
    inherited: &Table,
) -> BuildResult<PackageManifest> {
    let path = format!("{directory}/Cargo.toml");
    PackageManifest::parse(&read_manifest(context, &path)?, inherited)?
        .ok_or_else(|| BuildError::ReleaseRule(format!("manifest {path:?}; expected a package")))
}

fn initial_scope(workspace: &CargoWorkspace) -> ReleaseScope {
    ReleaseScope {
        baseline: None,
        tests: workspace.names(),
        packages: workspace
            .packages
            .iter()
            .filter(|p| p.name != BUILD_TOOL_PACKAGE)
            .map(|p| p.name.clone())
            .collect(),
    }
}

fn read_manifest(context: &BuildContext<'_>, path: &str) -> BuildResult<String> {
    let path = context.root().join(path);
    fs::read_to_string(&path).map_err(|cause| io_error(&path, cause))
}

fn workspace_fields(text: &str) -> BuildResult<Table> {
    let root = parse_table(text)?;
    Ok(root
        .get("workspace")
        .and_then(|workspace| workspace.get("package"))
        .and_then(toml::Value::as_table)
        .cloned()
        .unwrap_or_default())
}

fn published_manifests(
    context: &BuildContext<'_>,
    tags: &[String],
    version: &Version,
) -> BuildResult<BTreeMap<String, PackageManifest>> {
    let mut published: BTreeMap<String, PackageManifest> = BTreeMap::new();
    for tag in tags.iter().filter(|tag| prior_final(tag, version)) {
        for (name, manifest) in released_manifests(context, tag)? {
            if published
                .get(&name)
                .is_none_or(|previous| manifest.version.cmp_precedence(&previous.version).is_gt())
            {
                published.insert(name, manifest);
            }
        }
    }
    Ok(published)
}

fn prior_final(tag: &str, version: &Version) -> bool {
    parse_release_tag(tag).is_ok_and(|previous| previous.pre.is_empty() && previous < *version)
}

fn released_manifests(
    context: &BuildContext<'_>,
    reference: &str,
) -> BuildResult<BTreeMap<String, PackageManifest>> {
    let root = context.git.file_at(reference, "Cargo.toml")?;
    let inherited = workspace_fields(&root)?;
    let mut packages = BTreeMap::new();
    for path in context
        .git
        .paths_at(reference)?
        .iter()
        .filter(|p| p.ends_with("/Cargo.toml") || *p == "Cargo.toml")
    {
        let text = context.git.file_at(reference, path)?;
        insert_released(&mut packages, &text, &inherited)?;
    }
    Ok(packages)
}

fn insert_released(
    packages: &mut BTreeMap<String, PackageManifest>,
    text: &str,
    inherited: &Table,
) -> BuildResult<()> {
    let Some(manifest) = PackageManifest::parse(text, inherited)? else {
        return Ok(());
    };
    let name = manifest.name.clone();
    if packages.insert(name.clone(), manifest).is_some() {
        return Err(BuildError::ReleaseRule(format!(
            "released package {name:?}; expected a unique package name"
        )));
    }
    Ok(())
}

fn package_changed(
    current: &PackageManifest,
    previous: Option<&PackageManifest>,
    directory: &str,
    changed: &[ChangedFile],
) -> bool {
    let Some(previous) = previous else {
        return true;
    };
    current.source != previous.source
        || changed.iter().any(|file| {
            file.path.starts_with(&format!("{directory}/"))
                && file.path != format!("{directory}/Cargo.toml")
        })
}

fn check_package_version(
    current: &PackageManifest,
    previous: Option<&PackageManifest>,
    changed: bool,
) -> BuildResult<()> {
    let Some(previous) = previous else {
        return Ok(());
    };
    let allowed = if changed {
        current.version.cmp_precedence(&previous.version).is_gt()
    } else {
        current.version == previous.version
    };
    if allowed {
        return Ok(());
    }
    let expectation = if changed {
        "a newer version for changed package source"
    } else {
        "the same version for unchanged package source"
    };
    Err(BuildError::ReleaseRule(format!(
        "package {:?} version {} -> {}; expected {expectation}",
        current.name, previous.version, current.version
    )))
}

#[cfg(test)]
#[path = "release_scope_tests.rs"]
mod tests;
