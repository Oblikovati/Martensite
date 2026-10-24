//! Owned manifest comparison separates package source from release bookkeeping.

use semver::Version;
use toml::{Table, Value};

use crate::error::{BuildError, BuildResult};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PackageManifest {
    pub name: String,
    pub version: Version,
    pub source: Table,
}

impl PackageManifest {
    /// Separates a package's version from source-affecting manifest fields.
    /// Example: `PackageManifest::parse(text, &workspace_package_fields)?`.
    pub fn parse(text: &str, workspace: &Table) -> BuildResult<Option<Self>> {
        let mut source = parse_table(text)?;
        let Some(package) = source.get_mut("package").and_then(Value::as_table_mut) else {
            return Ok(None);
        };
        let name = string_field(package, "name")?.to_owned();
        let version = package_version(package, workspace)?;
        package.remove("version");
        resolve_inherited(package, workspace)?;
        normalize_dependencies(&mut source);
        Ok(Some(Self {
            name,
            version,
            source,
        }))
    }
}

/// Parses manifest fields while retaining the input in parse diagnostics.
/// Example: `let fields = parse_table("[workspace]\nmembers=[]")?;`.
pub(crate) fn parse_table(text: &str) -> BuildResult<Table> {
    text.parse().map_err(|error| {
        BuildError::ReleaseRule(format!(
            "manifest {text:?} could not be parsed: {error}; expected a Cargo TOML manifest"
        ))
    })
}

fn string_field<'a>(table: &'a Table, field: &str) -> BuildResult<&'a str> {
    table.get(field).and_then(Value::as_str).ok_or_else(|| {
        BuildError::ReleaseRule(format!(
            "manifest field {field:?} in {table:?}; expected a string"
        ))
    })
}

fn package_version(package: &Table, workspace: &Table) -> BuildResult<Version> {
    let text = if inherited(package.get("version")) {
        string_field(workspace, "version")?
    } else {
        string_field(package, "version")?
    };
    Version::parse(text).map_err(|error| {
        BuildError::ReleaseRule(format!(
            "package version {text:?}: {error}; expected semantic versioning"
        ))
    })
}

fn inherited(value: Option<&Value>) -> bool {
    value
        .and_then(Value::as_table)
        .and_then(|table| table.get("workspace"))
        .and_then(Value::as_bool)
        == Some(true)
}

fn resolve_inherited(package: &mut Table, workspace: &Table) -> BuildResult<()> {
    for (key, value) in package.iter_mut() {
        if !inherited(Some(value)) {
            continue;
        }
        *value = workspace.get(key).cloned().ok_or_else(|| {
            BuildError::ReleaseRule(format!(
                "inherited package field {key:?}; expected workspace.package.{key}"
            ))
        })?;
    }
    Ok(())
}

fn normalize_dependencies(table: &mut Table) {
    for (key, value) in table.iter_mut() {
        let Some(nested) = value.as_table_mut() else {
            continue;
        };
        if matches!(
            key.as_str(),
            "dependencies" | "dev-dependencies" | "build-dependencies"
        ) {
            for dependency in nested
                .iter_mut()
                .filter_map(|(_, value)| value.as_table_mut())
            {
                if dependency.contains_key("path") {
                    dependency.remove("version");
                }
            }
        } else {
            normalize_dependencies(nested);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(text: &str) -> PackageManifest {
        PackageManifest::parse(text, &Table::new())
            .unwrap()
            .unwrap()
    }

    #[test]
    fn version_and_local_requirement_changes_are_bookkeeping_only() {
        let before = manifest(
            "[package]\nname='a'\nversion='1.0.0'\n[dependencies]\nb={path='../b',version='1'}\n",
        );
        let after = manifest(
            "[package]\nname='a'\nversion='1.0.1'\n[dependencies]\nb={path='../b',version='1.0.1'}\n",
        );
        assert_eq!(before.source, after.source);
        assert_ne!(before.version, after.version);
    }

    #[test]
    fn registry_requirements_and_features_are_source_changes() {
        let before = manifest("[package]\nname='a'\nversion='1.0.0'\n[dependencies]\nb='1'\n");
        let after = manifest("[package]\nname='a'\nversion='1.0.0'\n[dependencies]\nb='2'\n");
        assert_ne!(before.source, after.source);
    }

    #[test]
    fn resolves_workspace_version_and_package_fields() {
        let workspace = parse_table("version='1.0.0'\nedition='2024'").unwrap();
        let package = PackageManifest::parse(
            "[package]\nname='a'\nversion.workspace=true\nedition.workspace=true",
            &workspace,
        )
        .unwrap()
        .unwrap();
        assert_eq!(package.version, Version::new(1, 0, 0));
        assert_eq!(package.source["package"]["edition"].as_str(), Some("2024"));
    }

    #[test]
    fn invalid_and_missing_fields_fail_closed() {
        assert!(parse_table("[broken").is_err());
        assert!(PackageManifest::parse("[package]\nname='a'", &Table::new()).is_err());
        assert!(
            PackageManifest::parse("[workspace]", &Table::new())
                .unwrap()
                .is_none()
        );
    }
}
