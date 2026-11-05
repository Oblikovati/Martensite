//! The registered factory, its place in the registry and the mesher it makes.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkmesh::brep_mesh::{
    BRepMeshDiscretAlgoFactory as _, BRepMeshDiscretRoot as _, BRepMeshIncrementalMesh,
    default_factory, factories, find_factory, unregister_factory,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tkxmesh::{XBREPMESH_FACTORY_NAME, XBRepMeshFactory};
use std::sync::Mutex;

/// The registry is process-wide; these tests read and change the same entry one at a time.
static REGISTRY_TURN: Mutex<()> = Mutex::new(());

fn box_shape() -> Shape {
    BRepPrimAPIMakeBox::from_sizes(10.0, 10.0, 10.0)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

#[test]
fn registers_by_name_without_case_behind_the_default() {
    let _turn = REGISTRY_TURN.lock().unwrap();
    let factory = XBRepMeshFactory::new();
    assert_eq!(factory.name(), XBREPMESH_FACTORY_NAME);
    assert_eq!(
        find_factory("xbrepmesh").get().map(|f| f.name()),
        Some("XBRepMesh")
    );
    assert_eq!(
        default_factory().get().map(|f| f.name()),
        Some("FastDiscret")
    );
}

#[test]
fn a_second_factory_of_the_name_does_not_register_twice() {
    let _turn = REGISTRY_TURN.lock().unwrap();
    let _first = XBRepMeshFactory::new();
    let _second = XBRepMeshFactory::new();
    let same_named = factories()
        .iter()
        .filter(|entry| {
            entry
                .get()
                .is_some_and(|f| f.name() == XBREPMESH_FACTORY_NAME)
        })
        .count();
    assert_eq!(same_named, 1);
}

#[test]
fn unregistering_removes_it_from_the_registry() {
    let _turn = REGISTRY_TURN.lock().unwrap();
    let _factory = XBRepMeshFactory::new();
    unregister_factory(XBREPMESH_FACTORY_NAME);
    assert!(find_factory(XBREPMESH_FACTORY_NAME).is_null());
}

#[test]
fn creates_an_unperformed_mesher_with_the_deflections_and_the_shape() {
    let factory = XBRepMeshFactory::new();
    let algorithm = factory.create_algorithm(&box_shape(), 0.1, 0.5);
    let mesher = algorithm.cast::<BRepMeshIncrementalMesh>();
    assert!(!mesher.is_null());
    assert!(!mesher.is_done());
    let parameters = mesher.parameters();
    assert_eq!(parameters.deflection, 0.1);
    assert_eq!(parameters.angle, 0.5);
    assert!(!parameters.in_parallel);
}

#[test]
fn the_created_mesher_meshes_a_box() {
    let factory = XBRepMeshFactory::new();
    let algorithm = factory.create_algorithm(&box_shape(), 0.1, 0.5);
    algorithm.perform(&ProgressRange::default()).unwrap();
    assert!(algorithm.is_done());
}
