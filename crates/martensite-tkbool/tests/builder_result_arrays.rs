//! Native array identity/bounds cases and explicit Rust lease-conflict behavior.
use martensite_tkbool::top_ope_brep_build::{
    BuilderResultArray, BuilderResultRead, BuilderResultWrite, TopOpeBRepBuildBuilder,
};
use martensite_tkbool::top_ope_brep_ds::{
    TopOpeBRepDSBuildTool, TopOpeBRepDSHDataStructure, TopOpeBRepDSPoint,
};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Shape};
use martensite_tkernel::ncollection::{Array1, List};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;

#[path = "builder_result_arrays/leases.rs"]
mod leases;

fn builder() -> TopOpeBRepBuildBuilder {
    TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new())
}
fn shape() -> Shape {
    let mut compound = Compound::default();
    BRepBuilder::default().make_compound(&mut compound);
    compound.into_shape()
}

#[test]
fn default_handles_are_null_but_an_allocated_empty_array_is_not() {
    let owner = builder();
    assert!(owner.extension_state().created_vertices().is_null());
    assert!(owner.extension_state().created_faces().is_null());
    assert_eq!(
        owner.new_vertex(0).unwrap_err().exception_type(),
        "Standard_OutOfRange"
    );
    assert_eq!(
        owner
            .new_faces(0)
            .err()
            .expect("null result array")
            .exception_type(),
        "Standard_OutOfRange"
    );
}

#[test]
fn empty_storage_is_distinct_from_null_storage() {
    let empty = BuilderResultArray::<Shape>::from_array(Array1::default());
    assert!(!empty.is_null());
    assert!(empty.read_array().unwrap().is_empty());
    assert!(
        BuilderResultArray::<Shape>::default().shares_storage_with(&BuilderResultArray::default())
    );
}

#[test]
fn vertex_handle_assignment_shares_negative_bounds_and_value_mutation() {
    let mut first = builder();
    let mut second = builder();
    *first.extension_state_mut().created_vertices_mut() =
        BuilderResultArray::with_bounds(-3, -1).unwrap();
    *second.extension_state_mut().created_vertices_mut() =
        first.extension_state().created_vertices().clone();
    let retained = shape();
    *first.change_new_vertex(-2).unwrap() = retained.clone();
    assert!(second.new_vertex(-2).unwrap().is_same(&retained));
    assert_vertex_bounds(&first, -3, -1);
    second.extension_state_mut().created_vertices_mut().clear();
    assert!(second.extension_state().created_vertices().is_null());
    assert!(!first.extension_state().created_vertices().is_null());
    assert!(first.new_vertex(-2).unwrap().is_same(&retained));
    assert!(first.new_vertex(-4).is_err());
    assert!(first.change_new_vertex(0).is_err());
}

fn assert_vertex_bounds(owner: &TopOpeBRepBuildBuilder, lower: i32, upper: i32) {
    let state = owner.extension_state();
    let values = state.created_vertices().read_array().unwrap();
    assert_eq!(values.lower(), lower);
    assert_eq!(values.upper(), upper);
}

#[test]
fn face_list_handle_assignment_shares_positive_bounds_and_list_nodes() {
    let mut first = builder();
    let mut second = builder();
    *first.extension_state_mut().created_faces_mut() =
        BuilderResultArray::with_bounds(4, 6).unwrap();
    *second.extension_state_mut().created_faces_mut() =
        first.extension_state().created_faces().clone();
    let retained = shape();
    first.change_new_faces(5).unwrap().append(retained.clone());
    assert_face_value(&second, 5, &retained);
    assert_face_bounds(&first, 4, 6);
    second.extension_state_mut().created_faces_mut().clear();
    assert!(second.extension_state().created_faces().is_null());
    assert!(!first.extension_state().created_faces().is_null());
    assert_eq!(first.new_faces(5).unwrap().extent(), 1);
    assert!(first.new_faces(3).is_err());
    assert!(first.change_new_faces(7).is_err());
}

fn assert_face_value(owner: &TopOpeBRepBuildBuilder, index: i32, expected: &Shape) {
    let values = owner.new_faces(index).unwrap();
    assert!(values.first().unwrap().is_same(expected));
}

fn assert_face_bounds(owner: &TopOpeBRepBuildBuilder, lower: i32, upper: i32) {
    let state = owner.extension_state();
    let values = state.created_faces().read_array().unwrap();
    assert_eq!(values.lower(), lower);
    assert_eq!(values.upper(), upper);
}

#[test]
fn ordinary_vertex_build_replaces_only_its_array_and_retains_zero_bounds() {
    let mut owner = builder();
    let old = BuilderResultArray::with_bounds(-2, 0).unwrap();
    *owner.extension_state_mut().created_vertices_mut() = old.clone();
    let hds = Handle::new(TopOpeBRepDSHDataStructure::new());
    let point = TopOpeBRepDSPoint::new(Point3::new(1., 2., 3.), 1e-7);
    hds.get().unwrap().change_ds().add_point(&point);
    owner.build_vertices(&hds).unwrap();
    assert_vertex_bounds(&owner, 0, 1);
    assert!(!owner.new_vertex(1).unwrap().is_null());
    assert_eq!(old.read_array().unwrap().lower(), -2);
    assert!(
        !owner
            .extension_state()
            .created_vertices()
            .shares_storage_with(&old)
    );
}

#[test]
fn null_and_invalid_range_failures_are_explicit() {
    let array = BuilderResultArray::<Shape>::default();
    assert_eq!(
        array.read_array().unwrap_err().exception_type(),
        "Standard_NullObject"
    );
    assert_eq!(
        array.write_array().unwrap_err().exception_type(),
        "Standard_NullObject"
    );
    let failure = BuilderResultArray::<Shape>::with_bounds(3, 1).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_RangeError");
    assert!(failure.message().contains("[3, 1]"));
}

#[test]
fn owned_array_is_send_sync_and_thread_confined_guards_are_sync() {
    fn send_sync<T: Send + Sync>() {}
    fn sync<T: Sync>() {}
    send_sync::<BuilderResultArray<Shape>>();
    send_sync::<BuilderResultArray<List<Shape>>>();
    sync::<BuilderResultRead<'static, Shape>>();
    sync::<BuilderResultWrite<'static, List<Shape>>>();
}
