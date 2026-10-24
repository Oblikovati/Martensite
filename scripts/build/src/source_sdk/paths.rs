//! Local Cargo paths must remain in the staged committed source tree.
use super::files;
use crate::{
    error::{BuildError, BuildResult, io_error},
    package_manifest::parse_table,
};
use std::{fs, path::Path};

pub(super) fn validate(root: &Path) -> BuildResult<()> {
    let root = fs::canonicalize(root).map_err(|cause| io_error(root, cause))?;
    for entry in files::entries(&root)?
        .iter()
        .filter(|entry| entry.archive_path.ends_with("Cargo.toml"))
    {
        let text =
            fs::read_to_string(&entry.source).map_err(|cause| io_error(&entry.source, cause))?;
        let value = toml::Value::Table(parse_table(&text)?);
        let parent = entry.source.parent().ok_or_else(|| {
            BuildError::ReleaseRule(format!(
                "SDK manifest {:?}; expected parent directory",
                entry.source
            ))
        })?;
        inspect(&root, parent, &value)?;
    }
    Ok(())
}

fn inspect(root: &Path, parent: &Path, value: &toml::Value) -> BuildResult<()> {
    match value {
        toml::Value::Table(table) => inspect_table(root, parent, table)?,
        toml::Value::Array(values) => {
            for value in values {
                inspect(root, parent, value)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn inspect_table(root: &Path, parent: &Path, table: &toml::Table) -> BuildResult<()> {
    for (key, value) in table {
        if key == "path" {
            check_path(root, parent, value)?;
        }
        inspect(root, parent, value)?;
    }
    Ok(())
}

fn check_path(root: &Path, parent: &Path, value: &toml::Value) -> BuildResult<()> {
    let text = value.as_str().ok_or_else(|| {
        BuildError::ReleaseRule(format!("SDK local path {value:?}; expected string"))
    })?;
    let path = parent.join(text);
    let resolved = fs::canonicalize(&path).map_err(|cause| io_error(&path, cause))?;
    if !resolved.starts_with(root) {
        return Err(BuildError::ReleaseRule(format!(
            "SDK local path {text:?} resolves to {resolved:?}; expected inside {root:?}"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escaping_local_dependencies_fail_before_vendoring() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path().join("sdk");
        fs::create_dir(&root).unwrap();
        files::write(
            &root,
            Path::new("Cargo.toml"),
            b"[dependencies]\noutside={path='..'}",
        )
        .unwrap();
        assert!(
            validate(&root)
                .unwrap_err()
                .to_string()
                .contains("expected inside")
        );
    }
}
