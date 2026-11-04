//! Build positioned topology directly; the command does not translate a named AIS object.
use crate::case_view::viewer_session::ViewerSession;
use crate::primitive_shapes::{box_at, box_sized};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkv3d::{
    ais::{AisInteractiveObject, AisShape},
    prs_mgr::PrsMgrDisplayStatus,
};

/// Build both command operands with their original placements; `source_boxes()?[0]` starts at(10,10,0).
pub(super) fn source_boxes() -> Result<[Shape; 2], Failure> {
    Ok([box_at([10., 10., 0.], [10.; 3])?, box_sized([10.; 3])?])
}

/// Preserve explicit edge selection and deferred width updates; `display_source_boxes(session)?`.
pub(super) fn display_source_boxes(
    session: &ViewerSession,
) -> Result<[Handle<dyn AisInteractiveObject>; 2], Failure> {
    let shapes = source_boxes()?;
    let objects = shapes.each_ref().map(|shape| AisShape::new(shape).cast());
    for object in &objects {
        session
            .context()
            .display_with_modes(object, 0, 2, false, PrsMgrDisplayStatus::None)?;
    }
    session.view().fit_all(0.01, true)?;
    for object in &objects {
        session.context().set_width(object, 3., false)?;
    }
    Ok(objects)
}
