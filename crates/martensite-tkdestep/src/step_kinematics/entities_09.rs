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
    /// Representation of STEP entity SphericalPairWithPinAndRange
    StepKinematicsSphericalPairWithPinAndRange, StepKinematicsSphericalPairWithPinAndRangeState, "StepKinematicsSphericalPairWithPinAndRange", StepKinematicsSphericalPairWithPin, {
        my_lower_limit_yaw: f64,
        my_upper_limit_yaw: f64,
        my_lower_limit_roll: f64,
        my_upper_limit_roll: f64,
        def_lower_limit_yaw: bool,
        def_upper_limit_yaw: bool,
        def_lower_limit_roll: bool,
        def_upper_limit_roll: bool,
    }
}
impl StepKinematicsSphericalPairWithPinAndRange {
    step_methods! {
        get lower_limit_yaw = my_lower_limit_yaw: f64;
        set_copy set_lower_limit_yaw = my_lower_limit_yaw: f64;
        get has_lower_limit_yaw = def_lower_limit_yaw: bool;
        get upper_limit_yaw = my_upper_limit_yaw: f64;
        set_copy set_upper_limit_yaw = my_upper_limit_yaw: f64;
        get has_upper_limit_yaw = def_upper_limit_yaw: bool;
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
        the_low_order_kinematic_pair_tx: bool,
        the_low_order_kinematic_pair_ty: bool,
        the_low_order_kinematic_pair_tz: bool,
        the_low_order_kinematic_pair_rx: bool,
        the_low_order_kinematic_pair_ry: bool,
        the_low_order_kinematic_pair_rz: bool,
        has_lower_limit_yaw: bool,
        the_lower_limit_yaw: f64,
        has_upper_limit_yaw: bool,
        the_upper_limit_yaw: f64,
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
            the_low_order_kinematic_pair_tx,
            the_low_order_kinematic_pair_ty,
            the_low_order_kinematic_pair_tz,
            the_low_order_kinematic_pair_rx,
            the_low_order_kinematic_pair_ry,
            the_low_order_kinematic_pair_rz,
        );
        let mut state = self.state.write();
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
    /// Representation of STEP entity SphericalPairWithRange
    StepKinematicsSphericalPairWithRange, StepKinematicsSphericalPairWithRangeState, "StepKinematicsSphericalPairWithRange", StepKinematicsSphericalPair, {
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
impl StepKinematicsSphericalPairWithRange {
    step_methods! {
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
        the_low_order_kinematic_pair_tx: bool,
        the_low_order_kinematic_pair_ty: bool,
        the_low_order_kinematic_pair_tz: bool,
        the_low_order_kinematic_pair_rx: bool,
        the_low_order_kinematic_pair_ry: bool,
        the_low_order_kinematic_pair_rz: bool,
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
            the_low_order_kinematic_pair_tx,
            the_low_order_kinematic_pair_ty,
            the_low_order_kinematic_pair_tz,
            the_low_order_kinematic_pair_rx,
            the_low_order_kinematic_pair_ry,
            the_low_order_kinematic_pair_rz,
        );
        let mut state = self.state.write();
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
    /// Representation of STEP entity SurfacePair
    StepKinematicsSurfacePair, StepKinematicsSurfacePairState, "StepKinematicsSurfacePair", StepKinematicsHighOrderKinematicPair, {
        my_surface1: Handle<StepGeomSurface>,
        my_surface2: Handle<StepGeomSurface>,
        my_orientation: bool,
    }
}
impl StepKinematicsSurfacePair {
    step_methods! {
        get surface1 = my_surface1: Handle<StepGeomSurface>;
        set set_surface1 = my_surface1: Handle<StepGeomSurface>;
        get surface2 = my_surface2: Handle<StepGeomSurface>;
        set set_surface2 = my_surface2: Handle<StepGeomSurface>;
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
        the_surface1: &Handle<StepGeomSurface>,
        the_surface2: &Handle<StepGeomSurface>,
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
        state.my_surface1 = the_surface1.clone();
        state.my_surface2 = the_surface2.clone();
        state.my_orientation = the_orientation;
    }
}

step_entity! {
    /// Representation of STEP entity SurfacePairWithRange
    StepKinematicsSurfacePairWithRange, StepKinematicsSurfacePairWithRangeState, "StepKinematicsSurfacePairWithRange", StepKinematicsSurfacePair, {
        my_range_on_surface1: Handle<StepGeomRectangularTrimmedSurface>,
        my_range_on_surface2: Handle<StepGeomRectangularTrimmedSurface>,
        my_lower_limit_actual_rotation: f64,
        my_upper_limit_actual_rotation: f64,
        def_lower_limit_actual_rotation: bool,
        def_upper_limit_actual_rotation: bool,
    }
}
impl StepKinematicsSurfacePairWithRange {
    step_methods! {
        get range_on_surface1 = my_range_on_surface1: Handle<StepGeomRectangularTrimmedSurface>;
        set set_range_on_surface1 = my_range_on_surface1: Handle<StepGeomRectangularTrimmedSurface>;
        get range_on_surface2 = my_range_on_surface2: Handle<StepGeomRectangularTrimmedSurface>;
        set set_range_on_surface2 = my_range_on_surface2: Handle<StepGeomRectangularTrimmedSurface>;
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
        the_surface_pair_surface1: &Handle<StepGeomSurface>,
        the_surface_pair_surface2: &Handle<StepGeomSurface>,
        the_surface_pair_orientation: bool,
        the_range_on_surface1: &Handle<StepGeomRectangularTrimmedSurface>,
        the_range_on_surface2: &Handle<StepGeomRectangularTrimmedSurface>,
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
            the_surface_pair_surface1,
            the_surface_pair_surface2,
            the_surface_pair_orientation,
        );
        let mut state = self.state.write();
        state.my_range_on_surface1 = the_range_on_surface1.clone();
        state.my_range_on_surface2 = the_range_on_surface2.clone();
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
    /// Representation of STEP entity UnconstrainedPair
    StepKinematicsUnconstrainedPair, StepKinematicsUnconstrainedPairState, "StepKinematicsUnconstrainedPair", StepKinematicsLowOrderKinematicPair, {
    }
}

step_entity! {
    /// Representation of STEP entity UnconstrainedPairValue
    StepKinematicsUnconstrainedPairValue, StepKinematicsUnconstrainedPairValueState, "StepKinematicsUnconstrainedPairValue", StepKinematicsPairValue, {
        my_actual_placement: Handle<StepGeomAxis2Placement3d>,
    }
}
impl StepKinematicsUnconstrainedPairValue {
    step_methods! {
        get actual_placement = my_actual_placement: Handle<StepGeomAxis2Placement3d>;
        set set_actual_placement = my_actual_placement: Handle<StepGeomAxis2Placement3d>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_pair_value_applies_to_pair: &Handle<StepKinematicsKinematicPair>,
        the_actual_placement: &Handle<StepGeomAxis2Placement3d>,
    ) {
        self.parent
            .init(the_representation_item_name, the_pair_value_applies_to_pair);
        self.state.write().my_actual_placement = the_actual_placement.clone();
    }
}
