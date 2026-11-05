use super::{prepared_vertex_pair, prepared_vertices};
use martensite_tkbo::bop_ds::{BOPDSIterator, BorrowedIterator, DataStructure};
use martensite_tkg3d::top_abs::ShapeEnum;

#[test]
fn binding_keeps_exact_source_identity_and_exclusive_mutation() {
    let mut ds = prepared_vertices();
    let identity = std::ptr::from_ref(&ds);
    let mut iterator = BOPDSIterator::default();
    let mut binding = iterator.bind_structure(&mut ds);
    assert!(std::ptr::eq(binding.structure(), identity));
    binding.structure_mut().add_shape_sd(3, 7);
    assert_eq!(binding.structure().shape_sd(3), Some(7));
}

#[test]
fn replacement_returns_prior_source_borrow_and_keeps_iterator_pair_state() {
    let mut original = prepared_vertices();
    let mut alternate = prepared_vertices();
    let identity = std::ptr::from_ref(&original);
    let alternate_identity = std::ptr::from_ref(&alternate);
    let mut iterator = prepared_vertex_pair(&original);
    let mut binding = iterator.bind_structure(&mut original);
    assert_eq!(binding.expected_length(), 1);
    let previous = binding.replace_structure(&mut alternate);
    assert_eq!(binding.expected_length(), 1);
    assert!(std::ptr::eq(previous, identity));
    assign_type(previous, ShapeEnum::Edge);
    assert!(std::ptr::eq(binding.structure(), alternate_identity));
    assert_eq!(
        binding.structure().shape_info(0).unwrap().shape_type(),
        ShapeEnum::Vertex
    );
}

static_assertions::assert_impl_all!(BorrowedIterator<'static>: Send, Sync);

#[test]
fn bound_value_observes_mutation_and_rebinding_without_repreparing() {
    let mut original = prepared_vertices();
    let mut alternate = prepared_vertices();
    let mut iterator = prepared_vertex_pair(&original);
    let mut binding = iterator.bind_structure(&mut original);
    assert_eq!(binding.value().unwrap(), (0, 1));
    assign_type(binding.structure_mut(), ShapeEnum::Edge);
    assert_eq!(binding.value().unwrap(), (1, 0));
    let previous = binding.replace_structure(&mut alternate);
    assign_type(previous, ShapeEnum::Face);
    assert_eq!(binding.value().unwrap(), (0, 1));
    let released = binding.replace_structure(previous);
    assert_eq!(
        released.shape_info(0).unwrap().shape_type(),
        ShapeEnum::Vertex
    );
    assert_eq!(binding.value().unwrap(), (1, 0));
}

fn assert_bound_pair(binding: &BorrowedIterator<'_>) {
    assert_eq!(binding.expected_length(), 1);
    assert!(binding.more());
    assert_eq!(binding.value().unwrap(), (0, 1));
}

#[test]
fn bound_preparation_and_cursor_forward_existing_candidate_operations() {
    let mut ds = prepared_vertices();
    let mut iterator = BOPDSIterator::default();
    let mut binding = iterator.bind_structure(&mut ds);
    binding.prepare().unwrap();
    binding.initialize(ShapeEnum::Vertex, ShapeEnum::Vertex);
    assert_bound_pair(&binding);
    binding.next();
    assert!(!binding.more());
    assert!(binding.value().is_err());
    binding.prepare_with_context(None, false, 1.0e-7).unwrap();
    binding.initialize(ShapeEnum::Vertex, ShapeEnum::Vertex);
    assert_bound_pair(&binding);
}

#[test]
fn replacing_source_preserves_an_exhausted_cursor_and_pair_count() {
    let mut original = prepared_vertices();
    let mut alternate = prepared_vertices();
    let mut iterator = prepared_vertex_pair(&original);
    iterator.next();
    let mut binding = iterator.bind_structure(&mut original);
    assert!(!binding.more());
    let _previous = binding.replace_structure(&mut alternate);
    assert!(!binding.more());
    assert_eq!(binding.expected_length(), 1);
    binding.prepare().unwrap();
    binding.initialize(ShapeEnum::Vertex, ShapeEnum::Vertex);
    assert_bound_pair(&binding);
}

#[test]
fn ending_binding_releases_both_owners_without_rewriting_cached_types() {
    let mut ds = prepared_vertices();
    let mut iterator = prepared_vertex_pair(&ds);
    {
        let mut binding = iterator.bind_structure(&mut ds);
        assign_type(binding.structure_mut(), ShapeEnum::Edge);
        assert_eq!(binding.value().unwrap(), (1, 0));
    }
    ds.change_shape_info(0)
        .unwrap()
        .set_shape_type(ShapeEnum::Face);
    assert_eq!(iterator.value().unwrap(), (0, 1));
    assert_eq!(iterator.value_with_structure(&ds).unwrap(), (1, 0));
    iterator.set_run_parallel(true);
    assert!(iterator.run_parallel());
}

fn assign_type(source: &mut DataStructure, kind: ShapeEnum) {
    source.change_shape_info(0).unwrap().set_shape_type(kind);
}
