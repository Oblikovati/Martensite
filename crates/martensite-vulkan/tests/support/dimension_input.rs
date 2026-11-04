//! Shared font registration and point presentation inputs for dimension cases.
use crate::session_run::SessionRun;
use martensite_tkernel::standard::Handle;
use std::error::Error;

type Outcome = Result<(), Box<dyn Error>>;

/// Register DejaVuSans.ttf as SansFont, resolved to the system copy as the cases do.
pub(super) fn register_sans_font() -> Outcome {
    let path = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";
    crate::case_view::viewer_session::register_font_file(path, "SansFont")?;
    Ok(())
}

/// A point marker `name` at `position`, as the point command displays it.
pub(super) fn display_point(run: &mut SessionRun, name: &str, [x, y, z]: [f64; 3]) -> Outcome {
    use martensite_tkg3d::geom::GeomCartesianPoint;
    use martensite_tkv3d::ais::AisPoint;
    let point = Handle::new(GeomCartesianPoint::new(x, y, z));
    run.session()
        .display_object(name, AisPoint::new(&point.cast())?.cast())?;
    Ok(())
}
