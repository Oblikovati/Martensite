//! Maps a change set to the packages that must be rebuilt and retested:
//! the owners of the changed files plus every package that depends on them.

use std::collections::BTreeSet;

use crate::git::ChangedFile;
use crate::workspace::CargoWorkspace;

/// Files that change how every package builds, lints or tests.
const WORKSPACE_WIDE_FILES: [&str; 5] = [
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain.toml",
    "rustfmt.toml",
    "clippy.toml",
];

/// Directories whose content changes how every package builds.
const WORKSPACE_WIDE_DIRS: [&str; 1] = [".cargo/"];

/// Names of the packages a change set affects, sorted; empty when only
/// non-Rust files (docs, workflows) changed.
///
/// ```ignore
/// let names = affected_packages(&workspace, &changed);
/// ```
pub fn affected_packages(workspace: &CargoWorkspace, changed: &[ChangedFile]) -> BTreeSet<String> {
    if changed.iter().any(|file| is_workspace_wide(&file.path)) {
        return workspace.names().into_iter().collect();
    }
    let owners = changed
        .iter()
        .filter_map(|file| owning_package(workspace, &file.path));
    with_dependents(workspace, owners.collect())
}

fn is_workspace_wide(path: &str) -> bool {
    WORKSPACE_WIDE_FILES.contains(&path)
        || WORKSPACE_WIDE_DIRS.iter().any(|dir| path.starts_with(dir))
}

/// The package whose directory is the longest prefix of `path`, so a crate
/// nested inside another crate's directory still owns its own files.
fn owning_package(workspace: &CargoWorkspace, path: &str) -> Option<String> {
    workspace
        .packages
        .iter()
        .filter(|package| path.starts_with(&format!("{}/", package.dir)))
        .max_by_key(|package| package.dir.len())
        .map(|package| package.name.clone())
}

/// Grows `names` until it holds every package that depends on a member.
fn with_dependents(workspace: &CargoWorkspace, mut names: BTreeSet<String>) -> BTreeSet<String> {
    loop {
        let dependents: Vec<String> = workspace
            .packages
            .iter()
            .filter(|package| !names.contains(&package.name))
            .filter(|package| {
                package
                    .path_dependencies
                    .iter()
                    .any(|dep| names.contains(dep))
            })
            .map(|package| package.name.clone())
            .collect();
        if dependents.is_empty() {
            return names;
        }
        names.extend(dependents);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::tests::sample_metadata;

    fn changed(path: &str) -> ChangedFile {
        ChangedFile {
            path: path.into(),
            deleted: false,
            untracked: false,
        }
    }

    fn affected(paths: &[&str]) -> Vec<String> {
        let workspace = CargoWorkspace::parse(&sample_metadata()).unwrap();
        let files: Vec<_> = paths.iter().map(|path| changed(path)).collect();
        affected_packages(&workspace, &files).into_iter().collect()
    }

    #[test]
    fn dependency_change_pulls_in_its_dependents() {
        let names = affected(&["crates/martensite-math/src/lib.rs"]);
        assert_eq!(names, ["martensite-kernel", "martensite-math"]);
    }

    #[test]
    fn leaf_change_affects_only_the_leaf() {
        assert_eq!(
            affected(&["crates/martensite-kernel/src/lib.rs"]),
            ["martensite-kernel"]
        );
    }

    #[test]
    fn docs_only_change_affects_nothing() {
        assert!(affected(&["docs/architecture/README.md", "README.md"]).is_empty());
    }

    #[test]
    fn lockfile_or_cargo_config_change_affects_everything() {
        assert_eq!(affected(&["Cargo.lock"]).len(), 2);
        assert_eq!(affected(&[".cargo/config.toml"]).len(), 2);
    }

    #[test]
    fn sibling_directory_with_common_prefix_is_not_an_owner() {
        assert!(affected(&["crates/martensite-math-extra/src/lib.rs"]).is_empty());
    }
}
