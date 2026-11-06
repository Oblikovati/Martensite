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
    /// Representation of STEP entity PlanarCurvePair
    StepKinematicsPlanarCurvePair, StepKinematicsPlanarCurvePairState, "StepKinematicsPlanarCurvePair", StepKinematicsHighOrderKinematicPair, {
        my_curve1: Handle<StepGeomCurve>,
        my_curve2: Handle<StepGeomCurve>,
        my_orientation: bool,
    }
}
impl StepKinematicsPlanarCurvePair {
    step_methods! {
        get curve1 = my_curve1: Handle<StepGeomCurve>;
        set set_curve1 = my_curve1: Handle<StepGeomCurve>;
        get curve2 = my_curve2: Handle<StepGeomCurve>;
        set set_curve2 = my_curve2: Handle<StepGeomCurve>;
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
        the_curve1: &Handle<StepGeomCurve>,
        the_curve2: &Handle<StepGeomCurve>,
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
        state.my_curve1 = the_curve1.clone();
        state.my_curve2 = the_curve2.clone();
        state.my_orientation = the_orientation;
    }
}

step_entity! {
    /// Representation of STEP entity PlanarCurvePairRange
    StepKinematicsPlanarCurvePairRange, StepKinematicsPlanarCurvePairRangeState, "StepKinematicsPlanarCurvePairRange", StepKinematicsPlanarCurvePair, {
        my_range_on_curve1: Handle<StepGeomTrimmedCurve>,
        my_range_on_curve2: Handle<StepGeomTrimmedCurve>,
    }
}
impl StepKinematicsPlanarCurvePairRange {
    step_methods! {
        get range_on_curve1 = my_range_on_curve1: Handle<StepGeomTrimmedCurve>;
        set set_range_on_curve1 = my_range_on_curve1: Handle<StepGeomTrimmedCurve>;
        get range_on_curve2 = my_range_on_curve2: Handle<StepGeomTrimmedCurve>;
        set set_range_on_curve2 = my_range_on_curve2: Handle<StepGeomTrimmedCurve>;
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
        the_planar_curve_pair_curve1: &Handle<StepGeomCurve>,
        the_planar_curve_pair_curve2: &Handle<StepGeomCurve>,
        the_planar_curve_pair_orientation: bool,
        the_range_on_curve1: &Handle<StepGeomTrimmedCurve>,
        the_range_on_curve2: &Handle<StepGeomTrimmedCurve>,
    ) {
        self.parent.init(
            the_representation_item_name,
            the_item_defined_transformation_name,
            has_item_defined_transformation_description,
            the_item_defined_transformation_description,
            the_item_defined_transformation_transform_item1,
            the_item_defined_transformation_transform_item2,
            the_kinematic_pair_joint,
            the_planar_curve_pair_curve1,
            the_planar_curve_pair_curve2,
            the_planar_curve_pair_orientation,
        );
        let mut state = self.state.write();
        state.my_range_on_curve1 = the_range_on_curve1.clone();
        state.my_range_on_curve2 = the_range_on_curve2.clone();
    }
}

step_entity! {
    /// Representation of STEP entity PlanarPair
    StepKinematicsPlanarPair, StepKinematicsPlanarPairState, "StepKinematicsPlanarPair", StepKinematicsLowOrderKinematicPair, {
    }
}

step_entity! {
    /// Representation of STEP entity PlanarPairValue
    StepKinematicsPlanarPairValue, StepKinematicsPlanarPairValueState, "StepKinematicsPlanarPairValue", StepKinematicsPairValue, {
        my_actual_rotation: f64,
        my_actual_translation_x: f64,
        my_actual_translation_y: f64,
    }
}
impl StepKinematicsPlanarPairValue {
    step_methods! {
        get actual_rotation = my_actual_rotation: f64;
        set_copy set_actual_rotation = my_actual_rotation: f64;
        get actual_translation_x = my_actual_translation_x: f64;
        set_copy set_actual_translation_x = my_actual_translation_x: f64;
        get actual_translation_y = my_actual_translation_y: f64;
        set_copy set_actual_translation_y = my_actual_translation_y: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_pair_value_applies_to_pair: &Handle<StepKinematicsKinematicPair>,
        the_actual_rotation: f64,
        the_actual_translation_x: f64,
        the_actual_translation_y: f64,
    ) {
        self.parent
            .init(the_representation_item_name, the_pair_value_applies_to_pair);
        let mut state = self.state.write();
        state.my_actual_rotation = the_actual_rotation;
        state.my_actual_translation_x = the_actual_translation_x;
        state.my_actual_translation_y = the_actual_translation_y;
    }
}

step_entity! {
    /// Representation of STEP entity PlanarPairWithRange
    StepKinematicsPlanarPairWithRange, StepKinematicsPlanarPairWithRangeState, "StepKinematicsPlanarPairWithRange", StepKinematicsPlanarPair, {
        my_lower_limit_actual_rotation: f64,
        my_upper_limit_actual_rotation: f64,
        my_lower_limit_actual_translation_x: f64,
        my_upper_limit_actual_translation_x: f64,
        my_lower_limit_actual_translation_y: f64,
        my_upper_limit_actual_translation_y: f64,
        def_lower_limit_actual_rotation: bool,
        def_upper_limit_actual_rotation: bool,
        def_lower_limit_actual_translation_x: bool,
        def_upper_limit_actual_translation_x: bool,
        def_lower_limit_actual_translation_y: bool,
        def_upper_limit_actual_translation_y: bool,
    }
}
impl StepKinematicsPlanarPairWithRange {
    step_methods! {
        get lower_limit_actual_rotation = my_lower_limit_actual_rotation: f64;
        set_copy set_lower_limit_actual_rotation = my_lower_limit_actual_rotation: f64;
        get has_lower_limit_actual_rotation = def_lower_limit_actual_rotation: bool;
        get upper_limit_actual_rotation = my_upper_limit_actual_rotation: f64;
        set_copy set_upper_limit_actual_rotation = my_upper_limit_actual_rotation: f64;
        get has_upper_limit_actual_rotation = def_upper_limit_actual_rotation: bool;
        get lower_limit_actual_translation_x = my_lower_limit_actual_translation_x: f64;
        set_copy set_lower_limit_actual_translation_x = my_lower_limit_actual_translation_x: f64;
        get has_lower_limit_actual_translation_x = def_lower_limit_actual_translation_x: bool;
        get upper_limit_actual_translation_x = my_upper_limit_actual_translation_x: f64;
        set_copy set_upper_limit_actual_translation_x = my_upper_limit_actual_translation_x: f64;
        get has_upper_limit_actual_translation_x = def_upper_limit_actual_translation_x: bool;
        get lower_limit_actual_translation_y = my_lower_limit_actual_translation_y: f64;
        set_copy set_lower_limit_actual_translation_y = my_lower_limit_actual_translation_y: f64;
        get has_lower_limit_actual_translation_y = def_lower_limit_actual_translation_y: bool;
        get upper_limit_actual_translation_y = my_upper_limit_actual_translation_y: f64;
        set_copy set_upper_limit_actual_translation_y = my_upper_limit_actual_translation_y: f64;
        get has_upper_limit_actual_translation_y = def_upper_limit_actual_translation_y: bool;
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
        has_lower_limit_actual_translation_x: bool,
        the_lower_limit_actual_translation_x: f64,
        has_upper_limit_actual_translation_x: bool,
        the_upper_limit_actual_translation_x: f64,
        has_lower_limit_actual_translation_y: bool,
        the_lower_limit_actual_translation_y: f64,
        has_upper_limit_actual_translation_y: bool,
        the_upper_limit_actual_translation_y: f64,
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
        state.def_lower_limit_actual_translation_x = has_lower_limit_actual_translation_x;
        if state.def_lower_limit_actual_translation_x {
            state.my_lower_limit_actual_translation_x = the_lower_limit_actual_translation_x;
        } else {
            state.my_lower_limit_actual_translation_x = 0.0;
        }
        state.def_upper_limit_actual_translation_x = has_upper_limit_actual_translation_x;
        if state.def_upper_limit_actual_translation_x {
            state.my_upper_limit_actual_translation_x = the_upper_limit_actual_translation_x;
        } else {
            state.my_upper_limit_actual_translation_x = 0.0;
        }
        state.def_lower_limit_actual_translation_y = has_lower_limit_actual_translation_y;
        if state.def_lower_limit_actual_translation_y {
            state.my_lower_limit_actual_translation_y = the_lower_limit_actual_translation_y;
        } else {
            state.my_lower_limit_actual_translation_y = 0.0;
        }
        state.def_upper_limit_actual_translation_y = has_upper_limit_actual_translation_y;
        if state.def_upper_limit_actual_translation_y {
            state.my_upper_limit_actual_translation_y = the_upper_limit_actual_translation_y;
        } else {
            state.my_upper_limit_actual_translation_y = 0.0;
        }
    }
}

step_entity! {
    /// Representation of STEP entity PointOnPlanarCurvePair
    StepKinematicsPointOnPlanarCurvePair, StepKinematicsPointOnPlanarCurvePairState, "StepKinematicsPointOnPlanarCurvePair", StepKinematicsHighOrderKinematicPair, {
        my_pair_curve: Handle<StepGeomCurve>,
        my_orientation: bool,
    }
}
impl StepKinematicsPointOnPlanarCurvePair {
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
    /// Representation of STEP entity PointOnPlanarCurvePairValue
    StepKinematicsPointOnPlanarCurvePairValue, StepKinematicsPointOnPlanarCurvePairValueState, "StepKinematicsPointOnPlanarCurvePairValue", StepKinematicsPairValue, {
        my_actual_point_on_curve: Handle<StepGeomPointOnCurve>,
        my_input_orientation: StepKinematicsSpatialRotation,
    }
}
impl StepKinematicsPointOnPlanarCurvePairValue {
    step_methods! {
        get actual_point_on_curve = my_actual_point_on_curve: Handle<StepGeomPointOnCurve>;
        set set_actual_point_on_curve = my_actual_point_on_curve: Handle<StepGeomPointOnCurve>;
        get input_orientation = my_input_orientation: StepKinematicsSpatialRotation;
        set set_input_orientation = my_input_orientation: StepKinematicsSpatialRotation;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_pair_value_applies_to_pair: &Handle<StepKinematicsKinematicPair>,
        the_actual_point_on_curve: &Handle<StepGeomPointOnCurve>,
        the_input_orientation: &StepKinematicsSpatialRotation,
    ) {
        self.parent
            .init(the_representation_item_name, the_pair_value_applies_to_pair);
        let mut state = self.state.write();
        state.my_actual_point_on_curve = the_actual_point_on_curve.clone();
        state.my_input_orientation = the_input_orientation.clone();
    }
}
