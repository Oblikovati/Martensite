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
    let projected: Handle<dyn MeshContext> = context.cast();
    let direct = context.dynamic_type().unwrap();
    let base_view = projected.dynamic_type().unwrap();
    assert!(std::ptr::eq(direct, BRepMeshContext::static_type()));
    assert!(std::ptr::eq(direct, base_view));
    assert!(projected.is_kind_name("IMeshTools_Context"));
    assert!(projected.is_kind_name("BRepMesh_Context"));
}
