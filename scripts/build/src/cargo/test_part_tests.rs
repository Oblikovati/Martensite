//! Disjoint, complete PR test parts (#470).
use super::*;
use crate::fake_runner::RecordingRunner;
use std::collections::BTreeMap;

const BASE: &str = "cargo test --locked --package martensite-sample --no-fail-fast";

fn artifact(kind: &str, name: &str, test: bool) -> String {
    serde_json::json!({
        "reason": "compiler-artifact",
        "executable": format!("/repo/target/debug/deps/{name}-0"),
        "profile": {"test": test},
        "target": {"kind": [kind], "name": name},
    })
    .to_string()
}

/// A package with a library, a listed integration test, a custom-harness test
/// and a build script whose artifact is not a test.
fn sample_runner() -> RecordingRunner {
    let build = [
        artifact("custom-build", "build-script-build", false),
        artifact("lib", "martensite_sample", true),
        artifact("test", "views", true),
        artifact("test", "harness", true),
    ]
    .join("\n");
    let lib: String = (0..40).map(|i| format!("unit::case_{i}: test\n")).collect();
    let views: String = (0..60).map(|i| format!("view_{i}: test\n")).collect();
    RecordingRunner::default()
        .with_stdout(
            &format!("{BASE} --no-run --message-format json-render-diagnostics"),
            &build,
        )
        .with_stdout(&format!("{BASE} --lib -- --list --format terse"), &lib)
        .with_stdout(
            &format!("{BASE} --test views -- --list --format terse"),
            &views,
        )
        .with_exit_code(
            &format!("{BASE} --test harness -- --list --format terse"),
            1,
        )
}

fn run_part(runner: &RecordingRunner, index: usize) {
    let tasks = CargoTasks::new(runner, Path::new("/repo"));
    let selection = PackageSelection::Packages(vec!["martensite-sample".into()]);
    let part = TestPart { index, count: 3 };
    tasks
        .test_part(&selection, &BuildTarget::Host, part)
        .unwrap();
}

/// Every `--exact` name each part ran, keyed by the selected target.
fn exact_runs(commands: &[String]) -> Vec<String> {
    commands
        .iter()
        .filter_map(|command| command.split_once(" -- --exact "))
        .flat_map(|(target, names)| {
            let target = target.trim_start_matches(BASE).trim().to_owned();
            names
                .split(' ')
                .map(move |name| format!("{target} {name}"))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[test]
fn parts_run_every_listed_test_exactly_once() {
    let mut seen = BTreeMap::new();
    for index in 0..3 {
        let runner = sample_runner();
        run_part(&runner, index);
        for name in exact_runs(&runner.commands()) {
            *seen.entry(name).or_insert(0) += 1;
        }
    }
    assert_eq!(seen.len(), 100);
    assert!(seen.values().all(|count| *count == 1), "{seen:?}");
}

#[test]
fn doctests_and_unlisted_targets_run_whole_in_exactly_one_part() {
    let mut doc_parts = Vec::new();
    let mut harness_parts = Vec::new();
    for index in 0..3 {
        let runner = sample_runner();
        run_part(&runner, index);
        let commands = runner.commands();
        if commands.contains(&format!("{BASE} --doc")) {
            doc_parts.push(index);
        }
        if commands.contains(&format!("{BASE} --test harness")) {
            harness_parts.push(index);
        }
    }
    assert_eq!(doc_parts, [0]);
    assert_eq!(harness_parts.len(), 1);
}

#[test]
fn planned_parts_follow_the_measured_table_and_refuse_other_indices() {
    let parts = crate::suite_weights::pr_parts("martensite-vulkan");
    for index in 0..parts {
        assert!(
            TestPart::planned("martensite-vulkan", index)
                .unwrap()
                .is_some()
        );
    }
    let error = TestPart::planned("martensite-vulkan", parts).unwrap_err();
    assert!(
        error
            .to_string()
            .contains(&format!("expected an index below {parts}"))
    );
    assert_eq!(TestPart::planned("martensite-tkmath", 0).unwrap(), None);
    assert!(TestPart::planned("martensite-tkmath", 1).is_err());
}

#[test]
fn ownership_assigns_each_key_to_one_part_and_uses_every_part() {
    let parts: Vec<_> = (0..4).map(|index| TestPart { index, count: 4 }).collect();
    let mut used = [0; 4];
    for key in (0..1000).map(|i| format!("test:views::case_{i}")) {
        let owners: Vec<_> = parts.iter().filter(|part| part.owns(&key)).collect();
        assert_eq!(owners.len(), 1, "{key}");
        used[owners[0].index] += 1;
    }
    assert!(used.iter().all(|count| *count > 150), "{used:?}");
}

#[test]
fn stable_hash_matches_the_published_fnv1a_vector() {
    assert_eq!(stable_hash("a"), 0xaf63_dc4c_8601_ec8c);
}

#[test]
fn long_filter_lists_split_below_the_command_line_budget_in_order() {
    let names: Vec<_> = (0..3000).map(|i| format!("module::case_{i:05}")).collect();
    let chunks = filter_chunks(names.clone());
    assert!(chunks.len() > 1);
    for chunk in &chunks {
        let bytes: usize = chunk.iter().map(|name| name.len() + 1).sum();
        assert!(bytes <= FILTER_BYTES, "{bytes}");
    }
    assert_eq!(chunks.concat(), names);
    assert!(filter_chunks(Vec::new()).is_empty());
}

#[test]
fn build_messages_keep_only_test_executables_once() {
    let line = artifact("test", "views", true);
    let stdout = [line.clone(), line, artifact("lib", "dep", false)].join("\n");
    let targets = parse_test_targets(&stdout).unwrap();
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0].selector, ["--test", "views"]);
    assert!(parse_test_targets("{not json").is_err());
}

fn isolated_target_runner() -> RecordingRunner {
    RecordingRunner::default()
        .with_stdout(
            &format!("{BASE} --no-run --message-format json-render-diagnostics"),
            &artifact("test", "parallel_speedup", true),
        )
        .with_stdout(
            &format!("{BASE} --test parallel_speedup -- --list --format terse"),
            "bugs_modalg_5::bug24639: test\n",
        )
}

#[test]
fn isolated_performance_target_runs_in_exactly_one_part_without_other_tests() {
    let mut runs = Vec::new();
    for index in 0..3 {
        let runner = isolated_target_runner();
        run_part(&runner, index);
        runs.extend(exact_runs(&runner.commands()));
    }
    assert_eq!(runs, ["--test parallel_speedup bugs_modalg_5::bug24639"]);
}
