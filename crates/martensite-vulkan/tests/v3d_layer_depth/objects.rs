//! Actual shaded primitives retain per-object material and inherit layer polygon offsets.
use crate::{
    case_view::viewer_session::{DisplayRequest, ViewerSession},
    primitive_shapes::{box_at, sphere as make_sphere},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, exceptions::program_error};
use martensite_tkservice::graphic3d::Graphic3dNameOfMaterial;
pub(super) fn thin_boxes(session: &mut ViewerSession) -> Result<(), Failure> {
    let mut boxes = Vec::new();
    for (name, corner) in [("b1", -0.75), ("b2", -0.50), ("b3", -0.25)] {
        boxes.push((name, box_at([corner, corner, 0.], [1., 1., 0.01])?));
    }
    display_boxes(session, &boxes)?;
    box_materials(session)?;
    for name in ["b1", "b2", "b3"] {
        inherit_offset(session, name)?;
    }
    Ok(())
}
fn box_materials(session: &ViewerSession) -> Result<(), Failure> {
    for (name, material) in [
        ("b1", Graphic3dNameOfMaterial::Brass),
        ("b2", Graphic3dNameOfMaterial::Silver),
        ("b3", Graphic3dNameOfMaterial::Copper),
    ] {
        session.set_material(name, material)?;
    }
    Ok(())
}
fn display_boxes(session: &mut ViewerSession, boxes: &[(&str, Shape)]) -> Result<(), Failure> {
    session.update_session(|session| {
        for (name, shape) in boxes {
            session.display(name, shape, &DisplayRequest::default().with_display_mode(1))?;
        }
        Ok(())
    })
}
pub(super) fn sphere(session: &mut ViewerSession) -> Result<(), Failure> {
    session.display(
        "s",
        &make_sphere(0.3)?,
        &DisplayRequest::default().with_display_mode(1),
    )?;
    material_and_offset(session, "s", Graphic3dNameOfMaterial::Brass)
}
fn material_and_offset(
    session: &ViewerSession,
    name: &str,
    material: Graphic3dNameOfMaterial,
) -> Result<(), Failure> {
    session.set_material(name, material)?;
    inherit_offset(session, name)
}
fn inherit_offset(session: &ViewerSession, name: &str) -> Result<(), Failure> {
    let object = session.object(name).ok_or_else(|| {
        program_error(&format!(
            "offset object {name:?}; expected displayed primitive"
        ))
    })?;
    session.update_context(|_| object.set_polygon_offsets(8, 0., 0.))
}
pub(super) fn crossing_box(session: &mut ViewerSession, layer: i32) -> Result<(), Failure> {
    let shape: Shape = box_at([0., 0., 0.01], [0.5, 0.5, -0.5])?;
    session.update_session(|session| {
        session.display("b", &shape, &DisplayRequest::default().with_display_mode(1))?;
        session.set_object_z_layer("b", layer)
    })?;
    material_and_offset(session, "b", Graphic3dNameOfMaterial::Brass)
}
