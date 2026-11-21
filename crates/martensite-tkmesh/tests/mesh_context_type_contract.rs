//! Concrete meshing context descriptor chain and projected handle identity.
use martensite_tkernel::standard::{Handle, RuntimeTyped};
use martensite_tkmesh::brep_mesh::BRepMeshContext;
use martensite_tkmesh::imesh_tools::{Context, MeshAlgoType, MeshContext};

#[test]
fn concrete_context_descriptor_names_its_declared_parent() {
    let descriptor = BRepMeshContext::static_type();
    assert_eq!(BRepMeshContext::TYPE_NAME, "BRepMesh_Context");
    assert_eq!(descriptor.name(), "BRepMesh_Context");
    let parent = descriptor.parent().unwrap();
    assert_eq!(parent.name(), "IMeshTools_Context");
    assert!(std::ptr::eq(parent, Context::static_type()));
}

#[test]
fn concrete_context_dynamic_descriptor_survives_base_projection() {
    let context =
        Handle::new(BRepMeshContext::with_environment(MeshAlgoType::Watson, &|_| None).unwrap());
    let projected = context.upcast::<dyn MeshContext>();
    let direct = context.dynamic_type().unwrap();
    let base_view = projected.dynamic_type().unwrap();
    assert!(std::ptr::eq(direct, BRepMeshContext::static_type()));
    assert!(std::ptr::eq(direct, base_view));
    assert!(projected.is_kind_name("IMeshTools_Context"));
    assert!(projected.is_kind_name("BRepMesh_Context"));
}

#[test]
fn concrete_context_interface_keeps_actual_owner_and_lends_composed_state() {
    let context =
        Handle::new(BRepMeshContext::with_environment(MeshAlgoType::Watson, &|_| None).unwrap());
    let original = context.object_address();
    let projected = context.upcast::<dyn MeshContext>();
    assert_eq!(context.ref_count(), 2);
    assert_eq!(projected.object_address(), original);
    assert!(projected.downcast_ref::<BRepMeshContext>().is_some());
    assert!(projected.downcast_ref::<Context>().is_none());
    projected.context_base().change_parameters().clean_model = false;
    assert!(!context.context_base().get_parameters().clean_model);
    drop(context);
    assert_eq!(projected.ref_count(), 1);
    assert!(!projected.context_base().get_parameters().clean_model);
}
