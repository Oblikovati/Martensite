//! Hidden-line removal of the active view (the view's computed mode).
use super::*;
use martensite_tkernel::ncollection::List;
use martensite_tkv3d::prs3d::Prs3dTypeOfHlr;

impl ViewerSession {
    /// Turn hidden-line removal of the active view on or off, choosing the default algorithm
    /// when `algorithm` is given: a changed algorithm
    /// recomputes the displayed shapes when the view already removes hidden lines.
    /// Example: `session.set_hidden_line_removal(true, Some(Prs3dTypeOfHlr::Algo))?`.
    pub fn set_hidden_line_removal(
        &self,
        on: bool,
        algorithm: Option<Prs3dTypeOfHlr>,
    ) -> Result<(), Failure> {
        let drawer = self.context.default_drawer();
        let changed =
            algorithm.filter(|kind| drawer.get().is_some_and(|d| d.type_of_hlr() != *kind));
        if let (Some(kind), Some(drawer)) = (changed, drawer.get()) {
            drawer.set_type_of_hlr(kind);
        }
        let view = self.view();
        if view.view_base().computed_mode() && on && changed.is_some() {
            self.redisplay_shapes()?;
        }
        self.update_view(|view| {
            view.set_computed_mode(on);
            Ok(())
        })
    }

    /// Redisplay every displayed shape presentation without updating the viewer.
    pub(crate) fn redisplay_shapes(&self) -> Result<(), Failure> {
        let mut displayed = List::default();
        self.context.displayed_objects(&mut displayed)?;
        for object in displayed.iter() {
            if !object.cast::<AisShape>().is_null() {
                self.context.redisplay(object, false, false)?;
            }
        }
        Ok(())
    }
}
