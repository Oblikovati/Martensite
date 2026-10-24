//! Inspect normalized dependency requirements from the actual archive bytes.
use super::Mismatch;
use crate::{
    error::{BuildError, BuildResult, io_error},
    package_manifest::parse_table,
};
use semver::{Version, VersionReq};
use std::{collections::BTreeMap, fs, io::Read, path::Path};
use toml::{Table, Value};

pub(super) fn read(path: &Path, name: &str, version: &Version) -> BuildResult<String> {
    let file = fs::File::open(path).map_err(|cause| io_error(path, cause))?;
    let mut tar = tar::Archive::new(flate2::read::GzDecoder::new(file));
    let expected = format!("{name}-{version}/Cargo.toml");
    let mut manifest = None;
    for item in tar.entries().map_err(|cause| io_error(path, cause))? {
        let mut item = item.map_err(|cause| io_error(path, cause))?;
        if item.path().map_err(|cause| io_error(path, cause))?.as_ref() != Path::new(&expected) {
            continue;
        }
        if manifest.is_some() || !item.header().entry_type().is_file() {
            return Err(invalid(&expected, "one regular manifest"));
        }
        let mut text = String::new();
        item.read_to_string(&mut text)
            .map_err(|cause| io_error(path, cause))?;
        manifest = Some(text);
    }
    let text = manifest.ok_or_else(|| invalid(&expected, "a normalized manifest"))?;
    identity(&parse_table(&text)?, name, version)?;
    Ok(text)
}

fn identity(table: &Table, name: &str, version: &Version) -> BuildResult<()> {
    let package = table.get("package");
    if package.and_then(|v| v.get("name")).and_then(Value::as_str) != Some(name)
        || package
            .and_then(|v| v.get("version"))
            .and_then(Value::as_str)
            != Some(version.to_string().as_str())
    {
        return Err(invalid(
            &format!("{name}-{version}"),
            "matching archive package identity",
        ));
    }
    Ok(())
}

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
