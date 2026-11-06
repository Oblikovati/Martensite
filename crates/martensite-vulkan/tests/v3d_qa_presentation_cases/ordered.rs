//! RegressionCommands::OCC128 displays the datum before relocating the shaded box.
use super::*;
use crate::case_view::viewer_session::ViewParameters;
use martensite_tkernel::quantity::Color;
use martensite_tkg3d::geom::GeomAxis2Placement;
use martensite_tkmath::{
    gp::{Axis2, Direction3, Point3, Transform3},
    toploc::Location,
};
use martensite_tkservice::graphic3d::{Graphic3dMaterialAspect, Graphic3dNameOfMaterial};
use martensite_tkv3d::ais::{
    AisInteractiveObject, AisShape, AisTrihedron, AisTrihedronSource as _,
};
use martensite_tkv3d::prs_mgr::{
    PrsMgrPresentableObject as _, PrsMgrPresentableObjectMembers as _,
};

/// Build the translated source placement; `source_datum()?`.
pub(super) fn source_datum() -> Result<Handle<AisTrihedron>, Failure> {
    let axes = Axis2::with_x_direction(
        Point3::new(100., 100., 0.),
        Direction3::new(0., 0., 1.)?,
        Direction3::new(1., 0., 0.)?,
    )?;
    let datum = AisTrihedron::new(&Handle::new(GeomAxis2Placement::new(axes)))?;
    datum.set_color(Color::from_name("LIGHTSTEELBLUE4").expect("named source colour"))?;
    datum.set_size(100.)?;
    Ok(datum)
}

/// Set material and mode before display, as the private QA command does; `source_box()?`.
pub(super) fn source_box() -> Result<Handle<AisShape>, Failure> {
    let object = AisShape::new(&crate::primitive_shapes::box_sized([50.; 3])?);
    object.set_display_mode(1);
    object.set_material(&Graphic3dMaterialAspect::new(
        Graphic3dNameOfMaterial::Plastified,
    ))?;
    object.set_color(Color::from_name("RED").expect("named source colour"))?;
    Ok(object)
}

/// Execute the authored display and relocation order; `ordered_objects(&session)?`.
pub(super) fn ordered_objects(session: &ViewerSession) -> Result<Handle<AisShape>, Failure> {
    let context = session.context();
    context.display(&source_datum()?.cast(), true)?;
    let object = source_box()?;
    let interactive: Handle<dyn AisInteractiveObject> = object.cast();
    context.display(&interactive, false)?;
    context.reset_location(&interactive)?;
    context.set_location(
        &interactive,
        &Location::new(Transform3::translation([20., 20., 0.])),
    )?;
    context.redisplay(&interactive, true, false)?;
    Ok(object)
}

/// Preserve every authored camera field; `source_camera(&session)?`.
pub(super) fn source_camera(session: &ViewerSession) -> Result<(), Failure> {
    session.set_view_parameters(&ViewParameters {
        scale: Some(4.9487928),
        proj: Some([0.7329295, 0.59461397, 0.33052679]),
        up: Some([-0.536849, 0.2071041, 0.81786]),
        at: Some([32.4556665273951, 52.9347942181675, -5.1270029887922]),
        ..Default::default()
    })
}

/// Complete `bug128`, including its unchanged pixel check; `ordered_shading()?`.
pub(crate) fn ordered_shading() -> Outcome {
    banner(&[
        "================",
        "OCC128: The shading is false: objects seem to be situated in front of the others while they are behind.",
        "================",
    ]);
    let mut run = SessionRun::new()?;
    ordered_objects(run.session())?;
    source_camera(run.session())?;
    run.session().check_color(171, 171, [0.43, 0.48, 0.54])?;
    run.dump("bug128.png")?;
    run.finish()
}
