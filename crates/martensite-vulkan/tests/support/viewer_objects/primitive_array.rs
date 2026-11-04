// Typed stand-in for the viewer command's private MyPArrayObject (ViewerObjectCommands, built from an existing array as the triangle and
// segment commands build it; the command crate keeps the object private.

//! `MyPArrayObject` of an array: drawn with the shading aspect, selected per triangle set or
//! per segment.
use martensite_tkernel::quantity::Color;
use martensite_tkernel::standard::{Failure, Handle, HandleReference, StandardTransient};
use martensite_tkmath::toploc::Location;
use martensite_tkservice::aspect::AspectTypeOfFacingModel;
use martensite_tkservice::graphic3d::{
    Graphic3dArrayOfPrimitives, Graphic3dArrayOfSegments, Graphic3dArrayOfTriangles,
    Graphic3dAspectsReplaceMap, Graphic3dAspectsSource, Graphic3dGroupMembers as _,
    Graphic3dPresentationAttributesSource as _, Graphic3dStructureMembers as _,
    Graphic3dStructureSource,
};
use martensite_tkv3d::ais::{AisInteractiveObject, AisInteractiveObjectBase};
use martensite_tkv3d::prs_mgr::{
    PrsMgrPresentableObject, PrsMgrPresentableObjectBase, PrsMgrPresentableObjectMembers as _,
    PrsMgrPresentableObjectUpdates as _, PrsMgrPresentationManager, PrsMgrTypeOfPresentation3d,
    prs_mgr_presentable_bind,
};
use martensite_tkv3d::select_mgr::{
    SelectMgrEntityOwner, SelectMgrEntityOwnerSource, SelectMgrSelectableObject,
    SelectMgrSelectableObjectBase, SelectMgrSelection,
};
use martensite_tkv3d::select3d::{
    Select3DSensitiveEntityMembers as _, Select3DSensitivePrimitiveArray, Select3DSensitiveSegment,
};

/// `MyPArrayObject`: an interactive object of a primitive array.
pub struct PrimitiveArrayObject {
    transient: StandardTransient,
    base: AisInteractiveObjectBase,
    parray: Handle<Graphic3dArrayOfPrimitives>,
}
martensite_tkernel::impl_transient!(PrimitiveArrayObject, "MyPArrayObject", dyn AisInteractiveObject, transient,
    interfaces: [dyn AisInteractiveObject, dyn SelectMgrSelectableObject, dyn PrsMgrPresentableObject]);

impl PrimitiveArrayObject {
    /// `MyPArrayObject(thePArray)`; `PrimitiveArrayObject::new(array)`.
    pub fn new(parray: Handle<Graphic3dArrayOfPrimitives>) -> Handle<Self> {
        let object = Handle::new(Self {
            transient: StandardTransient::default(),
            base: AisInteractiveObjectBase::new(PrsMgrTypeOfPresentation3d::AllView),
            parray,
        });
        prs_mgr_presentable_bind(&object.cast());
        object
    }

    /// `replaceShadingAspect`: the linked shading aspect replaced by the own one.
    fn replace_shading_aspect(&self) {
        let drawer = self.attributes();
        let link = drawer.link();
        let Some(link) = link.get() else {
            return;
        };
        let old: Handle<dyn Graphic3dAspectsSource> = link
            .shading_aspect()
            .get()
            .map(|s| s.aspect().cast())
            .unwrap_or_default();
        let new: Handle<dyn Graphic3dAspectsSource> = drawer
            .shading_aspect()
            .get()
            .map(|s| s.aspect().cast())
            .unwrap_or_default();
        let mut map = Graphic3dAspectsReplaceMap::default();
        map.insert(old.object_address() as usize, (old, new));
        self.replace_aspects(&map);
    }

    /// One sensitive segment per segment of the array, sensitivity 4.
    fn add_segments(
        &self,
        array: &Graphic3dArrayOfPrimitives,
        owner: &Handle<dyn SelectMgrEntityOwnerSource>,
        selection: &SelectMgrSelection,
    ) -> Result<(), Failure> {
        let by_edges = array.edge_number() > 0;
        let count = if by_edges {
            array.edge_number()
        } else {
            array.vertex_number()
        };
        let mut rank = 1;
        while rank <= count {
            let (a, b) = if by_edges {
                (array.edge(rank)?, array.edge(rank + 1)?)
            } else {
                (rank, rank + 1)
            };
            let segment =
                Select3DSensitiveSegment::new(owner, array.vertice(a)?, array.vertice(b)?);
            segment.set_sensitivity_factor(4)?;
            selection.add(&Handle::new(segment).cast())?;
            rank += 2;
        }
        Ok(())
    }
}

impl PrsMgrPresentableObject for PrimitiveArrayObject {
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
        if self.parray.is_null() || mode != 0 {
            return Ok(());
        }
        let group = prs.observe().new_group()?;
        if let Some(shading) = self.attributes().shading_aspect().get() {
            group.set_group_primitives_aspect(&shading.aspect().cast());
        }
        group.add_primitive_array(&self.parray, true);
        Ok(())
    }
    /// `SetColor`: the own color, then the shading color.
    fn set_color(&self, color: Color) -> Result<(), Failure> {
        self.presentable_base().read().drawer.set_color(color);
        self.presentable_base().fields.write().has_own_color = true;
        if self
            .attributes()
            .setup_own_shading_aspect(&Handle::default())?
        {
            self.replace_shading_aspect();
        }
        if let Some(shading) = self.attributes().shading_aspect().get() {
            shading.set_color(color, AspectTypeOfFacingModel::BothSide)?;
        }
        self.synchronize_aspects();
        Ok(())
    }
    martensite_tkv3d::ais_presentable_overrides!();
}

impl SelectMgrSelectableObject for PrimitiveArrayObject {
    fn selectable_base(&self) -> &SelectMgrSelectableObjectBase {
        &self.base.selectable
    }
    fn compute_selection(
        &self,
        selection: &dyn HandleReference<SelectMgrSelection>,
        mode: i32,
    ) -> Result<(), Failure> {
        let Some(array) = self.parray.get().filter(|_| mode == 0) else {
            return Ok(());
        };
        let this: Handle<dyn SelectMgrSelectableObject> = self.presentable_base().this().cast();
        let owner = SelectMgrEntityOwner::with_selectable(&this, 0).into_cast();
        let selection = selection.observe();
        if !self.parray.cast::<Graphic3dArrayOfSegments>().is_null() {
            return self.add_segments(array, &owner, &selection);
        }
        let sensitive = Select3DSensitivePrimitiveArray::new(&owner)?;
        if !self.parray.cast::<Graphic3dArrayOfTriangles>().is_null() {
            sensitive.init_triangulation_all(
                &array.attributes(),
                &array.indices(),
                &Location::default(),
                true,
                1,
            )?;
            return selection.add(&Handle::new(sensitive).cast());
        }
        sensitive.set_sensitivity_factor(8)?;
        sensitive.init_points_all(
            &array.attributes(),
            &array.indices(),
            &Location::default(),
            true,
            1,
        )?;
        selection.add(&Handle::new(sensitive).cast())
    }
}

impl AisInteractiveObject for PrimitiveArrayObject {
    fn interactive_base(&self) -> &AisInteractiveObjectBase {
        &self.base
    }
}
