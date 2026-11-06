//! Missing mutable style links are diagnostic refusals, never unchecked dereferences.
use super::*;
/// Read the existing default line safely; `default_line(session)?`.
pub(super) fn default_line(
    session: &ViewerSession,
) -> io::Result<Handle<dyn Graphic3dAspectsSource>> {
    let drawer = session.context().default_drawer().clone();
    let drawer = drawer
        .get()
        .ok_or_else(|| refused("default drawer null; expected existing context style"))?;
    let line = drawer.line_aspect();
    let line = line
        .get()
        .ok_or_else(|| refused("default LineAspect null; expected existing line style"))?;
    let aspect = line.aspect();
    if aspect.is_null() {
        return Err(refused(
            "default underlying Aspect null; expected existing graphic line style",
        ));
    }
    Ok(aspect.cast())
}
