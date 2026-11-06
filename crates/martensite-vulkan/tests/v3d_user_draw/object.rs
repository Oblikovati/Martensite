//! Presentation and sensitive curve share the exact interactive-object identity.
use super::element::{UserDrawElement, UserDrawPayload, user_draw_positions};
use martensite_tkernel::standard::{Failure, Handle, HandleReference, StandardTransient};
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::{ais::*, prs_mgr::*, select_mgr::*, select3d::Select3DSensitiveCurve};
use martensite_vulkan::Graphic3dDrawGroup;
use std::sync::Arc;

pub(super) struct UserDrawObject {
    transient: StandardTransient,
    base: AisInteractiveObjectBase,
    pub payload: Arc<UserDrawPayload>,
}
martensite_tkernel::impl_transient!(UserDrawObject,"VUserDrawObj",dyn AisInteractiveObject,transient,
    interfaces:[dyn AisInteractiveObject,dyn SelectMgrSelectableObject,dyn PrsMgrPresentableObject]);
impl UserDrawObject {
    /// Bind actual virtual presentation/selection callbacks; `UserDrawObject::new()`.
    pub fn new() -> Handle<Self> {
        let object = Handle::new(Self {
            transient: StandardTransient::default(),
            base: AisInteractiveObjectBase::new(PrsMgrTypeOfPresentation3d::AllView),
            payload: Arc::default(),
        });
        prs_mgr_presentable_bind(&object.cast());
        object
    }
    fn install_callback(
        &self,
        presentation: &Handle<dyn Graphic3dStructureSource>,
    ) -> Result<(), Failure> {
        presentation.clear(true);
        let group = presentation.new_group()?;
        group.set_min_max_values(self.payload.coordinates.read().map(f64::from));
        group.set_group_primitives_aspect(&self.attributes().line_aspect().aspect().cast());
        let actual = group.cast::<Graphic3dDrawGroup>();
        let actual = actual
            .get()
            .ok_or_else(|| unsupported_user_draw_group(&group))?;
        actual.add_element(Arc::new(UserDrawElement(self.payload.clone())));
        Ok(())
    }
}
fn unsupported_user_draw_group(group: &Handle<dyn Graphic3dGroup>) -> Failure {
    let kind = group.dynamic_type().map(|t| t.name().to_owned());
    Failure::with_message(
        "Standard_Failure",
        &format!("user-draw group {kind:?}; expected the Martensite rendering driver"),
    )
}
impl PrsMgrPresentableObject for UserDrawObject {
    fn presentable_base(&self) -> &PrsMgrPresentableObjectBase {
        &self.base.presentable
    }
    fn compute(
        &self,
        manager: &dyn HandleReference<PrsMgrPresentationManager>,
        presentation: &dyn HandleReference<dyn Graphic3dStructureSource>,
        mode: i32,
    ) -> Result<(), Failure> {
        if mode != 0 {
            return Ok(());
        }
        self.install_callback(&presentation.observe())?;
        manager
            .observe()
            .structure_manager()
            .update(GRAPHIC3D_ZLAYER_ID_UNKNOWN);
        Ok(())
    }
    martensite_tkv3d::ais_presentable_overrides!();
}
impl SelectMgrSelectableObject for UserDrawObject {
    fn selectable_base(&self) -> &SelectMgrSelectableObjectBase {
        &self.base.selectable
    }
    fn compute_selection(
        &self,
        selection: &dyn HandleReference<SelectMgrSelection>,
        mode: i32,
    ) -> Result<(), Failure> {
        if mode != 0 {
            return Ok(());
        }
        let owner =
            SelectMgrEntityOwner::with_selectable(&self.presentable_base().this().cast(), 0);
        let mut points = user_draw_positions(*self.payload.coordinates.read()).to_vec();
        points.push(points[0]);
        let curve = Select3DSensitiveCurve::from_points(&owner.cast(), &points)?;
        selection.observe().add(&Handle::new(curve).cast())
    }
}
impl AisInteractiveObject for UserDrawObject {
    fn interactive_base(&self) -> &AisInteractiveObjectBase {
        &self.base
    }
}
