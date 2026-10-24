//! Classifies shared build inputs before selecting release test suites.

use crate::affected::affected_packages;
use crate::context::BuildContext;
use crate::error::{BuildResult, io_error};
use crate::git::ChangedFile;
use crate::package_manifest::parse_table;
use crate::workspace::CargoWorkspace;
use std::collections::BTreeSet;
use toml::{Table, Value};

/// Selects tests affected by a PR while ignoring version-only bookkeeping.
/// Example: `let tests = pull_request_packages(context, &workspace, base, &changed)?;`.
pub(crate) fn pull_request_packages(
    context: &BuildContext<'_>,
    workspace: &CargoWorkspace,
    reference: &str,
    changed: &[ChangedFile],
) -> BuildResult<BTreeSet<String>> {
    let source =
        crate::release_scope::changed_manifest_packages(context, workspace, reference, changed)?;
    Ok(
        release_tests(context, workspace, reference, changed, &source)?
            .into_iter()
            .collect(),
    )
}

/// Includes source and dependency impact without selecting unaffected base suites.
/// Example: `let tests = release_tests(context, &workspace, base, &changed, &packages)?;`.
pub(crate) fn release_tests(
    context: &BuildContext<'_>,
    workspace: &CargoWorkspace,
    reference: &str,
    changed: &[ChangedFile],
    source_packages: &[String],
) -> BuildResult<Vec<String>> {
    let ordinary = ordinary_inputs(workspace, changed, source_packages);
    let mut selected = affected_packages(workspace, &ordinary);
    let Some(dependencies) = shared_dependencies(context, reference, changed)? else {
        return Ok(workspace.names());
    };
    selected.extend(dependency_consumers(workspace, &dependencies));
    Ok(selected.into_iter().collect())
}

fn ordinary_inputs(
    workspace: &CargoWorkspace,
    changed: &[ChangedFile],
    source_packages: &[String],
) -> Vec<ChangedFile> {
    changed
        .iter()
        .filter(|file| package_input(workspace, &file.path, source_packages))
        .filter(|file| {
            !matches!(
                file.path.as_str(),
                "Cargo.toml" | "Cargo.lock" | "rustfmt.toml" | "clippy.toml"
            )
        })
        .cloned()
        .collect()
}

fn shared_dependencies(
    context: &BuildContext<'_>,
    reference: &str,
    changed: &[ChangedFile],
) -> BuildResult<Option<BTreeSet<String>>> {
    let mut dependencies = BTreeSet::new();
    if changed.iter().any(|file| file.path == "Cargo.lock") {
        let (before, after) = shared_tables(context, reference, "Cargo.lock")?;
        dependencies.extend(lock_impact(&before, &after));
    }
    if changed.iter().any(|file| file.path == "Cargo.toml") {
        let Some(profiles) = profile_dependencies(context, reference)? else {
            return Ok(None);
        };
        dependencies.extend(profiles);
    }
    Ok(Some(dependencies))
}

fn profile_dependencies(
    context: &BuildContext<'_>,
    reference: &str,
) -> BuildResult<Option<BTreeSet<String>>> {
    let (before, after) = shared_tables(context, reference, "Cargo.toml")?;
    let Some(profiles) = profile_impact(&before, &after) else {
        return Ok(None);
    };
    let path = context.root().join("Cargo.lock");
    let lock = std::fs::read_to_string(&path).map_err(|cause| io_error(&path, cause))?;
    Ok(Some(profile_consumers(
        &lock_packages(&parse_table(&lock)?),
        &profiles,
    )))
}

fn package_input(workspace: &CargoWorkspace, path: &str, source_packages: &[String]) -> bool {
    let owner = workspace
        .packages
        .iter()
        .find(|package| path == format!("{}/Cargo.toml", package.dir));
    owner.is_none_or(|package| source_packages.contains(&package.name))
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

type LockIdentity<'a> = (&'a str, &'a str, Option<&'a str>);

fn lock_impact(before: &Table, after: &Table) -> BTreeSet<String> {
    let old = lock_packages(before);
    let new = lock_packages(after);
    let mut roots = snapshot_changes(&old, &new);
    roots.extend(snapshot_changes(&new, &old));
    roots
}

fn snapshot_changes(graph: &[&Table], other: &[&Table]) -> BTreeSet<String> {
    let changed = graph
        .iter()
        .filter(|package| changed_lock_record(package, other))
        .filter_map(|package| lock_identity(package))
        .collect();
    expand_consumers(graph, changed)
}

fn changed_lock_record(package: &Table, other: &[&Table]) -> bool {
    if package.contains_key("source") {
        return !other.contains(&package);
    }
    let previous = other
        .iter()
        .find(|old| !old.contains_key("source") && old.get("name") == package.get("name"));
    previous.is_none_or(|previous| previous.get("dependencies") != package.get("dependencies"))
}

fn lock_identity(package: &Table) -> Option<LockIdentity<'_>> {
    Some((
        package.get("name")?.as_str()?,
        package.get("version")?.as_str()?,
        package.get("source").and_then(Value::as_str),
    ))
}

fn profile_consumers(graph: &[&Table], names: &BTreeSet<String>) -> BTreeSet<String> {
    let changed = graph
        .iter()
        .filter_map(|package| lock_identity(package))
        .filter(|identity| names.contains(identity.0))
        .collect();
    expand_consumers(graph, changed)
}

fn expand_consumers<'a>(
    graph: &[&'a Table],
    mut changed: BTreeSet<LockIdentity<'a>>,
) -> BTreeSet<String> {
    loop {
        let consumers: Vec<_> = graph
            .iter()
            .filter(|package| consumes_changed(package, &changed))
            .filter_map(|package| lock_identity(package))
            .collect();
        let previous = changed.len();
        changed.extend(consumers);
        if previous == changed.len() {
            break;
        }
    }
    graph
        .iter()
        .filter(|package| !package.contains_key("source"))
        .filter_map(|package| lock_identity(package))
        .filter(|identity| changed.contains(identity))
        .map(|identity| identity.0.to_owned())
        .collect()
}

fn consumes_changed(package: &Table, changed: &BTreeSet<LockIdentity<'_>>) -> bool {
    package
        .get("dependencies")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .any(|dependency| {
            changed
                .iter()
                .any(|identity| dependency_matches(dependency, *identity))
        })
}

fn dependency_matches(dependency: &str, identity: LockIdentity<'_>) -> bool {
    let mut fields = dependency.split_whitespace();
    let name = fields.next();
    let version = fields.next();
    let source = fields
        .next()
        .map(|source| source.trim_start_matches('(').trim_end_matches(')'));
    name == Some(identity.0)
        && version.is_none_or(|version| version == identity.1)
        && source.is_none_or(|source| Some(source) == identity.2)
}

fn dependency_consumers(
    workspace: &CargoWorkspace,
    dependencies: &BTreeSet<String>,
) -> BTreeSet<String> {
    let synthetic: Vec<_> = workspace
        .packages
        .iter()
        .filter(|p| dependencies.contains(&p.name))
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
    let old_profiles = owned_table(old.remove("profile"));
    let new_profiles = owned_table(new.remove("profile"));
    if old != new {
        return None;
    }
    changed_profile_packages(&old_profiles, &new_profiles)
}

fn changed_profile_packages(old: &Table, new: &Table) -> Option<BTreeSet<String>> {
    let names = old.keys().chain(new.keys()).collect::<BTreeSet<_>>();
    let mut packages = BTreeSet::new();
    for name in names {
        packages.extend(profile_packages(old.get(name), new.get(name))?);
    }
    Some(packages)
}

fn owned_table(value: Option<Value>) -> Table {
    value
        .and_then(|value| value.as_table().cloned())
        .unwrap_or_default()
}

fn split_profile(value: Option<&Value>) -> (Table, Table) {
    let mut settings = value.and_then(Value::as_table).cloned().unwrap_or_default();
    let packages = owned_table(settings.remove("package"));
    (settings, packages)
}

fn profile_packages(before: Option<&Value>, after: Option<&Value>) -> Option<BTreeSet<String>> {
    let (old_settings, old) = split_profile(before);
    let (new_settings, new) = split_profile(after);
    if old_settings != new_settings {
        return None;
    }
    let changed: BTreeSet<_> = old
        .keys()
        .chain(new.keys())
        .filter(|name| old.get(*name) != new.get(*name))
        .cloned()
        .collect();
    if changed.iter().any(|name| !simple_package_name(name)) {
        return None;
    }
    Some(changed)
}

fn simple_package_name(name: &str) -> bool {
    name.chars()
        .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
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
            BTreeSet::from(["kernel".into()])
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
            dependency_consumers(&workspace, &BTreeSet::from(["martensite-kernel".into()])),
            BTreeSet::from(["martensite-kernel".into()])
        );
    }
}

#[cfg(test)]
mod bookkeeping_tests {
    use super::*;
    #[test]
    fn own_version_and_unchanged_dependency_bookkeeping_do_not_select_base_suites() {
        let workspace = CargoWorkspace::parse(&crate::workspace::tests::sample_metadata()).unwrap();
        assert!(!package_input(
            &workspace,
            "crates/martensite-math/Cargo.toml",
            &["martensite-kernel".into()]
        ));
        assert!(package_input(
            &workspace,
            "crates/martensite-kernel/Cargo.toml",
            &["martensite-kernel".into()]
        ));
        assert!(package_input(
            &workspace,
            "crates/martensite-math/src/lib.rs",
            &[]
        ));
    }
}

#[cfg(test)]
mod identity_tests {
    use super::*;
    fn duplicate_graph(checksum: &str) -> Table {
        parse_table(&format!("[[package]]\nname='ext'\nversion='1.0.0'\nsource='registry'\nchecksum='stable'\n[[package]]\nname='ext'\nversion='2.0.0'\nsource='registry'\nchecksum='{checksum}'\n[[package]]\nname='a'\nversion='1.0.0'\ndependencies=['ext 1.0.0']\n[[package]]\nname='b'\nversion='1.0.0'\ndependencies=['ext 2.0.0']")).unwrap()
    }
    #[test]
    fn changed_duplicate_dependency_selects_only_its_actual_consumers() {
        assert_eq!(
            lock_impact(&duplicate_graph("old"), &duplicate_graph("new")),
            BTreeSet::from(["b".into()])
        );
    }
    #[test]
    fn dependency_references_preserve_version_and_source_identity() {
        let identity = (
            "ext",
            "2.0.0",
            Some("registry+https://example.invalid/index"),
        );
        assert!(dependency_matches(
            "ext 2.0.0 (registry+https://example.invalid/index)",
            identity
        ));
        assert!(!dependency_matches("ext 1.0.0", identity));
        assert!(!dependency_matches(
            "ext 2.0.0 (registry+https://other.invalid/index)",
            identity
        ));
    }
}

#[cfg(test)]
mod rewired_lock_tests {
    use super::*;
    fn graph(dependency: &str) -> Table {
        parse_table(&format!("[[package]]\nname='ext'\nversion='1.0.0'\nsource='registry'\n[[package]]\nname='ext'\nversion='1.0.1'\nsource='registry'\n[[package]]\nname='a'\nversion='1.0.0'\ndependencies=['ext {dependency}']\n[[package]]\nname='b'\nversion='1.0.0'\ndependencies=['ext 1.0.0']\n[[package]]\nname='c'\nversion='1.0.0'\ndependencies=['ext 1.0.1']")).unwrap()
    }
    #[test]
    fn existing_external_versions_can_rewire_one_workspace_consumer() {
        assert_eq!(
            lock_impact(&graph("1.0.0"), &graph("1.0.1")),
            BTreeSet::from(["a".into()])
        );
    }
}
