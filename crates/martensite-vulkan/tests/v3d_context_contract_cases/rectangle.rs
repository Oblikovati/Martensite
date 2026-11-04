use crate::case_view::SourceViewOutput;
use crate::{
    case_view::viewer_session::ViewerSession,
    primitive_shapes::{box_at, box_sized},
    session_run::{SessionRun, banner},
};
use martensite_tkernel::{ncollection::Vec2, standard::Failure};
use martensite_tkv3d::ais::{AisSelectionScheme, AisStatusOfPick};
use std::error::Error;

/// Observe the returned pick status, never infer it from counts; `rectangle_pick(session)?`.
pub(super) fn rectangle_pick(session: &ViewerSession) -> Result<AisStatusOfPick, Failure> {
    let (width, height) = session.source_window_size();
    session.context().select_rectangle(
        Vec2::new(0, 0),
        Vec2::new(width, height),
        session.view(),
        AisSelectionScheme::Replace,
    )
}

fn pick_report(actual: AisStatusOfPick, expected: AisStatusOfPick, label: &str) {
    let result = if actual == expected {
        "OK"
    } else {
        "bugged - Faulty "
    };
    println!("status = INTERACTIVE_SOP_{label} : {result}");
}

/// Expand every command evaluated by the QA function, without fitting between picks.
pub(super) fn rectangle_sequence(session: &mut ViewerSession) -> Result<(), Failure> {
    report_rectangle(session, AisStatusOfPick::NothingSelected, "NothingSelected")?;
    session.display("b", &box_sized([10.; 3])?, &Default::default())?;
    report_rectangle(session, AisStatusOfPick::OneSelected, "OneSelected")?;
    session.display("w", &box_at([20.; 3], [20.; 3])?, &Default::default())?;
    let actual = rectangle_pick(session)?;
    session.context().update_current_viewer();
    pick_report(actual, AisStatusOfPick::SeveralSelected, "SeveralSelected");
    Ok(())
}

/// Replay the complete authored regression; `rectangle()?`.
pub(crate) fn rectangle() -> Result<(), Box<dyn Error>> {
    banner(&["========", "BUC60774", "========", ""]);
    let mut run = SessionRun::new()?;
    // The outer catch suppresses a command failure, then still fits and dumps.
    let _ = rectangle_sequence(run.session());
    run.session().fit()?;
    run.dump("buc60774.png")?;
    run.finish()
}

fn report_rectangle(
    session: &ViewerSession,
    expected: AisStatusOfPick,
    label: &str,
) -> Result<(), Failure> {
    pick_report(rectangle_pick(session)?, expected, label);
    Ok(())
}
