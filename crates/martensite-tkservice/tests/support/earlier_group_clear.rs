//! A real group callback removes a different earlier node before its own selected-node removal.
use martensite_tkernel::standard::*;
#[path = "null_group_members.rs"]
mod null_group_members;
use martensite_tkservice::{graphic3d::*, null_driver::NullGroup};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct EarlierGroupClear {
    inner: NullGroup,
    earlier: WeakHandle<dyn Graphic3dGroup>,
    armed: AtomicBool,
}
martensite_tkernel::impl_transient!(EarlierGroupClear, "EarlierGroupClear", StandardTransient, interfaces: [dyn Graphic3dGroup]);
impl EarlierGroupClear {
    /// Keep the victim nonowning and explicitly external; `EarlierGroupClear::new(prs, earlier.downgrade())`.
    pub fn new(
        presentation: &Handle<dyn Graphic3dStructureSource>,
        earlier: WeakHandle<dyn Graphic3dGroup>,
    ) -> Handle<Self> {
        Handle::new(Self {
            inner: NullGroup::new(presentation),
            earlier,
            armed: AtomicBool::new(true),
        })
    }
}
impl Graphic3dGroup for EarlierGroupClear {
    null_group_members::forward_null_group_members!(inner);
    fn clear(&self, update: bool) {
        if self.armed.swap(false, Ordering::Relaxed) {
            self.earlier.upgrade().remove();
        }
        self.inner.clear(update);
    }
}
