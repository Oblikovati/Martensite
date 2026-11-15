//! Compare only proven QA target changes without altering manifest storage or test selection (#481).
use crate::{git::ChangedFile, package_manifest::PackageManifest};
use std::{borrow::Cow, path::Path};
use toml::Value;

pub(crate) fn publication_manifest<'a>(
    root: &Path,
    current: &'a PackageManifest,
    previous: Option<&PackageManifest>,
    directory: &str,
    changed: &[ChangedFile],
) -> Cow<'a, PackageManifest> {
    let Some(previous) = previous else {
        return Cow::Borrowed(current);
    };
    let proven = eligible(root, current, previous, directory, changed);
    if !proven {
        return Cow::Borrowed(current);
    }
    let mut comparison = current.clone();
    comparison.source = previous.source.clone();
    Cow::Owned(comparison)
}

fn eligible(
    root: &Path,
    current: &PackageManifest,
    previous: &PackageManifest,
    directory: &str,
    changed: &[ChangedFile],
) -> bool {
    only_test_targets_changed(current, previous)
        && crate::release_test_inputs::manifest_is_unreferenced(
            &root.join(directory),
            current,
            changed,
        )
}

fn only_test_targets_changed(current: &PackageManifest, previous: &PackageManifest) -> bool {
    if current.source.get("test") == previous.source.get("test") {
        return false;
    }
    if !simple_test_targets(current.source.get("test"))
        || !simple_test_targets(previous.source.get("test"))
    {
        return false;
    }
    let mut current = current.source.clone();
    let mut previous = previous.source.clone();
    current.remove("test");
    previous.remove("test");
    current == previous
}

fn simple_test_targets(value: Option<&Value>) -> bool {
    let Some(value) = value else {
        return true;
    };
    let Some(targets) = value.as_array() else {
        return false;
    };
    targets.iter().all(simple_test_target)
}

fn simple_test_target(value: &Value) -> bool {
    let Some(fields) = value.as_table() else {
        return false;
    };
    fields.iter().all(|(key, value)| match key.as_str() {
        "name" | "path" => value.is_str(),
        "harness" => value.is_bool(),
        _ => false,
    })
}

#[cfg(test)]
#[path = "release_qa_manifest_tests.rs"]
mod tests;
