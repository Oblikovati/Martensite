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
    /// Representation of STEP entity KinematicTopologyStructure
    StepKinematicsKinematicTopologyStructure, StepKinematicsKinematicTopologyStructureState, "StepKinematicsKinematicTopologyStructure", StepReprRepresentation, {
    }
}

step_entity! {
    /// Representation of STEP entity LinearFlexibleAndPinionPair
    StepKinematicsLinearFlexibleAndPinionPair, StepKinematicsLinearFlexibleAndPinionPairState, "StepKinematicsLinearFlexibleAndPinionPair", StepKinematicsLowOrderKinematicPairWithMotionCoupling, {
        my_pinion_radius: f64,
    }
}
impl StepKinematicsLinearFlexibleAndPinionPair {
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
    /// Representation of STEP entity LinearFlexibleAndPlanarCurvePair
    StepKinematicsLinearFlexibleAndPlanarCurvePair, StepKinematicsLinearFlexibleAndPlanarCurvePairState, "StepKinematicsLinearFlexibleAndPlanarCurvePair", StepKinematicsHighOrderKinematicPair, {
        my_pair_curve: Handle<StepGeomCurve>,
        my_orientation: bool,
    }
}
impl StepKinematicsLinearFlexibleAndPlanarCurvePair {
    step_methods! {
        get pair_curve = my_pair_curve: Handle<StepGeomCurve>;
        set set_pair_curve = my_pair_curve: Handle<StepGeomCurve>;
        get orientation = my_orientation: bool;
        set_copy set_orientation = my_orientation: bool;
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
        the_pair_curve: &Handle<StepGeomCurve>,
        the_orientation: bool,
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
        state.my_pair_curve = the_pair_curve.clone();
        state.my_orientation = the_orientation;
    }
}

step_entity! {
    /// Representation of STEP entity LinearFlexibleLinkRepresentation
    StepKinematicsLinearFlexibleLinkRepresentation, StepKinematicsLinearFlexibleLinkRepresentationState, "StepKinematicsLinearFlexibleLinkRepresentation", StepKinematicsKinematicLinkRepresentation, {
    }
}

step_entity! {
    /// Representation of STEP entity LowOrderKinematicPair
    StepKinematicsLowOrderKinematicPair, StepKinematicsLowOrderKinematicPairState, "StepKinematicsLowOrderKinematicPair", StepKinematicsKinematicPair, {
        my_tx: bool,
        my_ty: bool,
        my_tz: bool,
        my_rx: bool,
        my_ry: bool,
        my_rz: bool,
    }
}
impl StepKinematicsLowOrderKinematicPair {
    step_methods! {
        get tx = my_tx: bool;
        set_copy set_tx = my_tx: bool;
        get ty = my_ty: bool;
        set_copy set_ty = my_ty: bool;
        get tz = my_tz: bool;
        set_copy set_tz = my_tz: bool;
        get rx = my_rx: bool;
        set_copy set_rx = my_rx: bool;
        get ry = my_ry: bool;
        set_copy set_ry = my_ry: bool;
        get rz = my_rz: bool;
        set_copy set_rz = my_rz: bool;
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
        the_tx: bool,
        the_ty: bool,
        the_tz: bool,
        the_rx: bool,
        the_ry: bool,
        the_rz: bool,
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
        state.my_tx = the_tx;
        state.my_ty = the_ty;
        state.my_tz = the_tz;
        state.my_rx = the_rx;
        state.my_ry = the_ry;
        state.my_rz = the_rz;
    }
}

step_entity! {
    /// Representation of STEP entity LowOrderKinematicPairValue
    StepKinematicsLowOrderKinematicPairValue, StepKinematicsLowOrderKinematicPairValueState, "StepKinematicsLowOrderKinematicPairValue", StepKinematicsPairValue, {
        my_actual_translation_x: f64,
        my_actual_translation_y: f64,
        my_actual_translation_z: f64,
        my_actual_rotation_x: f64,
        my_actual_rotation_y: f64,
        my_actual_rotation_z: f64,
    }
}
impl StepKinematicsLowOrderKinematicPairValue {
    step_methods! {
        get actual_translation_x = my_actual_translation_x: f64;
        set_copy set_actual_translation_x = my_actual_translation_x: f64;
        get actual_translation_y = my_actual_translation_y: f64;
        set_copy set_actual_translation_y = my_actual_translation_y: f64;
        get actual_translation_z = my_actual_translation_z: f64;
        set_copy set_actual_translation_z = my_actual_translation_z: f64;
        get actual_rotation_x = my_actual_rotation_x: f64;
        set_copy set_actual_rotation_x = my_actual_rotation_x: f64;
        get actual_rotation_y = my_actual_rotation_y: f64;
        set_copy set_actual_rotation_y = my_actual_rotation_y: f64;
        get actual_rotation_z = my_actual_rotation_z: f64;
        set_copy set_actual_rotation_z = my_actual_rotation_z: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_pair_value_applies_to_pair: &Handle<StepKinematicsKinematicPair>,
        the_actual_translation_x: f64,
        the_actual_translation_y: f64,
        the_actual_translation_z: f64,
        the_actual_rotation_x: f64,
        the_actual_rotation_y: f64,
        the_actual_rotation_z: f64,
    ) {
        self.parent
            .init(the_representation_item_name, the_pair_value_applies_to_pair);
        let mut state = self.state.write();
        state.my_actual_translation_x = the_actual_translation_x;
        state.my_actual_translation_y = the_actual_translation_y;
        state.my_actual_translation_z = the_actual_translation_z;
        state.my_actual_rotation_x = the_actual_rotation_x;
        state.my_actual_rotation_y = the_actual_rotation_y;
        state.my_actual_rotation_z = the_actual_rotation_z;
    }
}

step_entity! {
    /// Representation of STEP entity LowOrderKinematicPairWithMotionCoupling
    StepKinematicsLowOrderKinematicPairWithMotionCoupling, StepKinematicsLowOrderKinematicPairWithMotionCouplingState, "StepKinematicsLowOrderKinematicPairWithMotionCoupling", StepKinematicsKinematicPair, {
    }
}
