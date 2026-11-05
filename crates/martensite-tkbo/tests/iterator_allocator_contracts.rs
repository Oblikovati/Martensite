//! Candidate iterators retain nullable operation policy through preparation.
use martensite_tkbo::bop_algo::BOPAlgoPaveFiller;
use martensite_tkbo::bop_ds::{BOPDSIterator, BOPDSIteratorSI, DataStructure};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Shape, Vertex},
};
use martensite_tkernel::ncollection::BaseAllocator;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;

fn policies() -> [Option<BaseAllocator>; 3] {
    [
        None,
        Some(BaseAllocator::common()),
        Some(BaseAllocator::fresh_heap()),
    ]
}

fn vertices() -> [Shape; 2] {
    [0, 1].map(|_| {
        let mut vertex = Vertex::default();
        BRepBuilder::default()
            .make_vertex_at(&mut vertex, Point3::default(), 1.0e-7)
            .unwrap();
        vertex.into_shape()
    })
}

#[test]
fn iterator_constructors_keep_default_or_explicit_nullable_policy() {
    assert_eq!(
        BOPDSIterator::default().allocator(),
        Some(&BaseAllocator::common())
    );
    assert_eq!(
        BOPDSIteratorSI::new().allocator(),
        Some(&BaseAllocator::common())
    );
    for policy in policies() {
        let iterator = BOPDSIterator::with_allocator(policy.clone());
        assert_eq!(iterator.allocator(), policy.as_ref());
        assert_eq!(
            (iterator.expected_length(), iterator.block_length()),
            (0, 1)
        );
        assert!(!iterator.run_parallel());
        let own_check = BOPDSIteratorSI::with_allocator(policy.clone());
        assert_eq!(own_check.allocator(), policy.as_ref());
        assert_eq!(
            (own_check.expected_length(), own_check.block_length()),
            (0, 1)
        );
        assert!(!own_check.run_parallel());
    }
}

#[test]
fn preparation_and_si_conversion_keep_selected_policy_and_pairs() {
    for policy in policies() {
        let mut ds = DataStructure::default();
        ds.set_arguments(&vertices());
        ds.init(1.0e-7).unwrap();
        let mut iterator = BOPDSIterator::with_allocator(policy.clone());
        iterator.prepare(&ds).unwrap();
        iterator.initialize(ShapeEnum::Vertex, ShapeEnum::Vertex);
        assert_eq!(iterator.value().unwrap(), (0, 1));
        assert_eq!(iterator.allocator(), policy.as_ref());
        let own_check = BOPDSIteratorSI::with_allocator(policy.clone()).into_iterator();
        assert_eq!(own_check.allocator(), policy.as_ref());
    }
}

#[test]
fn filler_preparation_reinitialization_and_rejection_keep_iterator_policy() {
    for policy in policies() {
        let mut filler = BOPAlgoPaveFiller::with_allocator(policy.clone());
        for shape in vertices() {
            filler.add_argument(&shape);
        }
        filler.initialize().unwrap();
        assert_eq!(filler.iterator().unwrap().allocator(), policy.as_ref());
        filler.initialize().unwrap();
        assert_eq!(filler.iterator().unwrap().allocator(), policy.as_ref());
        filler.add_argument(&Shape::default());
        filler.initialize().unwrap();
        assert!(filler.options().has_errors());
        assert_eq!(filler.iterator().unwrap().allocator(), policy.as_ref());
        filler.clear();
        assert!(filler.iterator().is_none());
    }
}

fn prepared_vertices() -> DataStructure {
    let mut ds = DataStructure::default();
    ds.set_arguments(&vertices());
    ds.init(1.0e-7).unwrap();
    ds
}

fn prepared_vertex_pair(ds: &DataStructure) -> BOPDSIterator {
    let mut iterator = BOPDSIterator::default();
    iterator.prepare(ds).unwrap();
    iterator.initialize(ShapeEnum::Vertex, ShapeEnum::Vertex);
    iterator
}

#[test]
fn live_source_value_tracks_mutation_without_changing_cached_value() {
    let mut ds = prepared_vertices();
    let iterator = prepared_vertex_pair(&ds);
    assert_eq!(iterator.value_with_structure(&ds).unwrap(), (0, 1));
    ds.change_shape_info(0)
        .unwrap()
        .set_shape_type(ShapeEnum::Edge);
    assert_eq!(iterator.value_with_structure(&ds).unwrap(), (1, 0));
    assert_eq!(iterator.value().unwrap(), (0, 1));
    assert_eq!(iterator.expected_length(), 1);
}

#[test]
fn live_source_value_queries_alternate_source_without_repreparing() {
    let original = prepared_vertices();
    let iterator = prepared_vertex_pair(&original);
    let mut alternate = prepared_vertices();
    alternate
        .change_shape_info(0)
        .unwrap()
        .set_shape_type(ShapeEnum::Edge);
    assert_eq!(iterator.value_with_structure(&alternate).unwrap(), (1, 0));
    assert_eq!(iterator.value_with_structure(&original).unwrap(), (0, 1));
    assert_eq!(iterator.value_with_structure(&alternate).unwrap(), (1, 0));
    assert_eq!(iterator.value().unwrap(), (0, 1));
}

#[test]
fn live_source_value_reports_missing_records_and_exhaustion_without_consumption() {
    let ds = prepared_vertices();
    let mut iterator = prepared_vertex_pair(&ds);
    assert!(
        iterator
            .value_with_structure(&DataStructure::default())
            .is_err()
    );
    assert!(iterator.more());
    assert_eq!(iterator.value_with_structure(&ds).unwrap(), (0, 1));
    iterator.next();
    assert!(iterator.value_with_structure(&ds).is_err());
    assert!(iterator.value().is_err());
    assert!(!iterator.more());
}

#[test]
fn live_source_value_orients_all_source_type_pairs() {
    let mut ds = prepared_vertices();
    let iterator = prepared_vertex_pair(&ds);
    for (first_rank, first_type) in ordered_shape_types().into_iter().enumerate() {
        ds.change_shape_info(0).unwrap().set_shape_type(first_type);
        for (second_rank, second_type) in ordered_shape_types().into_iter().enumerate() {
            ds.change_shape_info(1).unwrap().set_shape_type(second_type);
            let expected = if first_rank < second_rank {
                (1, 0)
            } else {
                (0, 1)
            };
            assert_eq!(iterator.value_with_structure(&ds).unwrap(), expected);
            assert_eq!(iterator.value().unwrap(), (0, 1));
        }
    }
}

fn ordered_shape_types() -> [ShapeEnum; 9] {
    [
        ShapeEnum::Compound,
        ShapeEnum::CompSolid,
        ShapeEnum::Solid,
        ShapeEnum::Shell,
        ShapeEnum::Face,
        ShapeEnum::Wire,
        ShapeEnum::Edge,
        ShapeEnum::Vertex,
        ShapeEnum::Shape,
    ]
}

#[path = "iterator_allocator_contracts/subset_contracts.rs"]
mod subset_contracts;

#[path = "iterator_allocator_contracts/borrowed_contracts.rs"]
mod borrowed_contracts;

#[path = "iterator_allocator_contracts/self_prepare_contracts.rs"]
mod self_prepare_contracts;

#[path = "iterator_allocator_contracts/projection_contracts.rs"]
mod projection_contracts;
