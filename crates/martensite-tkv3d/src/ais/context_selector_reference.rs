//! MainSelector binds the original manager's actual slot, without a context lock.
use super::AisInteractiveContext;
use crate::select_mgr::{SelectMgrSelectionManager, SelectMgrViewerSelectorSource};
use martensite_tkernel::standard::{Handle, HandleObserver, HandleReference};
use std::ops::Deref;
/// Retain the original selector owner so its actual selector slot stays readable.
/// Example: `let reference=context.main_selector(); reference.observe().nb_picked()`.
///
/// Native `MainSelector` borrows the selector slot inside the manager. A Rust borrow of that
/// slot cannot survive a later replacement of the context's manager field without an owner,
/// so this reference is one real manager owner while it lives (#592). The selector slot is
/// still borrowed: selector counts and slot identity stay native; the manager count is one
/// higher than native until the reference drops.
pub struct AisMainSelectorRef {
    manager: Handle<SelectMgrSelectionManager>,
}
impl AisMainSelectorRef {
    pub(super) fn from_context(context: &AisInteractiveContext) -> Self {
        // Return the selection manager's selector, not a projection through the context.
        let manager = context.fields.read().selection_manager.clone();
        Self { manager }
    }
}
impl Deref for AisMainSelectorRef {
    type Target = Handle<dyn SelectMgrViewerSelectorSource>;
    fn deref(&self) -> &Self::Target {
        // Projecting through the current context would silently retarget this native reference.
        self.manager.selector()
    }
}
impl HandleReference<dyn SelectMgrViewerSelectorSource> for AisMainSelectorRef {
    fn observe(&self) -> HandleObserver<'_, dyn SelectMgrViewerSelectorSource> {
        // Borrow the retained manager's slot: selector ownership does not change.
        self.manager.selector().observe()
    }
    fn reference_address(&self) -> *const () {
        // Equal targets can occupy distinct handle slots, so target identity is insufficient.
        std::ptr::from_ref(self.manager.selector()).cast()
    }
}

#[cfg(test)]
#[path = "context_selector_reference_tests.rs"]
mod tests;
