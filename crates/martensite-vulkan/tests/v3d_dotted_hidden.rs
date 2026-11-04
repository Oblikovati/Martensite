//! The literal `-1,57` reads as -1: number parsing consumes the prefix before the comma.
use crate::{
    case_view::viewer_session::{DisplayRequest, ViewerSession},
    object_hlr_commands,
    session_run::{SessionRun, banner},
};
use martensite_tkernel::standard::Failure;
use martensite_tkv3d::prs3d::Prs3dTypeOfHlr;
type Outcome = Result<(), Box<dyn std::error::Error>>;

/// Render the authored cone with hidden lines disabled then enabled; `dotted_cone()?`.
pub(crate) fn dotted_cone() -> Outcome {
    heading();
    let shape = crate::primitive_shapes::cone(5., 0., 10.)?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("a", &shape, &DisplayRequest::default())?;
    run.session().rotate(35., 0., 0.)?;
    run.session().fit()?;
    hidden(run.session(), false)?;
    object_hlr_commands::set_object_algorithm(run.session(), "a", Prs3dTypeOfHlr::Algo)?;
    // VRotate uses Draw::Atof without checking RasterParseFailed; the original comma is retained.
    run.session().rotate(-1., -1., 0.)?;
    run.session().fit()?;
    check_then_dump(&mut run, false)?;
    hidden(run.session(), true)?;
    check_then_dump(&mut run, true)?;
    run.finish()
}
fn check_then_dump(run: &mut SessionRun, on: bool) -> Outcome {
    let channel = if on { 0.2 } else { 0. };
    if !run.session().check_color(131, 240, [channel; 3])? {
        println!("Error : Drawing hidden lines as dotted lines produces wrong result.");
    }
    run.dump(if on {
        "bug24388_2_with_lines.png"
    } else {
        "bug24388_2_without_lines.png"
    })?;
    Ok(())
}
pub(crate) fn hidden(session: &ViewerSession, on: bool) -> Result<(), Failure> {
    let drawer = session.context().default_drawer();
    let changed = drawer.draw_hidden_line() != on;
    if on {
        drawer.enable_draw_hidden_line();
    } else {
        drawer.disable_draw_hidden_line();
    }
    if session.view().view_base().computed_mode() && changed {
        session.redisplay_shapes()?;
    }
    session.set_hidden_line_removal(true, None)
}

fn heading() {
    banner(&[
        "===========",
        "0024388: Option to enable visualization of hidden lines in command-host",
        "Checks if dotted hidden lines are displayed properly in HLR mode.",
        "===========",
        "",
    ]);
}
#[cfg(test)]
mod cpu {
    use super::*;
    use crate::case_view::SourceViewOutput;
    #[test]
    fn hidden_state_switch_recomputes_the_actual_computed_shape() {
        let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
        let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
        let shape = crate::primitive_shapes::cone(5., 0., 10.).unwrap();
        session
            .display("a", &shape, &DisplayRequest::default())
            .unwrap();
        hidden(&session, false).unwrap();
        assert!(!session.context().default_drawer().draw_hidden_line());
        hidden(&session, true).unwrap();
        assert!(session.context().default_drawer().draw_hidden_line());
        assert!(session.view().view_base().computed_mode());
        session.close();
    }
}
