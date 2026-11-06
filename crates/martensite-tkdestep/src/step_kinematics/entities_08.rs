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
    /// Representation of STEP entity RollingSurfacePairValue
    StepKinematicsRollingSurfacePairValue, StepKinematicsRollingSurfacePairValueState, "StepKinematicsRollingSurfacePairValue", StepKinematicsPairValue, {
        my_actual_point_on_surface: Handle<StepGeomPointOnSurface>,
        my_actual_rotation: f64,
    }
}
impl StepKinematicsRollingSurfacePairValue {
    step_methods! {
        get actual_point_on_surface = my_actual_point_on_surface: Handle<StepGeomPointOnSurface>;
        set set_actual_point_on_surface = my_actual_point_on_surface: Handle<StepGeomPointOnSurface>;
        get actual_rotation = my_actual_rotation: f64;
        set_copy set_actual_rotation = my_actual_rotation: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_pair_value_applies_to_pair: &Handle<StepKinematicsKinematicPair>,
        the_actual_point_on_surface: &Handle<StepGeomPointOnSurface>,
        the_actual_rotation: f64,
    ) {
        self.parent
            .init(the_representation_item_name, the_pair_value_applies_to_pair);
        let mut state = self.state.write();
        state.my_actual_point_on_surface = the_actual_point_on_surface.clone();
        state.my_actual_rotation = the_actual_rotation;
    }
}

step_entity! {
    /// Representation of STEP entity RotationAboutDirection
    StepKinematicsRotationAboutDirection, StepKinematicsRotationAboutDirectionState, "StepKinematicsRotationAboutDirection", StepGeomGeometricRepresentationItem, {
        my_direction_of_axis: Handle<StepGeomDirection>,
        my_rotation_angle: f64,
    }
}
impl StepKinematicsRotationAboutDirection {
    step_methods! {
        get direction_of_axis = my_direction_of_axis: Handle<StepGeomDirection>;
        set set_direction_of_axis = my_direction_of_axis: Handle<StepGeomDirection>;
        get rotation_angle = my_rotation_angle: f64;
        set_copy set_rotation_angle = my_rotation_angle: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_direction_of_axis: &Handle<StepGeomDirection>,
        the_rotation_angle: f64,
    ) {
        self.parent.init(the_representation_item_name);
        let mut state = self.state.write();
        state.my_direction_of_axis = the_direction_of_axis.clone();
        state.my_rotation_angle = the_rotation_angle;
    }
}

step_entity! {
    /// Representation of STEP entity ScrewPair
    StepKinematicsScrewPair, StepKinematicsScrewPairState, "StepKinematicsScrewPair", StepKinematicsLowOrderKinematicPairWithMotionCoupling, {
        my_pitch: f64,
    }
}
impl StepKinematicsScrewPair {
    step_methods! {
        get pitch = my_pitch: f64;
        set_copy set_pitch = my_pitch: f64;
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
        the_pitch: f64,
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
        self.state.write().my_pitch = the_pitch;
    }
}

step_entity! {
    /// Representation of STEP entity ScrewPairValue
    StepKinematicsScrewPairValue, StepKinematicsScrewPairValueState, "StepKinematicsScrewPairValue", StepKinematicsPairValue, {
        my_actual_rotation: f64,
    }
}
impl StepKinematicsScrewPairValue {
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
    /// Representation of STEP entity ScrewPairWithRange
    StepKinematicsScrewPairWithRange, StepKinematicsScrewPairWithRangeState, "StepKinematicsScrewPairWithRange", StepKinematicsScrewPair, {
        my_lower_limit_actual_rotation: f64,
        my_upper_limit_actual_rotation: f64,
        def_lower_limit_actual_rotation: bool,
        def_upper_limit_actual_rotation: bool,
    }
}
impl StepKinematicsScrewPairWithRange {
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
        the_screw_pair_pitch: f64,
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
            the_screw_pair_pitch,
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
    /// Representation of STEP entity SlidingCurvePair
    StepKinematicsSlidingCurvePair, StepKinematicsSlidingCurvePairState, "StepKinematicsSlidingCurvePair", StepKinematicsPlanarCurvePair, {
    }
}

step_entity! {
    /// Representation of STEP entity SlidingCurvePairValue
    StepKinematicsSlidingCurvePairValue, StepKinematicsSlidingCurvePairValueState, "StepKinematicsSlidingCurvePairValue", StepKinematicsPairValue, {
        my_actual_point_on_curve1: Handle<StepGeomPointOnCurve>,
        my_actual_point_on_curve2: Handle<StepGeomPointOnCurve>,
    }
}
impl StepKinematicsSlidingCurvePairValue {
    step_methods! {
        get actual_point_on_curve1 = my_actual_point_on_curve1: Handle<StepGeomPointOnCurve>;
        set set_actual_point_on_curve1 = my_actual_point_on_curve1: Handle<StepGeomPointOnCurve>;
        get actual_point_on_curve2 = my_actual_point_on_curve2: Handle<StepGeomPointOnCurve>;
        set set_actual_point_on_curve2 = my_actual_point_on_curve2: Handle<StepGeomPointOnCurve>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_pair_value_applies_to_pair: &Handle<StepKinematicsKinematicPair>,
        the_actual_point_on_curve1: &Handle<StepGeomPointOnCurve>,
        the_actual_point_on_curve2: &Handle<StepGeomPointOnCurve>,
    ) {
        self.parent
            .init(the_representation_item_name, the_pair_value_applies_to_pair);
        let mut state = self.state.write();
        state.my_actual_point_on_curve1 = the_actual_point_on_curve1.clone();
        state.my_actual_point_on_curve2 = the_actual_point_on_curve2.clone();
    }
}

step_entity! {
    /// Representation of STEP entity SlidingSurfacePair
    StepKinematicsSlidingSurfacePair, StepKinematicsSlidingSurfacePairState, "StepKinematicsSlidingSurfacePair", StepKinematicsSurfacePair, {
    }
}

step_entity! {
    /// Representation of STEP entity SlidingSurfacePairValue
    StepKinematicsSlidingSurfacePairValue, StepKinematicsSlidingSurfacePairValueState, "StepKinematicsSlidingSurfacePairValue", StepKinematicsPairValue, {
        my_actual_point_on_surface1: Handle<StepGeomPointOnSurface>,
        my_actual_point_on_surface2: Handle<StepGeomPointOnSurface>,
        my_actual_rotation: f64,
    }
}
impl StepKinematicsSlidingSurfacePairValue {
    step_methods! {
        get actual_point_on_surface1 = my_actual_point_on_surface1: Handle<StepGeomPointOnSurface>;
        set set_actual_point_on_surface1 = my_actual_point_on_surface1: Handle<StepGeomPointOnSurface>;
        get actual_point_on_surface2 = my_actual_point_on_surface2: Handle<StepGeomPointOnSurface>;
        set set_actual_point_on_surface2 = my_actual_point_on_surface2: Handle<StepGeomPointOnSurface>;
        get actual_rotation = my_actual_rotation: f64;
        set_copy set_actual_rotation = my_actual_rotation: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_pair_value_applies_to_pair: &Handle<StepKinematicsKinematicPair>,
        the_actual_point_on_surface1: &Handle<StepGeomPointOnSurface>,
        the_actual_point_on_surface2: &Handle<StepGeomPointOnSurface>,
        the_actual_rotation: f64,
    ) {
        self.parent
            .init(the_representation_item_name, the_pair_value_applies_to_pair);
        let mut state = self.state.write();
        state.my_actual_point_on_surface1 = the_actual_point_on_surface1.clone();
        state.my_actual_point_on_surface2 = the_actual_point_on_surface2.clone();
        state.my_actual_rotation = the_actual_rotation;
    }
}

step_entity! {
    /// Representation of STEP entity SphericalPair
    StepKinematicsSphericalPair, StepKinematicsSphericalPairState, "StepKinematicsSphericalPair", StepKinematicsLowOrderKinematicPair, {
    }
}

step_entity! {
    /// Representation of STEP entity SphericalPairValue
    StepKinematicsSphericalPairValue, StepKinematicsSphericalPairValueState, "StepKinematicsSphericalPairValue", StepKinematicsPairValue, {
        my_input_orientation: StepKinematicsSpatialRotation,
    }
}
impl StepKinematicsSphericalPairValue {
    step_methods! {
        get input_orientation = my_input_orientation: StepKinematicsSpatialRotation;
        set set_input_orientation = my_input_orientation: StepKinematicsSpatialRotation;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_pair_value_applies_to_pair: &Handle<StepKinematicsKinematicPair>,
        the_input_orientation: &StepKinematicsSpatialRotation,
    ) {
        self.parent
            .init(the_representation_item_name, the_pair_value_applies_to_pair);
        self.state.write().my_input_orientation = the_input_orientation.clone();
    }
}

step_entity! {
    /// Representation of STEP entity SphericalPairWithPin
    StepKinematicsSphericalPairWithPin, StepKinematicsSphericalPairWithPinState, "StepKinematicsSphericalPairWithPin", StepKinematicsLowOrderKinematicPair, {
    }
}
