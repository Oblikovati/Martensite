//! The view a result check dumps, shared by the debug and release hosts.
use crate::case_view::viewer_session::DisplayRequest;
use crate::session_run::SessionRun;
use martensite_tkbrep::topods::Shape;
use std::error::Error;

/// The view is cleared, then shows only `result`, shaded
/// and fitted, dumped as image `image`; `display_result_view(&mut run, &result, "a.png")?`.
pub(crate) fn display_result_view(
    run: &mut SessionRun,
    result: &Shape,
    image: &'static str,
) -> Result<(), Box<dyn Error>> {
    run.session().clear()?;
    run.session()
        .display("result", result, &DisplayRequest::default())?;
    run.session().set_display_mode(1)?;
    run.session().fit()?;
    run.dump(image)
}
