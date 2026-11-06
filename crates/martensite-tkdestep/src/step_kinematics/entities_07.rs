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
    /// Representation of STEP entity PrismaticPairWithRange
    StepKinematicsPrismaticPairWithRange, StepKinematicsPrismaticPairWithRangeState, "StepKinematicsPrismaticPairWithRange", StepKinematicsPrismaticPair, {
        my_lower_limit_actual_translation: f64,
        my_upper_limit_actual_translation: f64,
        def_lower_limit_actual_translation: bool,
        def_upper_limit_actual_translation: bool,
    }
}
impl StepKinematicsPrismaticPairWithRange {
    step_methods! {
        get lower_limit_actual_translation = my_lower_limit_actual_translation: f64;
        set_copy set_lower_limit_actual_translation = my_lower_limit_actual_translation: f64;
        get has_lower_limit_actual_translation = def_lower_limit_actual_translation: bool;
        get upper_limit_actual_translation = my_upper_limit_actual_translation: f64;
        set_copy set_upper_limit_actual_translation = my_upper_limit_actual_translation: f64;
        get has_upper_limit_actual_translation = def_upper_limit_actual_translation: bool;
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
    }
}

step_entity! {
    /// Representation of STEP entity ProductDefinitionKinematics
    StepKinematicsProductDefinitionKinematics, StepKinematicsProductDefinitionKinematicsState, "StepKinematicsProductDefinitionKinematics", StepReprPropertyDefinition, {
    }
}

step_entity! {
    /// Representation of STEP entity ProductDefinitionRelationshipKinematics
    StepKinematicsProductDefinitionRelationshipKinematics, StepKinematicsProductDefinitionRelationshipKinematicsState, "StepKinematicsProductDefinitionRelationshipKinematics", StepReprPropertyDefinition, {
    }
}

step_entity! {
    /// Representation of STEP entity RackAndPinionPair
    StepKinematicsRackAndPinionPair, StepKinematicsRackAndPinionPairState, "StepKinematicsRackAndPinionPair", StepKinematicsLowOrderKinematicPairWithMotionCoupling, {
        my_pinion_radius: f64,
    }
}
impl StepKinematicsRackAndPinionPair {
    step_methods! {
        get pinion_radius = my_pinion_radius: f64;
        set_copy set_pinion_radius = my_pinion_radius: f64;
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
        the_pinion_radius: f64,
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
        self.state.write().my_pinion_radius = the_pinion_radius;
    }
}

step_entity! {
    /// Representation of STEP entity RackAndPinionPairValue
    StepKinematicsRackAndPinionPairValue, StepKinematicsRackAndPinionPairValueState, "StepKinematicsRackAndPinionPairValue", StepKinematicsPairValue, {
        my_actual_displacement: f64,
    }
}
impl StepKinematicsRackAndPinionPairValue {
    step_methods! {
        get actual_displacement = my_actual_displacement: f64;
        set_copy set_actual_displacement = my_actual_displacement: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_pair_value_applies_to_pair: &Handle<StepKinematicsKinematicPair>,
        the_actual_displacement: f64,
    ) {
        self.parent
            .init(the_representation_item_name, the_pair_value_applies_to_pair);
        self.state.write().my_actual_displacement = the_actual_displacement;
    }
}

step_entity! {
    /// Representation of STEP entity RackAndPinionPairWithRange
    StepKinematicsRackAndPinionPairWithRange, StepKinematicsRackAndPinionPairWithRangeState, "StepKinematicsRackAndPinionPairWithRange", StepKinematicsRackAndPinionPair, {
        my_lower_limit_rack_displacement: f64,
        my_upper_limit_rack_displacement: f64,
        def_lower_limit_rack_displacement: bool,
        def_upper_limit_rack_displacement: bool,
    }
}
impl StepKinematicsRackAndPinionPairWithRange {
    step_methods! {
        get lower_limit_rack_displacement = my_lower_limit_rack_displacement: f64;
        set_copy set_lower_limit_rack_displacement = my_lower_limit_rack_displacement: f64;
        get has_lower_limit_rack_displacement = def_lower_limit_rack_displacement: bool;
        get upper_limit_rack_displacement = my_upper_limit_rack_displacement: f64;
        set_copy set_upper_limit_rack_displacement = my_upper_limit_rack_displacement: f64;
        get has_upper_limit_rack_displacement = def_upper_limit_rack_displacement: bool;
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
        the_rack_and_pinion_pair_pinion_radius: f64,
        has_lower_limit_rack_displacement: bool,
        the_lower_limit_rack_displacement: f64,
        has_upper_limit_rack_displacement: bool,
        the_upper_limit_rack_displacement: f64,
    ) {
        self.parent.init(
            the_representation_item_name,
            the_item_defined_transformation_name,
            has_item_defined_transformation_description,
            the_item_defined_transformation_description,
            the_item_defined_transformation_transform_item1,
            the_item_defined_transformation_transform_item2,
            the_kinematic_pair_joint,
            the_rack_and_pinion_pair_pinion_radius,
        );
        let mut state = self.state.write();
        state.def_lower_limit_rack_displacement = has_lower_limit_rack_displacement;
        if state.def_lower_limit_rack_displacement {
            state.my_lower_limit_rack_displacement = the_lower_limit_rack_displacement;
        } else {
            state.my_lower_limit_rack_displacement = 0.0;
        }
        state.def_upper_limit_rack_displacement = has_upper_limit_rack_displacement;
        if state.def_upper_limit_rack_displacement {
            state.my_upper_limit_rack_displacement = the_upper_limit_rack_displacement;
        } else {
            state.my_upper_limit_rack_displacement = 0.0;
        }
    }
}

step_entity! {
    /// Representation of STEP entity RevolutePair
    StepKinematicsRevolutePair, StepKinematicsRevolutePairState, "StepKinematicsRevolutePair", StepKinematicsLowOrderKinematicPair, {
    }
}

step_entity! {
    /// Representation of STEP entity RevolutePairValue
    StepKinematicsRevolutePairValue, StepKinematicsRevolutePairValueState, "StepKinematicsRevolutePairValue", StepKinematicsPairValue, {
        my_actual_rotation: f64,
    }
}
impl StepKinematicsRevolutePairValue {
    step_methods! {
        get actual_rotation = my_actual_rotation: f64;
        set_copy set_actual_rotation = my_actual_rotation: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_pair_value_applies_to_pair: &Handle<StepKinematicsKinematicPair>,
        the_actual_rotation: f64,
    ) {
        self.parent
            .init(the_representation_item_name, the_pair_value_applies_to_pair);
        self.state.write().my_actual_rotation = the_actual_rotation;
    }
}

step_entity! {
    /// Representation of STEP entity RevolutePairWithRange
    StepKinematicsRevolutePairWithRange, StepKinematicsRevolutePairWithRangeState, "StepKinematicsRevolutePairWithRange", StepKinematicsRevolutePair, {
        my_lower_limit_actual_rotation: f64,
        my_upper_limit_actual_rotation: f64,
        def_lower_limit_actual_rotation: bool,
        def_upper_limit_actual_rotation: bool,
    }
}
impl StepKinematicsRevolutePairWithRange {
    step_methods! {
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

step_entity! {
    /// Representation of STEP entity RigidLinkRepresentation
    StepKinematicsRigidLinkRepresentation, StepKinematicsRigidLinkRepresentationState, "StepKinematicsRigidLinkRepresentation", StepKinematicsKinematicLinkRepresentation, {
    }
}

step_entity! {
    /// Representation of STEP entity RollingCurvePair
    StepKinematicsRollingCurvePair, StepKinematicsRollingCurvePairState, "StepKinematicsRollingCurvePair", StepKinematicsPlanarCurvePair, {
    }
}

step_entity! {
    /// Representation of STEP entity RollingCurvePairValue
    StepKinematicsRollingCurvePairValue, StepKinematicsRollingCurvePairValueState, "StepKinematicsRollingCurvePairValue", StepKinematicsPairValue, {
        my_actual_point_on_curve1: Handle<StepGeomPointOnCurve>,
    }
}
impl StepKinematicsRollingCurvePairValue {
    step_methods! {
        get actual_point_on_curve1 = my_actual_point_on_curve1: Handle<StepGeomPointOnCurve>;
        set set_actual_point_on_curve1 = my_actual_point_on_curve1: Handle<StepGeomPointOnCurve>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_pair_value_applies_to_pair: &Handle<StepKinematicsKinematicPair>,
        the_actual_point_on_curve1: &Handle<StepGeomPointOnCurve>,
    ) {
        self.parent
            .init(the_representation_item_name, the_pair_value_applies_to_pair);
        self.state.write().my_actual_point_on_curve1 = the_actual_point_on_curve1.clone();
    }
}

step_entity! {
    /// Representation of STEP entity RollingSurfacePair
    StepKinematicsRollingSurfacePair, StepKinematicsRollingSurfacePairState, "StepKinematicsRollingSurfacePair", StepKinematicsSurfacePair, {
    }
}
