//! Source-token ratchet complements compiler expansion checks across inactive cfgs (#81).
use std::collections::BTreeMap;
use std::path::Path;

use proc_macro2::{TokenStream, TokenTree};
use serde::{Deserialize, Serialize};

use crate::context::BuildContext;
use crate::error::{BuildError, BuildResult, io_error};

pub(crate) const SOURCE_BASELINE: &str = "docs/features/safe-toolkits/unsafe-source-baseline.json";

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
struct UnsafeTokens {
    unsafe_keywords: usize,
    unsafe_code_mentions: usize,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    kinds: BTreeMap<String, usize>,
}

type Inventory = BTreeMap<String, UnsafeTokens>;

fn count_tokens(stream: TokenStream, counts: &mut UnsafeTokens) {
    let mut tokens = stream.into_iter().peekable();
    while let Some(token) = tokens.next() {
        match token {
            TokenTree::Group(group) => count_tokens(group.stream(), counts),
            TokenTree::Ident(ident) if ident == "unsafe" => {
                counts.unsafe_keywords += 1;
                *counts
                    .kinds
                    .entry(unsafe_kind(tokens.peek()).into())
                    .or_default() += 1;
            }
            TokenTree::Ident(ident) if ident == "unsafe_code" => counts.unsafe_code_mentions += 1,
            _ => {}
        }
    }
}

fn unsafe_kind(next: Option<&TokenTree>) -> &'static str {
    match next {
        Some(TokenTree::Group(group)) if group.delimiter() == proc_macro2::Delimiter::Brace => {
            "block"
        }
        Some(TokenTree::Ident(ident)) => match ident.to_string().as_str() {
            "fn" => "function",
            "impl" => "implementation",
            "trait" => "trait",
            "extern" => "extern",
            _ => "other",
        },
        _ => "other",
    }
}

fn source_counts(source: &str) -> Result<UnsafeTokens, String> {
    let stream = source
        .parse::<TokenStream>()
        .map_err(|error| error.to_string())?;
    let mut counts = UnsafeTokens::default();
    count_tokens(stream, &mut counts);
    Ok(counts)
}

fn inventory_file(root: &Path, path: &Path, inventory: &mut Inventory) -> Result<(), String> {
    let source =
        std::fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let counts = source_counts(&source).map_err(|error| format!("{}: {error}", path.display()))?;
    if counts != UnsafeTokens::default() {
        let relative = path.strip_prefix(root).map_err(|error| error.to_string())?;
        inventory.insert(relative.to_string_lossy().replace('\\', "/"), counts);
    }
    Ok(())
}

fn inventory_tree(root: &Path, directory: &Path, inventory: &mut Inventory) -> Result<(), String> {
    let entries = std::fs::read_dir(directory).map_err(|error| error.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|error| error.to_string())?;
        let kind = entry.file_type().map_err(|error| error.to_string())?;
        let path = entry.path();
        if kind.is_symlink() {
            return Err(format!(
                "{}: expected regular source entry, found symlink",
                path.display()
            ));
        }
        if kind.is_dir() {
            inventory_tree(root, &path, inventory)?;
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            inventory_file(root, &path, inventory)?;
        }
    }
    Ok(())
}

fn compare_inventory(actual: &Inventory, baseline: &Inventory) -> Vec<String> {
    let mut differences = Vec::new();
    for (path, counts) in actual {
        if baseline.get(path) != Some(counts) {
            differences.push(format!(
                "{path}: got {counts:?}, expected {:?}",
                baseline.get(path)
            ));
        }
    }
    for path in baseline.keys().filter(|path| !actual.contains_key(*path)) {
        differences.push(format!("{path}: removed; delete its stale baseline entry"));
    }
    differences
}

fn collect_workspace(root: &Path) -> Result<Inventory, String> {
    let mut actual = Inventory::new();
    inventory_tree(root, &root.join("crates"), &mut actual)?;
    Ok(actual)
}

/// Enforces the event base's allowances and verifies the candidate's recorded reductions.
/// Example: `check_candidate_ceiling(context, base)?`.
pub(crate) fn check_candidate_ceiling(context: &BuildContext<'_>, base: &str) -> BuildResult<()> {
    let approved = parse_reviewed(&context.git.file_at(base, SOURCE_BASELINE)?)?;
    let path = context.root().join(SOURCE_BASELINE);
    let candidate = std::fs::read_to_string(&path).map_err(|error| io_error(&path, error))?;
    let candidate = parse_reviewed(&candidate)?;
    let actual = collect_workspace(context.root()).map_err(BuildError::LintFailed)?;
    enforce_ceiling(&actual, &approved)?;
    let differences = compare_inventory(&actual, &candidate);
    if differences.is_empty() {
        return Ok(());
    }
    Err(BuildError::LintFailed(differences.join("\n")))
}

fn parse_reviewed(json: &str) -> BuildResult<Inventory> {
    serde_json::from_str(json).map_err(|error| {
        BuildError::LintFailed(format!(
            "source unsafe inventory: {error}; expected file counts"
        ))
    })
}

fn enforce_ceiling(actual: &Inventory, approved: &Inventory) -> BuildResult<()> {
    let empty = UnsafeTokens::default();
    for (path, tokens) in actual {
        let allowance = approved.get(path).unwrap_or(&empty);
        if exceeds_ceiling(tokens, allowance) {
            return Err(BuildError::LintFailed(format!(
                "{path}: unsafe source counts {tokens:?} exceed the event base's reviewed ceiling {allowance:?}"
            )));
        }
    }
    Ok(())
}

fn exceeds_ceiling(actual: &UnsafeTokens, ceiling: &UnsafeTokens) -> bool {
    actual.unsafe_keywords > ceiling.unsafe_keywords
        || actual.unsafe_code_mentions > ceiling.unsafe_code_mentions
        || actual
            .kinds
            .iter()
            .any(|(kind, count)| *count > ceiling.kinds.get(kind).copied().unwrap_or(0))
}

#[test]
fn unsafe_inventory_matches_reviewed_baseline() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let actual = collect_workspace(root).unwrap();
    let baseline: Inventory = serde_json::from_str(include_str!(
        "../../../docs/features/safe-toolkits/unsafe-source-baseline.json"
    ))
    .unwrap();
    let differences = compare_inventory(&actual, &baseline);
    assert!(
        differences.is_empty(),
        "{}\nACTUAL_INVENTORY={}\nEND_INVENTORY",
        differences.join("\n"),
        serde_json::to_string_pretty(&actual).unwrap()
    );
}

#[test]
fn unsafe_inventory_ignores_comments_and_literals_but_visits_macros_and_cfgs() {
    let source = r##"
        // unsafe { ignored() }
        const TEXT: &str = r#"unsafe_code unsafe"#;
        #[cfg(windows)] #[allow(unsafe_code)] unsafe extern "C" {}
        macro_rules! make { () => { unsafe fn generated() { unsafe {} } } }
    "##;
    assert_eq!(
        source_counts(source).unwrap(),
        UnsafeTokens {
            unsafe_keywords: 3,
            unsafe_code_mentions: 1,
            kinds: BTreeMap::from([
                ("block".into(), 1),
                ("function".into(), 1),
                ("extern".into(), 1)
            ]),
        }
    );
}

#[test]
fn unsafe_inventory_rejects_unparseable_source() {
    assert!(source_counts("unsafe {").is_err());
}

#[test]
fn unsafe_inventory_requires_reviewed_additions_and_removals() {
    let baseline = Inventory::from([(
        "old.rs".into(),
        UnsafeTokens {
            unsafe_keywords: 1,
            unsafe_code_mentions: 1,
            kinds: BTreeMap::from([("block".into(), 1)]),
        },
    )]);
    assert!(compare_inventory(&baseline, &baseline).is_empty());
    assert_eq!(compare_inventory(&Inventory::new(), &baseline).len(), 1);
    assert_eq!(compare_inventory(&baseline, &Inventory::new()).len(), 1);
}

#[test]
fn unsafe_inventory_detects_equal_count_construct_substitutions() {
    let block = source_counts("fn f() { unsafe {} }").unwrap();
    let implementation = source_counts("unsafe impl Send for Value {}").unwrap();
    assert_eq!(block.unsafe_keywords, implementation.unsafe_keywords);
    let baseline = Inventory::from([("source.rs".into(), block)]);
    let actual = Inventory::from([("source.rs".into(), implementation)]);
    assert_eq!(compare_inventory(&actual, &baseline).len(), 1);
}

#[test]
fn unsafe_inventory_keeps_unclassified_macro_tokens_visible() {
    let counts = source_counts(
        "unsafe trait A {} unsafe extern fn f() {} macro_rules! m { (unsafe) => {} }",
    )
    .unwrap();
    assert_eq!(
        counts.kinds,
        BTreeMap::from([
            ("trait".into(), 1),
            ("extern".into(), 1),
            ("other".into(), 1)
        ])
    );
    assert_eq!(counts.kinds.values().sum::<usize>(), counts.unsafe_keywords);
}

#[test]
fn unsafe_inventory_reads_nested_sources_and_skips_non_rust_files() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::create_dir(directory.path().join("nested")).unwrap();
    std::fs::write(
        directory.path().join("nested/source.rs"),
        "unsafe fn example() {}",
    )
    .unwrap();
    std::fs::write(directory.path().join("note.txt"), "unsafe {").unwrap();
    let mut actual = Inventory::new();
    inventory_tree(directory.path(), directory.path(), &mut actual).unwrap();
    assert_eq!(actual.len(), 1);
    assert_eq!(actual["nested/source.rs"].unsafe_keywords, 1);
}

#[test]
fn migration_rejects_new_unsafe_even_when_the_candidate_baseline_matches() {
    let fixture = MigrationInventoryFixture::with_candidate(
        "{}",
        r#"#[cfg(windows)] fn node() { unsafe {} }"#,
    );
    assert!(fixture.exact_candidate_matches());
    let error = fixture.check().unwrap_err().to_string();
    assert!(error.contains(MigrationInventoryFixture::FILE));
    assert!(error.contains("exceed the event base's reviewed ceiling"));
}

#[test]
fn migration_accepts_only_recorded_source_inventory_reductions() {
    let fixture = MigrationInventoryFixture::with_candidate(
        r#"{"crates/toolkit/src/raw.rs":{"unsafe_keywords":2,"unsafe_code_mentions":0,"kinds":{"block":2}}}"#,
        "fn node() { unsafe {} }",
    );
    fixture.check().unwrap();
    std::fs::write(fixture.root.path().join(SOURCE_BASELINE), "{}").unwrap();
    let error = fixture.check().unwrap_err().to_string();
    assert!(error.contains("expected None"));
}

#[test]
fn migration_ceiling_tracks_lint_mentions_and_kind_counts_separately() {
    let approved = source_counts("fn node() { unsafe {} }").unwrap();
    let change_kind = source_counts("unsafe fn node() {}").unwrap();
    let change_lint = source_counts("#[allow(unsafe_code)] fn node() { unsafe {} }").unwrap();
    assert_eq!(change_kind.unsafe_keywords, approved.unsafe_keywords);
    assert_eq!(change_lint.unsafe_keywords, approved.unsafe_keywords);
    assert!(exceeds_ceiling(&change_kind, &approved));
    assert!(exceeds_ceiling(&change_lint, &approved));
}

#[cfg(test)]
struct MigrationInventoryFixture {
    root: tempfile::TempDir,
    runner: crate::fake_runner::RecordingRunner,
}

#[cfg(test)]
impl MigrationInventoryFixture {
    const FILE: &str = "crates/toolkit/src/raw.rs";
    const BASE: &str = "82b13dff919c19e79a581251cc41accd2fd00337";

    fn with_candidate(approved: &str, source: &str) -> Self {
        let root = tempfile::tempdir().unwrap();
        write_fixture_file(root.path(), Self::FILE, source);
        let current = collect_workspace(root.path()).unwrap();
        let baseline = serde_json::to_string(&current).unwrap();
        write_fixture_file(root.path(), SOURCE_BASELINE, &baseline);
        let runner = crate::fake_runner::RecordingRunner::default()
            .with_stdout(
                "git rev-parse --show-toplevel",
                &root.path().to_string_lossy(),
            )
            .with_stdout(
                &format!("git show {}:{SOURCE_BASELINE}", Self::BASE),
                approved,
            );
        Self { root, runner }
    }

    fn check(&self) -> BuildResult<()> {
        let context = crate::context::tests::context_at(&self.runner, self.root.path());
        check_candidate_ceiling(&context, Self::BASE)
    }

    fn exact_candidate_matches(&self) -> bool {
        let baseline = std::fs::read_to_string(self.root.path().join(SOURCE_BASELINE)).unwrap();
        let approved = parse_reviewed(&baseline).unwrap();
        let actual = collect_workspace(self.root.path()).unwrap();
        compare_inventory(&actual, &approved).is_empty()
    }
}

#[cfg(test)]
fn write_fixture_file(root: &Path, relative: &str, source: &str) {
    let path = root.join(relative);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, source).unwrap();
}
