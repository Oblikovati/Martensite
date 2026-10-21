//! Defined retained-slot behavior, independently checked with explicit vertex values.
use martensite_tkernel::{
    ncollection::{DynamicArray, Shared},
    standard::{Handle, SharedState},
};
use martensite_tkmath::gp::Xy;
use martensite_tkmesh::brep_mesh::{
    BRepMeshDataStructureOfDelaun, BRepMeshDelaun, BRepMeshVertex, BRepMeshVertexInspector,
    BRepMeshVertexTool, DegreeOfFreedom, MeshVertexCollection, VertexSlotRead, VertexSlotWrite,
};

fn vertex(x: f64, location: i32) -> BRepMeshVertex {
    BRepMeshVertex::new(Xy::new(x, 2.), location, DegreeOfFreedom::Free)
}
fn assert_vertex(actual: BRepMeshVertex, x: f64, location: i32, movability: DegreeOfFreedom) {
    assert_eq!(*actual.coord(), Xy::new(x, 2.));
    assert_eq!(actual.location_3d(), location);
    assert_eq!(actual.movability(), movability);
}

fn assert_free_vertex(actual: BRepMeshVertex, x: f64, location: i32) {
    assert_vertex(actual, x, location, DegreeOfFreedom::Free);
}

#[test]
fn mutable_slot_alias_observes_edits_deletion_and_fifo_reuse() {
    let mut inspector = BRepMeshVertexInspector::new(&Handle::default());
    inspector.add(&vertex(1., 3)).unwrap();
    let retained = inspector.get_vertex_reference(1).unwrap();
    let read = retained.read_only();
    retained.replace(vertex(4., 3)).unwrap();
    assert_free_vertex(inspector.get_vertex(1).unwrap(), 4., 3);
    inspector.delete(1).unwrap();
    assert_vertex(read.snapshot().unwrap(), 4., 3, DegreeOfFreedom::Deleted);
    assert_eq!(inspector.add(&vertex(8., 9)).unwrap(), 1);
    assert_free_vertex(retained.snapshot().unwrap(), 8., 9);
}

#[test]
fn aliases_preserve_slot_identity_without_intrusive_retention() {
    let mut inspector = BRepMeshVertexInspector::new(&Handle::default());
    inspector.add(&vertex(1., 3)).unwrap();
    let owner = inspector.vertices();
    let before = owner.ref_count();
    let first = inspector.get_vertex_reference(1).unwrap().read_only();
    let second = first.clone();
    let again = inspector.get_vertex_reference(1).unwrap().read_only();
    assert!(first.same_slot(&second) && first.same_slot(&again));
    assert_eq!(owner.ref_count(), before);
    assert_free_vertex(first.snapshot().unwrap(), 1., 3);
    assert_eq!(owner.ref_count(), before);
}

fn replacement_collection() -> Handle<MeshVertexCollection> {
    let mut values = DynamicArray::default();
    values.append(vertex(10., 11));
    Handle::new(Shared::new(SharedState::new(values)))
}

#[test]
fn replacing_a_collection_handle_does_not_retarget_a_live_old_slot() {
    let mut inspector = BRepMeshVertexInspector::new(&Handle::default());
    inspector.add(&vertex(8., 9)).unwrap();
    let original_owner = inspector.vertices();
    let retained = inspector.get_vertex_reference(1).unwrap();
    *inspector.change_vertices().write() = replacement_collection();
    let replacement = inspector.get_vertex_reference(1).unwrap();
    assert!(!retained.read_only().same_slot(&replacement.read_only()));
    assert_free_vertex(retained.snapshot().unwrap(), 8., 9);
    assert_free_vertex(replacement.snapshot().unwrap(), 10., 11);
    drop(original_owner);
    assert_eq!(
        retained.snapshot().unwrap_err().exception_type(),
        "Standard_NullObject"
    );
}

#[test]
fn tool_const_view_observes_in_place_substitution() {
    let tool = BRepMeshVertexTool::new(&Handle::default());
    tool.add(&vertex(1., 3), true).unwrap();
    let key = tool.find_key_reference(1).unwrap();
    tool.substitute(1, &vertex(12., 13)).unwrap();
    assert_free_vertex(key.snapshot().unwrap(), 12., 13);
}

#[test]
fn mesh_and_triangulator_const_views_observe_in_place_substitution() {
    let mesh = Handle::new(BRepMeshDataStructureOfDelaun::new(&Handle::default(), 4));
    mesh.add_node(&vertex(1., 3), true).unwrap();
    let node = mesh.get_node_reference(1).unwrap();
    let triangulator = BRepMeshDelaun::from_mesh(&mesh, 1, 1, false).unwrap();
    let from_delaun = triangulator.get_vertex_reference(1).unwrap();
    assert!(node.same_slot(&from_delaun));
    assert!(mesh.substitute_node(1, &vertex(14., 15)).unwrap());
    assert_free_vertex(node.snapshot().unwrap(), 14., 15);
    assert_free_vertex(from_delaun.snapshot().unwrap(), 14., 15);
}

#[test]
fn uninitialized_storage_reference_acquisition_returns_checked_failures() {
    let inspector = BRepMeshVertexInspector::new(&Handle::default());
    for index in [-1, 0, 1] {
        assert_eq!(
            inspector
                .get_vertex_reference(index)
                .unwrap_err()
                .exception_type(),
            "Standard_OutOfRange"
        );
    }
}

#[test]
fn clearing_storage_invalidates_existing_references_with_checked_failures() {
    // A reference retained across Clear() names a slot outside the logical size: native
    // reads the stale trivially destructible bytes (unchecked Value), Rust reports the
    // out-of-range slot as it does every access outside the logical size.
    let mut inspector = BRepMeshVertexInspector::new(&Handle::default());
    inspector.add(&vertex(1., 3)).unwrap();
    let retained = inspector.get_vertex_reference(1).unwrap();
    inspector.clear();
    let read_failure = retained.snapshot().unwrap_err();
    let write_failure = retained.replace(vertex(5., 6)).unwrap_err();
    assert_eq!(read_failure.exception_type(), "Standard_OutOfRange");
    assert_eq!(write_failure.exception_type(), "Standard_OutOfRange");
}

#[test]
fn public_slot_aliases_preserve_automatic_send_sync_traits() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<VertexSlotRead>();
    assert_send_sync::<VertexSlotWrite>();
}

#[test]
fn disjoint_slot_aliases_allow_concurrent_read_and_replace() {
    let mut inspector = BRepMeshVertexInspector::new(&Handle::default());
    let references: Vec<VertexSlotWrite> = (1..=8)
        .map(|index| {
            inspector.add(&vertex(0., index)).unwrap();
            inspector.get_vertex_reference(index).unwrap()
        })
        .collect();
    replace_slots_in_parallel(references);
    for index in 1..=8 {
        assert_free_vertex(inspector.get_vertex(index).unwrap(), 99., index);
    }
}

fn repeatedly_replace_slot(reference: VertexSlotWrite, location: i32) {
    let read = reference.read_only();
    for iteration in 0..100 {
        let x = f64::from(iteration);
        reference.replace(vertex(x, location)).unwrap();
        assert_free_vertex(read.snapshot().unwrap(), x, location);
    }
}

fn replace_slots_in_parallel(references: Vec<VertexSlotWrite>) {
    std::thread::scope(|scope| {
        for (index, reference) in references.into_iter().enumerate() {
            scope.spawn(move || {
                repeatedly_replace_slot(reference, i32::try_from(index + 1).unwrap())
            });
        }
    });
}
