//! Restrict QA proofs to production layouts covered by the scanner (#481, #482).
use crate::package_manifest::PackageManifest;
use std::path::{Component, Path};
pub(crate) fn known_production_layout(root: &Path, manifest: &PackageManifest) -> bool {
    !root.is_symlink()
        && crate::release_build_inputs::known_build_inputs(root, manifest)
        && !custom_target(manifest)
}
fn custom_target(manifest: &PackageManifest) -> bool {
    let library = manifest.source.get("lib").into_iter();
    let binaries = manifest
        .source
        .get("bin")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten();
    library.chain(binaries).any(|target| {
        target
            .get("path")
            .and_then(toml::Value::as_str)
            .is_some_and(|path| !scannable_target_path(path))
    })
}
pub(crate) fn scannable_target_path(path: &str) -> bool {
    let target = Path::new(path);
    path.starts_with("src/")
        && !path.contains('\\')
        && !path.contains(':')
        && target
            .extension()
            .is_some_and(|extension| extension == "rs")
        && target
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
}
