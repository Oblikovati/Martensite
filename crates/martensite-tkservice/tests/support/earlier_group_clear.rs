//! A real group callback removes a different earlier node before its own selected-node removal.
use martensite_tkernel::standard::*;
use martensite_tkmath::gp::Axis2;
use martensite_tkservice::{graphic3d::*, null_driver::NullGroup};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct EarlierGroupClear {
    transient: StandardTransient,
    inner: NullGroup,
    earlier: HandleObserver<dyn Graphic3dGroup>,
    armed: AtomicBool,
}
martensite_tkernel::impl_transient!(EarlierGroupClear, "EarlierGroupClear", StandardTransient, transient, interfaces: [dyn Graphic3dGroup]);
impl EarlierGroupClear {
    /// Keep the victim nonowning and explicitly external; `EarlierGroupClear::new(prs, earlier.observe())`.
    pub fn new(
        presentation: &Handle<dyn Graphic3dStructureSource>,
        earlier: HandleObserver<dyn Graphic3dGroup>,
    ) -> Handle<Self> {
        Handle::new(Self {
            transient: StandardTransient::default(),
            inner: NullGroup::new(presentation),
            earlier,
            armed: AtomicBool::new(true),
        })
    }
}
impl Graphic3dGroup for EarlierGroupClear {
    fn group_base(&self) -> &Graphic3dGroupBase {
        // All parent and removal state comes from the real null group.
        self.inner.group_base()
    }
    fn clear(&self, update: bool) {
        if self.armed.swap(false, Ordering::Relaxed) {
            self.earlier.remove();
        }
        self.inner.clear(update);
    }
    fn aspects(&self) -> Handle<dyn Graphic3dAspectsSource> {
        // This fixture owns no separate presentation state.
        self.inner.aspects()
    }
    fn set_group_primitives_aspect(
        &self,
        aspect: &dyn HandleReference<dyn Graphic3dAspectsSource>,
    ) {
        // Unobserved aspect methods retain the real null driver's behavior.
        self.inner.set_group_primitives_aspect(aspect);
    }
    fn set_primitives_aspect(&self, aspect: &dyn HandleReference<dyn Graphic3dAspectsSource>) {
        // The callback is limited to Clear, without replacing other driver operations.
        self.inner.set_primitives_aspect(aspect);
    }
    fn synchronize_aspects(&self) {
        // Delegate required methods to actual driver state instead of stubbing them.
        self.inner.synchronize_aspects();
    }
    fn replace_aspects(&self, replacements: &Graphic3dAspectsReplaceMap) {
        // Preserve the inherited aspect replacement path.
        self.inner.replace_aspects(replacements);
    }
    fn set_stencil_test_options(&self, enabled: bool) {
        // The existing group keeps stencil state and semantics.
        self.inner.set_stencil_test_options(enabled);
    }
    fn set_flipping_options(&self, enabled: bool, plane: &Axis2) {
        // Reuse the actual group implementation for unrelated view state.
        self.inner.set_flipping_options(enabled, plane);
    }
}
