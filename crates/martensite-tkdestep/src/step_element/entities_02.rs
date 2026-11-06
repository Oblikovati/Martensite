#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_repr::*;

step_entity! {
    /// Representation of STEP entity ElementMaterial
    StepElementElementMaterial, StepElementElementMaterialState, "StepElementElementMaterial", StandardTransient, {
        the_material_id: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_properties: Handle<HArray1<Handle<StepReprMaterialPropertyRepresentation>>>,
    }
}
impl StepElementElementMaterial {
    step_methods! {
        get material_id = the_material_id: Handle<HAsciiString>;
        set set_material_id = the_material_id: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get properties = the_properties: Handle<HArray1<Handle<StepReprMaterialPropertyRepresentation>>>;
        set set_properties = the_properties: Handle<HArray1<Handle<StepReprMaterialPropertyRepresentation>>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_material_id: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_properties: &Handle<HArray1<Handle<StepReprMaterialPropertyRepresentation>>>,
    ) {
        let mut state = self.state.write();
        state.the_material_id = a_material_id.clone();
        state.the_description = a_description.clone();
        state.the_properties = a_properties.clone();
    }
}

step_entity! {
    /// Representation of STEP entity Surface3dElementDescriptor
    StepElementSurface3dElementDescriptor, StepElementSurface3dElementDescriptorState, "StepElementSurface3dElementDescriptor", StepElementElementDescriptor, {
        the_purpose: Handle<HArray1<Handle<HSequence<Handle<StepElementSurfaceElementPurposeMember>>>>>,
        the_shape: StepElementElement2dShape,
    }
}
impl StepElementSurface3dElementDescriptor {
    step_methods! {
        get purpose = the_purpose: Handle<HArray1<Handle<HSequence<Handle<StepElementSurfaceElementPurposeMember>>>>>;
        set set_purpose = the_purpose: Handle<HArray1<Handle<HSequence<Handle<StepElementSurfaceElementPurposeMember>>>>>;
        get shape = the_shape: StepElementElement2dShape;
        set_copy set_shape = the_shape: StepElementElement2dShape;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_element_descriptor_topology_order: StepElementElementOrder,
        a_element_descriptor_description: &Handle<HAsciiString>,
        a_purpose: &Handle<
            HArray1<Handle<HSequence<Handle<StepElementSurfaceElementPurposeMember>>>>,
        >,
        a_shape: StepElementElement2dShape,
    ) {
        self.parent.init(
            a_element_descriptor_topology_order,
            a_element_descriptor_description,
        );
        let mut state = self.state.write();
        state.the_purpose = a_purpose.clone();
        state.the_shape = a_shape;
    }
}

step_entity! {
    /// Representation of STEP entity SurfaceElementProperty
    StepElementSurfaceElementProperty, StepElementSurfaceElementPropertyState, "StepElementSurfaceElementProperty", StandardTransient, {
        the_property_id: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_section: Handle<StepElementSurfaceSectionField>,
    }
}
impl StepElementSurfaceElementProperty {
    step_methods! {
        get property_id = the_property_id: Handle<HAsciiString>;
        set set_property_id = the_property_id: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get section = the_section: Handle<StepElementSurfaceSectionField>;
        set set_section = the_section: Handle<StepElementSurfaceSectionField>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_property_id: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_section: &Handle<StepElementSurfaceSectionField>,
    ) {
        let mut state = self.state.write();
        state.the_property_id = a_property_id.clone();
        state.the_description = a_description.clone();
        state.the_section = a_section.clone();
    }
}

step_entity! {
    /// Representation of STEP entity SurfaceSection
    StepElementSurfaceSection, StepElementSurfaceSectionState, "StepElementSurfaceSection", StandardTransient, {
        the_offset: StepElementMeasureOrUnspecifiedValue,
        the_non_structural_mass: StepElementMeasureOrUnspecifiedValue,
        the_non_structural_mass_offset: StepElementMeasureOrUnspecifiedValue,
    }
}
impl StepElementSurfaceSection {
    step_methods! {
        get offset = the_offset: StepElementMeasureOrUnspecifiedValue;
        set set_offset = the_offset: StepElementMeasureOrUnspecifiedValue;
        get non_structural_mass = the_non_structural_mass: StepElementMeasureOrUnspecifiedValue;
        set set_non_structural_mass = the_non_structural_mass: StepElementMeasureOrUnspecifiedValue;
        get non_structural_mass_offset = the_non_structural_mass_offset: StepElementMeasureOrUnspecifiedValue;
        set set_non_structural_mass_offset = the_non_structural_mass_offset: StepElementMeasureOrUnspecifiedValue;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_offset: &StepElementMeasureOrUnspecifiedValue,
        a_non_structural_mass: &StepElementMeasureOrUnspecifiedValue,
        a_non_structural_mass_offset: &StepElementMeasureOrUnspecifiedValue,
    ) {
        let mut state = self.state.write();
        state.the_offset = a_offset.clone();
        state.the_non_structural_mass = a_non_structural_mass.clone();
        state.the_non_structural_mass_offset = a_non_structural_mass_offset.clone();
    }
}

step_entity! {
    /// Representation of STEP entity SurfaceSectionField
    StepElementSurfaceSectionField, StepElementSurfaceSectionFieldState, "StepElementSurfaceSectionField", StandardTransient, {
    }
}

step_entity! {
    /// Representation of STEP entity SurfaceSectionFieldConstant
    StepElementSurfaceSectionFieldConstant, StepElementSurfaceSectionFieldConstantState, "StepElementSurfaceSectionFieldConstant", StepElementSurfaceSectionField, {
        the_definition: Handle<StepElementSurfaceSection>,
    }
}
impl StepElementSurfaceSectionFieldConstant {
    step_methods! {
        set init = the_definition: Handle<StepElementSurfaceSection>;
        get definition = the_definition: Handle<StepElementSurfaceSection>;
        set set_definition = the_definition: Handle<StepElementSurfaceSection>;
    }
}

step_entity! {
    /// Representation of STEP entity SurfaceSectionFieldVarying
    StepElementSurfaceSectionFieldVarying, StepElementSurfaceSectionFieldVaryingState, "StepElementSurfaceSectionFieldVarying", StepElementSurfaceSectionField, {
        the_definitions: Handle<HArray1<Handle<StepElementSurfaceSection>>>,
        the_additional_node_values: bool,
    }
}
impl StepElementSurfaceSectionFieldVarying {
    step_methods! {
        get definitions = the_definitions: Handle<HArray1<Handle<StepElementSurfaceSection>>>;
        set set_definitions = the_definitions: Handle<HArray1<Handle<StepElementSurfaceSection>>>;
        get additional_node_values = the_additional_node_values: bool;
        set_copy set_additional_node_values = the_additional_node_values: bool;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_definitions: &Handle<HArray1<Handle<StepElementSurfaceSection>>>,
        a_additional_node_values: bool,
    ) {
        let mut state = self.state.write();
        state.the_definitions = a_definitions.clone();
        state.the_additional_node_values = a_additional_node_values;
    }
}

step_entity! {
    /// Representation of STEP entity UniformSurfaceSection
    StepElementUniformSurfaceSection, StepElementUniformSurfaceSectionState, "StepElementUniformSurfaceSection", StepElementSurfaceSection, {
        the_thickness: f64,
        the_bending_thickness: StepElementMeasureOrUnspecifiedValue,
        the_shear_thickness: StepElementMeasureOrUnspecifiedValue,
    }
}
impl StepElementUniformSurfaceSection {
    step_methods! {
        get thickness = the_thickness: f64;
        set_copy set_thickness = the_thickness: f64;
        get bending_thickness = the_bending_thickness: StepElementMeasureOrUnspecifiedValue;
        set set_bending_thickness = the_bending_thickness: StepElementMeasureOrUnspecifiedValue;
        get shear_thickness = the_shear_thickness: StepElementMeasureOrUnspecifiedValue;
        set set_shear_thickness = the_shear_thickness: StepElementMeasureOrUnspecifiedValue;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_surface_section_offset: &StepElementMeasureOrUnspecifiedValue,
        a_surface_section_non_structural_mass: &StepElementMeasureOrUnspecifiedValue,
        a_surface_section_non_structural_mass_offset: &StepElementMeasureOrUnspecifiedValue,
        a_thickness: f64,
        a_bending_thickness: &StepElementMeasureOrUnspecifiedValue,
        a_shear_thickness: &StepElementMeasureOrUnspecifiedValue,
    ) {
        self.parent.init(
            a_surface_section_offset,
            a_surface_section_non_structural_mass,
            a_surface_section_non_structural_mass_offset,
        );
        let mut state = self.state.write();
        state.the_thickness = a_thickness;
        state.the_bending_thickness = a_bending_thickness.clone();
        state.the_shear_thickness = a_shear_thickness.clone();
    }
}

step_entity! {
    /// Representation of STEP entity Volume3dElementDescriptor
    StepElementVolume3dElementDescriptor, StepElementVolume3dElementDescriptorState, "StepElementVolume3dElementDescriptor", StepElementElementDescriptor, {
        the_purpose: Handle<HArray1<Handle<StepElementVolumeElementPurposeMember>>>,
        the_shape: StepElementVolume3dElementShape,
    }
}
impl StepElementVolume3dElementDescriptor {
    step_methods! {
        get purpose = the_purpose: Handle<HArray1<Handle<StepElementVolumeElementPurposeMember>>>;
        set set_purpose = the_purpose: Handle<HArray1<Handle<StepElementVolumeElementPurposeMember>>>;
        get shape = the_shape: StepElementVolume3dElementShape;
        set_copy set_shape = the_shape: StepElementVolume3dElementShape;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_element_descriptor_topology_order: StepElementElementOrder,
        a_element_descriptor_description: &Handle<HAsciiString>,
        a_purpose: &Handle<HArray1<Handle<StepElementVolumeElementPurposeMember>>>,
        a_shape: StepElementVolume3dElementShape,
    ) {
        self.parent.init(
            a_element_descriptor_topology_order,
            a_element_descriptor_description,
        );
        let mut state = self.state.write();
        state.the_purpose = a_purpose.clone();
        state.the_shape = a_shape;
    }
}
