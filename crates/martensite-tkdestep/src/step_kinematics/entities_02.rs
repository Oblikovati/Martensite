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
#[allow(unused_imports)]
use crate::step_shape::*;

step_entity! {
    /// Representation of STEP entity FullyConstrainedPair
    StepKinematicsFullyConstrainedPair, StepKinematicsFullyConstrainedPairState, "StepKinematicsFullyConstrainedPair", StepKinematicsLowOrderKinematicPair, {
    }
}

step_entity! {
    /// Representation of STEP entity GearPair
    StepKinematicsGearPair, StepKinematicsGearPairState, "StepKinematicsGearPair", StepKinematicsLowOrderKinematicPairWithMotionCoupling, {
        my_radius_first_link: f64,
        my_radius_second_link: f64,
        my_bevel: f64,
        my_helical_angle: f64,
        my_gear_ratio: f64,
    }
}
impl StepKinematicsGearPair {
    step_methods! {
        get radius_first_link = my_radius_first_link: f64;
        set_copy set_radius_first_link = my_radius_first_link: f64;
        get radius_second_link = my_radius_second_link: f64;
        set_copy set_radius_second_link = my_radius_second_link: f64;
        get bevel = my_bevel: f64;
        set_copy set_bevel = my_bevel: f64;
        get helical_angle = my_helical_angle: f64;
        set_copy set_helical_angle = my_helical_angle: f64;
        get gear_ratio = my_gear_ratio: f64;
        set_copy set_gear_ratio = my_gear_ratio: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_item_defined_transformation_name: &Handle<HAsciiString>,
        has_item_defined_transformation_description: bool,
        the_item_defined_transformation_description: &Handle<HAsciiString>,
        the_item_defined_transformation_transform_item1: &Handle<StepReprRepresentationItem>,
        the_item_defined_transformation_transform_item2: &Handle<StepReprRepresentationItem>,
        the_kinematic_pair_joint: &Handle<StepKinematicsKinematicJoint>,
        the_radius_first_link: f64,
        the_radius_second_link: f64,
        the_bevel: f64,
        the_helical_angle: f64,
        the_gear_ratio: f64,
    ) {
        self.parent.init(
            the_representation_item_name,
            the_item_defined_transformation_name,
            has_item_defined_transformation_description,
            the_item_defined_transformation_description,
            the_item_defined_transformation_transform_item1,
            the_item_defined_transformation_transform_item2,
            the_kinematic_pair_joint,
        );
        let mut state = self.state.write();
        state.my_radius_first_link = the_radius_first_link;
        state.my_radius_second_link = the_radius_second_link;
        state.my_bevel = the_bevel;
        state.my_helical_angle = the_helical_angle;
        state.my_gear_ratio = the_gear_ratio;
    }
}

step_entity! {
    /// Representation of STEP entity GearPairValue
    StepKinematicsGearPairValue, StepKinematicsGearPairValueState, "StepKinematicsGearPairValue", StepKinematicsPairValue, {
        my_actual_rotation1: f64,
    }
}
impl StepKinematicsGearPairValue {
    step_methods! {
        get actual_rotation1 = my_actual_rotation1: f64;
        set_copy set_actual_rotation1 = my_actual_rotation1: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_pair_value_applies_to_pair: &Handle<StepKinematicsKinematicPair>,
        the_actual_rotation1: f64,
    ) {
        self.parent
            .init(the_representation_item_name, the_pair_value_applies_to_pair);
        self.state.write().my_actual_rotation1 = the_actual_rotation1;
    }
}

step_entity! {
    /// Representation of STEP entity GearPairWithRange
    StepKinematicsGearPairWithRange, StepKinematicsGearPairWithRangeState, "StepKinematicsGearPairWithRange", StepKinematicsGearPair, {
        my_lower_limit_actual_rotation1: f64,
        my_upper_limit_actual_rotation1: f64,
        def_lower_limit_actual_rotation1: bool,
        def_upper_limit_actual_rotation1: bool,
    }
}
impl StepKinematicsGearPairWithRange {
    step_methods! {
        get lower_limit_actual_rotation1 = my_lower_limit_actual_rotation1: f64;
        set_copy set_lower_limit_actual_rotation1 = my_lower_limit_actual_rotation1: f64;
        get has_lower_limit_actual_rotation1 = def_lower_limit_actual_rotation1: bool;
        get upper_limit_actual_rotation1 = my_upper_limit_actual_rotation1: f64;
        set_copy set_upper_limit_actual_rotation1 = my_upper_limit_actual_rotation1: f64;
        get has_upper_limit_actual_rotation1 = def_upper_limit_actual_rotation1: bool;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_item_defined_transformation_name: &Handle<HAsciiString>,
        has_item_defined_transformation_description: bool,
        the_item_defined_transformation_description: &Handle<HAsciiString>,
        the_item_defined_transformation_transform_item1: &Handle<StepReprRepresentationItem>,
        the_item_defined_transformation_transform_item2: &Handle<StepReprRepresentationItem>,
        the_kinematic_pair_joint: &Handle<StepKinematicsKinematicJoint>,
        the_gear_pair_radius_first_link: f64,
        the_gear_pair_radius_second_link: f64,
        the_gear_pair_bevel: f64,
        the_gear_pair_helical_angle: f64,
        the_gear_pair_gear_ratio: f64,
        has_lower_limit_actual_rotation1: bool,
        the_lower_limit_actual_rotation1: f64,
        has_upper_limit_actual_rotation1: bool,
        the_upper_limit_actual_rotation1: f64,
    ) {
        self.parent.init(
            the_representation_item_name,
            the_item_defined_transformation_name,
            has_item_defined_transformation_description,
            the_item_defined_transformation_description,
            the_item_defined_transformation_transform_item1,
            the_item_defined_transformation_transform_item2,
            the_kinematic_pair_joint,
            the_gear_pair_radius_first_link,
            the_gear_pair_radius_second_link,
            the_gear_pair_bevel,
            the_gear_pair_helical_angle,
            the_gear_pair_gear_ratio,
        );
        let mut state = self.state.write();
        state.def_lower_limit_actual_rotation1 = has_lower_limit_actual_rotation1;
        if state.def_lower_limit_actual_rotation1 {
            state.my_lower_limit_actual_rotation1 = the_lower_limit_actual_rotation1;
        } else {
            state.my_lower_limit_actual_rotation1 = 0.0;
        }
        state.def_upper_limit_actual_rotation1 = has_upper_limit_actual_rotation1;
        if state.def_upper_limit_actual_rotation1 {
            state.my_upper_limit_actual_rotation1 = the_upper_limit_actual_rotation1;
        } else {
            state.my_upper_limit_actual_rotation1 = 0.0;
        }
    }
}

step_entity! {
    /// Representation of STEP entity HighOrderKinematicPair
    StepKinematicsHighOrderKinematicPair, StepKinematicsHighOrderKinematicPairState, "StepKinematicsHighOrderKinematicPair", StepKinematicsKinematicPair, {
    }
}

step_entity! {
    /// Representation of STEP entity HomokineticPair
    StepKinematicsHomokineticPair, StepKinematicsHomokineticPairState, "StepKinematicsHomokineticPair", StepKinematicsUniversalPair, {
    }
}

step_entity! {
    /// Representation of STEP entity KinematicJoint
    StepKinematicsKinematicJoint, StepKinematicsKinematicJointState, "StepKinematicsKinematicJoint", StepShapeEdge, {
    }
}

step_entity! {
    /// Representation of STEP entity KinematicLink
    StepKinematicsKinematicLink, StepKinematicsKinematicLinkState, "StepKinematicsKinematicLink", StepShapeVertex, {
    }
}

step_entity! {
    /// Representation of STEP entity KinematicLinkRepresentation
    StepKinematicsKinematicLinkRepresentation, StepKinematicsKinematicLinkRepresentationState, "StepKinematicsKinematicLinkRepresentation", StepReprRepresentation, {
        my_represented_link: Handle<StepKinematicsKinematicLink>,
    }
}
impl StepKinematicsKinematicLinkRepresentation {
    step_methods! {
        get represented_link = my_represented_link: Handle<StepKinematicsKinematicLink>;
        set set_represented_link = my_represented_link: Handle<StepKinematicsKinematicLink>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_name: &Handle<HAsciiString>,
        the_representation_items: &Handle<HArray1<Handle<StepReprRepresentationItem>>>,
        the_representation_context_of_items: &Handle<StepReprRepresentationContext>,
        the_represented_link: &Handle<StepKinematicsKinematicLink>,
    ) {
        self.parent.init(
            the_representation_name,
            the_representation_items,
            the_representation_context_of_items,
        );
        self.state.write().my_represented_link = the_represented_link.clone();
    }
}

step_entity! {
    /// Representation of STEP entity KinematicLinkRepresentationAssociation
    StepKinematicsKinematicLinkRepresentationAssociation, StepKinematicsKinematicLinkRepresentationAssociationState, "StepKinematicsKinematicLinkRepresentationAssociation", StepReprRepresentationRelationship, {
    }
}

step_entity! {
    /// Representation of STEP entity KinematicPair
    StepKinematicsKinematicPair, StepKinematicsKinematicPairState, "StepKinematicsKinematicPair", StepGeomGeometricRepresentationItem, {
        my_item_defined_transformation: Handle<StepReprItemDefinedTransformation>,
        my_joint: Handle<StepKinematicsKinematicJoint>,
    }
}
impl StepKinematicsKinematicPair {
    step_methods! {
        get item_defined_transformation = my_item_defined_transformation: Handle<StepReprItemDefinedTransformation>;
        set set_item_defined_transformation = my_item_defined_transformation: Handle<StepReprItemDefinedTransformation>;
        get joint = my_joint: Handle<StepKinematicsKinematicJoint>;
        set set_joint = my_joint: Handle<StepKinematicsKinematicJoint>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_item_defined_transformation_name: &Handle<HAsciiString>,
        _unused_2: bool,
        the_item_defined_transformation_description: &Handle<HAsciiString>,
        the_item_defined_transformation_transform_item1: &Handle<StepReprRepresentationItem>,
        the_item_defined_transformation_transform_item2: &Handle<StepReprRepresentationItem>,
        the_joint: &Handle<StepKinematicsKinematicJoint>,
    ) {
        self.parent.init(the_representation_item_name);
        self.state.write().my_item_defined_transformation =
            Handle::new(StepReprItemDefinedTransformation::new());
        let field_my_item_defined_transformation =
            self.state.read().my_item_defined_transformation.clone();
        field_my_item_defined_transformation.init(
            the_item_defined_transformation_name,
            the_item_defined_transformation_description,
            the_item_defined_transformation_transform_item1,
            the_item_defined_transformation_transform_item2,
        );
        self.state.write().my_joint = the_joint.clone();
    }
}

step_entity! {
    /// Representation of STEP entity KinematicPropertyDefinitionRepresentation
    StepKinematicsKinematicPropertyDefinitionRepresentation, StepKinematicsKinematicPropertyDefinitionRepresentationState, "StepKinematicsKinematicPropertyDefinitionRepresentation", StepReprPropertyDefinitionRepresentation, {
    }
}

step_entity! {
    /// Representation of STEP entity KinematicPropertyMechanismRepresentation
    StepKinematicsKinematicPropertyMechanismRepresentation, StepKinematicsKinematicPropertyMechanismRepresentationState, "StepKinematicsKinematicPropertyMechanismRepresentation", StepKinematicsKinematicPropertyDefinitionRepresentation, {
        my_base: Handle<StepKinematicsKinematicLinkRepresentation>,
    }
}
impl StepKinematicsKinematicPropertyMechanismRepresentation {
    step_methods! {
        get base = my_base: Handle<StepKinematicsKinematicLinkRepresentation>;
        set set_base = my_base: Handle<StepKinematicsKinematicLinkRepresentation>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_property_definition_representation_definition: &StepReprRepresentedDefinition,
        the_property_definition_representation_used_representation: &Handle<StepReprRepresentation>,
        the_base: &Handle<StepKinematicsKinematicLinkRepresentation>,
    ) {
        self.parent.init(
            the_property_definition_representation_definition,
            the_property_definition_representation_used_representation,
        );
        self.state.write().my_base = the_base.clone();
    }
}

step_entity! {
    /// Representation of STEP entity KinematicTopologyDirectedStructure
    StepKinematicsKinematicTopologyDirectedStructure, StepKinematicsKinematicTopologyDirectedStructureState, "StepKinematicsKinematicTopologyDirectedStructure", StepReprRepresentation, {
        my_parent: Handle<StepKinematicsKinematicTopologyStructure>,
    }
}
impl StepKinematicsKinematicTopologyDirectedStructure {
    step_methods! {
        get parent = my_parent: Handle<StepKinematicsKinematicTopologyStructure>;
        set set_parent = my_parent: Handle<StepKinematicsKinematicTopologyStructure>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_name: &Handle<HAsciiString>,
        the_representation_items: &Handle<HArray1<Handle<StepReprRepresentationItem>>>,
        the_representation_context_of_items: &Handle<StepReprRepresentationContext>,
        the_parent: &Handle<StepKinematicsKinematicTopologyStructure>,
    ) {
        self.parent.init(
            the_representation_name,
            the_representation_items,
            the_representation_context_of_items,
        );
        self.state.write().my_parent = the_parent.clone();
    }
}

step_entity! {
    /// Representation of STEP entity KinematicTopologyNetworkStructure
    StepKinematicsKinematicTopologyNetworkStructure, StepKinematicsKinematicTopologyNetworkStructureState, "StepKinematicsKinematicTopologyNetworkStructure", StepReprRepresentation, {
        my_parent: Handle<StepKinematicsKinematicTopologyStructure>,
    }
}
impl StepKinematicsKinematicTopologyNetworkStructure {
    step_methods! {
        get parent = my_parent: Handle<StepKinematicsKinematicTopologyStructure>;
        set set_parent = my_parent: Handle<StepKinematicsKinematicTopologyStructure>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_name: &Handle<HAsciiString>,
        the_representation_items: &Handle<HArray1<Handle<StepReprRepresentationItem>>>,
        the_representation_context_of_items: &Handle<StepReprRepresentationContext>,
        the_parent: &Handle<StepKinematicsKinematicTopologyStructure>,
    ) {
        self.parent.init(
            the_representation_name,
            the_representation_items,
            the_representation_context_of_items,
        );
        self.state.write().my_parent = the_parent.clone();
    }
}
