//! Typed primitive-array presentations retain mutable storage, marker aspects and selection.
use crate::ais::{AisInteractiveObject, AisInteractiveObjectBase};
use crate::prs_mgr::{
    PrsMgrPresentableObject, PrsMgrPresentableObjectBase, PrsMgrPresentableObjectMembers as _,
    PrsMgrPresentableObjectUpdates as _, PrsMgrPresentationManager, PrsMgrTypeOfPresentation3d,
    prs_mgr_presentable_bind,
};
use crate::select_mgr::{
    SelectMgrEntityOwner, SelectMgrSelectableObject, SelectMgrSelectableObjectBase,
    SelectMgrSelection,
};
use crate::select3d::{
    Select3DSensitiveEntityMembers as _, Select3DSensitivePrimitiveArray, Select3DSensitiveSegment,
};
use martensite_tkernel::ncollection::Vec3;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::{
    Failure, Handle, HandleReference, SharedState, StandardTransient,
};
use martensite_tkmath::toploc::Location;
use martensite_tkservice::aspect::AspectTypeOfFacingModel;
use martensite_tkservice::graphic3d::{
    Graphic3dArrayOfPrimitives, Graphic3dArrayOfSegments, Graphic3dArrayOfTriangles,
    Graphic3dAspectMarker3d, Graphic3dAspectsMembers as _, Graphic3dAspectsReplaceMap,
    Graphic3dAspectsSource, Graphic3dAttribBuffer, Graphic3dBufferMembers as _,
    Graphic3dGroupMembers as _, Graphic3dMaterialAspect, Graphic3dNameOfMaterial,
    Graphic3dPresentationAttributesSource as _, Graphic3dStructureMembers as _,
    Graphic3dStructureSource, Graphic3dTypeOfPrimitiveArray,
};

/// The array and the marker aspect of the object.
#[derive(Default)]
struct PArrayFields {
    marker_aspect: Handle<Graphic3dAspectMarker3d>,
    parray: Handle<Graphic3dArrayOfPrimitives>,
}

/// Retain a primitive array with its marker/shading presentation and sensitive entities.
/// This mutable AIS owner is confined to its viewer invocation; it is not a published result.
/// Example: `let object = PrimitiveArrayPresentation::from_array(vertices);`.
pub struct PrimitiveArrayPresentation {
    transient: StandardTransient,
    base: AisInteractiveObjectBase,
    fields: SharedState<PArrayFields>,
}
martensite_tkernel::impl_transient!(PrimitiveArrayPresentation, "MyPArrayObject", dyn AisInteractiveObject, transient,
    interfaces: [dyn AisInteractiveObject, dyn SelectMgrSelectableObject, dyn PrsMgrPresentableObject]);

impl PrimitiveArrayPresentation {
    /// The object of an existing array.
    fn allocate(parray: Handle<Graphic3dArrayOfPrimitives>) -> Handle<Self> {
        let object = Handle::new(Self {
            transient: StandardTransient::default(),
            base: AisInteractiveObjectBase::new(PrsMgrTypeOfPresentation3d::AllView),
            fields: SharedState::new(PArrayFields {
                marker_aspect: Handle::default(),
                parray,
            }),
        });
        prs_mgr_presentable_bind(&object.cast());
        object
    }
    /// Bind an existing array without altering its material; `PrimitiveArrayPresentation::from_array(array)`.
    pub fn from_array(parray: Handle<Graphic3dArrayOfPrimitives>) -> Handle<Self> {
        Self::allocate(parray)
    }

    /// Read the retained array without replacing its identity; `object.primitive_array()`.
    pub fn primitive_array(&self) -> Handle<Graphic3dArrayOfPrimitives> {
        self.fields.read().parray.clone()
    }
    /// Retain a replacement array before a presentation is computed; `object.replace_primitive_array(array)`.
    pub fn replace_primitive_array(&self, array: Handle<Graphic3dArrayOfPrimitives>) {
        self.fields.write().parray = array;
    }
    /// Retain the marker aspect independently of geometry; `object.set_marker_aspect(&marker)`.
    pub fn set_marker_aspect(&self, marker: &Handle<Graphic3dAspectMarker3d>) {
        self.fields.write().marker_aspect = marker.clone();
    }
    /// Read the retained marker aspect; `object.marker_aspect()`.
    pub fn marker_aspect(&self) -> Handle<Graphic3dAspectMarker3d> {
        self.fields.read().marker_aspect.clone()
    }
    /// Set the existing white plastified vertex-color material; `object.setup_vertex_color_shading()?`.
    pub fn setup_vertex_color_shading(&self) -> Result<(), Failure> {
        let drawer = self.attributes();
        drawer.setup_own_shading_aspect(&Handle::default())?;
        let material = vertex_color_material()?;
        let shading = drawer.shading_aspect();
        let shading = shading.get().ok_or_else(|| {
            martensite_tkernel::osd::sigsegv("SIGSEGV 'segmentation violation' detected.")
        })?;
        shading.set_material(&material, AspectTypeOfFacingModel::BothSide);
        shading.set_color(
            Color::named(NamedColor::WHITE),
            AspectTypeOfFacingModel::BothSide,
        )?;
        Ok(())
    }
    /// `replaceShadingAspect`: the linked shading aspect replaced by the own one.
    fn replace_shading_aspect(&self) {
        let drawer = self.attributes();
        let link = drawer.link();
        let Some(link) = link.get() else {
            return;
        };
        let old = shading_aspect_handle(link);
        let new = shading_aspect_handle(&drawer);
        let mut map = Graphic3dAspectsReplaceMap::default();
        map.insert(old.object_address() as usize, (old, new));
        self.replace_aspects(&map);
    }
}
impl PrsMgrPresentableObject for PrimitiveArrayPresentation {
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
        let fields = self.fields.read();
        if fields.parray.is_null() || mode != 0 {
            return Ok(());
        }
        let group = prs.observe().new_group()?;
        if !fields.marker_aspect.is_null() {
            group.set_group_primitives_aspect(&fields.marker_aspect.clone().cast());
        } else if let Some(shading) = self.attributes().shading_aspect().get() {
            group.set_group_primitives_aspect(&shading.aspect().cast());
        }
        group.add_primitive_array(&fields.parray, true);
        Ok(())
    }
    /// `SetColor`: the own color, then the shading and marker colors.
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
        if let Some(marker) = self.fields.read().marker_aspect.get() {
            marker.set_interior_color(color);
        }
        self.synchronize_aspects();
        Ok(())
    }
    crate::ais_presentable_overrides!();
}

impl AisInteractiveObject for PrimitiveArrayPresentation {
    fn interactive_base(&self) -> &AisInteractiveObjectBase {
        &self.base
    }
}

// The presentation follows AIS invocation-local ownership; no cross-thread claim is made.

mod patch;
mod selection;

#[cfg(test)]
mod contracts;

fn vertex_color_material() -> Result<Graphic3dMaterialAspect, Failure> {
    let mut material = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Plastified);
    material.set_specular_color(Color::named(NamedColor::BLACK));
    material.set_emissive_color(Color::named(NamedColor::BLACK));
    material.set_ambient_color(Color::from_vec3(Vec3::new(0.5, 0.5, 0.5))?);
    material.set_diffuse_color(Color::from_vec3(Vec3::new(0.5, 0.5, 0.5))?);
    Ok(material)
}

fn shading_aspect_handle(drawer: &crate::prs3d::Prs3dDrawer) -> Handle<dyn Graphic3dAspectsSource> {
    drawer
        .shading_aspect()
        .get()
        .map(|shading| shading.aspect().cast())
        .unwrap_or_default()
}
