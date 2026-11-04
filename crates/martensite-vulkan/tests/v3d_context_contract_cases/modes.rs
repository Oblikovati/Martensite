use super::named_pixel;
use crate::{
    case_view::viewer_session::ViewerSession,
    context_mode_transition::redisplay_selection_mode_on_context,
    primitive_shapes::box_sized,
    session_run::{SessionRun, banner},
};
use martensite_tkernel::{ncollection::List, standard::Failure};
use std::error::Error;

/// Read the last actual activated mode, as the QA getter does; `last_mode(session)`.
pub(super) fn last_mode(session: &ViewerSession) -> i32 {
    let mut modes = List::default();
    session
        .context()
        .activated_modes(session.object("b").unwrap(), &mut modes);
    modes.iter().last().copied().unwrap_or(-1)
}

/// Invoke the complete QA setter, then its getter; `transition(session, 2)?`.
pub(super) fn transition(session: &ViewerSession, mode: i32) -> Result<(), Failure> {
    redisplay_selection_mode_on_context(session.context(), session.object("b").unwrap(), mode)?;
    let actual = last_mode(session);
    if actual != mode {
        println!("Select Mode = {actual}\nOCC74: Get Select Mode Error ");
    }
    Ok(())
}

/// Replay all authored mode and pixel checks; `mode()?`.
pub(crate) fn mode() -> Result<(), Box<dyn Error>> {
    banner(&[
        "================",
        "OCC74",
        "BUC61051",
        "================",
        "",
    ]);
    let mut run = SessionRun::new()?;
    let session = run.session();
    session.display("b", &box_sized([10.; 3])?, &Default::default())?;
    session.fit()?;
    session.set_line_width("b", 5.)?;
    initial_yellow(session)?;
    activated_colors(session)?;
    transition(session, -1)?;
    final_yellow(session)?;
    run.dump("bug74.png")?;
    run.finish()
}

fn initial_yellow(session: &ViewerSession) -> Result<(), Failure> {
    named_pixel(
        session,
        100,
        346,
        "YELLOW",
        "Error : shape color is not yellow",
    )?;
    named_pixel(
        session,
        29,
        305,
        "YELLOW",
        "Error : vertex color is not yellow",
    )
}

fn activated_colors(session: &mut ViewerSession) -> Result<(), Failure> {
    for mode in 0..=6 {
        session.move_cursor(1, 1)?;
        transition(session, mode)?;
        let (x, y) = if mode == 1 { (29, 305) } else { (100, 346) };
        session.move_cursor(x, y)?;
        if mode == 0 {
            named_pixel(session, x, y, "CYAN", "Error : shape color is not cyan")?;
        } else {
            session.check_color(x, y, [0., 1., 1.])?;
        }
    }
    Ok(())
}

fn final_yellow(session: &mut ViewerSession) -> Result<(), Failure> {
    let shape_error = "Error : shape color is not yellow";
    let vertex_error = "Error : vertex color is not yellow";
    session.move_cursor(1, 1)?;
    named_pixel(session, 100, 346, "YELLOW", shape_error)?;
    session.move_cursor(1, 1)?;
    named_pixel(session, 29, 305, "YELLOW", vertex_error)
}
