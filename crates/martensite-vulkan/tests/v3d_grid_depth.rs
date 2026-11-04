//! Automatic camera depth fitting is exercised without changing its default or the grid plane.
use crate::{
    case_view::viewer_session::{DisplayRequest, GridLayout, GridRequest, ViewerSession},
    primitive_shapes::box_sized,
    session_run::{SessionRun, banner},
};
use martensite_tkernel::standard::Failure;
use martensite_tkservice::aspect::AspectGridDrawMode;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;
type Outcome = Result<(), Box<dyn std::error::Error>>;

/// Retain the TODO declaration and test the actual grid color/alpha; `automatic_grid_depth()?`.
pub(crate) fn automatic_grid_depth() -> Outcome {
    banner(&[
        "TODO OCC26154 ALL: ERROR: OCC26154 is reproduced.",
        "========",
        "OCC26154: 0026154: Visualization - v3d viewer grid disappears due to automatic z-fit",
        "========",
        "",
    ]);
    let mut run = SessionRun::new()?;
    let shape = box_sized([1.; 3])?;
    run.session()
        .display("b", &shape, &DisplayRequest::default())?;
    run.session().fit()?;
    source_grid(run.session())?;
    if run.session().pixel_rgba_name(86, 355)?.trim() != "GRAY50 1" {
        println!("ERROR: OCC26154 is reproduced. Grid is not visible.");
    }
    run.dump("bug26154.png")?;
    run.finish()
}
pub(super) fn source_grid(session: &ViewerSession) -> Result<(), Failure> {
    let layout = GridLayout {
        origin: Some((0., 0.)),
        step: Some((1., 1.)),
        ..Default::default()
    };
    session.show_grid(&GridRequest::rectangular(layout).drawn_as(AspectGridDrawMode::Lines))?;
    session.set_standard_projection(V3dTypeOfOrientation::Zpos)
}
#[cfg(test)]
mod cpu {
    use super::*;
    use crate::case_view::SourceViewOutput;
    #[test]
    fn grid_keeps_authored_values_and_auto_depth_default() {
        let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
        let session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
        let mode = session.view().view_base().auto_z_fit_mode();
        source_grid(&session).unwrap();
        let viewer = session.context().current_viewer();
        assert_eq!(
            viewer.rectangular_grid_values().unwrap(),
            (0., 0., 1., 1., 0.)
        );
        assert_eq!(session.view().view_base().auto_z_fit_mode(), mode);
        session.close();
    }
}

#[cfg(test)]
mod todo_cpu {
    use crate::case_todo::{TodoOutcome, todo_outcome};
    const HEADER: &str = "TODO OCC26154 ALL: ERROR: OCC26154 is reproduced.";
    const ERROR: &str = "ERROR: OCC26154 is reproduced.";
    fn outcome(report: &str) -> TodoOutcome {
        todo_outcome(report, &[ERROR], |line| {
            !crate::case_v3d_policy::accepted_report(&format!("{line}\nTEST COMPLETED"))
        })
    }
    #[test]
    fn todo_header_never_substitutes_for_actual_grid_error() {
        assert_eq!(outcome(HEADER), TodoOutcome::Improvement(ERROR));
        assert_eq!(
            outcome(&format!("{HEADER}\n{ERROR} Grid is not visible.")),
            TodoOutcome::Todo
        );
        assert!(matches!(
            outcome(&format!(
                "{HEADER}\n{ERROR} Grid is not visible.\nError: unexpected"
            )),
            TodoOutcome::Failed(_)
        ));
    }
}
