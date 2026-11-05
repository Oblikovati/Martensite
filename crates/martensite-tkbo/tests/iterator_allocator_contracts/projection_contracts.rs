use super::{policies, self_prepare_contracts::single_argument};
use martensite_tkbo::bop_algo::BOPAlgoCheckerSI;
use martensite_tkbo::bop_ds::{BOPDSIterator, BOPDSIteratorSI};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::bnd::BoundingBox3;

fn assert_projected_pair(iterator: &mut BOPDSIterator) {
    iterator.initialize(ShapeEnum::Vertex, ShapeEnum::Vertex);
    assert_eq!(iterator.expected_length(), 1);
    assert!(iterator.more());
    assert_eq!(iterator.value().unwrap(), (1, 2));
    iterator.next();
    assert!(!iterator.more());
}

#[test]
fn base_reference_preserves_self_interference_preparation_and_resources() {
    let source = single_argument(0.0);
    let mut default = BOPDSIteratorSI::new();
    let projection: &mut BOPDSIterator = &mut default;
    projection.prepare(&source).unwrap();
    assert_projected_pair(projection);
    for policy in policies() {
        let mut own = BOPDSIteratorSI::with_allocator(policy.clone());
        let projection: &mut BOPDSIterator = &mut own;
        projection
            .prepare_with_context(&source, None, false, 1.0e-7)
            .unwrap();
        assert_projected_pair(projection);
        assert_eq!(projection.allocator(), policy.as_ref());
    }
}

#[test]
fn owned_upcast_retains_self_interference_kind_on_repreparation() {
    let source = single_argument(0.0);
    let mut projection = BOPDSIteratorSI::new().into_iterator();
    projection.prepare(&source).unwrap();
    assert_projected_pair(&mut projection);
    projection.prepare(&single_argument(10.0)).unwrap();
    projection.initialize(ShapeEnum::Vertex, ShapeEnum::Vertex);
    assert_eq!(projection.expected_length(), 0);
    assert!(!projection.more());
    projection.prepare(&source).unwrap();
    assert_projected_pair(&mut projection);
}

#[test]
fn borrowed_self_projection_rebinds_without_changing_preparation_kind() {
    let mut together = single_argument(0.0);
    let mut apart = single_argument(10.0);
    let mut own = BOPDSIteratorSI::new();
    let mut view = own.bind_structure(&mut together);
    view.prepare().unwrap();
    view.initialize(ShapeEnum::Vertex, ShapeEnum::Vertex);
    assert_eq!(view.expected_length(), 1);
    assert_eq!(view.value().unwrap(), (1, 2));
    view.next();
    assert!(!view.more());
    let previous = view.replace_structure(&mut apart);
    view.prepare().unwrap();
    view.initialize(ShapeEnum::Vertex, ShapeEnum::Vertex);
    assert_eq!(view.expected_length(), 0);
    view.replace_structure(previous);
    view.prepare().unwrap();
    view.initialize(ShapeEnum::Vertex, ShapeEnum::Vertex);
    assert_eq!(view.value().unwrap(), (1, 2));
}

#[test]
fn checker_installed_iterator_keeps_self_kind_when_prepared_again() {
    let source = single_argument(0.0);
    let mut checker = BOPAlgoCheckerSI::new();
    checker.add_argument(&source.arguments()[0]);
    checker.perform();
    let projection = checker
        .iterator_mut()
        .expect("checker installs its self iterator");
    projection.prepare(&source).unwrap();
    assert_projected_pair(projection);
}

#[test]
fn projected_self_preparation_void_box_failure_clears_pairs_and_recovers() {
    let mut source = single_argument(0.0);
    let saved = source.shape_info(1).unwrap().box_bounds().clone();
    let mut projection = BOPDSIteratorSI::new().into_iterator();
    projection.prepare(&source).unwrap();
    assert_projected_pair(&mut projection);
    source
        .change_shape_info(1)
        .unwrap()
        .set_box(&BoundingBox3::default());
    let error = projection.prepare(&source).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_ConstructionError");
    assert_eq!(error.message(), "Bnd_Box is void");
    projection.initialize(ShapeEnum::Vertex, ShapeEnum::Vertex);
    assert_eq!(projection.expected_length(), 0);
    assert!(!projection.more());
    source.change_shape_info(1).unwrap().set_box(&saved);
    projection.prepare(&source).unwrap();
    assert_projected_pair(&mut projection);
}
