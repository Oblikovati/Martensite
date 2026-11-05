use super::policies;
use martensite_tkbo::bop_ds::{BOPDSIterator, BOPDSIteratorSI, DataStructure};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape, Vertex},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;

fn vertex_at(x: f64) -> Shape {
    let mut vertex = Vertex::default();
    BRepBuilder::default()
        .make_vertex_at(&mut vertex, Point3::new(x, 0.0, 0.0), 1.0e-7)
        .unwrap();
    vertex.into_shape()
}

pub(super) fn single_argument(distance: f64) -> DataStructure {
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    let mut shape = compound.into_shape();
    for x in [0.0, distance] {
        builder.add(&mut shape, &vertex_at(x)).unwrap();
    }
    let mut ds = DataStructure::default();
    ds.set_arguments(&[shape]);
    ds.init(1.0e-7).unwrap();
    assert_eq!((ds.nb_source_shapes(), ds.nb_ranges()), (3, 1));
    ds
}

fn assert_self_pair(iterator: &mut BOPDSIteratorSI) {
    iterator.initialize(ShapeEnum::Vertex, ShapeEnum::Vertex);
    assert_eq!(iterator.expected_length(), 1);
    assert!(iterator.more());
    assert_eq!(iterator.value().unwrap(), (1, 2));
    iterator.next();
    assert!(!iterator.more());
}

#[test]
fn direct_self_prepare_keeps_within_argument_pairs_while_base_filters_them() {
    let ds = single_argument(0.0);
    let mut base = BOPDSIterator::default();
    base.prepare(&ds).unwrap();
    base.initialize(ShapeEnum::Vertex, ShapeEnum::Vertex);
    assert_eq!(base.expected_length(), 0);
    let mut own_check = BOPDSIteratorSI::new();
    own_check.prepare(&ds).unwrap();
    assert_self_pair(&mut own_check);
}

#[test]
fn default_and_explicit_self_preparation_share_policy_and_level_reset_behavior() {
    let ds = single_argument(0.0);
    for policy in policies() {
        let mut own_check = BOPDSIteratorSI::with_allocator(policy.clone());
        own_check
            .prepare_with_context(&ds, None, false, 1.0e-7)
            .unwrap();
        assert_self_pair(&mut own_check);
        own_check.update_by_level_of_check(-1);
        own_check.initialize(ShapeEnum::Vertex, ShapeEnum::Vertex);
        assert_eq!(own_check.expected_length(), 0);
        own_check.prepare(&ds).unwrap();
        assert_self_pair(&mut own_check);
        assert_eq!(own_check.allocator(), policy.as_ref());
    }
}

#[test]
fn self_repreparation_replaces_candidates_for_separated_and_coincident_sources() {
    let together = single_argument(0.0);
    let apart = single_argument(10.0);
    let mut own_check = BOPDSIteratorSI::new();
    own_check.prepare(&together).unwrap();
    assert_self_pair(&mut own_check);
    own_check.prepare(&apart).unwrap();
    own_check.initialize(ShapeEnum::Vertex, ShapeEnum::Vertex);
    assert_eq!(own_check.expected_length(), 0);
    assert!(!own_check.more());
    own_check.prepare(&together).unwrap();
    assert_self_pair(&mut own_check);
}
