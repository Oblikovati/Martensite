//! Bounded release test shards, selected from the same scope as packaging (#250).

use std::collections::BTreeSet;

use crate::targets::{RELEASE_TARGETS, ReleaseTarget, runnable_test_packages};

/// Assigns the suites `target` can run to its shards, longest measured suite
/// first onto the least-loaded shard (#474). Ties keep sorted package order,
/// so every job of one plan computes the same assignment.
/// Example: `let shards = release_shards(&selected, &RELEASE_TARGETS[0]);`.
pub(crate) fn release_shards(
    packages: &BTreeSet<String>,
    target: &ReleaseTarget,
) -> Vec<Vec<String>> {
    let runnable = runnable_test_packages(packages.iter().cloned().collect(), Some(target.triple));
    let count = runnable.len().min(target.test_shards);
    let mut shards = vec![(0_u32, Vec::new()); count];
    for name in longest_first(runnable) {
        // `count` is positive whenever a name exists, so a lightest shard exists.
        if let Some(lightest) = shards.iter_mut().min_by_key(|(minutes, _)| *minutes) {
            lightest.0 += crate::suite_weights::package_minutes(&name);
            lightest.1.push(name);
        }
    }
    shards.into_iter().map(|(_, names)| names).collect()
}

fn longest_first(mut names: Vec<String>) -> Vec<String> {
    names.sort_by_key(|name| std::cmp::Reverse(crate::suite_weights::package_minutes(name)));
    names
}

/// Pairs every nonempty shard with its native release platform.
/// Example: `let matrix = release_test_matrix(&selected);`.
pub(crate) fn release_test_matrix(packages: &BTreeSet<String>) -> serde_json::Value {
    let include: Vec<_> = RELEASE_TARGETS
        .iter()
        .flat_map(|target| {
            (0..release_shards(packages, target).len()).map(move |shard| {
                serde_json::json!({
                    "target": target.triple, "runner": target.runner, "shard": shard,
                })
            })
        })
        .collect();
    serde_json::json!({"include": include})
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    const LONG_SUITES: [&str; 4] = [
        "martensite-vulkan",
        "martensite-tkbo",
        "martensite-boolean-cases",
        "martensite-tkoffset",
    ];
    const LINUX: ReleaseTarget = RELEASE_TARGETS[0];

    fn names(list: &[&str]) -> BTreeSet<String> {
        list.iter().map(|name| (*name).to_owned()).collect()
    }

    #[test]
    fn shards_cover_every_selected_package_once_and_separate_long_suites() {
        let packages = names(&LONG_SUITES)
            .into_iter()
            .chain(names(&["martensite-tkmath", "martensite-process-policy"]))
            .collect();
        let shards = release_shards(&packages, &LINUX);
        assert_eq!(shards.len(), 4);
        let flattened: Vec<_> = shards.iter().flatten().cloned().collect();
        assert_eq!(flattened.len(), packages.len());
        assert_eq!(flattened.into_iter().collect::<BTreeSet<_>>(), packages);
        assert_long_suites_separated(&shards);
    }

    fn assert_long_suites_separated(shards: &[Vec<String>]) {
        for shard in shards {
            let long = shard
                .iter()
                .filter(|name| LONG_SUITES.contains(&name.as_str()));
            assert_eq!(long.count(), 1, "{shards:?}");
        }
    }

    #[test]
    fn short_suites_fill_the_least_loaded_shard_instead_of_round_robin() {
        let packages = names(&[
            "martensite-vulkan",
            "martensite-tkbo",
            "martensite-tkmath",
            "martensite-tkg3d",
        ]);
        let target = ReleaseTarget {
            test_shards: 2,
            ..LINUX
        };
        let shards = release_shards(&packages, &target);
        assert_eq!(shards[0], ["martensite-vulkan"]);
        assert_eq!(
            shards[1],
            ["martensite-tkbo", "martensite-tkmath", "martensite-tkg3d"]
        );
    }

    #[test]
    fn platforms_without_the_renderer_never_receive_its_suite_or_an_empty_shard() {
        let packages = names(&["martensite-vulkan", "martensite-tkmath"]);
        let windows = RELEASE_TARGETS
            .into_iter()
            .find(|target| target.triple == "x86_64-pc-windows-msvc")
            .unwrap();
        assert_eq!(release_shards(&packages, &windows), [["martensite-tkmath"]]);
    }

    #[test]
    fn matrix_uses_each_platform_shard_count() {
        let packages = (0..8).map(|i| format!("package-{i}")).collect();
        let matrix = release_test_matrix(&packages);
        let rows = matrix["include"].as_array().unwrap();
        assert_eq!(rows.len(), 12);
        for target in RELEASE_TARGETS {
            let count = rows
                .iter()
                .filter(|row| row["target"] == target.triple)
                .count();
            assert_eq!(count, target.test_shards, "{}", target.triple);
        }
    }

    #[test]
    fn workflow_requires_release_tests_before_publication() {
        let workflow = include_str!("../../../.github/workflows/martensite.yml");
        let lf = workflow.replace("\r\n", "\n");
        for text in [&lf, &lf.replace('\n', "\r\n")] {
            assert_release_workflow_contract(text);
        }
    }

    fn assert_release_workflow_contract(workflow: &str) {
        let workflow = workflow.replace("\r\n", "\n");
        let verify = workflow
            .split("\n  verify:\n")
            .nth(1)
            .unwrap()
            .split("\n  publish:\n")
            .next()
            .unwrap();
        assert!(
            verify.contains(
                "needs: [plan, checks, tests, release-tests, thread-sanitizer, wasm-check]"
            )
        );
        assert!(verify.contains("if: always()"));
        assert!(verify.contains("needs.release-tests.result"));
        assert!(verify.contains("cargo xtask ci gate"));
        let publish = workflow.split("\n  publish:\n").nth(1).unwrap();
        assert!(publish.contains("needs: [plan, checks, verify]"));
        assert!(!publish.contains("always()"));
    }

    #[test]
    fn empty_and_small_scopes_never_create_empty_shards() {
        for size in 0..7 {
            let packages = (0..size).map(|i| format!("package-{i}")).collect();
            let shards = release_shards(&packages, &LINUX);
            assert_eq!(shards.len(), size.min(LINUX.test_shards));
            assert!(shards.iter().all(|shard| !shard.is_empty()));
        }
    }
}
