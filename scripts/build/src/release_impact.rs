//! Classifies shared build inputs before selecting release test suites.

use crate::affected::affected_packages;
use crate::context::BuildContext;
use crate::error::{BuildResult, io_error};
use crate::git::ChangedFile;
use crate::package_manifest::parse_table;
use crate::workspace::CargoWorkspace;
use std::collections::BTreeSet;
use toml::{Table, Value};

pub(crate) fn release_tests(
    context: &BuildContext<'_>,
    workspace: &CargoWorkspace,
    reference: &str,
    changed: &[ChangedFile],
    source_packages: &[String],
) -> BuildResult<Vec<String>> {
    let ordinary: Vec<_> = changed
        .iter()
        .filter(|f| package_input(workspace, &f.path, source_packages))
        .filter(|f| {
            !matches!(
                f.path.as_str(),
                "Cargo.toml" | "Cargo.lock" | "rustfmt.toml" | "clippy.toml"
            )
        })
        .cloned()
        .collect();
    let mut selected = affected_packages(workspace, &ordinary);
    let mut dependencies = BTreeSet::new();
    if changed.iter().any(|f| f.path == "Cargo.lock") {
        let (before, after) = shared_tables(context, reference, "Cargo.lock")?;
        dependencies.extend(lock_impact(&before, &after));
    }
    if changed.iter().any(|f| f.path == "Cargo.toml") {
        let (before, after) = shared_tables(context, reference, "Cargo.toml")?;
        let Some(profiles) = profile_impact(&before, &after) else {
            return Ok(workspace.names());
        };
        let lock_path = context.root().join("Cargo.lock");
        let lock =
            std::fs::read_to_string(&lock_path).map_err(|cause| io_error(&lock_path, cause))?;
        dependencies.extend(expand_consumers(
            &lock_packages(&parse_table(&lock)?),
            profiles,
        ));
    }
    selected.extend(dependency_consumers(workspace, &dependencies));
    Ok(selected.into_iter().collect())
}

fn package_input(workspace: &CargoWorkspace, path: &str, source_packages: &[String]) -> bool {
    let owner = workspace
        .packages
        .iter()
        .find(|package| path == format!("{}/Cargo.toml", package.dir));
    owner.is_none_or(|package| {
        package.name == crate::context::BUILD_TOOL_PACKAGE
            || source_packages.contains(&package.name)
    })
}

fn shared_tables(
    context: &BuildContext<'_>,
    reference: &str,
    path: &str,
) -> BuildResult<(Table, Table)> {
    let before = parse_table(&context.git.file_at(reference, path)?)?;
    let full_path = context.root().join(path);
    let after = std::fs::read_to_string(&full_path).map_err(|cause| io_error(&full_path, cause))?;
    Ok((before, parse_table(&after)?))
}

fn lock_packages(table: &Table) -> Vec<&Table> {
    table
        .get("package")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_table)
        .collect()
}

fn lock_impact(before: &Table, after: &Table) -> BTreeSet<String> {
    let old = lock_packages(before);
    let new = lock_packages(after);
    let graph: Vec<_> = old.iter().chain(&new).copied().collect();
    let changed: BTreeSet<_> = graph
        .iter()
        .filter(|p| p.contains_key("source"))
        .filter(|p| !(old.contains(p) && new.contains(p)))
        .filter_map(|p| package_name(p))
        .collect();
    expand_consumers(&graph, changed)
}

fn expand_consumers(graph: &[&Table], mut changed: BTreeSet<String>) -> BTreeSet<String> {
    loop {
        let consumers: Vec<_> = graph
            .iter()
            .filter(|p| dependency_names(p).any(|name| changed.contains(name)))
            .filter_map(|p| package_name(p))
            .collect();
        let previous = changed.len();
        changed.extend(consumers);
        if previous == changed.len() {
            return changed;
        }
    }
}

fn package_name(package: &Table) -> Option<String> {
    package
        .get("name")
        .and_then(Value::as_str)
        .map(str::to_owned)
}

fn dependency_names(package: &Table) -> impl Iterator<Item = &str> {
    package
        .get("dependencies")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .filter_map(|name| name.split_whitespace().next())
}

fn dependency_consumers(
    workspace: &CargoWorkspace,
    dependencies: &BTreeSet<String>,
) -> BTreeSet<String> {
    let synthetic: Vec<_> = workspace
        .packages
        .iter()
        .filter(|p| {
            dependencies.contains(&p.name)
                || p.dependencies.iter().any(|d| dependencies.contains(d))
        })
        .map(|p| ChangedFile {
            path: format!("{}/src/lib.rs", p.dir),
            deleted: false,
            untracked: false,
        })
        .collect();
    affected_packages(workspace, &synthetic)
}

fn profile_impact(before: &Table, after: &Table) -> Option<BTreeSet<String>> {
    let mut old = before.clone();
    let mut new = after.clone();
    let old_profiles = old.remove("profile");
    let new_profiles = new.remove("profile");
    if old != new {
        return None;
    }
    let empty = Table::new();
    let old_profiles = old_profiles
        .as_ref()
        .and_then(Value::as_table)
        .unwrap_or(&empty);
    let new_profiles = new_profiles
        .as_ref()
        .and_then(Value::as_table)
        .unwrap_or(&empty);
    let mut packages = BTreeSet::new();
    for name in old_profiles
        .keys()
        .chain(new_profiles.keys())
        .collect::<BTreeSet<_>>()
    {
        packages.extend(profile_packages(
            old_profiles.get(name),
            new_profiles.get(name),
        )?);
    }
    Some(packages)
}

fn profile_packages(before: Option<&Value>, after: Option<&Value>) -> Option<BTreeSet<String>> {
    let mut old = before
        .and_then(Value::as_table)
        .cloned()
        .unwrap_or_default();
    let mut new = after.and_then(Value::as_table).cloned().unwrap_or_default();
    let old_packages = old.remove("package");
    let new_packages = new.remove("package");
    if old != new {
        return None;
    }
    let old = old_packages
        .and_then(|v| v.as_table().cloned())
        .unwrap_or_default();
    let new = new_packages
        .and_then(|v| v.as_table().cloned())
        .unwrap_or_default();
    let changed: BTreeSet<_> = old
        .keys()
        .chain(new.keys())
        .filter(|name| old.get(*name) != new.get(*name))
        .cloned()
        .collect();
    if changed.iter().any(|name| {
        !name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
    }) {
        return None;
    }
    Some(changed)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lock_transitive_changes_reach_consumers_but_workspace_versions_alone_do_not() {
        let before = parse_table("[[package]]\nname='inner'\nversion='1'\nsource='registry'\n[[package]]\nname='outer'\nversion='1'\nsource='registry'\ndependencies=['inner']\n[[package]]\nname='kernel'\nversion='1'\ndependencies=['outer']").unwrap();
        let after = parse_table("[[package]]\nname='inner'\nversion='2'\nsource='registry'\n[[package]]\nname='outer'\nversion='1'\nsource='registry'\ndependencies=['inner']\n[[package]]\nname='kernel'\nversion='2'\ndependencies=['outer']").unwrap();
        assert_eq!(
            lock_impact(&before, &after),
            BTreeSet::from(["inner".into(), "outer".into(), "kernel".into()])
        );
        let before = parse_table("[[package]]\nname='kernel'\nversion='1'").unwrap();
        let after = parse_table("[[package]]\nname='kernel'\nversion='2'").unwrap();
        assert!(lock_impact(&before, &after).is_empty());
    }
    #[test]
    fn profile_override_selects_dependency_and_global_settings_select_all() {
        let before = parse_table("[workspace]\nmembers=[]").unwrap();
        let after =
            parse_table("[workspace]\nmembers=[]\n[profile.dev.package.inner]\nopt-level=3")
                .unwrap();
        assert_eq!(
            profile_impact(&before, &after),
            Some(BTreeSet::from(["inner".into()]))
        );
        let global = parse_table("[workspace]\nmembers=[]\n[profile.dev]\nopt-level=3").unwrap();
        assert!(profile_impact(&before, &global).is_none());
    }
    #[test]
    fn dependency_impact_includes_transitive_dependents_but_not_base_packages() {
        let workspace = CargoWorkspace::parse(&crate::workspace::tests::sample_metadata()).unwrap();
        assert_eq!(
            dependency_consumers(&workspace, &BTreeSet::from(["serde".into()])),
            BTreeSet::from(["martensite-kernel".into()])
        );
    }
}
