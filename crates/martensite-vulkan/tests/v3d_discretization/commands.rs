//! Thin view and selection commands use the existing service/context APIs unchanged.
use crate::case_view::viewer_session::{SubShapeStyle, ViewerSession};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::{Failure, exceptions::program_error},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkv3d::ais::AisSelectionModesConcurrency;
pub(super) fn camera(session: &ViewerSession) -> Result<(), Failure> {
    session.rotate(1., 1.5, 0.)?;
    session.fit()?;
    session.update_view(|view| view.translate(24., 18.5, 0., true))?;
    session.zoom(32.)?;
    session.update_view(|view| view.translate(-0.1, -0.6, 0., true))?;
    session.zoom(2.)
}
pub(super) fn single_mode(session: &ViewerSession, mode: i32) -> Result<(), Failure> {
    let object = session
        .object("p")
        .ok_or_else(|| program_error("selection name=\"p\"; expected authored cylinder binding"))?;
    session.context().set_selection_mode_active(
        object,
        mode,
        true,
        AisSelectionModesConcurrency::Single,
        false,
    )
}
pub(super) fn color_source_face(session: &mut ViewerSession, shape: &Shape) -> Result<(), Failure> {
    let faces = crate::boolean_typed_explode::all_distinct(shape, ShapeEnum::Face);
    let face = faces.get(2).ok_or_else(|| {
        program_error(&format!(
            "explode p F found {} faces; expected p_3 binding",
            faces.len()
        ))
    })?;
    session.style_sub_shape(
        "p",
        face,
        &SubShapeStyle {
            color: Some(Color::named(NamedColor::RED)),
            width: None,
        },
    )
}
