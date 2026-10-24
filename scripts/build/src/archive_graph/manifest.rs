//! Inspect normalized dependency requirements from the actual archive bytes.
use super::Mismatch;
use crate::{
    error::{BuildError, BuildResult},
    package_manifest::parse_table,
};
use semver::{Version, VersionReq};
use std::collections::BTreeMap;
use toml::{Table, Value};

pub(super) fn mismatches(
    package: &str,
    text: &str,
    selected: &BTreeMap<String, Version>,
) -> BuildResult<Vec<Mismatch>> {
    let mut results = Vec::new();
    inspect_tables(package, &parse_table(text)?, selected, &mut results)?;
    Ok(results)
}

fn inspect_tables(
    package: &str,
    table: &Table,
    selected: &BTreeMap<String, Version>,
    results: &mut Vec<Mismatch>,
) -> BuildResult<()> {
    for (key, value) in table {
        let Some(nested) = value.as_table() else {
            continue;
        };
        if matches!(
            key.as_str(),
            "dependencies" | "dev-dependencies" | "build-dependencies"
        ) {
            inspect_dependencies(package, nested, selected, results)?;
            continue;
        }
        inspect_tables(package, nested, selected, results)?;
    }
    Ok(())
}

fn inspect_dependencies(
    package: &str,
    table: &Table,
    selected: &BTreeMap<String, Version>,
    results: &mut Vec<Mismatch>,
) -> BuildResult<()> {
    for (alias, dependency) in table {
        inspect_dependency(package, alias, dependency, selected, results)?;
    }
    Ok(())
}

fn inspect_dependency(
    package: &str,
    alias: &str,
    value: &Value,
    selected: &BTreeMap<String, Version>,
    results: &mut Vec<Mismatch>,
) -> BuildResult<()> {
    let name = value
        .get("package")
        .and_then(Value::as_str)
        .unwrap_or(alias);
    if !name.starts_with("martensite-") {
        return Ok(());
    }
    let version = selected
        .get(name)
        .ok_or_else(|| invalid(name, "an internal catalog package"))?;
    let text = requirement_text(name, value)?;
    let requirement = parse_requirement(name, text)?;
    if !requirement.matches(version) {
        results.push(mismatch(package, name, text, version));
    }
    Ok(())
}

fn parse_requirement(name: &str, text: &str) -> BuildResult<VersionReq> {
    VersionReq::parse(text).map_err(|cause| {
        invalid(
            &format!("{name}={text}: {cause}"),
            "a semantic version requirement",
        )
    })
}

fn mismatch(package: &str, name: &str, text: &str, version: &Version) -> Mismatch {
    Mismatch {
        package: package.into(),
        dependency: name.into(),
        requirement: text.into(),
        selected_version: version.to_string(),
    }
}

fn requirement_text<'a>(name: &str, value: &'a Value) -> BuildResult<&'a str> {
    value
        .as_str()
        .or_else(|| value.get("version").and_then(Value::as_str))
        .ok_or_else(|| invalid(name, "a version requirement"))
}

fn invalid(value: &str, expected: &str) -> BuildError {
    BuildError::ReleaseRule(format!("archive value {value:?}; expected {expected}"))
}
