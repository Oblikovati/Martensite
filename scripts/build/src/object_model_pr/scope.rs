//! Only migrated packages can be selected. Test eligibility also needs dev dependencies.
use std::collections::BTreeSet;

use super::{SCOPE_EXEMPTIONS, next_package, parse_cursor, usage};
use crate::error::BuildResult;
use crate::workspace::{CargoWorkspace, WorkspacePackage};

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Selection {
    pub(super) next: String,
    pub(super) libraries: BTreeSet<String>,
    pub(super) tests: BTreeSet<String>,
    pub(super) deferred: BTreeSet<String>,
}

pub(super) fn select(
    workspace: &CargoWorkspace,
    order: &[String],
    prior: usize,
    cursor: &str,
    changed_packages: &BTreeSet<String>,
) -> BuildResult<Selection> {
    verify_order(workspace, order)?;
    let next = next_package(order, prior)?;
    let current = advance_cursor(cursor, prior, next)?;
    let before = available(order, prior);
    let after = available(order, current);
    let migrated = &order[..current];
    let selected = changed_packages
        .iter()
        .filter(|name| migrated.contains(name))
        .cloned()
        .chain([next.to_owned()])
        .collect::<BTreeSet<_>>();
    let tests = eligible_tests(workspace, &before, &after, &selected, migrated);
    Ok(Selection {
        next: next.to_owned(),
        libraries: selected.difference(&tests).cloned().collect(),
        tests,
        deferred: deferred_tests(workspace, &after, migrated),
    })
}

fn advance_cursor(cursor: &str, prior: usize, next: &str) -> BuildResult<usize> {
    let current = parse_cursor(cursor)?;
    if current != prior + 1 {
        return Err(usage(format!(
            "migration cursor {current}; expected {} ({next}) after {prior}",
            prior + 1
        )));
    }
    Ok(current)
}

fn verify_order(workspace: &CargoWorkspace, order: &[String]) -> BuildResult<()> {
    let scheduled = available(order, order.len());
    let members = workspace.names().into_iter().collect::<BTreeSet<_>>();
    if scheduled != members || scheduled.len() != order.len() + SCOPE_EXEMPTIONS.len() {
        return Err(usage(format!(
            "migration list differs from workspace membership: unlisted {:?}, unknown {:?}; expected one step per toolkit",
            members.difference(&scheduled).collect::<Vec<_>>(),
            scheduled.difference(&members).collect::<Vec<_>>()
        )));
    }
    for (step, name) in order.iter().enumerate() {
        let earlier = available(order, step);
        let package = workspace
            .packages
            .iter()
            .find(|package| package.name == *name);
        let Some(package) = package else {
            return Err(usage(format!(
                "missing migration crate {name:?}; expected a workspace member"
            )));
        };
        verify_library_order(package, &earlier)?;
    }
    Ok(())
}

fn verify_library_order(package: &WorkspacePackage, earlier: &BTreeSet<String>) -> BuildResult<()> {
    let missing = package
        .library_path_dependencies
        .iter()
        .filter(|name| !earlier.contains(*name))
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        return Err(usage(format!(
            "{} has library dependencies {missing:?} outside its completed migration prefix",
            package.name
        )));
    }
    Ok(())
}

fn available(order: &[String], count: usize) -> BTreeSet<String> {
    order[..count]
        .iter()
        .cloned()
        .chain(SCOPE_EXEMPTIONS.into_iter().map(str::to_owned))
        .collect()
}

fn test_eligible(package: &WorkspacePackage, available: &BTreeSet<String>) -> bool {
    package
        .path_dependencies
        .iter()
        .all(|name| available.contains(name))
}

fn eligible_tests(
    workspace: &CargoWorkspace,
    before: &BTreeSet<String>,
    after: &BTreeSet<String>,
    selected: &BTreeSet<String>,
    migrated: &[String],
) -> BTreeSet<String> {
    migrated_packages(workspace, migrated)
        .filter(|package| test_eligible(package, after))
        .filter(|package| {
            selected.contains(&package.name)
                || (before.contains(&package.name) && !test_eligible(package, before))
        })
        .map(|package| package.name.clone())
        .collect()
}

fn deferred_tests(
    workspace: &CargoWorkspace,
    after: &BTreeSet<String>,
    migrated: &[String],
) -> BTreeSet<String> {
    migrated_packages(workspace, migrated)
        .filter(|package| !test_eligible(package, after))
        .map(|package| package.name.clone())
        .collect()
}

fn migrated_packages<'a>(
    workspace: &'a CargoWorkspace,
    migrated: &'a [String],
) -> impl Iterator<Item = &'a WorkspacePackage> {
    workspace
        .packages
        .iter()
        .filter(|package| migrated.contains(&package.name))
}
