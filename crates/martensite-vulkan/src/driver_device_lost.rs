//! The device loss of the first ray-traced Redraw (GlView::Redraw and
//! GlGraphicDriver::setDeviceLost): ray tracing needs the primitive arrays kept after their
//! upload (`GlCaps::keepArrayData`), so the first ray-traced Redraw of any view switches it
//! on for the driver and marks the structure manager of every view already redrawn as having
//! lost its device. V3dView::Redraw then clears and recomputes every displayed structure
//! (Graphic3dStructureManager::RecomputeStructures) and redraws once more. Vulkan keeps the
//! arrays in any case; the switch and its device loss are kept for the native sequence.
use martensite_tkernel::standard::{SharedState, WeakHandle};
use martensite_tkservice::graphic3d::{Graphic3dCStructure, Graphic3dCView, Graphic3dCViewMembers};

/// The views and structures a driver created (`myMapOfView`, `myMapOfStructure`), shared with
/// its views; `DriverObjects::default()`.
#[derive(Default)]
pub(crate) struct DriverObjects {
    pub views: SharedState<Vec<WeakHandle<dyn Graphic3dCView>>>,
    pub structures: SharedState<Vec<WeakHandle<dyn Graphic3dCStructure>>>,
}

impl DriverObjects {
    /// `GlGraphicDriver::setDeviceLost`: nothing without structures; otherwise every view
    /// that was redrawn marks its structure manager. Example: `objects.set_device_lost`.
    pub fn set_device_lost(&self) {
        let structures = self.structures.read();
        if !structures.iter().any(|weak| !weak.upgrade().is_null()) {
            return;
        }
        drop(structures);
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
