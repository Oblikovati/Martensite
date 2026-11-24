//! MainSelector binds the original manager's actual slot, without a context lock.
use super::AisInteractiveContext;
use crate::select_mgr::{SelectMgrSelectionManager, SelectMgrViewerSelectorSource};
use martensite_tkernel::standard::{Handle, HandleObserver, HandleReference};
use std::{ops::Deref, sync::Arc};
/// Retain the selection manager's original selector slot across context edits.
/// Example: `let reference=context.main_selector(); reference.observe().nb_picked()`.
///
/// The shared slot keeps the selector alive without adding an object handle or retaining the
/// manager. Context replacement cannot retarget the reference.
pub struct AisMainSelectorRef {
    selector: Arc<Handle<dyn SelectMgrViewerSelectorSource>>,
}
impl AisMainSelectorRef {
    pub(super) fn from_context(context: &AisInteractiveContext) -> Self {
        let fields = context.fields.read();
        let selector = fields.selection_manager.get().map_or_else(
            || Arc::new(Handle::default()),
            SelectMgrSelectionManager::selector_slot,
        );
        Self { selector }
    }
}
impl Deref for AisMainSelectorRef {
    type Target = Handle<dyn SelectMgrViewerSelectorSource>;
    fn deref(&self) -> &Self::Target {
        &self.selector
    }
}
impl HandleReference<dyn SelectMgrViewerSelectorSource> for AisMainSelectorRef {
    fn observe(&self) -> HandleObserver<'_, dyn SelectMgrViewerSelectorSource> {
        self.selector.observe()
    }
    fn reference_address(&self) -> *const () {
        // Equal targets can occupy distinct handle slots, so target identity is insufficient.
        Arc::as_ptr(&self.selector).cast()
    }
}

#[cfg(test)]
#[path = "context_selector_reference_tests.rs"]
mod tests;
