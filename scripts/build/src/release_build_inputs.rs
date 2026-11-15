//! Bind the QA-only proof to the reviewed budget-marker script, never its filename (#481).
use crate::package_manifest::PackageManifest;
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

const BUDGET_MARKER_SHA256: &str =
    "dddebb2e5a4e8cfbf22875ca33cd1890f22b750466b38063feab8d798d2bfb29";

pub(crate) fn known_build_inputs(root: &Path, manifest: &PackageManifest) -> bool {
    let build = manifest.source.get("package").and_then(|p| p.get("build"));
    if build
        .and_then(toml::Value::as_str)
        .is_some_and(|path| path != "build.rs")
    {
        return false;
    }
    let path = root.join("build.rs");
    if !path.exists() {
        return build.and_then(toml::Value::as_str).is_none();
    }
    approved_budget_marker(&path)
}

fn approved_budget_marker(path: &Path) -> bool {
    let Ok(source) = fs::read_to_string(path) else {
        return false;
    };
    // The reviewed script reads only OPT_LEVEL/CARGO_CFG_TARGET_OS and emits cfg/rerun lines.
    // A script edit, including a new input, must fail closed until independently reviewed.
    let normalized = source.replace("\r\n", "\n");
    format!("{:x}", Sha256::digest(normalized.as_bytes())) == BUDGET_MARKER_SHA256
}
