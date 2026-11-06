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
    /// Representation of STEP entity UniversalPair
    StepKinematicsUniversalPair, StepKinematicsUniversalPairState, "StepKinematicsUniversalPair", StepKinematicsLowOrderKinematicPair, {
        my_input_skew_angle: f64,
        def_input_skew_angle: bool,
    }
}
impl StepKinematicsUniversalPair {
    step_methods! {
        get input_skew_angle = my_input_skew_angle: f64;
        set_copy set_input_skew_angle = my_input_skew_angle: f64;
        get has_input_skew_angle = def_input_skew_angle: bool;
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
        has_input_skew_angle: bool,
        the_input_skew_angle: f64,
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
        state.def_input_skew_angle = has_input_skew_angle;
        if state.def_input_skew_angle {
            state.my_input_skew_angle = the_input_skew_angle;
        } else {
            state.my_input_skew_angle = 0.0;
        }
    }
}

step_entity! {
    /// Representation of STEP entity UniversalPairValue
    StepKinematicsUniversalPairValue, StepKinematicsUniversalPairValueState, "StepKinematicsUniversalPairValue", StepKinematicsPairValue, {
        my_first_rotation_angle: f64,
        my_second_rotation_angle: f64,
    }
}
impl StepKinematicsUniversalPairValue {
    step_methods! {
        get first_rotation_angle = my_first_rotation_angle: f64;
        set_copy set_first_rotation_angle = my_first_rotation_angle: f64;
        get second_rotation_angle = my_second_rotation_angle: f64;
        set_copy set_second_rotation_angle = my_second_rotation_angle: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_pair_value_applies_to_pair: &Handle<StepKinematicsKinematicPair>,
        the_first_rotation_angle: f64,
        the_second_rotation_angle: f64,
    ) {
        self.parent
            .init(the_representation_item_name, the_pair_value_applies_to_pair);
        let mut state = self.state.write();
        state.my_first_rotation_angle = the_first_rotation_angle;
        state.my_second_rotation_angle = the_second_rotation_angle;
    }
}

step_entity! {
    /// Representation of STEP entity UniversalPairWithRange
    StepKinematicsUniversalPairWithRange, StepKinematicsUniversalPairWithRangeState, "StepKinematicsUniversalPairWithRange", StepKinematicsUniversalPair, {
        my_lower_limit_first_rotation: f64,
        my_upper_limit_first_rotation: f64,
        my_lower_limit_second_rotation: f64,
        my_upper_limit_second_rotation: f64,
        def_lower_limit_first_rotation: bool,
        def_upper_limit_first_rotation: bool,
        def_lower_limit_second_rotation: bool,
        def_upper_limit_second_rotation: bool,
    }
}
impl StepKinematicsUniversalPairWithRange {
    step_methods! {
        get lower_limit_first_rotation = my_lower_limit_first_rotation: f64;
        set_copy set_lower_limit_first_rotation = my_lower_limit_first_rotation: f64;
        get has_lower_limit_first_rotation = def_lower_limit_first_rotation: bool;
        get upper_limit_first_rotation = my_upper_limit_first_rotation: f64;
        set_copy set_upper_limit_first_rotation = my_upper_limit_first_rotation: f64;
        get has_upper_limit_first_rotation = def_upper_limit_first_rotation: bool;
        get lower_limit_second_rotation = my_lower_limit_second_rotation: f64;
        set_copy set_lower_limit_second_rotation = my_lower_limit_second_rotation: f64;
        get has_lower_limit_second_rotation = def_lower_limit_second_rotation: bool;
        get upper_limit_second_rotation = my_upper_limit_second_rotation: f64;
        set_copy set_upper_limit_second_rotation = my_upper_limit_second_rotation: f64;
        get has_upper_limit_second_rotation = def_upper_limit_second_rotation: bool;
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
        has_universal_pair_input_skew_angle: bool,
        the_universal_pair_input_skew_angle: f64,
        has_lower_limit_first_rotation: bool,
        the_lower_limit_first_rotation: f64,
        has_upper_limit_first_rotation: bool,
        the_upper_limit_first_rotation: f64,
        has_lower_limit_second_rotation: bool,
        the_lower_limit_second_rotation: f64,
        has_upper_limit_second_rotation: bool,
        the_upper_limit_second_rotation: f64,
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
            has_universal_pair_input_skew_angle,
            the_universal_pair_input_skew_angle,
        );
        let mut state = self.state.write();
        state.def_lower_limit_first_rotation = has_lower_limit_first_rotation;
        if state.def_lower_limit_first_rotation {
            state.my_lower_limit_first_rotation = the_lower_limit_first_rotation;
        } else {
            state.my_lower_limit_first_rotation = 0.0;
        }
        state.def_upper_limit_first_rotation = has_upper_limit_first_rotation;
        if state.def_upper_limit_first_rotation {
            state.my_upper_limit_first_rotation = the_upper_limit_first_rotation;
        } else {
            state.my_upper_limit_first_rotation = 0.0;
        }
        state.def_lower_limit_second_rotation = has_lower_limit_second_rotation;
        if state.def_lower_limit_second_rotation {
            state.my_lower_limit_second_rotation = the_lower_limit_second_rotation;
        } else {
            state.my_lower_limit_second_rotation = 0.0;
        }
        state.def_upper_limit_second_rotation = has_upper_limit_second_rotation;
        if state.def_upper_limit_second_rotation {
            state.my_upper_limit_second_rotation = the_upper_limit_second_rotation;
        } else {
            state.my_upper_limit_second_rotation = 0.0;
        }
    }
}
