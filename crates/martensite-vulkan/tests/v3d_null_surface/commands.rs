//! Deferred display modes and the explicit unset/set boundary commands retain source updates.
use crate::case_view::viewer_session::{
    DisplayRequest, FaceBoundary, SubShapeStyle, ViewerSession,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::{Failure, Handle, exceptions::program_error},
};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;
pub(super) fn presentation(
    session: &mut ViewerSession,
    shape: &Shape,
    faces: &[Shape],
) -> Result<(), Failure> {
    session.without_updates(|session| {
        session.display("s", shape, &DisplayRequest::default().with_display_mode(0))
    })?;
    style_first_face(session, faces)?;
    session.without_updates(|session| {
        session.display("s", shape, &DisplayRequest::default().with_display_mode(1))
    })?;
    unset_boundary(session)?;
    session.draw_face_boundaries("s", &boundary_style())
}
fn boundary_style() -> FaceBoundary {
    FaceBoundary {
        color: Some(Color::named(NamedColor::RED)),
        width: Some(2.),
        ..Default::default()
    }
}

fn style_first_face(session: &mut ViewerSession, faces: &[Shape]) -> Result<(), Failure> {
    let face = faces.first().ok_or_else(|| {
        program_error(&format!(
            "explode s F found {} faces; expected s_1",
            faces.len()
        ))
    })?;
    session.style_sub_shape(
        "s",
        face,
        &SubShapeStyle {
            color: Some(Color::named(NamedColor::RED)),
            width: None,
        },
    )
}
fn unset_boundary(session: &ViewerSession) -> Result<(), Failure> {
    let object = session
        .object("s")
        .ok_or_else(|| {
            program_error("boundary name=\"s\" absent; expected displayed source shape")
        })?
        .clone();
    let drawer = object.attributes();
    let draw = drawer.has_own_face_boundary_draw();
    let aspect = drawer.has_own_face_boundary_aspect();
    apply_boundary_reset(session, &object, &drawer, draw, aspect)
}
fn apply_boundary_reset(
    session: &ViewerSession,
    object: &Handle<dyn martensite_tkv3d::ais::AisInteractiveObject>,
    drawer: &Handle<martensite_tkv3d::prs3d::Prs3dDrawer>,
    draw: bool,
    aspect: bool,
) -> Result<(), Failure> {
    session.update_context(|context| {
        if draw {
            drawer.set_face_boundary_draw(false);
        }
        if aspect {
            drawer.set_face_boundary_aspect(Handle::default());
        }
        if draw || aspect {
            context.redisplay(&object, false, false)?;
        }
        Ok(())
    })
}
