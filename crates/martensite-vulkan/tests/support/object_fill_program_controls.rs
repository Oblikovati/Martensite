//! The real Null driver retains actual shader aspects without any GPU/compiler invocation.
use super::*;
use martensite_tkservice::graphic3d::Graphic3dAspectsRenderMembers;
use martensite_tkv3d::{ais::AisShape, v3d::V3dViewer};
#[path = "../case_host/primitive_shapes.rs"]
#[allow(dead_code)] // Only the box factory serves the independently included controls.
mod primitive_shapes;

fn actual_object() -> (
    Handle<AisInteractiveContext>,
    Handle<dyn AisInteractiveObject>,
) {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let viewer = V3dViewer::new(&driver).unwrap();
    let context = AisInteractiveContext::new(&viewer).unwrap();
    let shape = primitive_shapes::box_at([0.; 3], [1.; 3]).unwrap();
    let object = AisShape::new(&shape).cast();
    context.display(&object, false).unwrap();
    (context, object)
}

#[test]
fn existing_fill_program_assignment_preserves_actual_program_identity() {
    let (context, object) = actual_object();
    let program = Handle::new(martensite_vulkan::phong_shading_program());
    assign(&context, &[&object], || program.clone()).unwrap();
    assert_eq!(program_of(&object), program);
    assign(&context, &[&object], || program.clone()).unwrap();
    assert_eq!(program_of(&object), program);
}

#[test]
fn existing_fill_program_off_keeps_object_and_clears_actual_aspect() {
    let (context, object) = actual_object();
    let program = Handle::new(martensite_vulkan::phong_shading_program());
    assign(&context, &[&object], || program.clone()).unwrap();
    assign(&context, &[&object], Handle::default).unwrap();
    assert!(program_of(&object).is_null());
    assert!(context.is_displayed(&object));
}

fn program_of(object: &Handle<dyn AisInteractiveObject>) -> Handle<Graphic3dShaderProgram> {
    object
        .attributes()
        .shading_aspect()
        .aspect()
        .shader_program()
}
