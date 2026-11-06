#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_geom::*;
#[allow(unused_imports)]
use crate::step_repr::*;

step_entity! {
    /// Representation of STEP entity FeaLinearElasticity
    StepFEAFeaLinearElasticity, StepFEAFeaLinearElasticityState, "StepFEAFeaLinearElasticity", StepFEAFeaMaterialPropertyRepresentationItem, {
        the_fea_constants: StepFEASymmetricTensor43d,
    }
}
impl StepFEAFeaLinearElasticity {
    step_methods! {
        get fea_constants = the_fea_constants: StepFEASymmetricTensor43d;
        set set_fea_constants = the_fea_constants: StepFEASymmetricTensor43d;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_fea_constants: &StepFEASymmetricTensor43d,
    ) {
        self.parent.init(a_representation_item_name);
        self.state.write().the_fea_constants = a_fea_constants.clone();
    }
}

step_entity! {
    /// Representation of STEP entity FeaMassDensity
    StepFEAFeaMassDensity, StepFEAFeaMassDensityState, "StepFEAFeaMassDensity", StepFEAFeaMaterialPropertyRepresentationItem, {
        the_fea_constant: f64,
    }
}
impl StepFEAFeaMassDensity {
    step_methods! {
        get fea_constant = the_fea_constant: f64;
        set_copy set_fea_constant = the_fea_constant: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(&self, a_representation_item_name: &Handle<HAsciiString>, a_fea_constant: f64) {
        self.parent.init(a_representation_item_name);
        self.state.write().the_fea_constant = a_fea_constant;
    }
}

step_entity! {
    /// Representation of STEP entity FeaMaterialPropertyRepresentation
    StepFEAFeaMaterialPropertyRepresentation, StepFEAFeaMaterialPropertyRepresentationState, "StepFEAFeaMaterialPropertyRepresentation", StepReprMaterialPropertyRepresentation, {
    }
}

step_entity! {
    /// Representation of STEP entity FeaMaterialPropertyRepresentationItem
    StepFEAFeaMaterialPropertyRepresentationItem, StepFEAFeaMaterialPropertyRepresentationItemState, "StepFEAFeaMaterialPropertyRepresentationItem", StepReprRepresentationItem, {
    }
}

step_entity! {
    /// Representation of STEP entity FeaModel
    StepFEAFeaModel, StepFEAFeaModelState, "StepFEAFeaModel", StepReprRepresentation, {
        the_creating_software: Handle<HAsciiString>,
        the_intended_analysis_code: Handle<HArray1<AsciiString>>,
        the_description: Handle<HAsciiString>,
        the_analysis_type: Handle<HAsciiString>,
    }
}
impl StepFEAFeaModel {
    step_methods! {
        get creating_software = the_creating_software: Handle<HAsciiString>;
        set set_creating_software = the_creating_software: Handle<HAsciiString>;
        get intended_analysis_code = the_intended_analysis_code: Handle<HArray1<AsciiString>>;
        set set_intended_analysis_code = the_intended_analysis_code: Handle<HArray1<AsciiString>>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get analysis_type = the_analysis_type: Handle<HAsciiString>;
        set set_analysis_type = the_analysis_type: Handle<HAsciiString>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_representation_name: &Handle<HAsciiString>,
        a_representation_items: &Handle<HArray1<Handle<StepReprRepresentationItem>>>,
        a_representation_context_of_items: &Handle<StepReprRepresentationContext>,
        a_creating_software: &Handle<HAsciiString>,
        a_intended_analysis_code: &Handle<HArray1<AsciiString>>,
        a_description: &Handle<HAsciiString>,
        a_analysis_type: &Handle<HAsciiString>,
    ) {
        self.parent.init(
            a_representation_name,
            a_representation_items,
            a_representation_context_of_items,
        );
        let mut state = self.state.write();
        state.the_creating_software = a_creating_software.clone();
        state.the_intended_analysis_code = a_intended_analysis_code.clone();
        state.the_description = a_description.clone();
        state.the_analysis_type = a_analysis_type.clone();
    }
}

step_entity! {
    /// Representation of STEP entity FeaModel3d
    StepFEAFeaModel3d, StepFEAFeaModel3dState, "StepFEAFeaModel3d", StepFEAFeaModel, {
    }
}

step_entity! {
    /// Representation of STEP entity FeaModelDefinition
    StepFEAFeaModelDefinition, StepFEAFeaModelDefinitionState, "StepFEAFeaModelDefinition", StepReprShapeAspect, {
    }
}

step_entity! {
    /// Representation of STEP entity FeaMoistureAbsorption
    StepFEAFeaMoistureAbsorption, StepFEAFeaMoistureAbsorptionState, "StepFEAFeaMoistureAbsorption", StepFEAFeaMaterialPropertyRepresentationItem, {
        the_fea_constants: StepFEASymmetricTensor23d,
    }
}
impl StepFEAFeaMoistureAbsorption {
    step_methods! {
        get fea_constants = the_fea_constants: StepFEASymmetricTensor23d;
        set set_fea_constants = the_fea_constants: StepFEASymmetricTensor23d;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_fea_constants: &StepFEASymmetricTensor23d,
    ) {
        self.parent.init(a_representation_item_name);
        self.state.write().the_fea_constants = a_fea_constants.clone();
    }
}

step_entity! {
    /// Representation of STEP entity FeaParametricPoint
    StepFEAFeaParametricPoint, StepFEAFeaParametricPointState, "StepFEAFeaParametricPoint", StepGeomPoint, {
        the_coordinates: Handle<HArray1<f64>>,
    }
}
impl StepFEAFeaParametricPoint {
    step_methods! {
        get coordinates = the_coordinates: Handle<HArray1<f64>>;
        set set_coordinates = the_coordinates: Handle<HArray1<f64>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_coordinates: &Handle<HArray1<f64>>,
    ) {
        self.parent.init(a_representation_item_name);
        self.state.write().the_coordinates = a_coordinates.clone();
    }
}

step_entity! {
    /// Representation of STEP entity FeaRepresentationItem
    StepFEAFeaRepresentationItem, StepFEAFeaRepresentationItemState, "StepFEAFeaRepresentationItem", StepReprRepresentationItem, {
    }
}

step_entity! {
    /// Representation of STEP entity FeaSecantCoefficientOfLinearThermalExpansion
    StepFEAFeaSecantCoefficientOfLinearThermalExpansion, StepFEAFeaSecantCoefficientOfLinearThermalExpansionState, "StepFEAFeaSecantCoefficientOfLinearThermalExpansion", StepFEAFeaMaterialPropertyRepresentationItem, {
        the_fea_constants: StepFEASymmetricTensor23d,
        the_reference_temperature: f64,
    }
}
impl StepFEAFeaSecantCoefficientOfLinearThermalExpansion {
    step_methods! {
        get fea_constants = the_fea_constants: StepFEASymmetricTensor23d;
        set set_fea_constants = the_fea_constants: StepFEASymmetricTensor23d;
        get reference_temperature = the_reference_temperature: f64;
        set_copy set_reference_temperature = the_reference_temperature: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_fea_constants: &StepFEASymmetricTensor23d,
        a_reference_temperature: f64,
    ) {
        self.parent.init(a_representation_item_name);
        let mut state = self.state.write();
        state.the_fea_constants = a_fea_constants.clone();
        state.the_reference_temperature = a_reference_temperature;
    }
}

step_entity! {
    /// Representation of STEP entity FeaShellBendingStiffness
    StepFEAFeaShellBendingStiffness, StepFEAFeaShellBendingStiffnessState, "StepFEAFeaShellBendingStiffness", StepFEAFeaMaterialPropertyRepresentationItem, {
        the_fea_constants: StepFEASymmetricTensor42d,
    }
}
impl StepFEAFeaShellBendingStiffness {
    step_methods! {
        get fea_constants = the_fea_constants: StepFEASymmetricTensor42d;
        set set_fea_constants = the_fea_constants: StepFEASymmetricTensor42d;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_fea_constants: &StepFEASymmetricTensor42d,
    ) {
        self.parent.init(a_representation_item_name);
        self.state.write().the_fea_constants = a_fea_constants.clone();
    }
}

step_entity! {
    /// Representation of STEP entity FeaShellMembraneBendingCouplingStiffness
    StepFEAFeaShellMembraneBendingCouplingStiffness, StepFEAFeaShellMembraneBendingCouplingStiffnessState, "StepFEAFeaShellMembraneBendingCouplingStiffness", StepFEAFeaMaterialPropertyRepresentationItem, {
        the_fea_constants: StepFEASymmetricTensor42d,
    }
}
impl StepFEAFeaShellMembraneBendingCouplingStiffness {
    step_methods! {
        get fea_constants = the_fea_constants: StepFEASymmetricTensor42d;
        set set_fea_constants = the_fea_constants: StepFEASymmetricTensor42d;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_fea_constants: &StepFEASymmetricTensor42d,
    ) {
        self.parent.init(a_representation_item_name);
        self.state.write().the_fea_constants = a_fea_constants.clone();
    }
}

step_entity! {
    /// Representation of STEP entity FeaShellMembraneStiffness
    StepFEAFeaShellMembraneStiffness, StepFEAFeaShellMembraneStiffnessState, "StepFEAFeaShellMembraneStiffness", StepFEAFeaMaterialPropertyRepresentationItem, {
        the_fea_constants: StepFEASymmetricTensor42d,
    }
}
impl StepFEAFeaShellMembraneStiffness {
    step_methods! {
        get fea_constants = the_fea_constants: StepFEASymmetricTensor42d;
        set set_fea_constants = the_fea_constants: StepFEASymmetricTensor42d;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_fea_constants: &StepFEASymmetricTensor42d,
    ) {
        self.parent.init(a_representation_item_name);
        self.state.write().the_fea_constants = a_fea_constants.clone();
    }
}

step_entity! {
    /// Representation of STEP entity FeaShellShearStiffness
    StepFEAFeaShellShearStiffness, StepFEAFeaShellShearStiffnessState, "StepFEAFeaShellShearStiffness", StepFEAFeaMaterialPropertyRepresentationItem, {
        the_fea_constants: StepFEASymmetricTensor22d,
    }
}
impl StepFEAFeaShellShearStiffness {
    step_methods! {
        get fea_constants = the_fea_constants: StepFEASymmetricTensor22d;
        set set_fea_constants = the_fea_constants: StepFEASymmetricTensor22d;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_fea_constants: &StepFEASymmetricTensor22d,
    ) {
        self.parent.init(a_representation_item_name);
        self.state.write().the_fea_constants = a_fea_constants.clone();
    }
}
