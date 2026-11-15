//! Restrict QA proofs to production layouts covered by the scanner (#481, #482).
use crate::package_manifest::PackageManifest;
use std::path::{Component, Path};
use toml::Value;

pub(crate) fn known_production_layout(root: &Path, manifest: &PackageManifest) -> bool {
    !root.is_symlink()
        && crate::release_build_inputs::known_build_inputs(root, manifest)
        && !custom_target(root, manifest)
}
fn target_list<'a>(manifest: &'a PackageManifest, kind: &str) -> impl Iterator<Item = &'a Value> {
    manifest
        .source
        .get(kind)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
}
fn custom_target(root: &Path, manifest: &PackageManifest) -> bool {
    let library = manifest.source.get("lib").into_iter();
    library
        .chain(target_list(manifest, "bin"))
        .any(|target| !known_target(root, target, &["src"]))
        || target_list(manifest, "example")
            .any(|target| !known_target(root, target, &["src", "examples"]))
}
fn known_target(root: &Path, target: &Value, directories: &[&str]) -> bool {
    let Some(fields) = target.as_table() else {
        return false;
    };
    let Some(value) = fields.get("path") else {
        return true;
    };
    let Some(path) = value.as_str() else {
        return false;
    };
    directories
        .iter()
        .any(|directory| scannable_under(path, directory))
        && root
            .join(path)
            .canonicalize()
            .ok()
            .is_some_and(|path| covered_rust_input(root, &path))
}
pub(crate) fn scannable_under(path: &str, directory: &str) -> bool {
    let target = Path::new(path);
    portable_relative_path(path)
        && target.starts_with(directory)
        && target
            .extension()
            .is_some_and(|extension| extension == "rs")
        && target
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
}
pub(crate) fn portable_relative_path(value: &str) -> bool {
    value.is_ascii()
        && !Path::new(value).is_absolute()
        && value
            .chars()
            .all(|ch| !ch.is_control() && !matches!(ch, '\\' | ':' | '~'))
        && value.split('/').all(portable_component)
}
fn portable_component(component: &str) -> bool {
    if matches!(component, "." | "..") {
        return true;
    }
    !component.is_empty() && component.trim_end_matches([' ', '.']) == component
}
pub(crate) fn covered_rust_input(root: &Path, input: &Path) -> bool {
    let Ok(package) = root.canonicalize() else {
        return false;
    };
    if !input.is_file() || !input.starts_with(&package) {
        return false;
    }
    ["src", "examples"]
        .iter()
        .filter_map(|directory| package.join(directory).canonicalize().ok())
        .any(|directory| input.starts_with(directory))
}
/// Checks native relative components; `input_components_supported(Path::new("src/lib.rs"))`.
pub(crate) fn input_components_supported(path: &Path) -> bool {
    path.components().all(|component| {
        let Component::Normal(name) = component else {
            return false;
        };
        let Some(name) = name.to_str() else {
            return false;
        };
        portable_relative_path(name)
            && !name.eq_ignore_ascii_case("target")
            && !name.eq_ignore_ascii_case(".git")
    })
}
