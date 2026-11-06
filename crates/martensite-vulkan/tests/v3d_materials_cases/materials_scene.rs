//! The sample's bounding box controls framing before its 160 shaded objects are created.
use super::*;
use martensite_tkservice::aspect::AspectGradientFillMethod;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;

pub(super) fn materials_scene() -> Result<SessionRun, Box<dyn std::error::Error>> {
    let mut run = SessionRun::with_view_size(768, 768)?;
    let session = run.session();
    session.set_standard_projection(V3dTypeOfOrientation::ZUP_TOP)?;
    println!("{}", session.graphics_information());
    session.set_gradient_background(
        Color::from_hex("B4C8FF").unwrap(),
        Color::from_hex("B4B4B4").unwrap(),
        AspectGradientFillMethod::Vertical,
    )?;
    session.set_light_direction(0, [0.577, -0.577, -0.577])?;
    session.set_sample_count(8)?;
    bounding_fit(session)?;
    material_labels::draw_labels(session)?;
    for column in 0..4 {
        material_objects::draw_column(session, column)?;
    }
    session.view().redraw();
    Ok(run)
}
fn bounding_fit(session: &mut ViewerSession) -> Result<(), Failure> {
    let shape = crate::primitive_shapes::box_at([0.; 3], [180., 210., 1.])?;
    session.without_updates(|session| {
        session.display(
            "bnd",
            &shape,
            &DisplayRequest::default().with_display_mode(0),
        )
    })?;
    session.fit()?;
    session.without_updates(|session| session.remove(&["bnd"]))
}
