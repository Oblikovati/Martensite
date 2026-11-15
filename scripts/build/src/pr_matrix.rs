//! The PR test matrix: one row per package part, longest first (#469, #470).

use std::collections::BTreeSet;

use crate::suite_weights::{package_minutes, pr_parts};

/// Rows for every part of every selected package. GitHub starts matrix jobs in
/// row order, so the longest measured parts come first and the run is not left
/// waiting on a long suite that started last (#469).
///
/// ```ignore
/// let matrix = pr_test_matrix(&selected);
/// ```
pub(crate) fn pr_test_matrix(packages: &BTreeSet<String>) -> serde_json::Value {
    let mut rows: Vec<_> = packages.iter().flat_map(|name| part_rows(name)).collect();
    // A stable sort keeps the sorted package order between equal estimates.
    rows.sort_by_key(|(minutes, _)| std::cmp::Reverse(*minutes));
    let include: Vec<_> = rows.into_iter().map(|(_, row)| row).collect();
    serde_json::json!({"include": include})
}

fn part_rows(package: &str) -> Vec<(u32, serde_json::Value)> {
    let parts = pr_parts(package);
    let minutes = package_minutes(package).div_ceil(parts as u32);
    (0..parts)
        .map(|part| {
            let row = serde_json::json!({
                "package": package, "part": part, "parts": parts,
                "label": part_label(package, part, parts),
            });
            (minutes, row)
        })
        .collect()
}

// Workflow expressions have no arithmetic, so the 1-based job label is built here.
fn part_label(package: &str, part: usize, parts: usize) -> String {
    if parts == 1 {
        return package.to_owned();
    }
    format!("{package} {}/{parts}", part + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows(packages: &[&str]) -> Vec<(String, u64, u64)> {
        let packages = packages.iter().map(|name| (*name).to_owned()).collect();
        let matrix = pr_test_matrix(&packages);
        let include = matrix["include"].as_array().unwrap().clone();
        include
            .into_iter()
            .map(|row| {
                let package = row["package"].as_str().unwrap().to_owned();
                (
                    package,
                    row["part"].as_u64().unwrap(),
                    row["parts"].as_u64().unwrap(),
                )
            })
            .collect()
    }

    #[test]
    fn longest_parts_come_first_and_ties_keep_sorted_order() {
        let actual = rows(&["martensite-a", "martensite-tkoffset", "martensite-b"]);
        let names: Vec<_> = actual.iter().map(|(name, _, _)| name.as_str()).collect();
        assert_eq!(
            names,
            ["martensite-tkoffset", "martensite-a", "martensite-b"]
        );
    }

    #[test]
    fn split_packages_emit_every_part_once() {
        let actual = rows(&["martensite-vulkan"]);
        let parts: Vec<_> = actual.iter().map(|(_, part, _)| *part).collect();
        assert_eq!(
            parts,
            (0..pr_parts("martensite-vulkan") as u64).collect::<Vec<_>>()
        );
        assert!(actual.iter().all(|(_, _, parts)| *parts == 4));
    }

    #[test]
    fn labels_name_split_parts_from_one_and_leave_whole_packages_plain() {
        assert_eq!(
            part_label("martensite-vulkan", 0, 4),
            "martensite-vulkan 1/4"
        );
        assert_eq!(part_label("martensite-tkmath", 0, 1), "martensite-tkmath");
    }

    #[test]
    fn empty_selection_yields_an_empty_matrix() {
        assert_eq!(
            pr_test_matrix(&BTreeSet::new()),
            serde_json::json!({"include": []})
        );
    }
}
