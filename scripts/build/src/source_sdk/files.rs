//! Regular-file enumeration prevents local symlink traversal into release inputs.
use crate::{
    archive::BundleEntry,
    error::{BuildError, BuildResult, io_error},
};
use std::{
    fs,
    path::{Component, Path},
};

pub(super) fn relative(path: &Path) -> BuildResult<()> {
    if path.as_os_str().is_empty()
        || path
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
    {
        return Err(BuildError::ReleaseRule(format!(
            "SDK path {path:?}; expected a nonempty relative regular-file path"
        )));
    }
    if path.to_string_lossy().contains(['\\', ':']) {
        return Err(BuildError::ReleaseRule(format!(
            "SDK path {path:?}; expected portable path components"
        )));
    }
    Ok(())
}

/// Serialize native components before applying the archive policy (#290); `native_archive_path(Path::new("vendor").join("item.rs").as_path())`.
pub(super) fn native_archive_path(path: &Path) -> BuildResult<String> {
    let components = path
        .components()
        .map(native_component)
        .collect::<BuildResult<Vec<_>>>()?;
    let portable = components.join("/");
    relative(Path::new(&portable))?;
    Ok(portable)
}

fn native_component(component: Component<'_>) -> BuildResult<&str> {
    if let Component::Normal(name) = component
        && let Some(name) = name.to_str()
    {
        return Ok(name);
    }
    Err(BuildError::ReleaseRule(format!(
        "SDK component {component:?}; expected a Unicode normal path component"
    )))
}

pub(super) fn write(root: &Path, name: &Path, bytes: &[u8]) -> BuildResult<()> {
    relative(name)?;
    let path = root.join(name);
    let parent = path
        .parent()
        .ok_or_else(|| BuildError::ReleaseRule(format!("SDK path {path:?}; expected parent")))?;
    fs::create_dir_all(parent).map_err(|cause| io_error(parent, cause))?;
    fs::write(&path, bytes).map_err(|cause| io_error(&path, cause))
}

pub(super) fn entries(root: &Path) -> BuildResult<Vec<BundleEntry>> {
    let mut entries = Vec::new();
    collect(root, root, &mut entries)?;
    entries.sort_by(|a, b| a.archive_path.cmp(&b.archive_path));
    Ok(entries)
}

fn collect(root: &Path, directory: &Path, entries: &mut Vec<BundleEntry>) -> BuildResult<()> {
    for item in fs::read_dir(directory).map_err(|cause| io_error(directory, cause))? {
        let item = item.map_err(|cause| io_error(directory, cause))?;
        let kind = item
            .file_type()
            .map_err(|cause| io_error(&item.path(), cause))?;
        if kind.is_dir() {
            collect(root, &item.path(), entries)?;
            continue;
        }
        if !kind.is_file() {
            return Err(BuildError::ReleaseRule(format!(
                "SDK input {:?}; expected a regular file or directory",
                item.path()
            )));
        }
        entries.push(entry(root, &item.path())?);
    }
    Ok(())
}

fn entry(root: &Path, path: &Path) -> BuildResult<BundleEntry> {
    let relative = path.strip_prefix(root).map_err(|cause| {
        BuildError::ReleaseRule(format!(
            "SDK input {path:?}: {cause}; expected under {root:?}"
        ))
    })?;
    let archive_path = native_archive_path(relative)?;
    Ok(BundleEntry {
        source: path.into(),
        archive_path,
        executable: executable(path)?,
    })
}

#[cfg(unix)]
fn executable(path: &Path) -> BuildResult<bool> {
    use std::os::unix::fs::PermissionsExt;
    Ok(fs::metadata(path)
        .map_err(|cause| io_error(path, cause))?
        .permissions()
        .mode()
        & 0o111
        != 0)
}

#[cfg(not(unix))]
fn executable(_path: &Path) -> BuildResult<bool> {
    Ok(false)
}

pub(super) fn set_mode(path: &Path, executable: bool) -> BuildResult<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(
            path,
            fs::Permissions::from_mode(if executable { 0o755 } else { 0o644 }),
        )
        .map_err(|cause| io_error(path, cause))?;
    }
    let _ = (path, executable);
    Ok(())
}
