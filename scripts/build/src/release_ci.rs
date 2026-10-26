//! Bounded release test shards, selected from the same scope as packaging (#250).

use std::collections::BTreeSet;

use crate::targets::RELEASE_TARGETS;

// Four shards bound repeated compilation while separating the long suites in
// docs/features/ci-build-driver/parallel-tests.md (#250).
const RELEASE_SHARDS: usize = 4;
const LONG_SUITES: [&str; 4] = [
    "martensite-tkgeombase",
    "martensite-tkprim",
    "martensite-tkgeomalgo",
    "martensite-tktopalgo",
];

/// Assigns selected suites once, separating measured long suites to limit the tail.
/// Example: `let shards = release_shards(&selected);`.
pub(crate) fn release_shards(packages: &BTreeSet<String>) -> Vec<Vec<String>> {
    let count = packages.len().min(RELEASE_SHARDS);
    if count == 0 {
        return Vec::new();
    }
    let mut shards = vec![Vec::new(); count];
    let long = LONG_SUITES.iter().filter(|name| packages.contains(**name));
    let rest = packages
        .iter()
        .filter(|name| !LONG_SUITES.contains(&name.as_str()));
    let ordered = long.map(|name| (*name).to_owned()).chain(rest.cloned());
    for (index, name) in ordered.enumerate() {
        shards[index % count].push(name);
    }
    shards
}

/// Pairs every nonempty shard with each native release platform.
/// Example: `let matrix = release_test_matrix(&selected);`.
pub(crate) fn release_test_matrix(packages: &BTreeSet<String>) -> serde_json::Value {
    let shards = release_shards(packages);
    let include: Vec<_> = RELEASE_TARGETS
        .iter()
        .flat_map(|target| {
            (0..shards.len()).map(move |shard| {
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

    #[test]
    fn shards_cover_every_selected_package_once_and_separate_long_suites() {
        let packages = LONG_SUITES
            .iter()
            .copied()
            .chain(["martensite-tkmath", "martensite-process-policy"])
            .map(str::to_owned)
            .collect();
        let shards = release_shards(&packages);
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
            assert_eq!(long.count(), 1);
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
        assert!(verify.contains("needs: [plan, checks, tests, release-tests, thread-sanitizer]"));
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
            let shards = release_shards(&packages);
            assert_eq!(shards.len(), size.min(RELEASE_SHARDS));
            assert!(shards.iter().all(|shard| !shard.is_empty()));
        }
    }
}
