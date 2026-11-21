//! Native owners force detection highlight and dispatch every mouse click to the widget.
use super::{AisViewCube, AisViewCubeSource};
use crate::{select_mgr::*, v3d::V3dTypeOfOrientation};
use martensite_tkernel::{
    ncollection::Vec2,
    standard::{Handle, SharedState, raise_native_failure},
};
use martensite_tkservice::aspect::{AspectVKeyFlags, AspectVKeyMouse};
/// Owner identity and orientation; `owner.main_orientation()`.
pub struct AisViewCubeOwner {
    base: SelectMgrEntityOwnerBase,
    /// Native protected orientation, retained for picking and camera dispatch.
    pub orientation: SharedState<V3dTypeOfOrientation>,
}
martensite_tkernel::impl_transient!(AisViewCubeOwner,"AisViewCubeOwner",dyn SelectMgrEntityOwnerSource,
    interfaces:[dyn SelectMgrEntityOwnerSource]);
impl AisViewCubeOwner {
    /// Own a part with native priority five; `AisViewCubeOwner::new(&cube,orientation,5)`.
    pub fn new(
        cube: &Handle<AisViewCube>,
        orientation: V3dTypeOfOrientation,
        priority: i32,
    ) -> Handle<Self> {
        Self::with_source(&cube.upcast(), orientation, priority)
    }
    /// Retain a real widget subclass; `AisViewCubeOwner::with_source(&subclass.cast(),orientation,5)`.
    pub fn with_source(
        cube: &Handle<dyn AisViewCubeSource>,
        orientation: V3dTypeOfOrientation,
        priority: i32,
    ) -> Handle<Self> {
        let owner = Handle::new(Self {
            base: SelectMgrEntityOwnerBase::new(&cube.upcast(), priority),
            orientation: SharedState::new(orientation),
        });
        owner.base.fields.write().from_decomposition = true;
        select_mgr_entity_owner_bind(&owner.upcast());
        owner
    }
    /// Read the part's target orientation; `owner.main_orientation()`.
    pub fn main_orientation(&self) -> V3dTypeOfOrientation {
        *self.orientation.read()
    }
}
impl SelectMgrEntityOwnerSource for AisViewCubeOwner {
    fn owner_base(&self) -> &SelectMgrEntityOwnerBase {
        &self.base
    }
    fn is_forced_hilight(&self) -> bool {
        true
    }
    fn handle_mouse_click(
        &self,
        _: Vec2<i32>,
        _: AspectVKeyMouse,
        _: AspectVKeyFlags,
        _: bool,
    ) -> bool {
        let cube = cube_owner_selectable_lease(self);
        let Some(cube) = cube.get() else {
            return false;
        };
        // Native returns true only after HandleClick; exceptions cross this boolean virtual hook.
        let owner = cube_owner_click_argument(self);
        raise_native_failure(cube.handle_click(&owner));
        true
    }
}

fn cube_owner_click_argument(owner: &AisViewCubeOwner) -> Handle<AisViewCubeOwner> {
    owner.base.this().into_downcast()
}

fn cube_owner_selectable_lease(owner: &AisViewCubeOwner) -> Handle<dyn AisViewCubeSource> {
    // Application click dispatch can retire the selectable, so retain a real receiver scope.
    owner.selectable().into_cast::<dyn AisViewCubeSource>()
}
