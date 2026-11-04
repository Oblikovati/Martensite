//! OCC204 1 removes selected objects while restarting the invalidated iterator.
use super::Outcome;
use crate::case_view::SourceViewOutput;
use crate::{
    primitive_shapes::box_at,
    session_run::{SessionRun, banner},
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject, AisShape};

const SELECTED_BOX_INPUTS: [([f64; 3], [f64; 3]); 3] = [
    ([0., -500., 0.], [100.; 3]),
    ([120., -380., 120.], [180.; 3]),
    ([320., -180., 320.], [180.; 3]),
];

/// Execute the full original command sequence; `selected_removal()?`.
pub(crate) fn selected_removal() -> Outcome {
    banner(&["========", "OCC204", "========"]);
    let mut run = SessionRun::new()?;
    remove_selected_boxes(run.session().context())?;
    run.session().fit()?;
    if !run.session().check_color(150, 200, [0., 0., 0.])? {
        println!("Error : There are 3 boxes in the viewer! (should be nothing)");
    }
    run.dump("bug204_2.png")?;
    run.finish()
}

fn remove_selected_boxes(context: &AisInteractiveContext) -> Result<(), Failure> {
    prepare_selected_boxes(context)?;
    remove_selection(context, true)
}

pub(crate) fn prepare_selected_boxes(context: &AisInteractiveContext) -> Result<(), Failure> {
    let shapes = SELECTED_BOX_INPUTS
        .map(|(corner, size)| box_at(corner, size).map(|shape| AisShape::new(&shape).cast()));
    let shapes: Vec<Handle<dyn AisInteractiveObject>> =
        shapes.into_iter().collect::<Result<Vec<_>, Failure>>()?;
    for shape in &shapes {
        context.display(shape, false)?;
    }
    for shape in &shapes {
        context.add_or_remove_selected_object(shape, false)?;
    }
    context.update_current_viewer();
    Ok(())
}

pub(crate) fn remove_selection(
    context: &AisInteractiveContext,
    update: bool,
) -> Result<(), Failure> {
    context.init_selected();
    while context.more_selected() {
        let selected = context.selected_interactive()?;
        context.remove(&selected, update)?;
        context.init_selected();
    }
    Ok(())
}

#[test]
fn removing_selected_restarts_iterator_until_context_is_empty() {
    use crate::case_view::viewer_session::ViewerSession;
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    remove_selected_boxes(session.context()).unwrap();
    assert_eq!(session.context().nb_selected(), 0);
    let mut displayed = Default::default();
    session.context().displayed_objects(&mut displayed).unwrap();
    assert!(displayed.is_empty());
    session.close();
}

#[test]
fn three_anonymous_boxes_are_displayed_and_selected_before_removal() {
    use crate::case_view::viewer_session::ViewerSession;
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    prepare_selected_boxes(session.context()).unwrap();
    assert_eq!(session.context().nb_selected(), 3);
    let mut displayed = Default::default();
    session.context().displayed_objects(&mut displayed).unwrap();
    assert_eq!(displayed.len(), 3);
    remove_selection(session.context(), true).unwrap();
    assert_eq!(session.context().nb_selected(), 0);
    session.close();
}
