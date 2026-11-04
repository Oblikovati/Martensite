//! The genuine surface fixture drives selection, with the authored count assertion.
use crate::{
    case_view::viewer_session::{DisplayRequest, ViewerSession},
    dataset_shape::read_dataset_shape,
    session_run::{SessionRun, banner},
};
use martensite_tkernel::standard::Failure;
type Outcome = Result<(), Box<dyn std::error::Error>>;

pub(super) fn prepared(session: &mut ViewerSession) -> Result<(), Failure> {
    let shape = read_dataset_shape("cylinder_surface.brep")?;
    session.display("b", &shape, &DisplayRequest::default().with_display_mode(1))?;
    session.fit()?;
    session.display_active_sensitive()
}
/// Check the actual cylinder's sensitive area, then clear selection and dump; `cylinder_sensitive_area()?`.
pub(crate) fn cylinder_sensitive_area() -> Outcome {
    banner(&[
        "============",
        "0033664: Visualization - Selection does not work for simple shape",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    prepared(run.session())?;
    run.session().select_point(200, 200)?;
    if run.session().context().nb_selected() != 1 {
        println!("ERROR: wrong sensitive area");
    }
    run.session().select_point(0, 0)?;
    run.dump("bug33664_1_cylinder.png")?;
    run.finish()
}
#[cfg(test)]
mod cpu {
    use super::*;
    use crate::case_view::SourceViewOutput;
    #[test]
    fn cylinder_fixture_has_one_actual_selected_owner() {
        let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
        let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
        prepared(&mut session).unwrap();
        session.select_point(200, 200).unwrap();
        assert_eq!(session.context().nb_selected(), 1);
        session.select_point(0, 0).unwrap();
        assert_eq!(session.context().nb_selected(), 0);
        session.close();
    }
}
