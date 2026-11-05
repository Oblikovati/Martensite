//! Inspect the actual default argument checker; no cached diagnostic classification.
use crate::case_argument_response::default_argument_response;
use crate::case_view::viewer_session::DisplayRequest;
use crate::dataset_shape::read_dataset_shape;
use crate::session_run::{SessionRun, banner};
#[cfg(test)]
mod cpu;
type Outcome = Result<(), Box<dyn std::error::Error>>;
const FAULTY_PHRASE: &str = "Faulties, that can not be treated by BOP, are detected";

pub(super) fn diagnostic_message(expected_faulty: bool, diagnostic: &str) -> &'static str {
    match (expected_faulty, diagnostic.contains(FAULTY_PHRASE)) {
        (true, true) => "OK : message is correct",
        (true, false) => "Error : message is not correct",
        (false, true) => "Error : Wrong result",
        (false, false) => "OK : Good result",
    }
}

fn case_case(expected_faulty: bool) -> (&'static str, &'static str, &'static str) {
    if expected_faulty {
        return ("OCC24220", "bug24220_A.brep", "bug24220.png");
    }
    ("OCC24654", "bug24654_qf.brep", "bug24654.png")
}

/// Run the complete default analyzer, preserve the exact authored phrase predicate and view.
/// Example: `argument_diagnostic_view(true)?`.
pub(crate) fn argument_diagnostic_view(expected_faulty: bool) -> Outcome {
    let (number, fixture, image) = case_case(expected_faulty);
    banner(&["============", number, "============", ""]);
    let result = read_dataset_shape(fixture)?;
    println!(
        "{}",
        diagnostic_message(expected_faulty, default_argument_response(&result))
    );
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session().set_display_mode(1)?;
    run.session().fit()?;
    run.dump_and_finish(image, false)
}
