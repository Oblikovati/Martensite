//! Named application subclass keeps native inherited RTTI while overriding the real virtual interfaces.
use super::xr_recording_group::XrRecordingEvents;
use super::*;
use crate::inherited_native_type::inherited_native_type;
use crate::{prs_mgr::*, select_mgr::*};
use martensite_tkernel::standard::HandleReference;
use martensite_tkernel::standard::{Failure, Handle, SharedState};
use martensite_tkservice::{aspect::AspectXRTrackedDeviceRole, graphic3d::*};
use std::sync::Arc;
pub(super) struct XrDerivedFake {
    core: AisXrTrackedDevice,
    pub events: Arc<XrRecordingEvents>,
    pub hiding_calls: SharedState<i32>,
    pub fail_after_compute: SharedState<bool>,
}
impl XrDerivedFake {
    pub fn new(events: Arc<XrRecordingEvents>) -> Handle<Self> {
        let mesh = super::xr_state_fixture::xr_state_model(2);
        let object = Handle::new(Self {
            core: super::xr_tracked_device_object::unbound_xr_tracked_device_model(
                &mesh,
                &Handle::default(),
            )
            .unwrap(),
            events,
            hiding_calls: SharedState::new(0),
            fail_after_compute: SharedState::new(false),
        });
        prs_mgr_presentable_bind(&object.clone().cast());
        object
    }
    pub fn role(&self) -> AspectXRTrackedDeviceRole {
        // A native nonvirtual getter can be hidden by a subclass but never overrides base binding.
        AspectXRTrackedDeviceRole::Head
    }
    pub fn hidden_laser_ray(&self) {
        // Native hiding is deliberately distinct from the nonvirtual qualified base call.
        *self.hiding_calls.write() += 1;
    }
}
inherited_native_type!(
    XrDerivedFake,
    AisXrTrackedDevice,
    "Martensite_XrDerivedFake",
    named,
    [dyn AisXrTrackedDeviceSource, dyn AisInteractiveObject, dyn SelectMgrSelectableObject, dyn PrsMgrPresentableObject]
);
impl AisXrTrackedDeviceSource for XrDerivedFake {
    fn xr_tracked_device_base(&self) -> &AisXrTrackedDeviceBase {
        // Qualified native geometry and nonvirtual members borrow the actual inherited state.
        self.core.xr_tracked_device_base()
    }
}
impl AisInteractiveObject for XrDerivedFake {
    fn interactive_base(&self) -> &AisInteractiveObjectBase {
        // Context/owner links remain shared with the inherited core.
        self.core.interactive_base()
    }
}
impl PrsMgrPresentableObject for XrDerivedFake {
    fn presentable_base(&self) -> &PrsMgrPresentableObjectBase {
        // Bound this() resolves to this application object, as native selection owners do.
        self.core.presentable_base()
    }
    fn accept_display_mode(&self, mode: i32) -> bool {
        // Only the actual virtual accept hook changes; native base acceptance remains mode zero.
        mode == 7 || self.core.accept_display_mode(mode)
    }
    fn compute(
        &self,
        _: &dyn HandleReference<PrsMgrPresentationManager>,
        prs: &dyn HandleReference<dyn Graphic3dStructureSource>,
        mode: i32,
    ) -> Result<(), Failure> {
        self.events.callbacks.write().push(500 + mode);
        ais_xr_tracked_device_compute(self, &prs.observe(), mode)?;
        if *self.fail_after_compute.read() {
            return Err(Failure::new(
                "XR derived compute fail=true; expected completed callback",
            ));
        }
        Ok(())
    }
    crate::ais_presentable_overrides!();
}
impl SelectMgrSelectableObject for XrDerivedFake {
    fn selectable_base(&self) -> &SelectMgrSelectableObjectBase {
        // Native selected children refer back to this object through the shared bound base.
        self.core.selectable_base()
    }
    fn compute_selection(
        &self,
        selection: &dyn HandleReference<SelectMgrSelection>,
        mode: i32,
    ) -> Result<(), Failure> {
        self.events.callbacks.write().push(600 + mode);
        ais_xr_tracked_device_compute_selection(self, &selection.observe(), mode)
    }
}
