//! Actual object algorithms and drawer hidden-line state retain source command boundaries.
use super::Outcome;
use crate::{
    case_view::viewer_session::DisplayRequest,
    object_hlr_commands,
    session_run::{SessionRun, banner},
};
use martensite_tkv3d::prs3d::Prs3dTypeOfHlr;

/// Dotted exact hidden lines are highlighted and checked at both authored pixels; `dotted_box_highlight()?`.
pub(crate) fn dotted_box_highlight() -> Outcome {
    dotted_heading();
    let shape = crate::primitive_shapes::box_at([15., 0., 0.], [10.; 3])?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("b", &shape, &DisplayRequest::default())?;
    run.session().rotate(35., 0., 0.)?;
    run.session().fit()?;
    run.session()
        .context()
        .default_drawer()
        .enable_draw_hidden_line();
    run.session().set_hidden_line_removal(true, None)?;
    object_hlr_commands::set_object_algorithm(run.session(), "b", Prs3dTypeOfHlr::Algo)?;
    run.session().move_cursor(76, 304)?;
    check_dotted_pixels(run.session())?;
    run.dump("bug24388_1_3.png")?;
    run.finish()
}
fn check_dotted_pixels(session: &crate::case_view::viewer_session::ViewerSession) -> Outcome {
    if !session.check_color(115, 203, [0., 1., 1.])? {
        println!("Error : Highlighting of dotted hidden lines in the box is wrong.");
    }
    if !session.check_color(122, 200, [0.; 3])? {
        println!("Error : Highlighting of dotted hidden lines in the box is wrong.");
    }
    Ok(())
}
/// Restore the original surface-cone input before exact HLR and width five; `restored_exact_surface()?`.
pub(crate) fn restored_exact_surface() -> Outcome {
    restored_heading();
    let shape = crate::dataset_shape::read_dataset_shape("bug23709_surfcone.brep")?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("result", &shape, &DisplayRequest::default())?;
    run.session().rotate(35., 0., 0.)?;
    run.session().fit()?;
    run.session().set_hidden_line_removal(true, None)?;
    object_hlr_commands::set_object_algorithm(run.session(), "result", Prs3dTypeOfHlr::Algo)?;
    object_hlr_commands::set_all_line_width(run.session(), &["result"], 5.)?;
    if run.session().pixel_color_name(183, 267)? != "YELLOW" {
        println!("Error : color is not yellow");
    }
    run.dump("bug23709_4.png")?;
    run.finish()
}

fn dotted_heading() {
    banner(&[
        "===========",
        "0024388: Option to enable visualization of hidden lines in command-host",
        "Checks if dotted hidden lines are highlighted properly.",
        "===========",
        "",
    ]);
}

fn restored_heading() {
    banner(&[
        "===========",
        "OCC23709: Redesign of hlrtest command",
        "===========",
        "",
    ]);
}
