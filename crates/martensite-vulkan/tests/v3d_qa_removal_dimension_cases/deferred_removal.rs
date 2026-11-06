//! OCC204 zero retains the exact deferred Remove update argument.
use super::Outcome;
use crate::session_run::{SessionRun, banner};
use crate::v3d_qa_command_cases::selected_removal::{prepare_selected_boxes, remove_selection};

/// Full original including both literal known-bug expectations; `deferred_selected_removal()?`.
pub(crate) fn deferred_selected_removal() -> Outcome {
    banner(&[
        "TODO OCC12345 ALL: Error : 3 boxes missing in the viewer!",
        "TODO OCC12345 ALL: Error : Colors are not equal in default coordinate and in the near coordinates too",
        "========",
        "OCC204",
        "========",
    ]);
    let mut run = SessionRun::new()?;
    prepare_selected_boxes(run.session().context())?;
    remove_selection(run.session().context(), false)?;
    run.session().fit()?;
    if !run.session().check_color(150, 200, [0.8; 3])? {
        println!("Error : 3 boxes missing in the viewer!");
    }
    run.dump("bug204_1.png")?;
    run.finish()
}
