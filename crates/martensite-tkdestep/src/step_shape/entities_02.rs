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
    /// Closed shell.
    StepShapeClosedShell, StepShapeClosedShellState, "StepShapeClosedShell", StepShapeConnectedFaceSet, {
    }
}

step_entity! {
    /// Representation of STEP entity CompoundShapeRepresentation
    StepShapeCompoundShapeRepresentation, StepShapeCompoundShapeRepresentationState, "StepShapeCompoundShapeRepresentation", StepShapeShapeRepresentation, {
    }
}

step_entity! {
    /// Representation of STEP entity ConnectedEdgeSet
    StepShapeConnectedEdgeSet, StepShapeConnectedEdgeSetState, "StepShapeConnectedEdgeSet", StepShapeTopologicalRepresentationItem, {
        the_ces_edges: Handle<HArray1<Handle<StepShapeEdge>>>,
    }
}
impl StepShapeConnectedEdgeSet {
    step_methods! {
        get ces_edges = the_ces_edges: Handle<HArray1<Handle<StepShapeEdge>>>;
        set set_ces_edges = the_ces_edges: Handle<HArray1<Handle<StepShapeEdge>>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_ces_edges: &Handle<HArray1<Handle<StepShapeEdge>>>,
    ) {
        self.parent.init(a_representation_item_name);
        self.state.write().the_ces_edges = a_ces_edges.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ConnectedFaceShapeRepresentation
    StepShapeConnectedFaceShapeRepresentation, StepShapeConnectedFaceShapeRepresentationState, "StepShapeConnectedFaceShapeRepresentation", StepReprRepresentation, {
    }
}

step_entity! {
    /// Representation of STEP entity ConnectedFaceSubSet
    StepShapeConnectedFaceSubSet, StepShapeConnectedFaceSubSetState, "StepShapeConnectedFaceSubSet", StepShapeConnectedFaceSet, {
        the_parent_face_set: Handle<StepShapeConnectedFaceSet>,
    }
}
impl StepShapeConnectedFaceSubSet {
    step_methods! {
        get parent_face_set = the_parent_face_set: Handle<StepShapeConnectedFaceSet>;
        set set_parent_face_set = the_parent_face_set: Handle<StepShapeConnectedFaceSet>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_connected_face_set_cfs_faces: &Handle<HArray1<Handle<StepShapeFace>>>,
        a_parent_face_set: &Handle<StepShapeConnectedFaceSet>,
    ) {
        self.parent
            .init(a_representation_item_name, a_connected_face_set_cfs_faces);
        self.state.write().the_parent_face_set = a_parent_face_set.clone();
    }
}

step_entity! {
    /// Context dependent shape representation.
    StepShapeContextDependentShapeRepresentation, StepShapeContextDependentShapeRepresentationState, "StepShapeContextDependentShapeRepresentation", StandardTransient, {
        the_rep_rel: Handle<StepReprShapeRepresentationRelationship>,
        the_pro_rel: Handle<StepReprProductDefinitionShape>,
    }
}
impl StepShapeContextDependentShapeRepresentation {
    step_methods! {
        get representation_relation = the_rep_rel: Handle<StepReprShapeRepresentationRelationship>;
        set set_representation_relation = the_rep_rel: Handle<StepReprShapeRepresentationRelationship>;
        get represented_product_relation = the_pro_rel: Handle<StepReprProductDefinitionShape>;
        set set_represented_product_relation = the_pro_rel: Handle<StepReprProductDefinitionShape>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_rep_rel: &Handle<StepReprShapeRepresentationRelationship>,
        a_pro_rel: &Handle<StepReprProductDefinitionShape>,
    ) {
        let mut state = self.state.write();
        state.the_rep_rel = a_rep_rel.clone();
        state.the_pro_rel = a_pro_rel.clone();
    }
}

step_entity! {
    /// Csg shape representation.
    StepShapeCsgShapeRepresentation, StepShapeCsgShapeRepresentationState, "StepShapeCsgShapeRepresentation", StepShapeShapeRepresentation, {
    }
}

step_entity! {
    /// Csg solid.
    StepShapeCsgSolid, StepShapeCsgSolidState, "StepShapeCsgSolid", StepShapeSolidModel, {
        tree_root_expression: StepShapeCsgSelect,
    }
}
impl StepShapeCsgSolid {
    step_methods! {
        set set_tree_root_expression = tree_root_expression: StepShapeCsgSelect;
        get tree_root_expression = tree_root_expression: StepShapeCsgSelect;
    }
    /// `init`.
    pub fn init(&self, a_name: &Handle<HAsciiString>, a_tree_root_expression: &StepShapeCsgSelect) {
        self.state.write().tree_root_expression = a_tree_root_expression.clone();
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Implements complex type
    /// (DEFINITIONAL_REPRESENTATION,REPRESENTATION,SHAPE_REPRESENTATION)
    StepShapeDefinitionalRepresentationAndShapeRepresentation, StepShapeDefinitionalRepresentationAndShapeRepresentationState, "StepShapeDefinitionalRepresentationAndShapeRepresentation", StepReprDefinitionalRepresentation, {
    }
}

step_entity! {
    /// Representation of STEP entity DimensionalCharacteristicRepresentation
    StepShapeDimensionalCharacteristicRepresentation, StepShapeDimensionalCharacteristicRepresentationState, "StepShapeDimensionalCharacteristicRepresentation", StandardTransient, {
        the_dimension: StepShapeDimensionalCharacteristic,
        the_representation: Handle<StepShapeShapeDimensionRepresentation>,
    }
}
impl StepShapeDimensionalCharacteristicRepresentation {
    step_methods! {
        get dimension = the_dimension: StepShapeDimensionalCharacteristic;
        set set_dimension = the_dimension: StepShapeDimensionalCharacteristic;
        get representation = the_representation: Handle<StepShapeShapeDimensionRepresentation>;
        set set_representation = the_representation: Handle<StepShapeShapeDimensionRepresentation>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_dimension: &StepShapeDimensionalCharacteristic,
        a_representation: &Handle<StepShapeShapeDimensionRepresentation>,
    ) {
        let mut state = self.state.write();
        state.the_dimension = a_dimension.clone();
        state.the_representation = a_representation.clone();
    }
}

step_entity! {
    /// Representation of STEP entity DimensionalLocation
    StepShapeDimensionalLocation, StepShapeDimensionalLocationState, "StepShapeDimensionalLocation", StepReprShapeAspectRelationship, {
    }
}

step_entity! {
    /// Representation of STEP entity DimensionalLocationWithPath
    StepShapeDimensionalLocationWithPath, StepShapeDimensionalLocationWithPathState, "StepShapeDimensionalLocationWithPath", StepShapeDimensionalLocation, {
        the_path: Handle<StepReprShapeAspect>,
    }
}
impl StepShapeDimensionalLocationWithPath {
    step_methods! {
        get path = the_path: Handle<StepReprShapeAspect>;
        set set_path = the_path: Handle<StepReprShapeAspect>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_shape_aspect_relationship_name: &Handle<HAsciiString>,
        has_shape_aspect_relationship_description: bool,
        a_shape_aspect_relationship_description: &Handle<HAsciiString>,
        a_shape_aspect_relationship_relating_shape_aspect: &Handle<StepReprShapeAspect>,
        a_shape_aspect_relationship_related_shape_aspect: &Handle<StepReprShapeAspect>,
        a_path: &Handle<StepReprShapeAspect>,
    ) {
        self.parent.init(
            a_shape_aspect_relationship_name,
            has_shape_aspect_relationship_description,
            a_shape_aspect_relationship_description,
            a_shape_aspect_relationship_relating_shape_aspect,
            a_shape_aspect_relationship_related_shape_aspect,
        );
        self.state.write().the_path = a_path.clone();
    }
}

step_entity! {
    /// Representation of STEP entity DimensionalSize
    StepShapeDimensionalSize, StepShapeDimensionalSizeState, "StepShapeDimensionalSize", StandardTransient, {
        the_applies_to: Handle<StepReprShapeAspect>,
        the_name: Handle<HAsciiString>,
    }
}
impl StepShapeDimensionalSize {
    step_methods! {
        get applies_to = the_applies_to: Handle<StepReprShapeAspect>;
        set set_applies_to = the_applies_to: Handle<StepReprShapeAspect>;
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(&self, a_applies_to: &Handle<StepReprShapeAspect>, a_name: &Handle<HAsciiString>) {
        let mut state = self.state.write();
        state.the_applies_to = a_applies_to.clone();
        state.the_name = a_name.clone();
    }
}

step_entity! {
    /// Representation of STEP entity DimensionalSizeWithPath
    StepShapeDimensionalSizeWithPath, StepShapeDimensionalSizeWithPathState, "StepShapeDimensionalSizeWithPath", StepShapeDimensionalSize, {
        the_path: Handle<StepReprShapeAspect>,
    }
}
impl StepShapeDimensionalSizeWithPath {
    step_methods! {
        get path = the_path: Handle<StepReprShapeAspect>;
        set set_path = the_path: Handle<StepReprShapeAspect>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_dimensional_size_applies_to: &Handle<StepReprShapeAspect>,
        a_dimensional_size_name: &Handle<HAsciiString>,
        a_path: &Handle<StepReprShapeAspect>,
    ) {
        self.parent
            .init(a_dimensional_size_applies_to, a_dimensional_size_name);
        self.state.write().the_path = a_path.clone();
    }
}

step_entity! {
    /// Representation of STEP entity DirectedDimensionalLocation
    StepShapeDirectedDimensionalLocation, StepShapeDirectedDimensionalLocationState, "StepShapeDirectedDimensionalLocation", StepShapeDimensionalLocation, {
    }
}

step_entity! {
    /// Representation of STEP entity EdgeBasedWireframeModel
    StepShapeEdgeBasedWireframeModel, StepShapeEdgeBasedWireframeModelState, "StepShapeEdgeBasedWireframeModel", StepGeomGeometricRepresentationItem, {
        the_ebwm_boundary: Handle<HArray1<Handle<StepShapeConnectedEdgeSet>>>,
    }
}
impl StepShapeEdgeBasedWireframeModel {
    step_methods! {
        get ebwm_boundary = the_ebwm_boundary: Handle<HArray1<Handle<StepShapeConnectedEdgeSet>>>;
        set set_ebwm_boundary = the_ebwm_boundary: Handle<HArray1<Handle<StepShapeConnectedEdgeSet>>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_ebwm_boundary: &Handle<HArray1<Handle<StepShapeConnectedEdgeSet>>>,
    ) {
        self.parent.init(a_representation_item_name);
        self.state.write().the_ebwm_boundary = a_ebwm_boundary.clone();
    }
}

step_entity! {
    /// Representation of STEP entity EdgeBasedWireframeShapeRepresentation
    StepShapeEdgeBasedWireframeShapeRepresentation, StepShapeEdgeBasedWireframeShapeRepresentationState, "StepShapeEdgeBasedWireframeShapeRepresentation", StepShapeShapeRepresentation, {
    }
}
