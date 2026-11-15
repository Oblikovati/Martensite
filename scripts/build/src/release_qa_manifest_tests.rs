use super::*;
use std::fs;
const BUDGET_SCRIPT: &str = include_str!("../../../crates/martensite-boolean-cases/build.rs");

fn manifest(extra: &str) -> PackageManifest {
    PackageManifest::parse(
        &format!("[package]\nname='a'\nversion='1.0.0'\n{extra}"),
        &toml::Table::new(),
    )
    .unwrap()
    .unwrap()
}
fn fixture(source: &str, script: Option<&str>) -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir_all(root.path().join("a/src")).unwrap();
    fs::write(root.path().join("a/src/lib.rs"), source).unwrap();
    fs::write(
        root.path().join("a/Cargo.toml"),
        "[package]\nname='a'\nversion='1.0.0'",
    )
    .unwrap();
    if let Some(script) = script {
        fs::write(root.path().join("a/build.rs"), script).unwrap();
    }
    root
}
fn changed(path: &str) -> ChangedFile {
    ChangedFile {
        path: format!("a/{path}"),
        deleted: false,
        untracked: false,
    }
}
fn publication(root: &Path, current: &PackageManifest) -> PackageManifest {
    let changed = [changed("Cargo.toml"), changed("tests/performance.rs")];
    publication_manifest(root, current, Some(&manifest("")), "a", &changed).into_owned()
}
fn test_target(extra: &str) -> PackageManifest {
    manifest(&format!(
        "[[test]]\nname='performance'\npath='tests/performance.rs'\n{extra}"
    ))
}
fn test_edit_count(root: &Path) -> usize {
    let edits = [changed("tests/performance.rs")];
    crate::release_test_inputs::publication_changes(root, &manifest(""), "a", &edits).len()
}
#[test]
fn test_target_addition_is_qa_only_with_no_build_script() {
    let root = fixture("pub fn value() {}", None);
    assert_eq!(
        publication(root.path(), &test_target("")).source,
        manifest("").source
    );
    assert_eq!(
        publication(root.path(), &test_target("harness=false")).source,
        manifest("").source
    );
}
#[test]
fn approved_budget_marker_allows_test_paths_and_target_changes() {
    let normalized = BUDGET_SCRIPT.replace("\r\n", "\n");
    for script in [normalized.clone(), normalized.replace('\n', "\r\n")] {
        let root = fixture("pub fn value() {}", Some(&script));
        assert_eq!(
            publication(root.path(), &test_target("")).source,
            manifest("").source
        );
        assert_eq!(test_edit_count(root.path()), 0);
    }
}
#[test]
fn unknown_or_modified_build_script_keeps_qa_edits_as_source() {
    let edited = format!("{BUDGET_SCRIPT}\nfn another_input() {{}}\n");
    for script in ["fn main() {}", edited.as_str()] {
        let root = fixture("pub fn value() {}", Some(script));
        assert_eq!(
            publication(root.path(), &test_target("")).source,
            test_target("").source
        );
        assert_eq!(test_edit_count(root.path()), 1);
    }
}
#[test]
fn production_manifest_includes_keep_test_target_edits_as_source() {
    for source in [
        "pub const VALUE: &str = include_str!(\"../Cargo.toml\");",
        "pub const VALUE: &str = include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/Cargo.toml\"));",
    ] {
        let root = fixture(source, None);
        assert_eq!(
            publication(root.path(), &test_target("")).source,
            test_target("").source
        );
    }
}
#[test]
fn production_manifest_fields_and_unknown_test_fields_remain_source() {
    let root = fixture("pub fn value() {}", None);
    for extra in [
        "[dependencies]\nx='1'",
        "[features]\nmore=[]",
        "[lib]\npath='src/lib.rs'",
    ] {
        let current = test_target(extra);
        assert_eq!(publication(root.path(), &current).source, current.source);
    }
    let current = test_target("required-features=['extra']");
    assert_eq!(publication(root.path(), &current).source, current.source);
}
#[test]
fn custom_build_path_and_missing_package_history_remain_source() {
    let root = fixture("pub fn value() {}", Some(BUDGET_SCRIPT));
    let current =
        manifest("build='other.rs'\n[[test]]\nname='performance'\npath='tests/performance.rs'");
    assert_eq!(publication(root.path(), &current).source, current.source);
    assert!(matches!(
        publication_manifest(root.path(), &test_target(""), None, "a", &[]),
        Cow::Borrowed(_)
    ));
}

fn actual_budget_package() -> (std::path::PathBuf, PackageManifest) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap();
    let workspace =
        crate::package_manifest::parse_table(include_str!("../../../Cargo.toml")).unwrap();
    let inherited = workspace["workspace"]["package"].as_table().unwrap();
    let manifest = PackageManifest::parse(
        include_str!("../../../crates/martensite-boolean-cases/Cargo.toml"),
        inherited,
    )
    .unwrap()
    .unwrap();
    (root, manifest)
}

#[test]
fn actual_boolean_package_test_edits_do_not_create_a_publication_identity() {
    let (root, current) = actual_budget_package();
    let directory = "crates/martensite-boolean-cases";
    let changes = [ChangedFile {
        path: format!("{directory}/tests/parallel_speedup/main.rs"),
        deleted: false,
        untracked: false,
    }];
    let inputs =
        crate::release_test_inputs::publication_changes(&root, &current, directory, &changes);
    assert!(
        inputs.is_empty(),
        "unexpected publication inputs: {inputs:?}"
    );
}

#[test]
fn actual_budget_package_qa_manifest_change_remains_unpublished() {
    let (root, current) = actual_budget_package();
    let mut previous = current.clone();
    previous.source.remove("test");
    let directory = "crates/martensite-boolean-cases";
    let changes = [ChangedFile {
        path: format!("{directory}/Cargo.toml"),
        deleted: false,
        untracked: false,
    }];
    let comparison = publication_manifest(&root, &current, Some(&previous), directory, &changes);
    assert_eq!(comparison.source, previous.source);
}

#[test]
fn dynamic_manifest_reference_stays_source_without_a_recent_manifest_diff() {
    let source = "pub const VALUE: &str = include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/Cargo.toml\"));";
    let root = fixture(source, None);
    let current = test_target("");
    let previous = manifest("");
    let comparison = publication_manifest(root.path(), &current, Some(&previous), "a", &[]);
    assert_eq!(comparison.source, current.source);
}

fn manifest_with_target(section: &str, path: &str) -> PackageManifest {
    let name = if section == "[[bin]]" {
        "name='runtime'\n"
    } else {
        ""
    };
    test_target(&format!("{section}\n{name}path={path:?}"))
}
fn target_comparison(root: &Path, current: &PackageManifest) -> PackageManifest {
    let mut previous = current.clone();
    previous.source.remove("test");
    let changes = [changed("Cargo.toml")];
    publication_manifest(root, current, Some(&previous), "a", &changes).into_owned()
}
fn write_target(root: &Path, path: &str, source: &str) {
    let target = root.join("a").join(path);
    fs::create_dir_all(target.parent().unwrap()).unwrap();
    fs::write(target, source).unwrap();
}
fn assert_target_retains_manifest(section: &str, path: &str) {
    let root = fixture("pub fn value() {}", None);
    write_target(
        root.path(),
        path,
        "pub const SOURCE: &str = include_str!(\"../Cargo.toml\"); fn main() { std::hint::black_box(SOURCE); }",
    );
    let current = manifest_with_target(section, path);
    assert_eq!(
        target_comparison(root.path(), &current).source,
        current.source,
        "{section} {path}"
    );
}
#[test]
fn traversing_and_unscanned_production_targets_cannot_hide_manifest_inputs() {
    for section in ["[lib]", "[[bin]]"] {
        for path in ["src/../tests/runtime.rs", "src/runtime.input"] {
            assert_target_retains_manifest(section, path);
        }
    }
}
#[test]
fn ordinary_rust_targets_under_src_remain_eligible_for_the_qa_proof() {
    for section in ["[lib]", "[[bin]]"] {
        let root = fixture("pub fn value() {}", None);
        write_target(root.path(), "src/runtime.rs", "fn main() {}");
        let current = manifest_with_target(section, "src/runtime.rs");
        let mut previous = current.clone();
        previous.source.remove("test");
        assert_eq!(
            target_comparison(root.path(), &current).source,
            previous.source
        );
    }
}

#[cfg(unix)]
#[test]
fn linked_production_source_file_disables_the_qa_proof() {
    let root = fixture("mod linked;", None);
    let package = root.path().join("a");
    fs::write(package.join("owned.rs"), "pub fn value() {}").unwrap();
    std::os::unix::fs::symlink(package.join("owned.rs"), package.join("src/linked.rs")).unwrap();
    assert_eq!(test_edit_count(root.path()), 1);
}

#[cfg(unix)]
#[test]
fn linked_production_source_directory_disables_the_qa_proof() {
    let root = fixture("pub fn value() {}", None);
    let package = root.path().join("a");
    fs::rename(package.join("src"), package.join("owned_src")).unwrap();
    std::os::unix::fs::symlink(package.join("owned_src"), package.join("src")).unwrap();
    assert_eq!(test_edit_count(root.path()), 1);
}

#[cfg(unix)]
#[test]
fn linked_package_root_disables_the_qa_proof() {
    let root = fixture("pub fn value() {}", None);
    fs::rename(root.path().join("a"), root.path().join("owned")).unwrap();
    std::os::unix::fs::symlink(root.path().join("owned"), root.path().join("a")).unwrap();
    assert_eq!(test_edit_count(root.path()), 1);
}

fn assert_manifest_input_refused(source: &str) {
    let root = fixture(source, None);
    let current = test_target("");
    assert_eq!(publication(root.path(), &current).source, current.source);
}
#[test]
fn literal_manifest_aliases_and_unbounded_inputs_remain_source() {
    for source in [
        "#[cfg(windows)] pub const SOURCE: &str = include_str!(\"../CARGO.TOML\");",
        "#[cfg(windows)] pub const SOURCE: &str = include_str!(\"../CARGO~1.TOM\");",
        "use std::include_str as read; pub const SOURCE: &str = read!(\"../CARGO.TOML\");",
        "pub const SOURCE: &str = include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/config.txt\"));",
    ] {
        assert_manifest_input_refused(source);
    }
}
#[test]
fn owned_literal_input_with_distinct_bytes_keeps_the_qa_exemption() {
    let root = fixture("pub const SOURCE: &str = include_str!(\"data.txt\");", None);
    let size = fs::metadata(root.path().join("a/Cargo.toml"))
        .unwrap()
        .len() as usize;
    fs::write(root.path().join("a/src/data.txt"), vec![b'x'; size]).unwrap();
    let current = test_target("");
    assert_eq!(
        publication(root.path(), &current).source,
        manifest("").source
    );
}

#[test]
fn manifest_case_alias_is_refused_even_when_local_bytes_differ() {
    let root = fixture(
        "#[cfg(windows)] pub const SOURCE: &str = include_str!(\"../CARGO.TOML\");",
        None,
    );
    fs::write(
        root.path().join("a/CARGO.TOML"),
        "local case-distinct input",
    )
    .unwrap();
    let current = test_target("");
    assert_eq!(publication(root.path(), &current).source, current.source);
}
