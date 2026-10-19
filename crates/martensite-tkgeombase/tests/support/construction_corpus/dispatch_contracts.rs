//! The original family inventory is independent of the executable dispatch table.
use std::collections::BTreeSet;

const ORIGINAL_NAMES: &str = include_str!("../../fixtures/construction_corpus/operation-names.txt");

#[test]
fn numeric_operation_names_are_unique_and_complete() {
    let expected: Vec<_> = ORIGINAL_NAMES.lines().collect();
    let unique: BTreeSet<_> = expected.iter().copied().collect();
    assert_eq!(expected.len(), 328);
    assert_eq!(unique.len(), 328);
    let ordinary: BTreeSet<_> = unique
        .into_iter()
        .filter(|name| !name.starts_with("gt_"))
        .collect();
    let registered: BTreeSet<_> = super::OPERATIONS.iter().map(|(name, _)| *name).collect();
    assert_eq!(super::OPERATIONS.len(), 317);
    assert_eq!(registered.len(), super::OPERATIONS.len());
    assert_eq!(registered, ordinary);
}

#[test]
fn geometry_stream_names_resolve_through_the_actual_dispatcher() {
    let names: Vec<_> = ORIGINAL_NAMES
        .lines()
        .filter(|name| name.starts_with("gt_"))
        .collect();
    assert_eq!(names.len(), 11);
    for name in names {
        assert!(
            super::super::geom_tools::supports_operation(name),
            "missing {name}"
        );
    }
    assert!(!super::super::geom_tools::supports_operation(
        "gt_not_registered"
    ));
}

#[test]
fn unknown_operations_do_not_return_successful_empty_output() {
    for name in [
        "",
        "not_registered",
        "gt_not_registered",
        "apc_not_registered",
    ] {
        assert_eq!(super::run_values(name, &[]), None, "unknown {name:?}");
    }
}
