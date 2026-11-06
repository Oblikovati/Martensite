#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
#[allow(unused_imports)]
use crate::step_element::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_repr::*;

step_entity! {
    /// Representation of STEP entity Surface3dElementRepresentation
    StepFEASurface3dElementRepresentation, StepFEASurface3dElementRepresentationState, "StepFEASurface3dElementRepresentation", StepFEAElementRepresentation, {
        the_model_ref: Handle<StepFEAFeaModel3d>,
        the_element_descriptor: Handle<StepElementSurface3dElementDescriptor>,
        the_property: Handle<StepElementSurfaceElementProperty>,
        the_material: Handle<StepElementElementMaterial>,
    }
}
impl StepFEASurface3dElementRepresentation {
    step_methods! {
        get model_ref = the_model_ref: Handle<StepFEAFeaModel3d>;
        set set_model_ref = the_model_ref: Handle<StepFEAFeaModel3d>;
        get element_descriptor = the_element_descriptor: Handle<StepElementSurface3dElementDescriptor>;
        set set_element_descriptor = the_element_descriptor: Handle<StepElementSurface3dElementDescriptor>;
        get property = the_property: Handle<StepElementSurfaceElementProperty>;
        set set_property = the_property: Handle<StepElementSurfaceElementProperty>;
        get material = the_material: Handle<StepElementElementMaterial>;
        set set_material = the_material: Handle<StepElementElementMaterial>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_representation_name: &Handle<HAsciiString>,
        a_representation_items: &Handle<HArray1<Handle<StepReprRepresentationItem>>>,
        a_representation_context_of_items: &Handle<StepReprRepresentationContext>,
        a_element_representation_node_list: &Handle<HArray1<Handle<StepFEANodeRepresentation>>>,
        a_model_ref: &Handle<StepFEAFeaModel3d>,
        a_element_descriptor: &Handle<StepElementSurface3dElementDescriptor>,
        a_property: &Handle<StepElementSurfaceElementProperty>,
        a_material: &Handle<StepElementElementMaterial>,
    ) {
        self.parent.init(
            a_representation_name,
            a_representation_items,
            a_representation_context_of_items,
            a_element_representation_node_list,
        );
        let mut state = self.state.write();
        state.the_model_ref = a_model_ref.clone();
        state.the_element_descriptor = a_element_descriptor.clone();
        state.the_property = a_property.clone();
        state.the_material = a_material.clone();
    }
}

step_entity! {
    /// Representation of STEP entity Volume3dElementRepresentation
    StepFEAVolume3dElementRepresentation, StepFEAVolume3dElementRepresentationState, "StepFEAVolume3dElementRepresentation", StepFEAElementRepresentation, {
        the_model_ref: Handle<StepFEAFeaModel3d>,
        the_element_descriptor: Handle<StepElementVolume3dElementDescriptor>,
        the_material: Handle<StepElementElementMaterial>,
    }
}
impl StepFEAVolume3dElementRepresentation {
    step_methods! {
        get model_ref = the_model_ref: Handle<StepFEAFeaModel3d>;
        set set_model_ref = the_model_ref: Handle<StepFEAFeaModel3d>;
        get element_descriptor = the_element_descriptor: Handle<StepElementVolume3dElementDescriptor>;
        set set_element_descriptor = the_element_descriptor: Handle<StepElementVolume3dElementDescriptor>;
        get material = the_material: Handle<StepElementElementMaterial>;
        set set_material = the_material: Handle<StepElementElementMaterial>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_representation_name: &Handle<HAsciiString>,
        a_representation_items: &Handle<HArray1<Handle<StepReprRepresentationItem>>>,
        a_representation_context_of_items: &Handle<StepReprRepresentationContext>,
        a_element_representation_node_list: &Handle<HArray1<Handle<StepFEANodeRepresentation>>>,
        a_model_ref: &Handle<StepFEAFeaModel3d>,
        a_element_descriptor: &Handle<StepElementVolume3dElementDescriptor>,
        a_material: &Handle<StepElementElementMaterial>,
    ) {
        self.parent.init(
            a_representation_name,
            a_representation_items,
            a_representation_context_of_items,
            a_element_representation_node_list,
        );
        let mut state = self.state.write();
        state.the_model_ref = a_model_ref.clone();
        state.the_element_descriptor = a_element_descriptor.clone();
        state.the_material = a_material.clone();
    }
}
