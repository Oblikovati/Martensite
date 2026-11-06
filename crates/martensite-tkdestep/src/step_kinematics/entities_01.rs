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

/// Actuated direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepKinematicsActuatedDirection {
    /// Bidirectional.
    #[default]
    AdBidirectional,
    /// Positive only.
    AdPositiveOnly,
    /// Negative only.
    AdNegativeOnly,
    /// Not actuated.
    AdNotActuated,
}

step_select! {
    /// Representation of STEP SELECT type KinematicTopologyRepresentationSelect
    StepKinematicsKinematicTopologyRepresentationSelect, [StepKinematicsKinematicTopologyDirectedStructure, StepKinematicsKinematicTopologyNetworkStructure, StepKinematicsKinematicTopologyStructure], {
        kinematic_topology_directed_structure: StepKinematicsKinematicTopologyDirectedStructure,
        kinematic_topology_network_structure: StepKinematicsKinematicTopologyNetworkStructure,
        kinematic_topology_structure: StepKinematicsKinematicTopologyStructure,
    }
}

step_select! {
    /// Representation of STEP SELECT type SpatialRotation
    StepKinematicsSpatialRotation, [StepKinematicsRotationAboutDirection, HArray1<f64>], {
        rotation_about_direction: StepKinematicsRotationAboutDirection,
        ypr_rotation: HArray1<f64>,
    }
}

step_entity! {
    /// Representation of STEP entity ActuatedKinPairAndOrderKinPair
    StepKinematicsActuatedKinPairAndOrderKinPair, StepKinematicsActuatedKinPairAndOrderKinPairState, "StepKinematicsActuatedKinPairAndOrderKinPair", StepKinematicsKinematicPair, {
        my_actuated_kinematic_pair: Handle<StepKinematicsActuatedKinematicPair>,
        my_order_kinematic_pair: Handle<StepKinematicsKinematicPair>,
    }
}
impl StepKinematicsActuatedKinPairAndOrderKinPair {
    step_methods! {
        set set_actuated_kinematic_pair = my_actuated_kinematic_pair: Handle<StepKinematicsActuatedKinematicPair>;
        get get_actuated_kinematic_pair = my_actuated_kinematic_pair: Handle<StepKinematicsActuatedKinematicPair>;
        set set_order_kinematic_pair = my_order_kinematic_pair: Handle<StepKinematicsKinematicPair>;
        get get_order_kinematic_pair = my_order_kinematic_pair: Handle<StepKinematicsKinematicPair>;
    }
    /// `init`.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_item_defined_transformation_name: &Handle<HAsciiString>,
        has_item_defined_transformation_description: bool,
        the_item_defined_transformation_description: &Handle<HAsciiString>,
        the_item_defined_transformation_transform_item1: &Handle<StepReprRepresentationItem>,
        the_item_defined_transformation_transform_item2: &Handle<StepReprRepresentationItem>,
        the_joint: &Handle<StepKinematicsKinematicJoint>,
        the_actuated_kinematic_pair: &Handle<StepKinematicsActuatedKinematicPair>,
        the_order_kinematic_pair: &Handle<StepKinematicsKinematicPair>,
    ) {
        self.parent.init(
            the_representation_item_name,
            the_item_defined_transformation_name,
            has_item_defined_transformation_description,
            the_item_defined_transformation_description,
            the_item_defined_transformation_transform_item1,
            the_item_defined_transformation_transform_item2,
            the_joint,
        );
        self.set_actuated_kinematic_pair(the_actuated_kinematic_pair);
        self.set_order_kinematic_pair(the_order_kinematic_pair);
    }
}

step_entity! {
    /// Representation of STEP entity ActuatedKinematicPair
    StepKinematicsActuatedKinematicPair, StepKinematicsActuatedKinematicPairState, "StepKinematicsActuatedKinematicPair", StepKinematicsKinematicPair, {
        my_tx: StepKinematicsActuatedDirection,
        my_ty: StepKinematicsActuatedDirection,
        my_tz: StepKinematicsActuatedDirection,
        my_rx: StepKinematicsActuatedDirection,
        my_ry: StepKinematicsActuatedDirection,
        my_rz: StepKinematicsActuatedDirection,
        def_tx: bool,
        def_ty: bool,
        def_tz: bool,
        def_rx: bool,
        def_ry: bool,
        def_rz: bool,
    }
}
impl StepKinematicsActuatedKinematicPair {
    step_methods! {
        get tx = my_tx: StepKinematicsActuatedDirection;
        set_copy set_tx = my_tx: StepKinematicsActuatedDirection;
        get has_tx = def_tx: bool;
        get ty = my_ty: StepKinematicsActuatedDirection;
        set_copy set_ty = my_ty: StepKinematicsActuatedDirection;
        get has_ty = def_ty: bool;
        get tz = my_tz: StepKinematicsActuatedDirection;
        set_copy set_tz = my_tz: StepKinematicsActuatedDirection;
        get has_tz = def_tz: bool;
        get rx = my_rx: StepKinematicsActuatedDirection;
        set_copy set_rx = my_rx: StepKinematicsActuatedDirection;
        get has_rx = def_rx: bool;
        get ry = my_ry: StepKinematicsActuatedDirection;
        set_copy set_ry = my_ry: StepKinematicsActuatedDirection;
        get has_ry = def_ry: bool;
        get rz = my_rz: StepKinematicsActuatedDirection;
        set_copy set_rz = my_rz: StepKinematicsActuatedDirection;
        get has_rz = def_rz: bool;
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
        has_tx: bool,
        the_tx: StepKinematicsActuatedDirection,
        has_ty: bool,
        the_ty: StepKinematicsActuatedDirection,
        has_tz: bool,
        the_tz: StepKinematicsActuatedDirection,
        has_rx: bool,
        the_rx: StepKinematicsActuatedDirection,
        has_ry: bool,
        the_ry: StepKinematicsActuatedDirection,
        has_rz: bool,
        the_rz: StepKinematicsActuatedDirection,
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
        state.def_tx = has_tx;
        if state.def_tx {
            state.my_tx = the_tx;
        } else {
            state.my_tx = StepKinematicsActuatedDirection::default();
        }
        state.def_ty = has_ty;
        if state.def_ty {
            state.my_ty = the_ty;
        } else {
            state.my_ty = StepKinematicsActuatedDirection::default();
        }
        state.def_tz = has_tz;
        if state.def_tz {
            state.my_tz = the_tz;
        } else {
            state.my_tz = StepKinematicsActuatedDirection::default();
        }
        state.def_rx = has_rx;
        if state.def_rx {
            state.my_rx = the_rx;
        } else {
            state.my_rx = StepKinematicsActuatedDirection::default();
        }
        state.def_ry = has_ry;
        if state.def_ry {
            state.my_ry = the_ry;
        } else {
            state.my_ry = StepKinematicsActuatedDirection::default();
        }
        state.def_rz = has_rz;
        if state.def_rz {
            state.my_rz = the_rz;
        } else {
            state.my_rz = StepKinematicsActuatedDirection::default();
        }
    }
}

step_entity! {
    /// Representation of STEP entity ContextDependentKinematicLinkRepresentation
    StepKinematicsContextDependentKinematicLinkRepresentation, StepKinematicsContextDependentKinematicLinkRepresentationState, "StepKinematicsContextDependentKinematicLinkRepresentation", StandardTransient, {
        my_representation_relation: Handle<StepKinematicsKinematicLinkRepresentationAssociation>,
        my_represented_product_relation: Handle<StepKinematicsProductDefinitionRelationshipKinematics>,
    }
}
impl StepKinematicsContextDependentKinematicLinkRepresentation {
    step_methods! {
        get representation_relation = my_representation_relation: Handle<StepKinematicsKinematicLinkRepresentationAssociation>;
        set set_representation_relation = my_representation_relation: Handle<StepKinematicsKinematicLinkRepresentationAssociation>;
        get represented_product_relation = my_represented_product_relation: Handle<StepKinematicsProductDefinitionRelationshipKinematics>;
        set set_represented_product_relation = my_represented_product_relation: Handle<StepKinematicsProductDefinitionRelationshipKinematics>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_relation: &Handle<StepKinematicsKinematicLinkRepresentationAssociation>,
        the_represented_product_relation: &Handle<
            StepKinematicsProductDefinitionRelationshipKinematics,
        >,
    ) {
        let mut state = self.state.write();
        state.my_representation_relation = the_representation_relation.clone();
        state.my_represented_product_relation = the_represented_product_relation.clone();
    }
}

step_entity! {
    /// Representation of STEP entity CylindricalPair
    StepKinematicsCylindricalPair, StepKinematicsCylindricalPairState, "StepKinematicsCylindricalPair", StepKinematicsLowOrderKinematicPair, {
    }
}

step_entity! {
    /// Representation of STEP entity CylindricalPairValue
    StepKinematicsCylindricalPairValue, StepKinematicsCylindricalPairValueState, "StepKinematicsCylindricalPairValue", StepKinematicsPairValue, {
        my_actual_translation: f64,
        my_actual_rotation: f64,
    }
}
impl StepKinematicsCylindricalPairValue {
    step_methods! {
        get actual_translation = my_actual_translation: f64;
        set_copy set_actual_translation = my_actual_translation: f64;
        get actual_rotation = my_actual_rotation: f64;
        set_copy set_actual_rotation = my_actual_rotation: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_pair_value_applies_to_pair: &Handle<StepKinematicsKinematicPair>,
        the_actual_translation: f64,
        the_actual_rotation: f64,
    ) {
        self.parent
            .init(the_representation_item_name, the_pair_value_applies_to_pair);
        let mut state = self.state.write();
        state.my_actual_translation = the_actual_translation;
        state.my_actual_rotation = the_actual_rotation;
    }
}

step_entity! {
    /// Representation of STEP entity CylindricalPairWithRange
    StepKinematicsCylindricalPairWithRange, StepKinematicsCylindricalPairWithRangeState, "StepKinematicsCylindricalPairWithRange", StepKinematicsCylindricalPair, {
        my_lower_limit_actual_translation: f64,
        my_upper_limit_actual_translation: f64,
        my_lower_limit_actual_rotation: f64,
        my_upper_limit_actual_rotation: f64,
        def_lower_limit_actual_translation: bool,
        def_upper_limit_actual_translation: bool,
        def_lower_limit_actual_rotation: bool,
        def_upper_limit_actual_rotation: bool,
    }
}
impl StepKinematicsCylindricalPairWithRange {
    step_methods! {
        get lower_limit_actual_translation = my_lower_limit_actual_translation: f64;
        set_copy set_lower_limit_actual_translation = my_lower_limit_actual_translation: f64;
        get has_lower_limit_actual_translation = def_lower_limit_actual_translation: bool;
        get upper_limit_actual_translation = my_upper_limit_actual_translation: f64;
        set_copy set_upper_limit_actual_translation = my_upper_limit_actual_translation: f64;
        get has_upper_limit_actual_translation = def_upper_limit_actual_translation: bool;
        get lower_limit_actual_rotation = my_lower_limit_actual_rotation: f64;
        set_copy set_lower_limit_actual_rotation = my_lower_limit_actual_rotation: f64;
        get has_lower_limit_actual_rotation = def_lower_limit_actual_rotation: bool;
        get upper_limit_actual_rotation = my_upper_limit_actual_rotation: f64;
        set_copy set_upper_limit_actual_rotation = my_upper_limit_actual_rotation: f64;
        get has_upper_limit_actual_rotation = def_upper_limit_actual_rotation: bool;
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
        the_low_order_kinematic_pair_tx: bool,
        the_low_order_kinematic_pair_ty: bool,
        the_low_order_kinematic_pair_tz: bool,
        the_low_order_kinematic_pair_rx: bool,
        the_low_order_kinematic_pair_ry: bool,
        the_low_order_kinematic_pair_rz: bool,
        has_lower_limit_actual_translation: bool,
        the_lower_limit_actual_translation: f64,
        has_upper_limit_actual_translation: bool,
        the_upper_limit_actual_translation: f64,
        has_lower_limit_actual_rotation: bool,
        the_lower_limit_actual_rotation: f64,
        has_upper_limit_actual_rotation: bool,
        the_upper_limit_actual_rotation: f64,
    ) {
        self.parent.init(
            the_representation_item_name,
            the_item_defined_transformation_name,
            has_item_defined_transformation_description,
            the_item_defined_transformation_description,
            the_item_defined_transformation_transform_item1,
            the_item_defined_transformation_transform_item2,
            the_kinematic_pair_joint,
            the_low_order_kinematic_pair_tx,
            the_low_order_kinematic_pair_ty,
            the_low_order_kinematic_pair_tz,
            the_low_order_kinematic_pair_rx,
            the_low_order_kinematic_pair_ry,
            the_low_order_kinematic_pair_rz,
        );
        let mut state = self.state.write();
        state.def_lower_limit_actual_translation = has_lower_limit_actual_translation;
        if state.def_lower_limit_actual_translation {
            state.my_lower_limit_actual_translation = the_lower_limit_actual_translation;
        } else {
            state.my_lower_limit_actual_translation = 0.0;
        }
        state.def_upper_limit_actual_translation = has_upper_limit_actual_translation;
        if state.def_upper_limit_actual_translation {
            state.my_upper_limit_actual_translation = the_upper_limit_actual_translation;
        } else {
            state.my_upper_limit_actual_translation = 0.0;
        }
        state.def_lower_limit_actual_rotation = has_lower_limit_actual_rotation;
        if state.def_lower_limit_actual_rotation {
            state.my_lower_limit_actual_rotation = the_lower_limit_actual_rotation;
        } else {
            state.my_lower_limit_actual_rotation = 0.0;
        }
        state.def_upper_limit_actual_rotation = has_upper_limit_actual_rotation;
        if state.def_upper_limit_actual_rotation {
            state.my_upper_limit_actual_rotation = the_upper_limit_actual_rotation;
        } else {
            state.my_upper_limit_actual_rotation = 0.0;
        }
    }
}
