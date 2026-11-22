// SPDX-License-Identifier: GPL-2.0-only
//! Derived Assign dispatch uses the actual owned base and the original caller slot.
use super::MeshVSDrawer;
use martensite_tkernel::standard::{
    Handle, HandleObserver, HandleReference, RuntimeType, RuntimeTyped, Transient,
};
/// Extend actual storage; `source.drawer_base().set_integer(7,42)`.
pub trait MeshVSDrawerSource: Transient {
    /// Lend the actual owned drawer for qualified storage operations; `source.owned_drawer().set_integer(7, 42)`.
    fn owned_drawer(&self) -> &MeshVSDrawer;
    /// Extension accessor; `source.drawer_base()` does not control qualified operations.
    fn drawer_base(&self) -> &MeshVSDrawer {
        self.owned_drawer()
    }
    /// Virtual Assign; `source.assign(&other)` preserves borrowed argument counts.
    fn assign(&self, other: &dyn HandleReference<dyn MeshVSDrawerSource>) {
        meshvs_drawer_assign(self.owned_drawer(), other);
    }
}
impl MeshVSDrawerSource for MeshVSDrawer {
    fn owned_drawer(&self) -> &MeshVSDrawer {
        self
    }
}
impl RuntimeTyped for dyn MeshVSDrawerSource {
    const TYPE_NAME: &str = "MeshVSDrawer";
    fn static_type() -> &'static RuntimeType {
        MeshVSDrawer::static_type()
    }
}
martensite_tkernel::impl_transient_interface_target!(dyn MeshVSDrawerSource);
impl std::ops::Deref for dyn MeshVSDrawerSource {
    type Target = MeshVSDrawer;
    fn deref(&self) -> &MeshVSDrawer {
        self.owned_drawer()
    }
}
impl HandleReference<dyn MeshVSDrawerSource> for Handle<MeshVSDrawer> {
    fn observe(&self) -> HandleObserver<'_, dyn MeshVSDrawerSource> {
        HandleObserver::Snapshot(self.upcast())
    }
    fn reference_address(&self) -> *const () {
        std::ptr::from_ref(self).cast()
    }
    fn object_address(&self) -> *const () {
        // A handle reads the original pointer, without projecting a dying pointee.
        Handle::object_address(self)
    }
    fn ref_count(&self) -> usize {
        Handle::ref_count(self)
    }
    fn is_null(&self) -> bool {
        // Metadata remains meaningful after a virtual Delete failure at count zero.
        Handle::is_null(self)
    }
}
/// Execute the qualified base assignment; `meshvs_drawer_assign(&drawer,&other)`.
pub fn meshvs_drawer_assign(
    target: &MeshVSDrawer,
    other: &dyn HandleReference<dyn MeshVSDrawerSource>,
) {
    if other.is_null() {
        return;
    }
    assign_channel(target, other, |base| &base.maps.integers);
    assign_channel(target, other, |base| &base.maps.doubles);
    assign_channel(target, other, |base| &base.maps.booleans);
    assign_channel(target, other, |base| &base.maps.colors);
    assign_channel(target, other, |base| &base.maps.materials);
    assign_channel(target, other, |base| &base.maps.strings);
}
fn assign_channel<T: Clone>(
    target: &MeshVSDrawer,
    other: &dyn HandleReference<dyn MeshVSDrawerSource>,
    select: fn(
        &MeshVSDrawer,
    ) -> &martensite_tkernel::standard::SharedState<
        martensite_tkernel::ncollection::DataMap<i32, T>,
    >,
) {
    let observed = other.observe();
    let Some(source) = observed.get() else {
        return;
    };
    let source = source.owned_drawer();
    if std::ptr::eq(target, source) {
        return;
    }
    select(target).write().assign(&select(source).read());
}
