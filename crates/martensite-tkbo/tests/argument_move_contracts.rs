//! Move input transfers list nodes and allocator while preserving independent headers.
use martensite_tkbo::bop_algo::BOPAlgoPaveFiller;
use martensite_tkernel::ncollection::{BaseAllocator, List};
#[path = "argument_move_contracts/support.rs"]
mod support;
use support::*;

#[test]
fn moved_arguments_keep_allocator_and_nodes_and_empty_source() {
    let resource = BaseAllocator::fresh_heap();
    let mut source = two_shapes(&resource);
    let first = std::ptr::from_ref(source.first().unwrap());
    let last = std::ptr::from_ref(source.last().unwrap());
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments_moved(&mut source);
    assert!(source.is_empty());
    assert_eq!(source.allocator(), &resource);
    assert_eq!(filler.arguments().allocator(), &resource);
    assert_eq!(filler.arguments().extent(), 2);
    assert!(std::ptr::eq(filler.arguments().first().unwrap(), first));
    assert!(std::ptr::eq(filler.arguments().last().unwrap(), last));
}

#[test]
fn empty_move_adopts_policy_and_discards_prior_arguments() {
    let resource = BaseAllocator::fresh_heap();
    let mut source = List::with_allocator(resource.clone());
    let mut filler = BOPAlgoPaveFiller::new();
    filler.add_argument(&compound_shape());
    filler.set_arguments_moved(&mut source);
    assert!(source.is_empty() && filler.arguments().is_empty());
    assert_eq!(filler.arguments().allocator(), &resource);
    assert_eq!(source.allocator(), &resource);
}

#[test]
fn move_preserves_options_and_clear_retains_adopted_arguments() {
    let options = BaseAllocator::fresh_heap();
    let resource = BaseAllocator::fresh_heap();
    let mut filler = BOPAlgoPaveFiller::with_allocator(Some(options.clone()));
    let mut source = two_shapes(&resource);
    filler.set_arguments_moved(&mut source);
    filler.clear();
    assert_eq!(filler.options().allocator(), Some(&options));
    assert_eq!(filler.arguments().allocator(), &resource);
    assert_eq!(filler.arguments().extent(), 2);
    assert_eq!(filler.increased_ss().allocator(), &options);
}

#[test]
fn source_header_is_reusable_without_changing_destination() {
    let resource = BaseAllocator::fresh_heap();
    let mut source = two_shapes(&resource);
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments_moved(&mut source);
    let replacement = compound_shape();
    source.append(replacement.clone());
    assert_eq!(source.allocator(), &resource);
    assert_eq!((source.extent(), filler.arguments().extent()), (1, 2));
    assert!(source.first().unwrap().is_same(&replacement));
    assert!(!filler.arguments().first().unwrap().is_same(&replacement));
}

#[test]
fn borrowed_assignment_after_move_keeps_the_adopted_policy() {
    let resource = BaseAllocator::fresh_heap();
    let mut source = two_shapes(&resource);
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments_moved(&mut source);
    let replacement = two_shapes(&BaseAllocator::common());
    filler.set_arguments(&replacement);
    assert_eq!(filler.arguments().allocator(), &resource);
    assert_eq!(filler.arguments().extent(), 2);
    assert!(
        filler
            .arguments()
            .first()
            .unwrap()
            .is_same(replacement.first().unwrap())
    );
}

#[test]
fn repeated_moves_adopt_each_source_policy_and_keep_shape_order() {
    let mut filler = BOPAlgoPaveFiller::new();
    for resource in [
        BaseAllocator::common(),
        BaseAllocator::fresh_heap(),
        BaseAllocator::fresh_heap(),
    ] {
        let mut source = two_shapes(&resource);
        let expected = source.iter().cloned().collect::<Vec<_>>();
        filler.set_arguments_moved(&mut source);
        assert!(source.is_empty());
        assert_eq!(filler.arguments().allocator(), &resource);
        assert_shape_order(filler.arguments(), &expected);
    }
}
