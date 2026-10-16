//! Every `tests/*.rs` case file runs: either the `toolkit` executable lists it in
//! `tests/suites/`, or `Cargo.toml` declares it as its own `[[test]]` target.
//! Cargo's `autotests = false` would otherwise skip a new file without a warning.
use std::collections::BTreeSet;
use std::path::Path;

fn case_files(tests: &Path) -> BTreeSet<String> {
    let entries = std::fs::read_dir(tests).expect("tests directory is readable");
    entries
        .map(|entry| entry.expect("tests directory entry").file_name())
        .filter_map(|name| name.into_string().ok())
        .filter(|name| name.ends_with(".rs") && name != "toolkit.rs")
        .collect()
}

fn suite_listed_files(tests: &Path) -> BTreeSet<String> {
    let entries = std::fs::read_dir(tests.join("suites")).expect("suites directory is readable");
    let mut listed = BTreeSet::new();
    for entry in entries {
        let text = std::fs::read_to_string(entry.expect("suite entry").path()).expect("suite file");
        listed.extend(quoted_after(&text, "#[path = \"../"));
    }
    listed
}

fn separate_target_files(manifest: &str) -> BTreeSet<String> {
    quoted_after(manifest, "path = \"tests/").collect()
}

fn quoted_after<'a>(text: &'a str, prefix: &'a str) -> impl Iterator<Item = String> + 'a {
    text.lines()
        .filter_map(move |line| line.trim().strip_prefix(prefix))
        .filter_map(|rest| rest.split('"').next())
        .filter(|path| !path.contains('/'))
        .map(str::to_owned)
}

#[test]
fn every_case_file_belongs_to_a_test_target() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = std::fs::read_to_string(root.join("Cargo.toml")).expect("crate manifest");
    let mut covered = suite_listed_files(&root.join("tests"));
    covered.extend(separate_target_files(&manifest));
    let missing: Vec<String> = case_files(&root.join("tests"))
        .difference(&covered)
        .cloned()
        .collect();
    assert!(
        missing.is_empty(),
        "case files {missing:?}: expected each in tests/suites/ or a [[test]] target"
    );
}
