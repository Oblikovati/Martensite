//! The tutorial's actual virtual presentation and primitive-array selection callbacks.
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::{Failure, Handle, HandleReference, StandardTransient},
};
use martensite_tkmath::{gp::Transform3, toploc::Location};
use martensite_tkservice::{aspect::AspectTypeOfLine, graphic3d::*};
use martensite_tkv3d::{
    ais::*, prs_mgr::*, prs3d::*, select_mgr::*, select3d::Select3DSensitivePrimitiveArray,
};

pub(super) struct TutorialObject {
    transient: StandardTransient,
    base: AisInteractiveObjectBase,
    pub animation: Handle<dyn AisAnimationSource>,
}
martensite_tkernel::impl_transient!(TutorialObject,"MyAisObject",dyn AisInteractiveObject,transient,
    interfaces:[dyn AisInteractiveObject,dyn SelectMgrSelectableObject,dyn PrsMgrPresentableObject]);
impl TutorialObject {
    pub(super) fn new(animation: Handle<dyn AisAnimationSource>) -> Result<Handle<Self>, Failure> {
        let object = Handle::new(Self {
            transient: StandardTransient::default(),
            base: AisInteractiveObjectBase::new(PrsMgrTypeOfPresentation3d::AllView),
            animation,
        });
        prs_mgr_presentable_bind(&object.cast());
        object.configure_style()?;
        Ok(object)
    }
    fn configure_style(&self) -> Result<(), Failure> {
        let drawer = self.attributes();
        drawer.setup_own_shading_aspect(&Handle::default())?;
        drawer.shading_aspect().set_material(
            &Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Silver),
            martensite_tkservice::aspect::AspectTypeOfFacingModel::BothSide,
        );
        drawer.set_wire_aspect(Handle::new(Prs3dLineAspect::new(
            Color::named(NamedColor::GREEN),
            AspectTypeOfLine::Solid,
            2.,
        )?));
        Ok(())
    }
    fn main_presentation(&self, prs: &Handle<dyn Graphic3dStructureSource>) -> Result<(), Failure> {
        let triangles = super::geometry::triangles()?;
        let group = prs.new_group()?;
        group.set_group_primitives_aspect(&self.attributes().shading_aspect().aspect().cast());
        group.add_primitive_array(&triangles.cast(), true);
        group.set_closed(true);
        let section = prs.new_group()?;
        section.set_group_primitives_aspect(&self.attributes().wire_aspect().aspect().cast());
        section.add_primitive_array(&super::geometry::section()?.cast(), true);
        Ok(())
    }
}
impl PrsMgrPresentableObject for TutorialObject {
    fn presentable_base(&self) -> &PrsMgrPresentableObjectBase {
        &self.base.presentable
    }
    fn compute(
        &self,
        _: &dyn HandleReference<PrsMgrPresentationManager>,
        prs: &dyn HandleReference<dyn Graphic3dStructureSource>,
        mode: i32,
    ) -> Result<(), Failure> {
        // Both display modes still construct the same exact BRep cylinder before branching.
        let shape = crate::primitive_shapes::cylinder(100., 100.)?;
        match mode {
            0 => self.main_presentation(&prs.observe()),
            1 => Prs3dBndBox::add_box(
                &prs.observe(),
                &super::geometry::bounds_of(&shape)?,
                &self.attributes(),
            ),
            _ => Ok(()),
        }
    }
    fn accept_display_mode(&self, mode: i32) -> bool {
        mode == 0 || mode == 1
    }
    martensite_tkv3d::ais_presentable_overrides!();
}
impl SelectMgrSelectableObject for TutorialObject {
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
        let _bounds = super::geometry::bounds()?;
        let object = self.presentable_base().this().cast::<TutorialObject>();
        let owner = super::owner::TutorialOwner::new(&object);
        let sensitive = cylinder_sensitive(&owner.cast())?;
        selection.observe().add(&Handle::new(sensitive).cast())
    }
}
impl AisInteractiveObject for TutorialObject {
    fn interactive_base(&self) -> &AisInteractiveObjectBase {
        &self.base
    }
}

fn cylinder_sensitive(
    owner: &Handle<dyn SelectMgrEntityOwnerSource>,
) -> Result<Select3DSensitivePrimitiveArray, Failure> {
    let array = Prs3dToolCylinder::create(100., 100., 100., 25, 25, &Transform3::default())?;
    let sensitive = Select3DSensitivePrimitiveArray::new(owner)?;
    sensitive.init_triangulation_all(
        &array.attributes(),
        &array.indices(),
        &Location::default(),
        true,
        1,
    )?;
    Ok(sensitive)
}
