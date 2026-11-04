//! Assemblies edited after display, and instances shown with hidden lines removed.
use super::prelude::*;
use crate::dataset_shape::read_dataset_shape;
use crate::primitive_shapes::sphere;
use martensite_tkv3d::prs3d::Prs3dTypeOfHlr;

/// `bug24837_1`: nested assemblies of boxes and a sphere, one child added and one removed,
/// shown in wireframe, with hidden lines removed, then shaded under the cursor.
pub(crate) fn nested_assembly_edits() -> Outcome {
    banner(&[
        "============",
        "CR24837_1: Visualization - revise design and implementation of connected Interactive Objects",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    let request = DisplayRequest::default();
    run.session()
        .display("b", &box_sized([1., 1., 1.])?, &request)?;
    run.session().display("s", &sphere(0.5)?, &request)?;
    run.session().set_location("s", [0., 2.5, 0.])?;
    run.session()
        .display("d", &box_sized([0.5, 0.5, 3.])?, &request)?;
    run.session()
        .display("d2", &box_sized([0.5, 3., 0.5])?, &request)?;
    connect_nested(run.session())?;
    run.session().fit()?;
    run.dump("bug24837_1_1_1.png")?;
    let sphere_part = ConnectedPart::object("s");
    run.session()
        .add_connected("z4", [0., 4., 0.], sphere_part)?;
    run.session().disconnect_child("z4", 1)?;
    run.session().fit()?;
    run.dump("bug24837_1_1_2.png")?;
    run.session().set_hidden_line_removal(true, None)?;
    run.dump("bug24837_1_1_3.png")?;
    run.session().set_hidden_line_removal(false, None)?;
    run.session().set_display_mode(1)?;
    run.session().move_cursor(320, 330)?;
    run.dump("bug24837_1_1_4.png")?;
    run.finish()
}

/// Connect `b1` to `b` at (-2, 0, 0), then the connected chain z, z2, z3 and z4.
fn connect_nested(
    session: &mut crate::case_view::viewer_session::ViewerSession,
) -> Result<(), Box<dyn Error>> {
    use ConnectedPart as Part;
    session.connect_instance("b1", [-2., 0., 0.], Part::object("b"))?;
    session.connect_assembly("z", [2., 0., 0.], &[Part::object("b"), Part::object("s")])?;
    session.connect_assembly("z2", [4., 0., 0.], &[Part::object("d"), Part::object("d2")])?;
    session.connect_assembly("z3", [6., 0., 0.], &[Part::object("z"), Part::object("z2")])?;
    session.connect_assembly(
        "z4",
        [10., 0., 0.],
        &[Part::object("z3"), Part::object("z2")],
    )?;
    Ok(())
}

/// `bug27869`: an instance of a dataset part is shown with exact hidden-line removal.
pub(crate) fn instance_exact_hidden_lines() -> Outcome {
    banner(&[
        "================",
        "0027869: Visualization - AisConnectedInteractive does not support exact HLR mode",
        "================",
        "",
    ]);
    let obj = read_dataset_shape("CrankArm.brep")?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session()
        .display("obj", &obj, &DisplayRequest::default())?;
    run.session()
        .connect_assembly("new", [50., 50., 50.], &[ConnectedPart::object("obj")])?;
    run.session().fit()?;
    run.session()
        .set_hidden_line_removal(true, Some(Prs3dTypeOfHlr::Algo))?;
    run.dump("bug27869.png")?;
    run.finish()
}
