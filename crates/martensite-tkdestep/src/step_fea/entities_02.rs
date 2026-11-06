#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_basic::*;
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
    /// Representation of STEP entity CurveElementInterval
    StepFEACurveElementInterval, StepFEACurveElementIntervalState, "StepFEACurveElementInterval", StandardTransient, {
        the_finish_position: Handle<StepFEACurveElementLocation>,
        the_eu_angles: Handle<StepBasicEulerAngles>,
    }
}
impl StepFEACurveElementInterval {
    step_methods! {
        get finish_position = the_finish_position: Handle<StepFEACurveElementLocation>;
        set set_finish_position = the_finish_position: Handle<StepFEACurveElementLocation>;
        get eu_angles = the_eu_angles: Handle<StepBasicEulerAngles>;
        set set_eu_angles = the_eu_angles: Handle<StepBasicEulerAngles>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_finish_position: &Handle<StepFEACurveElementLocation>,
        a_eu_angles: &Handle<StepBasicEulerAngles>,
    ) {
        let mut state = self.state.write();
        state.the_finish_position = a_finish_position.clone();
        state.the_eu_angles = a_eu_angles.clone();
    }
}

step_entity! {
    /// Representation of STEP entity CurveElementIntervalConstant
    StepFEACurveElementIntervalConstant, StepFEACurveElementIntervalConstantState, "StepFEACurveElementIntervalConstant", StepFEACurveElementInterval, {
        the_section: Handle<StepElementCurveElementSectionDefinition>,
    }
}
impl StepFEACurveElementIntervalConstant {
    step_methods! {
        get section = the_section: Handle<StepElementCurveElementSectionDefinition>;
        set set_section = the_section: Handle<StepElementCurveElementSectionDefinition>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_curve_element_interval_finish_position: &Handle<StepFEACurveElementLocation>,
        a_curve_element_interval_eu_angles: &Handle<StepBasicEulerAngles>,
        a_section: &Handle<StepElementCurveElementSectionDefinition>,
    ) {
        self.parent.init(
            a_curve_element_interval_finish_position,
            a_curve_element_interval_eu_angles,
        );
        self.state.write().the_section = a_section.clone();
    }
}

step_entity! {
    /// Representation of STEP entity CurveElementIntervalLinearlyVarying
    StepFEACurveElementIntervalLinearlyVarying, StepFEACurveElementIntervalLinearlyVaryingState, "StepFEACurveElementIntervalLinearlyVarying", StepFEACurveElementInterval, {
        the_sections: Handle<HArray1<Handle<StepElementCurveElementSectionDefinition>>>,
    }
}
impl StepFEACurveElementIntervalLinearlyVarying {
    step_methods! {
        get sections = the_sections: Handle<HArray1<Handle<StepElementCurveElementSectionDefinition>>>;
        set set_sections = the_sections: Handle<HArray1<Handle<StepElementCurveElementSectionDefinition>>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_curve_element_interval_finish_position: &Handle<StepFEACurveElementLocation>,
        a_curve_element_interval_eu_angles: &Handle<StepBasicEulerAngles>,
        a_sections: &Handle<HArray1<Handle<StepElementCurveElementSectionDefinition>>>,
    ) {
        self.parent.init(
            a_curve_element_interval_finish_position,
            a_curve_element_interval_eu_angles,
        );
        self.state.write().the_sections = a_sections.clone();
    }
}

step_entity! {
    /// Representation of STEP entity CurveElementLocation
    StepFEACurveElementLocation, StepFEACurveElementLocationState, "StepFEACurveElementLocation", StandardTransient, {
        the_coordinate: Handle<StepFEAFeaParametricPoint>,
    }
}
impl StepFEACurveElementLocation {
    step_methods! {
        set init = the_coordinate: Handle<StepFEAFeaParametricPoint>;
        get coordinate = the_coordinate: Handle<StepFEAFeaParametricPoint>;
        set set_coordinate = the_coordinate: Handle<StepFEAFeaParametricPoint>;
    }
}

step_entity! {
    /// Representation of STEP entity DummyNode
    StepFEADummyNode, StepFEADummyNodeState, "StepFEADummyNode", StepFEANodeRepresentation, {
    }
}

step_entity! {
    /// Representation of STEP entity ElementGeometricRelationship
    StepFEAElementGeometricRelationship, StepFEAElementGeometricRelationshipState, "StepFEAElementGeometricRelationship", StandardTransient, {
        the_element_ref: StepFEAElementOrElementGroup,
        the_item: Handle<StepElementAnalysisItemWithinRepresentation>,
        the_aspect: StepElementElementAspect,
    }
}
impl StepFEAElementGeometricRelationship {
    step_methods! {
        get element_ref = the_element_ref: StepFEAElementOrElementGroup;
        set set_element_ref = the_element_ref: StepFEAElementOrElementGroup;
        get item = the_item: Handle<StepElementAnalysisItemWithinRepresentation>;
        set set_item = the_item: Handle<StepElementAnalysisItemWithinRepresentation>;
        get aspect = the_aspect: StepElementElementAspect;
        set set_aspect = the_aspect: StepElementElementAspect;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_element_ref: &StepFEAElementOrElementGroup,
        a_item: &Handle<StepElementAnalysisItemWithinRepresentation>,
        a_aspect: &StepElementElementAspect,
    ) {
        let mut state = self.state.write();
        state.the_element_ref = a_element_ref.clone();
        state.the_item = a_item.clone();
        state.the_aspect = a_aspect.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ElementGroup
    StepFEAElementGroup, StepFEAElementGroupState, "StepFEAElementGroup", StepFEAFeaGroup, {
        the_elements: Handle<HArray1<Handle<StepFEAElementRepresentation>>>,
    }
}
impl StepFEAElementGroup {
    step_methods! {
        get elements = the_elements: Handle<HArray1<Handle<StepFEAElementRepresentation>>>;
        set set_elements = the_elements: Handle<HArray1<Handle<StepFEAElementRepresentation>>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_group_name: &Handle<HAsciiString>,
        a_group_description: &Handle<HAsciiString>,
        a_fea_group_model_ref: &Handle<StepFEAFeaModel>,
        a_elements: &Handle<HArray1<Handle<StepFEAElementRepresentation>>>,
    ) {
        self.parent
            .init(a_group_name, a_group_description, a_fea_group_model_ref);
        self.state.write().the_elements = a_elements.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ElementRepresentation
    StepFEAElementRepresentation, StepFEAElementRepresentationState, "StepFEAElementRepresentation", StepReprRepresentation, {
        the_node_list: Handle<HArray1<Handle<StepFEANodeRepresentation>>>,
    }
}
impl StepFEAElementRepresentation {
    step_methods! {
        get node_list = the_node_list: Handle<HArray1<Handle<StepFEANodeRepresentation>>>;
        set set_node_list = the_node_list: Handle<HArray1<Handle<StepFEANodeRepresentation>>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_name: &Handle<HAsciiString>,
        a_representation_items: &Handle<HArray1<Handle<StepReprRepresentationItem>>>,
        a_representation_context_of_items: &Handle<StepReprRepresentationContext>,
        a_node_list: &Handle<HArray1<Handle<StepFEANodeRepresentation>>>,
    ) {
        self.parent.init(
            a_representation_name,
            a_representation_items,
            a_representation_context_of_items,
        );
        self.state.write().the_node_list = a_node_list.clone();
    }
}

step_entity! {
    /// Representation of STEP entity FeaAreaDensity
    StepFEAFeaAreaDensity, StepFEAFeaAreaDensityState, "StepFEAFeaAreaDensity", StepFEAFeaMaterialPropertyRepresentationItem, {
        the_fea_constant: f64,
    }
}
impl StepFEAFeaAreaDensity {
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
    /// Representation of STEP entity FeaAxis2Placement3d
    StepFEAFeaAxis2Placement3d, StepFEAFeaAxis2Placement3dState, "StepFEAFeaAxis2Placement3d", StepGeomAxis2Placement3d, {
        the_system_type: StepFEACoordinateSystemType,
        the_description: Handle<HAsciiString>,
    }
}
impl StepFEAFeaAxis2Placement3d {
    step_methods! {
        get system_type = the_system_type: StepFEACoordinateSystemType;
        set_copy set_system_type = the_system_type: StepFEACoordinateSystemType;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_placement_location: &Handle<StepGeomCartesianPoint>,
        has_axis2_placement3d_axis: bool,
        a_axis2_placement3d_axis: &Handle<StepGeomDirection>,
        has_axis2_placement3d_ref_direction: bool,
        a_axis2_placement3d_ref_direction: &Handle<StepGeomDirection>,
        a_system_type: StepFEACoordinateSystemType,
        a_description: &Handle<HAsciiString>,
    ) {
        self.parent.init(
            a_representation_item_name,
            a_placement_location,
            has_axis2_placement3d_axis,
            a_axis2_placement3d_axis,
            has_axis2_placement3d_ref_direction,
            a_axis2_placement3d_ref_direction,
        );
        let mut state = self.state.write();
        state.the_system_type = a_system_type;
        state.the_description = a_description.clone();
    }
}

step_entity! {
    /// Representation of STEP entity FeaCurveSectionGeometricRelationship
    StepFEAFeaCurveSectionGeometricRelationship, StepFEAFeaCurveSectionGeometricRelationshipState, "StepFEAFeaCurveSectionGeometricRelationship", StandardTransient, {
        the_section_ref: Handle<StepElementCurveElementSectionDefinition>,
        the_item: Handle<StepElementAnalysisItemWithinRepresentation>,
    }
}
impl StepFEAFeaCurveSectionGeometricRelationship {
    step_methods! {
        get section_ref = the_section_ref: Handle<StepElementCurveElementSectionDefinition>;
        set set_section_ref = the_section_ref: Handle<StepElementCurveElementSectionDefinition>;
        get item = the_item: Handle<StepElementAnalysisItemWithinRepresentation>;
        set set_item = the_item: Handle<StepElementAnalysisItemWithinRepresentation>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_section_ref: &Handle<StepElementCurveElementSectionDefinition>,
        a_item: &Handle<StepElementAnalysisItemWithinRepresentation>,
    ) {
        let mut state = self.state.write();
        state.the_section_ref = a_section_ref.clone();
        state.the_item = a_item.clone();
    }
}

step_entity! {
    /// Representation of STEP entity FeaGroup
    StepFEAFeaGroup, StepFEAFeaGroupState, "StepFEAFeaGroup", StepBasicGroup, {
        the_model_ref: Handle<StepFEAFeaModel>,
    }
}
impl StepFEAFeaGroup {
    step_methods! {
        get model_ref = the_model_ref: Handle<StepFEAFeaModel>;
        set set_model_ref = the_model_ref: Handle<StepFEAFeaModel>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_group_name: &Handle<HAsciiString>,
        a_group_description: &Handle<HAsciiString>,
        a_model_ref: &Handle<StepFEAFeaModel>,
    ) {
        self.parent.init(a_group_name, true, a_group_description);
        self.state.write().the_model_ref = a_model_ref.clone();
    }
}
