//! The notices that can change the main layers of a view (#594).
//!
//! `redraw_immediate` reuses the retained main image while its view has had no such notice,
//! as OCCT's `RedrawImmediate` reuses its back buffer until `Invalidate`. The structure
//! manager sends them: a structure or group update, a display, an erase, a layer or priority
//! change. A notice for an immediate layer does not count. A notice for every layer
//! (`GRAPHIC3D_ZLAYER_ID_UNKNOWN`) and a layer the view does not have do count. View state
//! that changes in place, such as the camera, has no notice: `immediate_elision` compares it.
use crate::view_statistics::RenderRetainedView;
use martensite_tkernel::standard::{Failure, Handle, HandleReference};
use martensite_tkservice::graphic3d::*;
use std::sync::atomic::{AtomicU64, Ordering};

/// A count that grows with each main-layer notice of one view; `notices.count()`.
#[derive(Default)]
pub(crate) struct MainLayerNotices(AtomicU64);

impl MainLayerNotices {
    /// The notices so far. A retained main image keeps the count read before its frame.
    pub(crate) fn count(&self) -> u64 {
        self.0.load(Ordering::Acquire)
    }

    pub(crate) fn notice(&self) {
        self.0.fetch_add(1, Ordering::AcqRel);
    }
}

/// Forwards a retained-view call after a notice for every main layer.
macro_rules! notice_all_then_forward {
    ($($name:ident($($arg:ident: $ty:ty),*) -> $ret:ty;)*) => {$(
        pub(crate) fn $name(&self, $($arg: $ty),*) -> $ret {
            self.main_notices.notice();
            self.shared_view().$name($($arg),*)
        }
    )*};
}

// The shared forwarding macro calls these through `self.retained`; these inherent methods take
// precedence over the dereferenced ones, as the statistics methods of this wrapper do.
impl RenderRetainedView {
    /// Counts a notice for `layer` unless this view draws it as an immediate layer;
    /// `retained.notice_layer(GRAPHIC3D_ZLAYER_ID_UNKNOWN)`.
    pub(crate) fn notice_layer(&self, layer: Graphic3dZLayerId) {
        let immediate = layer != GRAPHIC3D_ZLAYER_ID_UNKNOWN
            && self
                .shared_view()
                .layer(layer)
                .get()
                .is_some_and(Graphic3dLayer::is_immediate);
        if !immediate {
            self.main_notices.notice();
        }
    }

    fn notice_structure(&self, cstructure: &Handle<dyn Graphic3dCStructure>) {
        match cstructure.get() {
            Some(structure) => self.notice_layer(structure.z_layer()),
            None => self.main_notices.notice(),
        }
    }

    pub(crate) fn display_cstructure(
        &self,
        cstructure: &Handle<dyn Graphic3dCStructure>,
        priority: Graphic3dDisplayPriority,
    ) {
        self.notice_structure(cstructure);
        self.shared_view().display_cstructure(cstructure, priority);
    }

    pub(crate) fn erase_cstructure(&self, cstructure: &Handle<dyn Graphic3dCStructure>) {
        self.notice_structure(cstructure);
        self.shared_view().erase_cstructure(cstructure);
    }

    pub(crate) fn change_cstructure_z_layer(
        &self,
        cstructure: &Handle<dyn Graphic3dCStructure>,
        layer_id: Graphic3dZLayerId,
    ) {
        self.notice_structure(cstructure);
        self.notice_layer(layer_id);
        self.shared_view()
            .change_cstructure_z_layer(cstructure, layer_id);
    }

    pub(crate) fn change_cstructure_priority(
        &self,
        cstructure: &Handle<dyn Graphic3dCStructure>,
        priority: Graphic3dDisplayPriority,
    ) {
        self.notice_structure(cstructure);
        self.shared_view()
            .change_cstructure_priority(cstructure, priority);
    }

    pub(crate) fn invalidate_bvh_data(&self, layer_id: Graphic3dZLayerId) {
        self.notice_layer(layer_id);
        self.shared_view().invalidate_bvh_data(layer_id);
    }

    notice_all_then_forward! {
        invalidate() -> ();
        insert_layer_before(
            new_layer_id: Graphic3dZLayerId,
            settings: &Graphic3dZLayerSettings,
            layer_after: Graphic3dZLayerId
        ) -> Result<(), Failure>;
        insert_layer_after(
            new_layer_id: Graphic3dZLayerId,
            settings: &Graphic3dZLayerSettings,
            layer_before: Graphic3dZLayerId
        ) -> Result<(), Failure>;
        remove_z_layer(layer_id: Graphic3dZLayerId) -> Result<(), Failure>;
        set_z_layer_settings(
            layer_id: Graphic3dZLayerId,
            settings: &Graphic3dZLayerSettings
        ) -> Result<(), Failure>;
        set_gradient_background(background: &martensite_tkservice::aspect::AspectGradientBackground) -> ();
        set_background_image_style(style: martensite_tkservice::aspect::AspectFillMethod) -> ();
        set_texture_env(texture_env: &Handle<Graphic3dTextureEnv>) -> ();
        set_lights(lights: &dyn HandleReference<Graphic3dLightSet>) -> ();
        set_clip_planes(
            planes: &dyn HandleReference<dyn Graphic3dClipPlaneSequenceSource>
        ) -> ();
    }
}

#[cfg(test)]
#[path = "main_layer_notices_tests.rs"]
mod tests;
