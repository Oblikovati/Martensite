//! V2/ViewLeft and V2/ViewRight deliberately share their real viewer and context.
use crate::{
    case_view::viewer_session::{DisplayRequest, FaceBoundary, ViewerSession},
    primitive_shapes::{box_sized, sphere},
};
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::Failure,
};
use martensite_tkservice::graphic3d::Graphic3dCViewSubviews;
use martensite_tkservice::graphic3d::Graphic3dNameOfMaterial;
pub(super) fn prepare(session: &mut ViewerSession) -> Result<(), Failure> {
    let root = session.view().clone();
    root.get().unwrap().view().set_subview_composer(false);
    session.set_background_color(Color::named(NamedColor::GRAY20))?;
    session.open_viewer()?;
    session.bind_subview(&root, [0.5, 1.], [0., 0.])?;
    left(session)?;
    session.open_view(512, 512)?;
    session.bind_subview(&root, [0.5, 1.], [0.5, 0.])?;
    right(session)
}
fn left(session: &mut ViewerSession) -> Result<(), Failure> {
    session.set_background_color(Color::named(NamedColor::GRAY30))?;
    session.display(
        "b",
        &box_sized([100., 200., 300.])?,
        &DisplayRequest::default().with_display_mode(1),
    )?;
    session.draw_face_boundaries("b", &FaceBoundary::default())?;
    session.fit()?;
    let cube = session.view_cube("vc")?;
    session.display_object("vc", cube.cast())
}
fn right(session: &mut ViewerSession) -> Result<(), Failure> {
    session.set_background_color(Color::named(NamedColor::GRAY40))?;
    session.display(
        "s",
        &sphere(300.)?,
        &DisplayRequest::default().with_display_mode(1),
    )?;
    session.set_material("s", Graphic3dNameOfMaterial::Silver)?;
    session.fit()?;
    session.show_view_trihedron(&Default::default())
}
