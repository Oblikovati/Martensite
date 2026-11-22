//! Custom owner identity and retained per-invocation click state, without a global registry.
use super::object::TutorialObject;
use martensite_tkernel::standard::{Handle, SharedState};
use martensite_tkmath::math::BullardGenerator;
use martensite_tkservice::graphic3d::Graphic3dStructureSource;
use martensite_tkv3d::select_mgr::*;

pub(super) struct TutorialOwner {
    pub base: SelectMgrEntityOwnerBase,
    pub presentation: SharedState<Handle<dyn Graphic3dStructureSource>>,
    pub random: SharedState<BullardGenerator>,
    pub first_click: SharedState<bool>,
}
martensite_tkernel::impl_transient!(TutorialOwner,"MyAisOwner",dyn SelectMgrEntityOwnerSource, interfaces:[dyn SelectMgrEntityOwnerSource]);
impl TutorialOwner {
    pub(super) fn new(object: &Handle<TutorialObject>) -> Handle<Self> {
        let owner = Handle::new(Self {
            base: SelectMgrEntityOwnerBase::new(&object.cast(), 0),
            presentation: SharedState::new(Handle::default()),
            random: SharedState::new(BullardGenerator::default()),
            first_click: SharedState::new(true),
        });
        select_mgr_entity_owner_bind(&owner.cast());
        owner
    }
}
