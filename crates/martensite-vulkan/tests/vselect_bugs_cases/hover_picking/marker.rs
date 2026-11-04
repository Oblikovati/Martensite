use super::*;

pub(super) fn display_no_selection(
    session: &mut ViewerSession,
    name: &str,
    [x, y]: [i32; 2],
) -> Result<(), Failure> {
    // The point displays first and only then deactivates selection.
    reused_datums::point_2d(session, name, [f64::from(x), f64::from(y)])?;
    let point = hover_object(session, name)?;
    session.context().deactivate(&point)
}
