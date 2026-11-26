//! CI archguard rejecting new `thread_local!` or `static mut` in crate sources (#62).

use std::fs;
use std::path::Path;

use crate::error::{BuildError, BuildResult, io_error};
use crate::git::ChangedFile;

const ALLOWLISTED_PATHS: &[&str] = &[
    "crates/martensite-tkernel/src/standard/error_handler/handler_stack.rs",
    "crates/martensite-tkernel/src/ncollection/packed_map/storage/allocation/observation.rs",
    "crates/martensite-tkbool/src/top_ope_brep_build/globals.rs",
    "crates/martensite-tkbool/src/top_ope_brep_build/builder/split.rs",
    "crates/martensite-tkbool/src/top_ope_brep_build/builder/ffsfs.rs",
    "crates/martensite-tkbool/src/top_ope_brep_build/builder/grid_ss.rs",
    "crates/martensite-tkbool/src/top_ope_brep_build/builder1/mod.rs",
    "crates/martensite-tkbool/src/top_ope_brep_build/hbuilder/mod.rs",
    "crates/martensite-tkbool/src/top_ope_brep_tool/two_d/mod.rs",
    "crates/martensite-tkbool/src/top_ope_brep_tool/shape_classifier/perform.rs",
    "crates/martensite-tkbool/src/top_ope_brep_ds/face_interference_tool/mod.rs",
    "crates/martensite-tkbool/src/top_ope_brep_ds/connex.rs",
    "crates/martensite-tkxsbase/src/if_select/work_session/mod.rs",
    "crates/martensite-tkdestep/src/step_selections/selects.rs",
    "crates/martensite-tkdestep/src/step_selections/select_instances.rs",
    "crates/martensite-vulkan/src/backend/draw_table.rs",
    "crates/martensite-boolean-cases/src/support/process.rs",
    "crates/martensite-boolean-cases/src/support/heal_commands.rs",
];

/// Fails CI if any changed non-allowlisted `crates/*/src/*.rs` introduces
/// `thread_local!` or `static mut` (#62).
pub fn check_global_mutable_state(root: &Path, changed: &[ChangedFile]) -> BuildResult<()> {
    let mut violations = Vec::new();
    for file in changed {
        if file.deleted || !is_guarded_source(&file.path) || is_allowlisted(&file.path) {
            continue;
        }
        let full = root.join(&file.path);
        if !full.is_file() {
            continue;
        }
        let source = fs::read_to_string(&full).map_err(|err| io_error(&full, err))?;
        for (idx, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                continue;
            }
            if trimmed.contains("thread_local!") || trimmed.contains("static mut ") {
                violations.push(format!("{}:{}: {}", file.path, idx + 1, trimmed));
            }
        }
    }
    if violations.is_empty() {
        return Ok(());
    }
    Err(BuildError::UnexpectedOutput {
        origin: "global-state-guard (#62)".to_owned(),
        detail: format!(
            "forbidden thread_local! or static mut in kernel source:\n{}",
            violations.join("\n")
        ),
    })
}

fn is_guarded_source(path: &str) -> bool {
    path.starts_with("crates/")
        && path.contains("/src/")
        && path.ends_with(".rs")
        && !path.ends_with("_tests.rs")
        && !path.ends_with("/tests.rs")
}

fn is_allowlisted(path: &str) -> bool {
    ALLOWLISTED_PATHS.contains(&path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_new_thread_local_in_kernel_source() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir
            .path()
            .join("crates/martensite-tkfillet/src/new_algo.rs");
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(&file, "thread_local! { static BAD: usize = 0; }\n").unwrap();
        let changed = vec![ChangedFile {
            path: "crates/martensite-tkfillet/src/new_algo.rs".into(),
            deleted: false,
            untracked: false,
        }];
        let err = check_global_mutable_state(dir.path(), &changed).unwrap_err();
        assert!(err.to_string().contains("thread_local!"));
    }

    #[test]
    fn allows_clean_kernel_source_and_comments() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("crates/martensite-tkfillet/src/clean.rs");
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(&file, "// thread_local! is forbidden\npub struct Clean;\n").unwrap();
        let changed = vec![ChangedFile {
            path: "crates/martensite-tkfillet/src/clean.rs".into(),
            deleted: false,
            untracked: false,
        }];
        check_global_mutable_state(dir.path(), &changed).unwrap();
    }
}
