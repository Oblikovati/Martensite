//! Read committed blob bytes instead of checkout conversions or untracked local files.
use super::{commands, files};
use crate::{
    context::BuildContext,
    error::{BuildError, BuildResult},
    process::CommandSpec,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct Snapshot {
    pub revision: String,
    pub gitlinks: BTreeMap<String, String>,
    pub executable_paths: Vec<String>,
    pub committed_sha256: BTreeMap<String, String>,
}

pub(super) fn stage(context: &BuildContext<'_>, stage: &Path) -> BuildResult<Snapshot> {
    let revision = clean_revision(context)?;
    let command = CommandSpec::new("git", context.root()).args(["ls-tree", "-r", "-z", &revision]);
    let tree = commands::output(context.runner, &command)?;
    let mut snapshot = Snapshot {
        revision,
        gitlinks: BTreeMap::new(),
        executable_paths: Vec::new(),
        committed_sha256: BTreeMap::new(),
    };
    for record in tree
        .split(|byte| *byte == 0)
        .filter(|record| !record.is_empty())
    {
        stage_record(context, stage, record, &mut snapshot)?;
    }
    Ok(snapshot)
}

fn clean_revision(context: &BuildContext<'_>) -> BuildResult<String> {
    require_clean(context)?;
    let revision = context
        .runner
        .capture(&CommandSpec::new("git", context.root()).args(["rev-parse", "HEAD"]))?
        .trim()
        .to_owned();
    if revision.len() != 40 || !revision.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(BuildError::ReleaseRule(format!(
            "SDK revision {revision:?}; expected full commit SHA"
        )));
    }
    Ok(revision)
}

fn require_clean(context: &BuildContext<'_>) -> BuildResult<()> {
    let status = CommandSpec::new("git", context.root()).args([
        "status",
        "--porcelain",
        "--untracked-files=normal",
    ]);
    let dirty = commands::output(context.runner, &status)?;
    if !dirty.is_empty() {
        return Err(BuildError::ReleaseRule(format!(
            "SDK dirty checkout {:?}; expected committed clean source",
            String::from_utf8_lossy(&dirty)
        )));
    }
    Ok(())
}

fn stage_record(
    context: &BuildContext<'_>,
    stage: &Path,
    record: &[u8],
    snapshot: &mut Snapshot,
) -> BuildResult<()> {
    let (mode, object, name) = parse_record(record)?;
    files::relative(Path::new(&name))?;
    reserved_path(&name)?;
    if mode == "160000" {
        return gitlink(&name, &object, snapshot);
    }
    if mode != "100644" && mode != "100755" {
        return Err(BuildError::ReleaseRule(format!(
            "SDK input {name:?} mode {mode}; expected regular committed file"
        )));
    }
    let hash = store_blob(context, stage, &name, &object, mode == "100755")?;
    snapshot.committed_sha256.insert(name.clone(), hash);
    if mode == "100755" {
        snapshot.executable_paths.push(name);
    }
    Ok(())
}

fn store_blob(
    context: &BuildContext<'_>,
    stage: &Path,
    name: &str,
    object: &str,
    executable: bool,
) -> BuildResult<String> {
    let command = CommandSpec::new("git", context.root()).args(["cat-file", "blob", object]);
    let bytes = commands::output(context.runner, &command)?;
    files::write(stage, Path::new(name), &bytes)?;
    files::set_mode(&stage.join(name), executable)?;
    crate::archive::sha256_hex(&stage.join(name))
}

fn parse_record(record: &[u8]) -> BuildResult<(String, String, String)> {
    let text = std::str::from_utf8(record).map_err(|cause| {
        BuildError::ReleaseRule(format!(
            "SDK tree record {record:?}: {cause}; expected UTF-8"
        ))
    })?;
    let (header, name) = text.split_once('\t').ok_or_else(|| {
        BuildError::ReleaseRule(format!(
            "SDK tree {text:?}; expected mode type object<TAB>path"
        ))
    })?;
    let fields: Vec<_> = header.split_whitespace().collect();
    if fields.len() != 3 {
        return Err(BuildError::ReleaseRule(format!(
            "SDK tree {text:?}; expected three header fields"
        )));
    }
    Ok((fields[0].into(), fields[2].into(), name.into()))
}

fn gitlink(name: &str, object: &str, snapshot: &mut Snapshot) -> BuildResult<()> {
    if name != "external/martensite-test-data" {
        return Err(BuildError::ReleaseRule(format!(
            "SDK gitlink {name:?}; expected only external/martensite-test-data"
        )));
    }
    snapshot.gitlinks.insert(name.into(), object.into());
    Ok(())
}

fn reserved_path(name: &str) -> BuildResult<()> {
    if name == "martensite-source-workspace.json" || name.starts_with("vendor/") {
        return Err(BuildError::ReleaseRule(format!(
            "SDK committed path {name:?}; expected generated manifest/vendor paths to be unused"
        )));
    }
    Ok(())
}
