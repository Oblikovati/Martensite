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
    /// Representation of STEP entity LowOrderKinematicPairWithRange
    StepKinematicsLowOrderKinematicPairWithRange, StepKinematicsLowOrderKinematicPairWithRangeState, "StepKinematicsLowOrderKinematicPairWithRange", StepKinematicsLowOrderKinematicPair, {
        my_lower_limit_actual_rotation_x: f64,
        my_upper_limit_actual_rotation_x: f64,
        my_lower_limit_actual_rotation_y: f64,
        my_upper_limit_actual_rotation_y: f64,
        my_lower_limit_actual_rotation_z: f64,
        my_upper_limit_actual_rotation_z: f64,
        my_lower_limit_actual_translation_x: f64,
        my_upper_limit_actual_translation_x: f64,
        my_lower_limit_actual_translation_y: f64,
        my_upper_limit_actual_translation_y: f64,
        my_lower_limit_actual_translation_z: f64,
        my_upper_limit_actual_translation_z: f64,
        def_lower_limit_actual_rotation_x: bool,
        def_upper_limit_actual_rotation_x: bool,
        def_lower_limit_actual_rotation_y: bool,
        def_upper_limit_actual_rotation_y: bool,
        def_lower_limit_actual_rotation_z: bool,
        def_upper_limit_actual_rotation_z: bool,
        def_lower_limit_actual_translation_x: bool,
        def_upper_limit_actual_translation_x: bool,
        def_lower_limit_actual_translation_y: bool,
        def_upper_limit_actual_translation_y: bool,
        def_lower_limit_actual_translation_z: bool,
        def_upper_limit_actual_translation_z: bool,
    }
}
impl StepKinematicsLowOrderKinematicPairWithRange {
    step_methods! {
        get lower_limit_actual_rotation_x = my_lower_limit_actual_rotation_x: f64;
        set_copy set_lower_limit_actual_rotation_x = my_lower_limit_actual_rotation_x: f64;
        get has_lower_limit_actual_rotation_x = def_lower_limit_actual_rotation_x: bool;
        get upper_limit_actual_rotation_x = my_upper_limit_actual_rotation_x: f64;
        set_copy set_upper_limit_actual_rotation_x = my_upper_limit_actual_rotation_x: f64;
        get has_upper_limit_actual_rotation_x = def_upper_limit_actual_rotation_x: bool;
        get lower_limit_actual_rotation_y = my_lower_limit_actual_rotation_y: f64;
        set_copy set_lower_limit_actual_rotation_y = my_lower_limit_actual_rotation_y: f64;
        get has_lower_limit_actual_rotation_y = def_lower_limit_actual_rotation_y: bool;
        get upper_limit_actual_rotation_y = my_upper_limit_actual_rotation_y: f64;
        set_copy set_upper_limit_actual_rotation_y = my_upper_limit_actual_rotation_y: f64;
        get has_upper_limit_actual_rotation_y = def_upper_limit_actual_rotation_y: bool;
        get lower_limit_actual_rotation_z = my_lower_limit_actual_rotation_z: f64;
        set_copy set_lower_limit_actual_rotation_z = my_lower_limit_actual_rotation_z: f64;
        get has_lower_limit_actual_rotation_z = def_lower_limit_actual_rotation_z: bool;
        get upper_limit_actual_rotation_z = my_upper_limit_actual_rotation_z: f64;
        set_copy set_upper_limit_actual_rotation_z = my_upper_limit_actual_rotation_z: f64;
        get has_upper_limit_actual_rotation_z = def_upper_limit_actual_rotation_z: bool;
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
        get lower_limit_actual_translation_z = my_lower_limit_actual_translation_z: f64;
        set_copy set_lower_limit_actual_translation_z = my_lower_limit_actual_translation_z: f64;
        get has_lower_limit_actual_translation_z = def_lower_limit_actual_translation_z: bool;
        get upper_limit_actual_translation_z = my_upper_limit_actual_translation_z: f64;
        set_copy set_upper_limit_actual_translation_z = my_upper_limit_actual_translation_z: f64;
        get has_upper_limit_actual_translation_z = def_upper_limit_actual_translation_z: bool;
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
        has_lower_limit_actual_rotation_x: bool,
        the_lower_limit_actual_rotation_x: f64,
        has_upper_limit_actual_rotation_x: bool,
        the_upper_limit_actual_rotation_x: f64,
        has_lower_limit_actual_rotation_y: bool,
        the_lower_limit_actual_rotation_y: f64,
        has_upper_limit_actual_rotation_y: bool,
        the_upper_limit_actual_rotation_y: f64,
        has_lower_limit_actual_rotation_z: bool,
        the_lower_limit_actual_rotation_z: f64,
        has_upper_limit_actual_rotation_z: bool,
        the_upper_limit_actual_rotation_z: f64,
        has_lower_limit_actual_translation_x: bool,
        the_lower_limit_actual_translation_x: f64,
        has_upper_limit_actual_translation_x: bool,
        the_upper_limit_actual_translation_x: f64,
        has_lower_limit_actual_translation_y: bool,
        the_lower_limit_actual_translation_y: f64,
        has_upper_limit_actual_translation_y: bool,
        the_upper_limit_actual_translation_y: f64,
        has_lower_limit_actual_translation_z: bool,
        the_lower_limit_actual_translation_z: f64,
        has_upper_limit_actual_translation_z: bool,
        the_upper_limit_actual_translation_z: f64,
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
        state.def_lower_limit_actual_rotation_x = has_lower_limit_actual_rotation_x;
        if state.def_lower_limit_actual_rotation_x {
            state.my_lower_limit_actual_rotation_x = the_lower_limit_actual_rotation_x;
        } else {
            state.my_lower_limit_actual_rotation_x = 0.0;
        }
        state.def_upper_limit_actual_rotation_x = has_upper_limit_actual_rotation_x;
        if state.def_upper_limit_actual_rotation_x {
            state.my_upper_limit_actual_rotation_x = the_upper_limit_actual_rotation_x;
        } else {
            state.my_upper_limit_actual_rotation_x = 0.0;
        }
        state.def_lower_limit_actual_rotation_y = has_lower_limit_actual_rotation_y;
        if state.def_lower_limit_actual_rotation_y {
            state.my_lower_limit_actual_rotation_y = the_lower_limit_actual_rotation_y;
        } else {
            state.my_lower_limit_actual_rotation_y = 0.0;
        }
        state.def_upper_limit_actual_rotation_y = has_upper_limit_actual_rotation_y;
        if state.def_upper_limit_actual_rotation_y {
            state.my_upper_limit_actual_rotation_y = the_upper_limit_actual_rotation_y;
        } else {
            state.my_upper_limit_actual_rotation_y = 0.0;
        }
        state.def_lower_limit_actual_rotation_z = has_lower_limit_actual_rotation_z;
        if state.def_lower_limit_actual_rotation_z {
            state.my_lower_limit_actual_rotation_z = the_lower_limit_actual_rotation_z;
        } else {
            state.my_lower_limit_actual_rotation_z = 0.0;
        }
        state.def_upper_limit_actual_rotation_z = has_upper_limit_actual_rotation_z;
        if state.def_upper_limit_actual_rotation_z {
            state.my_upper_limit_actual_rotation_z = the_upper_limit_actual_rotation_z;
        } else {
            state.my_upper_limit_actual_rotation_z = 0.0;
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
        state.def_lower_limit_actual_translation_z = has_lower_limit_actual_translation_z;
        if state.def_lower_limit_actual_translation_z {
            state.my_lower_limit_actual_translation_z = the_lower_limit_actual_translation_z;
        } else {
            state.my_lower_limit_actual_translation_z = 0.0;
        }
        state.def_upper_limit_actual_translation_z = has_upper_limit_actual_translation_z;
        if state.def_upper_limit_actual_translation_z {
            state.my_upper_limit_actual_translation_z = the_upper_limit_actual_translation_z;
        } else {
            state.my_upper_limit_actual_translation_z = 0.0;
        }
    }
}

step_entity! {
    /// Representation of STEP entity MechanismRepresentation
    StepKinematicsMechanismRepresentation, StepKinematicsMechanismRepresentationState, "StepKinematicsMechanismRepresentation", StepReprRepresentation, {
        my_represented_topology: StepKinematicsKinematicTopologyRepresentationSelect,
    }
}
impl StepKinematicsMechanismRepresentation {
    step_methods! {
        get represented_topology = my_represented_topology: StepKinematicsKinematicTopologyRepresentationSelect;
        set set_represented_topology = my_represented_topology: StepKinematicsKinematicTopologyRepresentationSelect;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_name: &Handle<HAsciiString>,
        the_representation_items: &Handle<HArray1<Handle<StepReprRepresentationItem>>>,
        the_representation_context_of_items: &Handle<StepReprRepresentationContext>,
        the_represented_topology: &StepKinematicsKinematicTopologyRepresentationSelect,
    ) {
        self.parent.init(
            the_representation_name,
            the_representation_items,
            the_representation_context_of_items,
        );
        self.state.write().my_represented_topology = the_represented_topology.clone();
    }
}

step_entity! {
    /// Mechanism state representation.
    StepKinematicsMechanismStateRepresentation, StepKinematicsMechanismStateRepresentationState, "StepKinematicsMechanismStateRepresentation", StepReprRepresentation, {
        my_represented_mechanism: Handle<StepKinematicsMechanismRepresentation>,
    }
}
impl StepKinematicsMechanismStateRepresentation {
    step_methods! {
        set set_mechanism = my_represented_mechanism: Handle<StepKinematicsMechanismRepresentation>;
        get mechanism = my_represented_mechanism: Handle<StepKinematicsMechanismRepresentation>;
    }
    /// `init`.
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_items: &Handle<HArray1<Handle<StepReprRepresentationItem>>>,
        the_context_of_items: &Handle<StepReprRepresentationContext>,
        the_mechanism: &Handle<StepKinematicsMechanismRepresentation>,
    ) {
        self.parent.init(the_name, the_items, the_context_of_items);
        self.state.write().my_represented_mechanism = the_mechanism.clone();
    }
}

step_entity! {
    /// Representation of STEP entity OrientedJoint
    StepKinematicsOrientedJoint, StepKinematicsOrientedJointState, "StepKinematicsOrientedJoint", StepShapeOrientedEdge, {
    }
}

step_entity! {
    /// Representation of STEP entity PairRepresentationRelationship
    StepKinematicsPairRepresentationRelationship, StepKinematicsPairRepresentationRelationshipState, "StepKinematicsPairRepresentationRelationship", StepGeomGeometricRepresentationItem, {
        my_representation_relationship_with_transformation: Handle<StepReprRepresentationRelationshipWithTransformation>,
    }
}
impl StepKinematicsPairRepresentationRelationship {
    step_methods! {
        get representation_relationship_with_transformation = my_representation_relationship_with_transformation: Handle<StepReprRepresentationRelationshipWithTransformation>;
        set set_representation_relationship_with_transformation = my_representation_relationship_with_transformation: Handle<StepReprRepresentationRelationshipWithTransformation>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_representation_relationship_name: &Handle<HAsciiString>,
        _unused_2: bool,
        the_representation_relationship_description: &Handle<HAsciiString>,
        the_representation_relationship_rep1: &StepReprRepresentationOrRepresentationReference,
        the_representation_relationship_rep2: &StepReprRepresentationOrRepresentationReference,
        the_representation_relationship_with_transformation_transformation_operator: &StepReprTransformation,
    ) {
        self.parent.init(the_representation_item_name);
        self.state
            .write()
            .my_representation_relationship_with_transformation =
            Handle::new(StepReprRepresentationRelationshipWithTransformation::new());
        let field_my_representation_relationship_with_transformation = self
            .state
            .read()
            .my_representation_relationship_with_transformation
            .clone();
        field_my_representation_relationship_with_transformation.init(
            the_representation_relationship_name,
            the_representation_relationship_description,
            &the_representation_relationship_rep1.representation(),
            &the_representation_relationship_rep2.representation(),
            the_representation_relationship_with_transformation_transformation_operator,
        );
    }
}

step_entity! {
    /// Representation of STEP entity PairValue
    StepKinematicsPairValue, StepKinematicsPairValueState, "StepKinematicsPairValue", StepGeomGeometricRepresentationItem, {
        my_applies_to_pair: Handle<StepKinematicsKinematicPair>,
    }
}
impl StepKinematicsPairValue {
    step_methods! {
        get applies_to_pair = my_applies_to_pair: Handle<StepKinematicsKinematicPair>;
        set set_applies_to_pair = my_applies_to_pair: Handle<StepKinematicsKinematicPair>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_applies_to_pair: &Handle<StepKinematicsKinematicPair>,
    ) {
        self.parent.init(the_representation_item_name);
        self.state.write().my_applies_to_pair = the_applies_to_pair.clone();
    }
}
