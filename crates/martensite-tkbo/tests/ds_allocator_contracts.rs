//! Shape storage retains operation-local allocation policy across its lifecycle.
use martensite_tkbo::bop_algo::BOPAlgoPaveFiller;
use martensite_tkbo::bop_ds::DataStructure;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Shape, Vertex};
use martensite_tkernel::ncollection::BaseAllocator;
use martensite_tkmath::gp::Point3;

fn vertex() -> Shape {
    let mut vertex = Vertex::default();
    BRepBuilder::default()
        .make_vertex_at(&mut vertex, Point3::default(), 1.0e-7)
        .unwrap();
    vertex.into_shape()
}

fn policies() -> [Option<BaseAllocator>; 3] {
    [
        None,
        Some(BaseAllocator::common()),
        Some(BaseAllocator::fresh_heap()),
    ]
}

fn assert_policy(ds: &mut DataStructure, policy: &Option<BaseAllocator>) {
    assert_eq!(ds.allocator(), policy.as_ref());
    let normalized = policy.clone().unwrap_or_else(BaseAllocator::common);
    assert_eq!(ds.interferences().allocator(), &normalized);
    assert_eq!(ds.shapes_sd().allocator(), &normalized);
    assert_array_policy(ds, policy);
}

fn assert_array_policy(ds: &mut DataStructure, policy: &Option<BaseAllocator>) {
    assert_eq!(ds.interf_vv().allocator(), policy.as_ref());
    assert_eq!(ds.interf_ve().allocator(), policy.as_ref());
    assert_eq!(ds.interf_vf().allocator(), policy.as_ref());
    assert_eq!(ds.interf_ee().allocator(), policy.as_ref());
    assert_eq!(ds.interf_ef().allocator(), policy.as_ref());
    assert_eq!(ds.interf_ff().allocator(), policy.as_ref());
    assert_eq!(ds.interf_vz().allocator(), policy.as_ref());
    assert_eq!(ds.interf_ez().allocator(), policy.as_ref());
    assert_eq!(ds.interf_fz().allocator(), policy.as_ref());
    assert_eq!(ds.interf_zz().allocator(), policy.as_ref());
}

#[test]
fn constructor_preserves_nullable_policy_and_normalizes_map_policy() {
    for policy in policies() {
        let mut ds = DataStructure::with_allocator(policy.clone());
        assert_eq!(ds.interferences().bucket_count(), 100);
        assert_eq!(ds.shapes_sd().bucket_count(), 100);
        assert_policy(&mut ds, &policy);
        assert_eq!(
            (ds.nb_shapes(), ds.nb_source_shapes(), ds.nb_ranges()),
            (0, 0, 0)
        );
    }
    assert_policy(
        &mut DataStructure::default(),
        &Some(BaseAllocator::common()),
    );
}

#[test]
fn clear_and_valid_reinitialization_preserve_policy() {
    for policy in policies() {
        let mut ds = DataStructure::with_allocator(policy.clone());
        ds.set_arguments(&[vertex()]);
        ds.init(1.0e-7).unwrap();
        assert_eq!((ds.nb_shapes(), ds.nb_source_shapes()), (1, 1));
        ds.add_shape_sd(2, 3);
        ds.add_interference(2, 3);
        ds.clear();
        assert!(ds.arguments().is_empty() && ds.interferences().is_empty());
        assert!(ds.shapes_sd().is_empty());
        assert_policy(&mut ds, &policy);
        ds.set_arguments(&[vertex()]);
        ds.init(1.0e-7).unwrap();
        assert_eq!((ds.nb_shapes(), ds.nb_source_shapes()), (1, 1));
        assert_policy(&mut ds, &policy);
    }
}

#[test]
fn filler_initialization_and_reinitialization_propagate_options_policy() {
    for policy in policies() {
        let mut filler = BOPAlgoPaveFiller::with_allocator(policy.clone());
        for _ in 0..2 {
            filler.add_argument(&vertex());
            filler.initialize().unwrap();
            assert_policy(filler.data_structure_mut().unwrap(), &policy);
        }
        filler.clear();
        assert!(filler.data_structure().is_none());
        filler.initialize().unwrap();
        assert_policy(filler.data_structure_mut().unwrap(), &policy);
    }
}

#[test]
fn rejected_null_argument_keeps_prepared_storage_and_policy() {
    for policy in policies() {
        let mut filler = BOPAlgoPaveFiller::with_allocator(policy.clone());
        filler.add_argument(&vertex());
        filler.initialize().unwrap();
        filler.data_structure_mut().unwrap().add_shape_sd(7, 9);
        filler.add_argument(&Shape::default());
        filler.initialize().unwrap();
        assert!(filler.options().has_errors());
        let ds = filler.data_structure_mut().unwrap();
        assert_eq!(ds.shape_sd(7), Some(9));
        assert_eq!(ds.nb_source_shapes(), 1);
        assert_policy(ds, &policy);
    }
}
