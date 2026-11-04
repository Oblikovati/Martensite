//! Keep the authored QA operation order and exact color predicates.
mod axial;
#[cfg(test)]
mod controls;
mod highlight;
mod modes;
mod rectangle;
pub(crate) use axial::axial;
pub(crate) use highlight::highlight;
pub(crate) use modes::mode;
pub(crate) use rectangle::rectangle;

use crate::case_view::viewer_session::ViewerSession;
use martensite_tkernel::standard::Failure;

/// Preserve the named-pixel equality and authored diagnostic; `named_pixel(session, 1, 1, "YELLOW", "...")?`.
fn named_pixel(
    session: &ViewerSession,
    x: i32,
    y: i32,
    color: &str,
    diagnostic: &str,
) -> Result<(), Failure> {
    if session.pixel_color_name(x, y)? != color {
        println!("{diagnostic}");
    }
    Ok(())
}
