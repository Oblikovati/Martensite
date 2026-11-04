//! Direct access for regressions that build their own interactive objects (datums,
//! trihedrons, dimensions, manipulators) or change view-level settings, inside the same
//! immediate-update boundaries as the session's own operations.
use super::*;

impl ViewerSession {
    /// The interactive context of the viewer; `session.context().selection_style()`.
    pub fn context(&self) -> &Handle<AisInteractiveContext> {
        &self.context
    }

    /// Run `operation` on the context, then update the viewer once, as a command does.
    /// Example: `session.update_context(|context| context.display(&object, false))?`.
    pub fn update_context(
        &self,
        operation: impl FnOnce(&AisInteractiveContext) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        self.with_update(operation)
    }

    /// Batch named-object mutations under one source update; `session.update_session(|s| s.display("b", &shape, &request))?`.
    pub fn update_session(
        &mut self,
        operation: impl FnOnce(&mut Self) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        let view = self.view().clone();
        let context = self.context.clone();
        with_source_view_update(
            &**view,
            || operation(self),
            || context.update_current_viewer(),
        )
    }

    /// Run `operation` on the active view, then redraw it once, as a view command does.
    /// Example: `session.update_view(|view| view.set_axial_scale(1., 3., 10.))?`.
    pub fn update_view(
        &self,
        operation: impl FnOnce(&V3dView) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        let view = self.view();
        with_source_view_update(&**view, || operation(view.view_base()), || view.redraw())
    }

    /// Bind `object` under `name` without displaying it, replacing an earlier binding (the
    /// earlier object leaves the context); `session.bind_object("t", trihedron.cast())?`.
    pub fn bind_object(
        &mut self,
        name: &str,
        object: Handle<dyn AisInteractiveObject>,
    ) -> Result<(), Failure> {
        if let Some(replaced) = self.objects.remove(name)
            && replaced.object_address() != object.object_address()
        {
            self.context.remove(&replaced, false)?;
        }
        self.objects.push(name, object);
        Ok(())
    }
}
