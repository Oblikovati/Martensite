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

/// Curve edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepElementCurveEdge {
    /// Element edge.
    #[default]
    ElementEdge,
}

/// Element2d shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepElementElement2dShape {
    /// Quadrilateral.
    #[default]
    Quadrilateral,
    /// Triangle.
    Triangle,
}

/// Element order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepElementElementOrder {
    /// Linear.
    #[default]
    Linear,
    /// Quadratic.
    Quadratic,
    /// Cubic.
    Cubic,
}

/// Element volume.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepElementElementVolume {
    /// Volume.
    #[default]
    Volume,
}

/// Enumerated curve element freedom.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepElementEnumeratedCurveElementFreedom {
    /// X translation.
    #[default]
    XTranslation,
    /// Y translation.
    YTranslation,
    /// Z translation.
    ZTranslation,
    /// X rotation.
    XRotation,
    /// Y rotation.
    YRotation,
    /// Z rotation.
    ZRotation,
    /// Warp.
    Warp,
    /// None.
    None,
}

/// Unspecified value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepElementUnspecifiedValue {
    /// Unspecified.
    #[default]
    Unspecified,
}

/// Volume3d element shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepElementVolume3dElementShape {
    /// Hexahedron.
    #[default]
    Hexahedron,
    /// Wedge.
    Wedge,
    /// Tetrahedron.
    Tetrahedron,
    /// Pyramid.
    Pyramid,
}

step_entity! {
    /// Representation of STEP entity AnalysisItemWithinRepresentation
    StepElementAnalysisItemWithinRepresentation, StepElementAnalysisItemWithinRepresentationState, "StepElementAnalysisItemWithinRepresentation", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_item: Handle<StepReprRepresentationItem>,
        the_rep: Handle<StepReprRepresentation>,
    }
}
impl StepElementAnalysisItemWithinRepresentation {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get item = the_item: Handle<StepReprRepresentationItem>;
        set set_item = the_item: Handle<StepReprRepresentationItem>;
        get rep = the_rep: Handle<StepReprRepresentation>;
        set set_rep = the_rep: Handle<StepReprRepresentation>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_item: &Handle<StepReprRepresentationItem>,
        a_rep: &Handle<StepReprRepresentation>,
    ) {
        let mut state = self.state.write();
        state.the_name = a_name.clone();
        state.the_description = a_description.clone();
        state.the_item = a_item.clone();
        state.the_rep = a_rep.clone();
    }
}

step_entity! {
    /// Representation of STEP entity Curve3dElementDescriptor
    StepElementCurve3dElementDescriptor, StepElementCurve3dElementDescriptorState, "StepElementCurve3dElementDescriptor", StepElementElementDescriptor, {
        the_purpose: Handle<HArray1<Handle<HSequence<Handle<StepElementCurveElementPurposeMember>>>>>,
    }
}
impl StepElementCurve3dElementDescriptor {
    step_methods! {
        get purpose = the_purpose: Handle<HArray1<Handle<HSequence<Handle<StepElementCurveElementPurposeMember>>>>>;
        set set_purpose = the_purpose: Handle<HArray1<Handle<HSequence<Handle<StepElementCurveElementPurposeMember>>>>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_element_descriptor_topology_order: StepElementElementOrder,
        a_element_descriptor_description: &Handle<HAsciiString>,
        a_purpose: &Handle<
            HArray1<Handle<HSequence<Handle<StepElementCurveElementPurposeMember>>>>,
        >,
    ) {
        self.parent.init(
            a_element_descriptor_topology_order,
            a_element_descriptor_description,
        );
        self.state.write().the_purpose = a_purpose.clone();
    }
}

step_entity! {
    /// Representation of STEP entity CurveElementEndReleasePacket
    StepElementCurveElementEndReleasePacket, StepElementCurveElementEndReleasePacketState, "StepElementCurveElementEndReleasePacket", StandardTransient, {
        the_release_freedom: StepElementCurveElementFreedom,
        the_release_stiffness: f64,
    }
}
impl StepElementCurveElementEndReleasePacket {
    step_methods! {
        get release_freedom = the_release_freedom: StepElementCurveElementFreedom;
        set set_release_freedom = the_release_freedom: StepElementCurveElementFreedom;
        get release_stiffness = the_release_stiffness: f64;
        set_copy set_release_stiffness = the_release_stiffness: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_release_freedom: &StepElementCurveElementFreedom,
        a_release_stiffness: f64,
    ) {
        let mut state = self.state.write();
        state.the_release_freedom = a_release_freedom.clone();
        state.the_release_stiffness = a_release_stiffness;
    }
}

step_entity! {
    /// Representation of STEP entity CurveElementSectionDefinition
    StepElementCurveElementSectionDefinition, StepElementCurveElementSectionDefinitionState, "StepElementCurveElementSectionDefinition", StandardTransient, {
        the_description: Handle<HAsciiString>,
        the_section_angle: f64,
    }
}
impl StepElementCurveElementSectionDefinition {
    step_methods! {
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get section_angle = the_section_angle: f64;
        set_copy set_section_angle = the_section_angle: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(&self, a_description: &Handle<HAsciiString>, a_section_angle: f64) {
        let mut state = self.state.write();
        state.the_description = a_description.clone();
        state.the_section_angle = a_section_angle;
    }
}

step_entity! {
    /// Representation of STEP entity CurveElementSectionDerivedDefinitions
    StepElementCurveElementSectionDerivedDefinitions, StepElementCurveElementSectionDerivedDefinitionsState, "StepElementCurveElementSectionDerivedDefinitions", StepElementCurveElementSectionDefinition, {
        the_cross_sectional_area: f64,
        the_shear_area: Handle<HArray1<StepElementMeasureOrUnspecifiedValue>>,
        the_second_moment_of_area: Handle<HArray1<f64>>,
        the_torsional_constant: f64,
        the_warping_constant: StepElementMeasureOrUnspecifiedValue,
        the_location_of_centroid: Handle<HArray1<StepElementMeasureOrUnspecifiedValue>>,
        the_location_of_shear_centre: Handle<HArray1<StepElementMeasureOrUnspecifiedValue>>,
        the_location_of_non_structural_mass: Handle<HArray1<StepElementMeasureOrUnspecifiedValue>>,
        the_non_structural_mass: StepElementMeasureOrUnspecifiedValue,
        the_polar_moment: StepElementMeasureOrUnspecifiedValue,
    }
}
impl StepElementCurveElementSectionDerivedDefinitions {
    step_methods! {
        get cross_sectional_area = the_cross_sectional_area: f64;
        set_copy set_cross_sectional_area = the_cross_sectional_area: f64;
        get shear_area = the_shear_area: Handle<HArray1<StepElementMeasureOrUnspecifiedValue>>;
        set set_shear_area = the_shear_area: Handle<HArray1<StepElementMeasureOrUnspecifiedValue>>;
        get second_moment_of_area = the_second_moment_of_area: Handle<HArray1<f64>>;
        set set_second_moment_of_area = the_second_moment_of_area: Handle<HArray1<f64>>;
        get torsional_constant = the_torsional_constant: f64;
        set_copy set_torsional_constant = the_torsional_constant: f64;
        get warping_constant = the_warping_constant: StepElementMeasureOrUnspecifiedValue;
        set set_warping_constant = the_warping_constant: StepElementMeasureOrUnspecifiedValue;
        get location_of_centroid = the_location_of_centroid: Handle<HArray1<StepElementMeasureOrUnspecifiedValue>>;
        set set_location_of_centroid = the_location_of_centroid: Handle<HArray1<StepElementMeasureOrUnspecifiedValue>>;
        get location_of_shear_centre = the_location_of_shear_centre: Handle<HArray1<StepElementMeasureOrUnspecifiedValue>>;
        set set_location_of_shear_centre = the_location_of_shear_centre: Handle<HArray1<StepElementMeasureOrUnspecifiedValue>>;
        get location_of_non_structural_mass = the_location_of_non_structural_mass: Handle<HArray1<StepElementMeasureOrUnspecifiedValue>>;
        set set_location_of_non_structural_mass = the_location_of_non_structural_mass: Handle<HArray1<StepElementMeasureOrUnspecifiedValue>>;
        get non_structural_mass = the_non_structural_mass: StepElementMeasureOrUnspecifiedValue;
        set set_non_structural_mass = the_non_structural_mass: StepElementMeasureOrUnspecifiedValue;
        get polar_moment = the_polar_moment: StepElementMeasureOrUnspecifiedValue;
        set set_polar_moment = the_polar_moment: StepElementMeasureOrUnspecifiedValue;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_curve_element_section_definition_description: &Handle<HAsciiString>,
        a_curve_element_section_definition_section_angle: f64,
        a_cross_sectional_area: f64,
        a_shear_area: &Handle<HArray1<StepElementMeasureOrUnspecifiedValue>>,
        a_second_moment_of_area: &Handle<HArray1<f64>>,
        a_torsional_constant: f64,
        a_warping_constant: &StepElementMeasureOrUnspecifiedValue,
        a_location_of_centroid: &Handle<HArray1<StepElementMeasureOrUnspecifiedValue>>,
        a_location_of_shear_centre: &Handle<HArray1<StepElementMeasureOrUnspecifiedValue>>,
        a_location_of_non_structural_mass: &Handle<HArray1<StepElementMeasureOrUnspecifiedValue>>,
        a_non_structural_mass: &StepElementMeasureOrUnspecifiedValue,
        a_polar_moment: &StepElementMeasureOrUnspecifiedValue,
    ) {
        self.parent.init(
            a_curve_element_section_definition_description,
            a_curve_element_section_definition_section_angle,
        );
        let mut state = self.state.write();
        state.the_cross_sectional_area = a_cross_sectional_area;
        state.the_shear_area = a_shear_area.clone();
        state.the_second_moment_of_area = a_second_moment_of_area.clone();
        state.the_torsional_constant = a_torsional_constant;
        state.the_warping_constant = a_warping_constant.clone();
        state.the_location_of_centroid = a_location_of_centroid.clone();
        state.the_location_of_shear_centre = a_location_of_shear_centre.clone();
        state.the_location_of_non_structural_mass = a_location_of_non_structural_mass.clone();
        state.the_non_structural_mass = a_non_structural_mass.clone();
        state.the_polar_moment = a_polar_moment.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ElementDescriptor
    StepElementElementDescriptor, StepElementElementDescriptorState, "StepElementElementDescriptor", StandardTransient, {
        the_topology_order: StepElementElementOrder,
        the_description: Handle<HAsciiString>,
    }
}
impl StepElementElementDescriptor {
    step_methods! {
        get topology_order = the_topology_order: StepElementElementOrder;
        set_copy set_topology_order = the_topology_order: StepElementElementOrder;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_topology_order: StepElementElementOrder,
        a_description: &Handle<HAsciiString>,
    ) {
        let mut state = self.state.write();
        state.the_topology_order = a_topology_order;
        state.the_description = a_description.clone();
    }
}
