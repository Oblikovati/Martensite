//! An interactive object drawing one triangle array, as a viewer's primitive-array object:
//! mode 0 draws the array with the drawer's shading aspect, selection mode 0 picks it.
use martensite_tkernel::standard::{Failure, Handle, HandleReference, StandardTransient};
use martensite_tkmath::toploc::Location;
use martensite_tkservice::graphic3d::{
    Graphic3dArrayOfTriangles, Graphic3dGroupMembers as _, Graphic3dStructureMembers as _,
    Graphic3dStructureSource,
};
use martensite_tkv3d::ais::{AisInteractiveObject, AisInteractiveObjectBase};
use martensite_tkv3d::prs_mgr::{
    PrsMgrPresentableObject, PrsMgrPresentableObjectBase, PrsMgrPresentableObjectMembers as _,
    PrsMgrPresentationManager, PrsMgrTypeOfPresentation3d, prs_mgr_presentable_bind,
};
use martensite_tkv3d::select_mgr::{
    SelectMgrEntityOwner, SelectMgrSelectableObject, SelectMgrSelectableObjectBase,
    SelectMgrSelection,
};
use martensite_tkv3d::select3d::Select3DSensitivePrimitiveArray;

/// A displayed triangle array; `TriangleArrayObject::new(triangles)`.
pub struct TriangleArrayObject {
    transient: StandardTransient,
    base: AisInteractiveObjectBase,
    triangles: Handle<Graphic3dArrayOfTriangles>,
}
martensite_tkernel::impl_transient!(TriangleArrayObject, "TriangleArrayObject", dyn AisInteractiveObject, transient,
    interfaces: [dyn AisInteractiveObject, dyn SelectMgrSelectableObject, dyn PrsMgrPresentableObject]);

impl TriangleArrayObject {
    /// The bound object of `triangles`; `TriangleArrayObject::new(cylinder_triangles)`.
    pub fn new(triangles: Handle<Graphic3dArrayOfTriangles>) -> Handle<Self> {
        let object = Handle::new(Self {
            transient: StandardTransient::default(),
            base: AisInteractiveObjectBase::new(PrsMgrTypeOfPresentation3d::AllView),
            triangles,
        });
        prs_mgr_presentable_bind(&object.cast());
        object
    }
}

impl PrsMgrPresentableObject for TriangleArrayObject {
    fn presentable_base(&self) -> &PrsMgrPresentableObjectBase {
        &self.base.presentable
    }
    fn accept_display_mode(&self, mode: i32) -> bool {
        mode == 0
    }
    fn compute(
        &self,
        _: &dyn HandleReference<PrsMgrPresentationManager>,
        prs: &dyn HandleReference<dyn Graphic3dStructureSource>,
        mode: i32,
    ) -> Result<(), Failure> {
        if self.triangles.is_null() || mode != 0 {
            return Ok(());
        }
        let group = prs.observe().new_group()?;
        if let Some(shading) = self.attributes().shading_aspect().get() {
            group.set_group_primitives_aspect(&shading.aspect().cast());
        }
        group.add_primitive_array(&self.triangles.clone().cast(), true);
        Ok(())
    }
    martensite_tkv3d::ais_presentable_overrides!();
}

impl SelectMgrSelectableObject for TriangleArrayObject {
    fn selectable_base(&self) -> &SelectMgrSelectableObjectBase {
        &self.base.selectable
    }
    fn compute_selection(
        &self,
        selection: &dyn HandleReference<SelectMgrSelection>,
        mode: i32,
    ) -> Result<(), Failure> {
        let Some(array) = self.triangles.get().filter(|_| mode == 0) else {
            return Ok(());
        };
        let this: Handle<dyn SelectMgrSelectableObject> = self.presentable_base().this().cast();
        let owner = SelectMgrEntityOwner::with_selectable(&this, 0).into_cast();
        let sensitive = Select3DSensitivePrimitiveArray::new(&owner)?;
        sensitive.init_triangulation_all(
            &array.attributes(),
            &array.indices(),
            &Location::default(),
            true,
            1,
        )?;
        selection.observe().add(&Handle::new(sensitive).cast())
    }
}

impl AisInteractiveObject for TriangleArrayObject {
    fn interactive_base(&self) -> &AisInteractiveObjectBase {
        &self.base
    }
}
