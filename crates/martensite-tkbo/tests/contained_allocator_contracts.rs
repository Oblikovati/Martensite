//! Contained allocation-policy metadata follows native normalization and assignment.
use martensite_tkbo::bop_algo::{
    BOPAlgoBuilder, BOPAlgoPaveFiller, BOPAlgoToolsProvider, BuilderArea, ShellSplitter,
};
use martensite_tkernel::ncollection::BaseAllocator;
#[path = "contained_allocator_contracts/support.rs"]
mod support;
use support::*;

#[test]
fn builder_collections_keep_policy_after_clear_and_argument_replacement() {
    for policy in policy_cases() {
        let expected = normalized(&policy);
        let mut builder = BOPAlgoBuilder::with_allocator(policy);
        assert_builder_metadata(&builder, &expected);
        builder.set_arguments(&source_shapes());
        assert_builder_metadata(&builder, &expected);
        assert_eq!(builder.arguments().extent(), 1);
        builder.clear();
        assert_builder_metadata(&builder, &expected);
        assert!(builder.arguments().is_empty() && builder.images().is_empty());
    }
}

#[test]
fn area_constructor_lists_keep_explicit_or_normalized_policy() {
    for policy in policy_cases() {
        let expected = normalized(&policy);
        let area = BuilderArea::with_allocator(policy);
        assert_area_metadata(&area, &expected);
    }
}

#[test]
fn area_assignment_keeps_default_destination_policy_and_copies_values() {
    let mut area = BuilderArea::new();
    let shapes = source_shapes();
    area.set_shapes(&shapes);
    assert_area_metadata(&area, &BaseAllocator::common());
    assert_eq!(area.shapes().extent(), 1);
    assert!(
        area.shapes()
            .first()
            .unwrap()
            .is_same(shapes.first().unwrap())
    );
    area.set_shapes(&martensite_tkernel::ncollection::List::default());
    assert!(area.shapes().is_empty());
    assert_area_metadata(&area, &BaseAllocator::common());
}

#[test]
fn filler_assignment_keeps_common_argument_policy_and_copies_values() {
    let mut filler = BOPAlgoPaveFiller::new();
    let shapes = source_shapes();
    filler.set_arguments(&shapes);
    assert_eq!(filler.arguments().allocator(), &BaseAllocator::common());
    assert_eq!(filler.arguments().extent(), 1);
    assert!(
        filler
            .arguments()
            .first()
            .unwrap()
            .is_same(shapes.first().unwrap())
    );
    filler.clear();
    assert_eq!(filler.arguments().allocator(), &BaseAllocator::common());
    assert_eq!(filler.arguments().extent(), 1);
}

#[test]
fn provider_and_shell_lists_keep_normalized_identity_after_reset() {
    for policy in policy_cases() {
        let expected = normalized(&policy);
        let mut provider = BOPAlgoToolsProvider::with_allocator(policy.clone());
        provider.set_tools(&source_shapes());
        assert_eq!(provider.tools().allocator(), &expected);
        provider.clear();
        assert_eq!(provider.tools().allocator(), &expected);
        let mut shell = ShellSplitter::with_allocator(policy);
        assert_eq!(shell.start_elements().allocator(), &expected);
        assert_eq!(shell.shells().allocator(), &expected);
        shell.perform().unwrap();
        assert_eq!(shell.shells().allocator(), &expected);
    }
}

#[test]
fn filler_map_uses_explicit_policy_while_arguments_stay_common() {
    for policy in policy_cases() {
        let expected = normalized(&policy);
        let mut filler = BOPAlgoPaveFiller::with_allocator(policy);
        assert_eq!(filler.increased_ss().allocator(), &expected);
        assert_eq!(filler.arguments().allocator(), &BaseAllocator::common());
        filler.clear();
        assert_eq!(filler.increased_ss().allocator(), &expected);
    }
}

#[test]
fn inherited_builders_keep_base_collection_identity() {
    for policy in policy_cases() {
        assert_general_fuse_inheritance(&policy);
        assert_area_inheritance(&policy);
    }
}
