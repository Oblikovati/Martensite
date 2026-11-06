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
use crate::step_geom::*;
#[allow(unused_imports)]
use crate::step_repr::*;

step_entity! {
    /// Representation of STEP entity FeaSurfaceSectionGeometricRelationship
    StepFEAFeaSurfaceSectionGeometricRelationship, StepFEAFeaSurfaceSectionGeometricRelationshipState, "StepFEAFeaSurfaceSectionGeometricRelationship", StandardTransient, {
        the_section_ref: Handle<StepElementSurfaceSection>,
        the_item: Handle<StepElementAnalysisItemWithinRepresentation>,
    }
}
impl StepFEAFeaSurfaceSectionGeometricRelationship {
    step_methods! {
        get section_ref = the_section_ref: Handle<StepElementSurfaceSection>;
        set set_section_ref = the_section_ref: Handle<StepElementSurfaceSection>;
        get item = the_item: Handle<StepElementAnalysisItemWithinRepresentation>;
        set set_item = the_item: Handle<StepElementAnalysisItemWithinRepresentation>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_section_ref: &Handle<StepElementSurfaceSection>,
        a_item: &Handle<StepElementAnalysisItemWithinRepresentation>,
    ) {
        let mut state = self.state.write();
        state.the_section_ref = a_section_ref.clone();
        state.the_item = a_item.clone();
    }
}

step_entity! {
    /// Representation of STEP entity FeaTangentialCoefficientOfLinearThermalExpansion
    StepFEAFeaTangentialCoefficientOfLinearThermalExpansion, StepFEAFeaTangentialCoefficientOfLinearThermalExpansionState, "StepFEAFeaTangentialCoefficientOfLinearThermalExpansion", StepFEAFeaMaterialPropertyRepresentationItem, {
        the_fea_constants: StepFEASymmetricTensor23d,
    }
}
impl StepFEAFeaTangentialCoefficientOfLinearThermalExpansion {
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
    /// Representation of STEP entity FreedomAndCoefficient
    StepFEAFreedomAndCoefficient, StepFEAFreedomAndCoefficientState, "StepFEAFreedomAndCoefficient", StandardTransient, {
        the_freedom: StepFEADegreeOfFreedom,
        the_a: StepElementMeasureOrUnspecifiedValue,
    }
}
impl StepFEAFreedomAndCoefficient {
    step_methods! {
        get freedom = the_freedom: StepFEADegreeOfFreedom;
        set set_freedom = the_freedom: StepFEADegreeOfFreedom;
        get a = the_a: StepElementMeasureOrUnspecifiedValue;
        set set_a = the_a: StepElementMeasureOrUnspecifiedValue;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_freedom: &StepFEADegreeOfFreedom,
        a_a: &StepElementMeasureOrUnspecifiedValue,
    ) {
        let mut state = self.state.write();
        state.the_freedom = a_freedom.clone();
        state.the_a = a_a.clone();
    }
}

step_entity! {
    /// Representation of STEP entity FreedomsList
    StepFEAFreedomsList, StepFEAFreedomsListState, "StepFEAFreedomsList", StandardTransient, {
        the_freedoms: Handle<HArray1<StepFEADegreeOfFreedom>>,
    }
}
impl StepFEAFreedomsList {
    step_methods! {
        set init = the_freedoms: Handle<HArray1<StepFEADegreeOfFreedom>>;
        get freedoms = the_freedoms: Handle<HArray1<StepFEADegreeOfFreedom>>;
        set set_freedoms = the_freedoms: Handle<HArray1<StepFEADegreeOfFreedom>>;
    }
}

step_entity! {
    /// Representation of STEP entity GeometricNode
    StepFEAGeometricNode, StepFEAGeometricNodeState, "StepFEAGeometricNode", StepFEANodeRepresentation, {
    }
}

step_entity! {
    /// Representation of STEP entity Node
    StepFEANode, StepFEANodeState, "StepFEANode", StepFEANodeRepresentation, {
    }
}

step_entity! {
    /// Representation of STEP entity NodeDefinition
    StepFEANodeDefinition, StepFEANodeDefinitionState, "StepFEANodeDefinition", StepReprShapeAspect, {
    }
}

step_entity! {
    /// Representation of STEP entity NodeGroup
    StepFEANodeGroup, StepFEANodeGroupState, "StepFEANodeGroup", StepFEAFeaGroup, {
        the_nodes: Handle<HArray1<Handle<StepFEANodeRepresentation>>>,
    }
}
impl StepFEANodeGroup {
    step_methods! {
        get nodes = the_nodes: Handle<HArray1<Handle<StepFEANodeRepresentation>>>;
        set set_nodes = the_nodes: Handle<HArray1<Handle<StepFEANodeRepresentation>>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_group_name: &Handle<HAsciiString>,
        a_group_description: &Handle<HAsciiString>,
        a_fea_group_model_ref: &Handle<StepFEAFeaModel>,
        a_nodes: &Handle<HArray1<Handle<StepFEANodeRepresentation>>>,
    ) {
        self.parent
            .init(a_group_name, a_group_description, a_fea_group_model_ref);
        self.state.write().the_nodes = a_nodes.clone();
    }
}

step_entity! {
    /// Representation of STEP entity NodeRepresentation
    StepFEANodeRepresentation, StepFEANodeRepresentationState, "StepFEANodeRepresentation", StepReprRepresentation, {
        the_model_ref: Handle<StepFEAFeaModel>,
    }
}
impl StepFEANodeRepresentation {
    step_methods! {
        get model_ref = the_model_ref: Handle<StepFEAFeaModel>;
        set set_model_ref = the_model_ref: Handle<StepFEAFeaModel>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_name: &Handle<HAsciiString>,
        a_representation_items: &Handle<HArray1<Handle<StepReprRepresentationItem>>>,
        a_representation_context_of_items: &Handle<StepReprRepresentationContext>,
        a_model_ref: &Handle<StepFEAFeaModel>,
    ) {
        self.parent.init(
            a_representation_name,
            a_representation_items,
            a_representation_context_of_items,
        );
        self.state.write().the_model_ref = a_model_ref.clone();
    }
}

step_entity! {
    /// Representation of STEP entity NodeSet
    StepFEANodeSet, StepFEANodeSetState, "StepFEANodeSet", StepGeomGeometricRepresentationItem, {
        the_nodes: Handle<HArray1<Handle<StepFEANodeRepresentation>>>,
    }
}
impl StepFEANodeSet {
    step_methods! {
        get nodes = the_nodes: Handle<HArray1<Handle<StepFEANodeRepresentation>>>;
        set set_nodes = the_nodes: Handle<HArray1<Handle<StepFEANodeRepresentation>>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_nodes: &Handle<HArray1<Handle<StepFEANodeRepresentation>>>,
    ) {
        self.parent.init(a_representation_item_name);
        self.state.write().the_nodes = a_nodes.clone();
    }
}

step_entity! {
    /// Representation of STEP entity NodeWithSolutionCoordinateSystem
    StepFEANodeWithSolutionCoordinateSystem, StepFEANodeWithSolutionCoordinateSystemState, "StepFEANodeWithSolutionCoordinateSystem", StepFEANode, {
    }
}

step_entity! {
    /// Representation of STEP entity NodeWithVector
    StepFEANodeWithVector, StepFEANodeWithVectorState, "StepFEANodeWithVector", StepFEANode, {
    }
}

step_entity! {
    /// Representation of STEP entity ParametricCurve3dElementCoordinateDirection
    StepFEAParametricCurve3dElementCoordinateDirection, StepFEAParametricCurve3dElementCoordinateDirectionState, "StepFEAParametricCurve3dElementCoordinateDirection", StepFEAFeaRepresentationItem, {
        the_orientation: Handle<StepGeomDirection>,
    }
}
impl StepFEAParametricCurve3dElementCoordinateDirection {
    step_methods! {
        get orientation = the_orientation: Handle<StepGeomDirection>;
        set set_orientation = the_orientation: Handle<StepGeomDirection>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_orientation: &Handle<StepGeomDirection>,
    ) {
        self.parent.init(a_representation_item_name);
        self.state.write().the_orientation = a_orientation.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ParametricCurve3dElementCoordinateSystem
    StepFEAParametricCurve3dElementCoordinateSystem, StepFEAParametricCurve3dElementCoordinateSystemState, "StepFEAParametricCurve3dElementCoordinateSystem", StepFEAFeaRepresentationItem, {
        the_direction: Handle<StepFEAParametricCurve3dElementCoordinateDirection>,
    }
}
impl StepFEAParametricCurve3dElementCoordinateSystem {
    step_methods! {
        get direction = the_direction: Handle<StepFEAParametricCurve3dElementCoordinateDirection>;
        set set_direction = the_direction: Handle<StepFEAParametricCurve3dElementCoordinateDirection>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_direction: &Handle<StepFEAParametricCurve3dElementCoordinateDirection>,
    ) {
        self.parent.init(a_representation_item_name);
        self.state.write().the_direction = a_direction.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ParametricSurface3dElementCoordinateSystem
    StepFEAParametricSurface3dElementCoordinateSystem, StepFEAParametricSurface3dElementCoordinateSystemState, "StepFEAParametricSurface3dElementCoordinateSystem", StepFEAFeaRepresentationItem, {
        the_axis: i32,
        the_angle: f64,
    }
}
impl StepFEAParametricSurface3dElementCoordinateSystem {
    step_methods! {
        get axis = the_axis: i32;
        set_copy set_axis = the_axis: i32;
        get angle = the_angle: f64;
        set_copy set_angle = the_angle: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_axis: i32,
        a_angle: f64,
    ) {
        self.parent.init(a_representation_item_name);
        let mut state = self.state.write();
        state.the_axis = a_axis;
        state.the_angle = a_angle;
    }
}
