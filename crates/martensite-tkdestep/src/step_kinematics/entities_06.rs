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
    /// Representation of STEP entity PointOnPlanarCurvePairWithRange
    StepKinematicsPointOnPlanarCurvePairWithRange, StepKinematicsPointOnPlanarCurvePairWithRangeState, "StepKinematicsPointOnPlanarCurvePairWithRange", StepKinematicsPointOnPlanarCurvePair, {
        my_range_on_pair_curve: Handle<StepGeomTrimmedCurve>,
        my_lower_limit_yaw: f64,
        my_upper_limit_yaw: f64,
        my_lower_limit_pitch: f64,
        my_upper_limit_pitch: f64,
        my_lower_limit_roll: f64,
        my_upper_limit_roll: f64,
        def_lower_limit_yaw: bool,
        def_upper_limit_yaw: bool,
        def_lower_limit_pitch: bool,
        def_upper_limit_pitch: bool,
        def_lower_limit_roll: bool,
        def_upper_limit_roll: bool,
    }
}
impl StepKinematicsPointOnPlanarCurvePairWithRange {
    step_methods! {
        get range_on_pair_curve = my_range_on_pair_curve: Handle<StepGeomTrimmedCurve>;
        set set_range_on_pair_curve = my_range_on_pair_curve: Handle<StepGeomTrimmedCurve>;
        get lower_limit_yaw = my_lower_limit_yaw: f64;
        set_copy set_lower_limit_yaw = my_lower_limit_yaw: f64;
        get has_lower_limit_yaw = def_lower_limit_yaw: bool;
        get upper_limit_yaw = my_upper_limit_yaw: f64;
        set_copy set_upper_limit_yaw = my_upper_limit_yaw: f64;
        get has_upper_limit_yaw = def_upper_limit_yaw: bool;
        get lower_limit_pitch = my_lower_limit_pitch: f64;
        set_copy set_lower_limit_pitch = my_lower_limit_pitch: f64;
        get has_lower_limit_pitch = def_lower_limit_pitch: bool;
        get upper_limit_pitch = my_upper_limit_pitch: f64;
        set_copy set_upper_limit_pitch = my_upper_limit_pitch: f64;
        get has_upper_limit_pitch = def_upper_limit_pitch: bool;
        get lower_limit_roll = my_lower_limit_roll: f64;
        set_copy set_lower_limit_roll = my_lower_limit_roll: f64;
        get has_lower_limit_roll = def_lower_limit_roll: bool;
        get upper_limit_roll = my_upper_limit_roll: f64;
        set_copy set_upper_limit_roll = my_upper_limit_roll: f64;
        get has_upper_limit_roll = def_upper_limit_roll: bool;
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
        the_point_on_planar_curve_pair_pair_curve: &Handle<StepGeomCurve>,
        the_point_on_planar_curve_pair_orientation: bool,
        the_range_on_pair_curve: &Handle<StepGeomTrimmedCurve>,
        has_lower_limit_yaw: bool,
        the_lower_limit_yaw: f64,
        has_upper_limit_yaw: bool,
        the_upper_limit_yaw: f64,
        has_lower_limit_pitch: bool,
        the_lower_limit_pitch: f64,
        has_upper_limit_pitch: bool,
        the_upper_limit_pitch: f64,
        has_lower_limit_roll: bool,
        the_lower_limit_roll: f64,
        has_upper_limit_roll: bool,
        the_upper_limit_roll: f64,
    ) {
        self.parent.init(
            the_representation_item_name,
            the_item_defined_transformation_name,
            has_item_defined_transformation_description,
            the_item_defined_transformation_description,
            the_item_defined_transformation_transform_item1,
            the_item_defined_transformation_transform_item2,
            the_kinematic_pair_joint,
            the_point_on_planar_curve_pair_pair_curve,
            the_point_on_planar_curve_pair_orientation,
        );
        let mut state = self.state.write();
        state.my_range_on_pair_curve = the_range_on_pair_curve.clone();
        state.def_lower_limit_yaw = has_lower_limit_yaw;
        if state.def_lower_limit_yaw {
            state.my_lower_limit_yaw = the_lower_limit_yaw;
        } else {
            state.my_lower_limit_yaw = 0.0;
        }
        state.def_upper_limit_yaw = has_upper_limit_yaw;
        if state.def_upper_limit_yaw {
            state.my_upper_limit_yaw = the_upper_limit_yaw;
        } else {
            state.my_upper_limit_yaw = 0.0;
        }
        state.def_lower_limit_pitch = has_lower_limit_pitch;
        if state.def_lower_limit_pitch {
            state.my_lower_limit_pitch = the_lower_limit_pitch;
        } else {
            state.my_lower_limit_pitch = 0.0;
        }
        state.def_upper_limit_pitch = has_upper_limit_pitch;
        if state.def_upper_limit_pitch {
            state.my_upper_limit_pitch = the_upper_limit_pitch;
        } else {
            state.my_upper_limit_pitch = 0.0;
        }
        state.def_lower_limit_roll = has_lower_limit_roll;
        if state.def_lower_limit_roll {
            state.my_lower_limit_roll = the_lower_limit_roll;
        } else {
            state.my_lower_limit_roll = 0.0;
        }
        state.def_upper_limit_roll = has_upper_limit_roll;
        if state.def_upper_limit_roll {
            state.my_upper_limit_roll = the_upper_limit_roll;
        } else {
            state.my_upper_limit_roll = 0.0;
        }
    }
}

step_entity! {
    /// Representation of STEP entity PointOnSurfacePair
    StepKinematicsPointOnSurfacePair, StepKinematicsPointOnSurfacePairState, "StepKinematicsPointOnSurfacePair", StepKinematicsHighOrderKinematicPair, {
        my_pair_surface: Handle<StepGeomSurface>,
    }
}
impl StepKinematicsPointOnSurfacePair {
    step_methods! {
        get pair_surface = my_pair_surface: Handle<StepGeomSurface>;
        set set_pair_surface = my_pair_surface: Handle<StepGeomSurface>;
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
        the_pair_surface: &Handle<StepGeomSurface>,
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
        self.state.write().my_pair_surface = the_pair_surface.clone();
    }
}

step_entity! {
    /// Representation of STEP entity PointOnSurfacePairValue
    StepKinematicsPointOnSurfacePairValue, StepKinematicsPointOnSurfacePairValueState, "StepKinematicsPointOnSurfacePairValue", StepKinematicsPairValue, {
        my_actual_point_on_surface: Handle<StepGeomPointOnSurface>,
        my_input_orientation: StepKinematicsSpatialRotation,
    }
}
impl StepKinematicsPointOnSurfacePairValue {
    step_methods! {
        get actual_point_on_surface = my_actual_point_on_surface: Handle<StepGeomPointOnSurface>;
        set set_actual_point_on_surface = my_actual_point_on_surface: Handle<StepGeomPointOnSurface>;
        get input_orientation = my_input_orientation: StepKinematicsSpatialRotation;
        set set_input_orientation = my_input_orientation: StepKinematicsSpatialRotation;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_pair_value_applies_to_pair: &Handle<StepKinematicsKinematicPair>,
        the_actual_point_on_surface: &Handle<StepGeomPointOnSurface>,
        the_input_orientation: &StepKinematicsSpatialRotation,
    ) {
        self.parent
            .init(the_representation_item_name, the_pair_value_applies_to_pair);
        let mut state = self.state.write();
        state.my_actual_point_on_surface = the_actual_point_on_surface.clone();
        state.my_input_orientation = the_input_orientation.clone();
    }
}

step_entity! {
    /// Representation of STEP entity PointOnSurfacePairWithRange
    StepKinematicsPointOnSurfacePairWithRange, StepKinematicsPointOnSurfacePairWithRangeState, "StepKinematicsPointOnSurfacePairWithRange", StepKinematicsPointOnSurfacePair, {
        my_range_on_pair_surface: Handle<StepGeomRectangularTrimmedSurface>,
        my_lower_limit_yaw: f64,
        my_upper_limit_yaw: f64,
        my_lower_limit_pitch: f64,
        my_upper_limit_pitch: f64,
        my_lower_limit_roll: f64,
        my_upper_limit_roll: f64,
        def_lower_limit_yaw: bool,
        def_upper_limit_yaw: bool,
        def_lower_limit_pitch: bool,
        def_upper_limit_pitch: bool,
        def_lower_limit_roll: bool,
        def_upper_limit_roll: bool,
    }
}
impl StepKinematicsPointOnSurfacePairWithRange {
    step_methods! {
        get range_on_pair_surface = my_range_on_pair_surface: Handle<StepGeomRectangularTrimmedSurface>;
        set set_range_on_pair_surface = my_range_on_pair_surface: Handle<StepGeomRectangularTrimmedSurface>;
        get lower_limit_yaw = my_lower_limit_yaw: f64;
        set_copy set_lower_limit_yaw = my_lower_limit_yaw: f64;
        get has_lower_limit_yaw = def_lower_limit_yaw: bool;
        get upper_limit_yaw = my_upper_limit_yaw: f64;
        set_copy set_upper_limit_yaw = my_upper_limit_yaw: f64;
        get has_upper_limit_yaw = def_upper_limit_yaw: bool;
        get lower_limit_pitch = my_lower_limit_pitch: f64;
        set_copy set_lower_limit_pitch = my_lower_limit_pitch: f64;
        get has_lower_limit_pitch = def_lower_limit_pitch: bool;
        get upper_limit_pitch = my_upper_limit_pitch: f64;
        set_copy set_upper_limit_pitch = my_upper_limit_pitch: f64;
        get has_upper_limit_pitch = def_upper_limit_pitch: bool;
        get lower_limit_roll = my_lower_limit_roll: f64;
        set_copy set_lower_limit_roll = my_lower_limit_roll: f64;
        get has_lower_limit_roll = def_lower_limit_roll: bool;
        get upper_limit_roll = my_upper_limit_roll: f64;
        set_copy set_upper_limit_roll = my_upper_limit_roll: f64;
        get has_upper_limit_roll = def_upper_limit_roll: bool;
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
        the_point_on_surface_pair_pair_surface: &Handle<StepGeomSurface>,
        the_range_on_pair_surface: &Handle<StepGeomRectangularTrimmedSurface>,
        has_lower_limit_yaw: bool,
        the_lower_limit_yaw: f64,
        has_upper_limit_yaw: bool,
        the_upper_limit_yaw: f64,
        has_lower_limit_pitch: bool,
        the_lower_limit_pitch: f64,
        has_upper_limit_pitch: bool,
        the_upper_limit_pitch: f64,
        has_lower_limit_roll: bool,
        the_lower_limit_roll: f64,
        has_upper_limit_roll: bool,
        the_upper_limit_roll: f64,
    ) {
        self.parent.init(
            the_representation_item_name,
            the_item_defined_transformation_name,
            has_item_defined_transformation_description,
            the_item_defined_transformation_description,
            the_item_defined_transformation_transform_item1,
            the_item_defined_transformation_transform_item2,
            the_kinematic_pair_joint,
            the_point_on_surface_pair_pair_surface,
        );
        let mut state = self.state.write();
        state.my_range_on_pair_surface = the_range_on_pair_surface.clone();
        state.def_lower_limit_yaw = has_lower_limit_yaw;
        if state.def_lower_limit_yaw {
            state.my_lower_limit_yaw = the_lower_limit_yaw;
        } else {
            state.my_lower_limit_yaw = 0.0;
        }
        state.def_upper_limit_yaw = has_upper_limit_yaw;
        if state.def_upper_limit_yaw {
            state.my_upper_limit_yaw = the_upper_limit_yaw;
        } else {
            state.my_upper_limit_yaw = 0.0;
        }
        state.def_lower_limit_pitch = has_lower_limit_pitch;
        if state.def_lower_limit_pitch {
            state.my_lower_limit_pitch = the_lower_limit_pitch;
        } else {
            state.my_lower_limit_pitch = 0.0;
        }
        state.def_upper_limit_pitch = has_upper_limit_pitch;
        if state.def_upper_limit_pitch {
            state.my_upper_limit_pitch = the_upper_limit_pitch;
        } else {
            state.my_upper_limit_pitch = 0.0;
        }
        state.def_lower_limit_roll = has_lower_limit_roll;
        if state.def_lower_limit_roll {
            state.my_lower_limit_roll = the_lower_limit_roll;
        } else {
            state.my_lower_limit_roll = 0.0;
        }
        state.def_upper_limit_roll = has_upper_limit_roll;
        if state.def_upper_limit_roll {
            state.my_upper_limit_roll = the_upper_limit_roll;
        } else {
            state.my_upper_limit_roll = 0.0;
        }
    }
}

step_entity! {
    /// Representation of STEP entity PrismaticPair
    StepKinematicsPrismaticPair, StepKinematicsPrismaticPairState, "StepKinematicsPrismaticPair", StepKinematicsLowOrderKinematicPair, {
    }
}

step_entity! {
    /// Representation of STEP entity PrismaticPairValue
    StepKinematicsPrismaticPairValue, StepKinematicsPrismaticPairValueState, "StepKinematicsPrismaticPairValue", StepKinematicsPairValue, {
        my_actual_translation: f64,
    }
}
impl StepKinematicsPrismaticPairValue {
    step_methods! {
        get actual_translation = my_actual_translation: f64;
        set_copy set_actual_translation = my_actual_translation: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_pair_value_applies_to_pair: &Handle<StepKinematicsKinematicPair>,
        the_actual_translation: f64,
    ) {
        self.parent
            .init(the_representation_item_name, the_pair_value_applies_to_pair);
        self.state.write().my_actual_translation = the_actual_translation;
    }
}
