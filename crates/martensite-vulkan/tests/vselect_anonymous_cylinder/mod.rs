//! Preserve the QA command's anonymous object and own shaded highlight without named display.
use crate::session_run::presentation_role_scope as boundary_scope;
#[cfg(test)]
mod controls;

use crate::{
    primitive_shapes::cylinder,
    session_run::{SessionRun, banner},
    source_output::control_capture::control_dump,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkv3d::{
    ais::{AisInteractiveObject, AisShape},
    prs_mgr::PrsMgrPresentableObjectMembers as _,
};
use std::error::Error;

/// Keep the authored cylinder dimensions; `source_cylinder()?` has radius100 and height300.
fn source_cylinder() -> Result<Shape, Failure> {
    cylinder(100., 300.)
}

/// Set the QA command's own highlight mode before display; `source_object(&shape)` stays anonymous.
fn source_object(shape: &Shape) -> Handle<dyn AisInteractiveObject> {
    let object: Handle<dyn AisInteractiveObject> = AisShape::new(shape).cast();
    object.set_hilight_mode(1);
    object
}

/// Execute the unchanged selection and color contract; `complete_anonymous_cylinder()?`.
pub(crate) fn complete_anonymous_cylinder() -> Result<(), Box<dyn Error>> {
    banner(&["========", "BUC60876", "========", ""]);
    let shape = source_cylinder()?;
    let mut run = SessionRun::new()?;
    let object = source_object(&shape);
    // RegressionCommands::BUC60876_ uses the bare context Display(true), without ViewerCommands name binding.
    run.session().context().display(&object, true)?;
    run.session().fit()?;
    run.session().move_cursor(200, 200)?;
    run.session().check_color(200, 200, [0., 1., 1.])?;
    run.dump("buc60876.png")?;
    {
        let _restore = boundary_scope::BoundaryPatternScope::new(&object);
        control_dump(&*run.session(), "buc60876.png", "uncovered")?;
    }
    run.finish()
}
