//! The explicit -noupdate scope restores source ownership without scheduling a redraw.
use super::*;

struct DeferredSourceUpdates {
    view: Handle<dyn V3dViewSource>,
    previous: bool,
}

impl DeferredSourceUpdates {
    fn new(view: Handle<dyn V3dViewSource>) -> Self {
        let previous = view.set_immediate_update(false);
        Self { view, previous }
    }
}

impl Drop for DeferredSourceUpdates {
    fn drop(&mut self) {
        self.view.set_immediate_update(self.previous);
    }
}

impl ViewerSession {
    /// Preserve -noupdate across a whole command loop; `session.without_updates(|s| s.clear())?`.
    pub fn without_updates<T, E>(
        &mut self,
        operation: impl FnOnce(&mut Self) -> Result<T, E>,
    ) -> Result<T, E> {
        let _scope = DeferredSourceUpdates::new(self.view().clone());
        operation(self)
    }

    /// Preserve a source window origin without resizing it; `session.set_window_origin(32,32)`.
    pub fn set_window_origin(&self, x: i32, y: i32) {
        self.views[self.active].0.set_position(x, y);
    }
}

#[cfg(test)]
#[path = "deferred_updates_tests.rs"]
mod tests;
