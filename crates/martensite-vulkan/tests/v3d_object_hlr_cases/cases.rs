//! Original assertions and image ordering share only the genuinely identical tail.
use super::{Outcome, commands};
use crate::{
    case_view::viewer_session::DisplayRequest,
    primitive_shapes::{box_at, cone},
    session_run::{SessionRun, banner},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkv3d::prs3d::Prs3dTypeOfHlr;

pub(super) fn shapes() -> Result<[Shape; 2], Failure> {
    let cone = cone(5., 0., 10.)?;
    let box_shape = box_at([15., 0., 0.], [10.; 3])?;
    Ok([cone, box_shape])
}

fn heading() {
    banner(&[
        "============",
        "OCC23709: Redesign of hlrtest command",
        "============",
        "",
    ]);
}

pub(super) fn computed_view(
    session: &crate::case_view::viewer_session::ViewerSession,
) -> Result<(), Failure> {
    session.rotate(35., 0., 0.)?;
    session.fit()?;
    session.set_hidden_line_removal(true, None)
}

fn finish(mut run: SessionRun, image: &'static str) -> Outcome {
    commands::set_all_line_width(run.session(), &["a", "b"], 5.)?;
    if run.session().pixel_color_name(290, 170)? != "YELLOW" {
        println!("Error : color is not yellow");
    }
    run.session().check_color(86, 221, [0.; 3])?;
    run.dump(image)?;
    run.finish()
}

/// Two separate source display commands, then explicit PolyAlgo on b; `polygonal_box_algorithm()?`.
pub(crate) fn polygonal_box_algorithm() -> Outcome {
    heading();
    let [cone, box_shape] = shapes()?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("a", &cone, &DisplayRequest::default())?;
    run.session()
        .display("b", &box_shape, &DisplayRequest::default())?;
    computed_view(run.session())?;
    commands::set_object_algorithm(run.session(), "b", Prs3dTypeOfHlr::PolyAlgo)?;
    finish(run, "bug23709_1.png")
}

/// One combined display command, then exact HLR only on a; `exact_cone_algorithm()?`.
pub(crate) fn exact_cone_algorithm() -> Outcome {
    heading();
    let [cone, box_shape] = shapes()?;
    let mut run = SessionRun::new()?;
    run.session().display_many(
        &[("a", &cone), ("b", &box_shape)],
        &DisplayRequest::default(),
    )?;
    computed_view(run.session())?;
    commands::set_object_algorithm(run.session(), "a", Prs3dTypeOfHlr::Algo)?;
    finish(run, "bug23709_2.png")
}
