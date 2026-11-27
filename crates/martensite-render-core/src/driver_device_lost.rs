//! The device loss of the first ray-traced Redraw (GlView::Redraw and
//! GlGraphicDriver::setDeviceLost): ray tracing needs the primitive arrays kept after their
//! upload (`GlCaps::keepArrayData`), so the first ray-traced Redraw of any view switches it
//! on for the driver and marks the structure manager of every view already redrawn as having
//! lost its device. V3dView::Redraw then clears and recomputes every displayed structure
//! (Graphic3dStructureManager::RecomputeStructures) and redraws once more. Vulkan keeps the
//! arrays in any case; the switch and its device loss are kept for the native sequence.
use martensite_tkernel::standard::{Handle, SharedState, WeakHandle};
use martensite_tkservice::graphic3d::{Graphic3dCStructure, Graphic3dCView, Graphic3dCViewMembers};
use std::collections::HashMap;
use std::sync::Arc;

/// The views and structures a driver created (`myMapOfView`, `myMapOfStructure`), shared with
/// its views; `DriverObjects::default()`.
#[derive(Default)]
pub(crate) struct DriverObjects {
    pub views: SharedState<Vec<WeakHandle<dyn Graphic3dCView>>>,
    structures: SharedState<HashMap<usize, WeakHandle<dyn Graphic3dCStructure>>>,
    connection_revision: SharedState<Arc<()>>,
}

impl DriverObjects {
    pub(crate) fn connection_revision(&self) -> Arc<()> {
        self.connection_revision.read().clone()
    }

    pub(crate) fn connections_match(&self, revision: &Arc<()>) -> bool {
        Arc::ptr_eq(&self.connection_revision.read(), revision)
    }

    pub(crate) fn connection_changed(&self) {
        *self.connection_revision.write() = Arc::new(());
    }

    pub(crate) fn register_structure(&self, structure: &Handle<dyn Graphic3dCStructure>) {
        self.structures
            .write()
            .insert(structure.object_address().addr(), structure.downgrade());
    }

    pub(crate) fn forget_structure(&self, address: usize) {
        self.structures.write().remove(&address);
    }

    fn has_live_structure(&self) -> bool {
        self.structures
            .read()
            .values()
            .any(|structure| structure.ref_count() != 0)
    }

    /// `GlGraphicDriver::setDeviceLost`: nothing without structures; otherwise every view
    /// that was redrawn marks its structure manager. Example: `objects.set_device_lost`.
    pub fn set_device_lost(&self) {
        if !self.has_live_structure() {
            return;
        }
        let views: Vec<_> = self.views.read().iter().map(WeakHandle::upgrade).collect();
        for view in views.iter().filter(|view| !view.is_null()) {
            let render = view.cast::<crate::Graphic3dRenderView>();
            let manager = view.structure_manager();
            if render.get().is_some_and(|render| render.was_redrawn())
                && let Some(manager) = manager.get()
            {
                manager.set_device_lost();
            }
        }
    }
}

impl crate::Graphic3dRenderView {
    /// The start of GlView::Redraw: a ray-traced Redraw while the driver does not keep its
    /// primitive arrays turns `keep_array_data` on and loses the device of the redrawn views.
    pub(crate) fn keep_ray_traced_arrays(&self) {
        let ray_traced = self.rendering_params().method
            == martensite_tkservice::graphic3d::Graphic3dRenderingMode::Raytracing;
        if !ray_traced || self.options.read().keep_array_data {
            return;
        }
        // Caps are shared across all views of the driver, so all of them are invalidated.
        self.driver_objects.set_device_lost();
        self.options.write().keep_array_data = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::{FrameCapture, Graphic3dRenderDriver, RenderBackend, RenderError, RenderFrame};
    use martensite_tkernel::standard::Handle;
    use martensite_tkservice::graphic3d::{Graphic3dGraphicDriver, Graphic3dStructureManager};

    struct RegistryOnly;

    impl RenderBackend for RegistryOnly {
        fn render(&mut self, _: &RenderFrame) -> Result<FrameCapture, RenderError> {
            panic!("the registry test never renders a frame")
        }
    }

    #[test]
    fn explicit_removal_and_last_owner_drop_forget_only_their_own_entries() {
        let driver = Graphic3dRenderDriver::with_backend(RegistryOnly, 32).unwrap();
        let manager = Handle::new(Graphic3dStructureManager::new(&driver.cast()).unwrap());
        let mut removed = driver.create_structure(&manager).unwrap();
        let retained_handle = removed.clone();
        let dropped = driver.create_structure(&manager).unwrap();
        assert_eq!(driver.objects.structures.read().len(), 2);

        driver.remove_structure(&mut removed);
        assert!(removed.is_null());
        assert_eq!(driver.objects.structures.read().len(), 1);
        drop(retained_handle);
        assert!(driver.objects.has_live_structure());
        drop(dropped);
        assert!(driver.objects.structures.read().is_empty());
        assert!(!driver.objects.has_live_structure());
    }
}
