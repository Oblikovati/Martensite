//! CI gate enforcing that every public type is classified in `docs/architecture/thread-safety-manifest.tsv` (#61).

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::error::{BuildError, BuildResult, io_error};
use crate::git::ChangedFile;

const MANIFEST_REL_PATH: &str = "docs/architecture/thread-safety-manifest.tsv";

#[derive(Debug, Clone, PartialEq, Eq)]
struct ManifestEntry {
    classification: String,
    reason: String,
}

/// Validates changed crate source files against the thread-safety manifest (#61).
pub fn check_thread_safety_manifest(root: &Path, changed: &[ChangedFile]) -> BuildResult<()> {
    let manifest_path = root.join(MANIFEST_REL_PATH);
    if !manifest_path.is_file() {
        return Ok(());
    }
    let relevant: Vec<&ChangedFile> = changed
        .iter()
        .filter(|file| !file.deleted && is_crate_src_rust(&file.path))
        .collect();
    let manifest_changed = changed.iter().any(|file| file.path == MANIFEST_REL_PATH);
    if relevant.is_empty() && !manifest_changed {
        return Ok(());
    }
    let entries = load_manifest(&manifest_path)?;
    let mut missing = Vec::new();
    for file in relevant {
        let Some(crate_name) = crate_name_for_path(&file.path) else {
            continue;
        };
        let full = root.join(&file.path);
        if !full.is_file() {
            continue;
        }
        let source = fs::read_to_string(&full).map_err(|err| io_error(&full, err))?;
        for type_name in extract_public_types(&source) {
            if !entries.contains_key(&(crate_name.to_owned(), type_name.clone())) {
                missing.push(format!("{crate_name}::{type_name} ({})", file.path));
            }
        }
    }
    if missing.is_empty() {
        return Ok(());
    }
    Err(BuildError::UnexpectedOutput {
        origin: MANIFEST_REL_PATH.to_owned(),
        detail: format!(
            "public type(s) missing from {MANIFEST_REL_PATH}: {}",
            missing.join(", ")
        ),
    })
}

fn load_manifest(path: &Path) -> BuildResult<BTreeMap<(String, String), ManifestEntry>> {
    let text = fs::read_to_string(path).map_err(|err| io_error(path, err))?;
    let mut entries = BTreeMap::new();
    for (idx, line) in text.lines().enumerate().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() != 4 {
            return Err(BuildError::UnexpectedOutput {
                origin: MANIFEST_REL_PATH.to_owned(),
                detail: format!(
                    "{MANIFEST_REL_PATH}:{} has {} columns; expected 4",
                    idx + 1,
                    cols.len()
                ),
            });
        }
        validate_classification(idx + 1, cols[2], cols[3])?;
        entries.insert(
            (cols[0].to_owned(), cols[1].to_owned()),
            ManifestEntry {
                classification: cols[2].to_owned(),
                reason: cols[3].to_owned(),
            },
        );
    }
    Ok(entries)
}

fn validate_classification(line: usize, classification: &str, reason: &str) -> BuildResult<()> {
    match classification {
        "shareable" | "movable" => Ok(()),
        "thread-confined" if !reason.trim().is_empty() => Ok(()),
        "thread-confined" => Err(BuildError::UnexpectedOutput {
            origin: MANIFEST_REL_PATH.to_owned(),
            detail: format!(
                "{MANIFEST_REL_PATH}:{line}: thread-confined entry requires a non-empty reason"
            ),
        }),
        other => Err(BuildError::UnexpectedOutput {
            origin: MANIFEST_REL_PATH.to_owned(),
            detail: format!(
                "{MANIFEST_REL_PATH}:{line}: unknown classification {other:?}; expected shareable, movable, or thread-confined"
            ),
        }),
    }
}

fn is_crate_src_rust(path: &str) -> bool {
    if !path.starts_with("crates/") || !path.ends_with(".rs") || !path.contains("/src/") {
        return false;
    }
    !path.contains("/tests/") && !path.ends_with("_tests.rs") && !path.ends_with("/tests.rs")
}

fn crate_name_for_path(path: &str) -> Option<&str> {
    let rest = path.strip_prefix("crates/")?;
    rest.split('/').next()
}

fn extract_public_types(source: &str) -> Vec<String> {
    let mut types = Vec::new();
    let mut in_cfg_test = false;
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("#[cfg(test)]") {
            in_cfg_test = true;
            continue;
        }
        if in_cfg_test {
            continue;
        }
        if let Some(name) = parse_pub_type_decl(trimmed) {
            types.push(name.to_owned());
        }
    }
    types
}

fn parse_pub_type_decl(trimmed: &str) -> Option<&str> {
    let rest = trimmed.strip_prefix("pub ")?;
    let after_kw = rest
        .strip_prefix("struct ")
        .or_else(|| rest.strip_prefix("enum "))
        .or_else(|| rest.strip_prefix("union "))?;
    let ident: &str = after_kw
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .next()?;
    if ident.is_empty() || !ident.chars().next()?.is_ascii_alphabetic() {
        return None;
    }
    Some(ident)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_fixture(root: &Path, manifest: &str, source: &str) {
        fs::create_dir_all(root.join("docs/architecture")).unwrap();
        fs::create_dir_all(root.join("crates/martensite-demo/src")).unwrap();
        fs::write(root.join(MANIFEST_REL_PATH), manifest).unwrap();
        fs::write(root.join("crates/martensite-demo/src/lib.rs"), source).unwrap();
    }

    #[test]
    fn unlisted_public_type_fails_the_thread_safety_gate() {
        let dir = tempfile::tempdir().unwrap();
        write_fixture(
            dir.path(),
            "crate\ttype_name\tclassification\treason\nmartensite-demo\tListed\tshareable\timmutable\n",
            "pub struct Listed;\npub enum Unlisted { A }\n",
        );
        let changed = vec![ChangedFile {
            path: "crates/martensite-demo/src/lib.rs".into(),
            deleted: false,
            untracked: false,
        }];
        let err = check_thread_safety_manifest(dir.path(), &changed).unwrap_err();
        assert!(err.to_string().contains("martensite-demo::Unlisted"));
    }

    #[test]
    fn listed_public_types_pass_the_thread_safety_gate() {
        let dir = tempfile::tempdir().unwrap();
        write_fixture(
            dir.path(),
            "crate\ttype_name\tclassification\treason\nmartensite-demo\tListed\tshareable\timmutable\n",
            "pub struct Listed;\n#[cfg(test)]\nmod tests { pub struct Ignored; }\n",
        );
        let changed = vec![ChangedFile {
            path: "crates/martensite-demo/src/lib.rs".into(),
            deleted: false,
            untracked: false,
        }];
        check_thread_safety_manifest(dir.path(), &changed).unwrap();
    }

    #[test]
    fn thread_confined_entry_without_reason_is_rejected() {
        let dir = tempfile::tempdir().unwrap();
        write_fixture(
            dir.path(),
            "crate\ttype_name\tclassification\treason\nmartensite-demo\tLocal\tthread-confined\t\n",
            "pub struct Local;\n",
        );
        let changed = vec![ChangedFile {
            path: MANIFEST_REL_PATH.into(),
            deleted: false,
            untracked: false,
        }];
        let err = check_thread_safety_manifest(dir.path(), &changed).unwrap_err();
        assert!(err.to_string().contains("non-empty reason"));
    }
}
