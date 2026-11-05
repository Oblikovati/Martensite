//! Explicit allocation-policy identity survives construction and clearing.
use martensite_tkbo::bop_algo::{BOPAlgoBuilder, BOPAlgoPaveFiller, BuilderArea, BuilderShape};
use martensite_tkernel::ncollection::BaseAllocator;

fn allocator_cases() -> [Option<BaseAllocator>; 3] {
    [
        None,
        Some(BaseAllocator::common()),
        Some(BaseAllocator::fresh_heap()),
    ]
}

#[test]
fn explicit_builder_shape_allocator_preserves_nullable_identity() {
    for allocator in allocator_cases() {
        let mut builder = BuilderShape::with_allocator(allocator.clone());
        assert_eq!(builder.options().allocator(), allocator.as_ref());
        assert!(builder.shape().is_null() && builder.has_history());
        assert!(!builder.has_modified() && !builder.has_generated() && !builder.has_deleted());
        builder.clear();
        assert_eq!(builder.options().allocator(), allocator.as_ref());
    }
}

#[test]
fn explicit_area_allocator_preserves_nullable_identity() {
    for allocator in allocator_cases() {
        let builder = BuilderArea::with_allocator(allocator.clone());
        assert_eq!(builder.options().allocator(), allocator.as_ref());
        assert!(builder.context().is_null() && builder.shapes().is_empty());
        assert!(builder.loops().is_empty() && builder.areas().is_empty());
        assert!(!builder.is_avoid_internal_shapes());
    }
}

#[test]
fn explicit_general_fuse_allocator_preserves_nullable_identity() {
    for allocator in allocator_cases() {
        let mut builder = BOPAlgoBuilder::with_allocator(allocator.clone());
        assert_eq!(builder.options().allocator(), allocator.as_ref());
        assert!(builder.arguments().is_empty() && builder.shape().is_null());
        assert!(builder.context().is_null() && builder.pave_filler().is_none());
        assert!(!builder.non_destructive() && builder.check_inverted());
        builder.clear();
        assert_eq!(builder.options().allocator(), allocator.as_ref());
    }
}

#[test]
fn explicit_intersection_allocator_preserves_nullable_identity() {
    for allocator in allocator_cases() {
        let mut filler = BOPAlgoPaveFiller::with_allocator(allocator.clone());
        assert_eq!(filler.options().allocator(), allocator.as_ref());
        assert!(filler.arguments().is_empty() && filler.data_structure().is_none());
        assert!(filler.context().is_none() && filler.iterator().is_none());
        assert!(filler.is_primary() && !filler.non_destructive());
        assert!(!filler.is_avoid_build_pcurve());
        filler.clear();
        assert_eq!(filler.options().allocator(), allocator.as_ref());
    }
}

#[test]
fn default_constructors_retain_the_common_policy() {
    let common = BaseAllocator::common();
    assert_eq!(BuilderShape::new().options().allocator(), Some(&common));
    assert_eq!(BuilderArea::new().options().allocator(), Some(&common));
    assert_eq!(BOPAlgoBuilder::new().options().allocator(), Some(&common));
    assert_eq!(
        BOPAlgoPaveFiller::new().options().allocator(),
        Some(&common)
    );
}
