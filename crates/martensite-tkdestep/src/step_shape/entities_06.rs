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
    /// Representation of STEP entity SeamEdge
    StepShapeSeamEdge, StepShapeSeamEdgeState, "StepShapeSeamEdge", StepShapeOrientedEdge, {
        the_pcurve_reference: Handle<StepGeomPcurve>,
    }
}
impl StepShapeSeamEdge {
    step_methods! {
        get pcurve_reference = the_pcurve_reference: Handle<StepGeomPcurve>;
        set set_pcurve_reference = the_pcurve_reference: Handle<StepGeomPcurve>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_oriented_edge_edge_element: &Handle<StepShapeEdge>,
        a_oriented_edge_orientation: bool,
        a_pcurve_reference: &Handle<StepGeomPcurve>,
    ) {
        self.parent.init(
            a_representation_item_name,
            a_oriented_edge_edge_element,
            a_oriented_edge_orientation,
        );
        self.state.write().the_pcurve_reference = a_pcurve_reference.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ShapeDefinitionRepresentation
    StepShapeShapeDefinitionRepresentation, StepShapeShapeDefinitionRepresentationState, "StepShapeShapeDefinitionRepresentation", StepReprPropertyDefinitionRepresentation, {
    }
}

step_entity! {
    /// Representation of STEP entity ShapeDimensionRepresentation
    StepShapeShapeDimensionRepresentation, StepShapeShapeDimensionRepresentationState, "StepShapeShapeDimensionRepresentation", StepShapeShapeRepresentation, {
        items_ap242: Handle<HArray1<StepShapeShapeDimensionRepresentationItem>>,
    }
}
impl StepShapeShapeDimensionRepresentation {
    step_methods! {
        set set_items_ap242 = items_ap242: Handle<HArray1<StepShapeShapeDimensionRepresentationItem>>;
        get items_ap242 = items_ap242: Handle<HArray1<StepShapeShapeDimensionRepresentationItem>>;
    }
    /// `init`.
    /// Initialize all fields AP214
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_items: &Handle<HArray1<Handle<StepReprRepresentationItem>>>,
        the_context_of_items: &Handle<StepReprRepresentationContext>,
    ) {
        self.parent
            .parent
            .init(the_name, the_items, the_context_of_items);
    }
    /// `init`.
    /// Initialize all fields AP242
    pub fn init_fields(
        &self,
        the_name: &Handle<HAsciiString>,
        the_items: &Handle<HArray1<StepShapeShapeDimensionRepresentationItem>>,
        the_context_of_items: &Handle<StepReprRepresentationContext>,
    ) {
        self.parent
            .parent
            .init(the_name, &Default::default(), the_context_of_items);
        self.state.write().items_ap242 = the_items.clone();
    }
}

step_entity! {
    /// Shape representation.
    StepShapeShapeRepresentation, StepShapeShapeRepresentationState, "StepShapeShapeRepresentation", StepReprRepresentation, {
    }
}

step_entity! {
    /// Representation of STEP entity ShapeRepresentationWithParameters
    StepShapeShapeRepresentationWithParameters, StepShapeShapeRepresentationWithParametersState, "StepShapeShapeRepresentationWithParameters", StepShapeShapeRepresentation, {
    }
}

step_entity! {
    /// Shell based surface model.
    StepShapeShellBasedSurfaceModel, StepShapeShellBasedSurfaceModelState, "StepShapeShellBasedSurfaceModel", StepGeomGeometricRepresentationItem, {
        sbsm_boundary: Handle<HArray1<StepShapeShell>>,
    }
}
impl StepShapeShellBasedSurfaceModel {
    step_methods! {
        set set_sbsm_boundary = sbsm_boundary: Handle<HArray1<StepShapeShell>>;
        get sbsm_boundary = sbsm_boundary: Handle<HArray1<StepShapeShell>>;
        item sbsm_boundary_value = sbsm_boundary: StepShapeShell;
        len_checked nb_sbsm_boundary = sbsm_boundary;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_sbsm_boundary: &Handle<HArray1<StepShapeShell>>,
    ) {
        self.state.write().sbsm_boundary = a_sbsm_boundary.clone();
        self.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Solid model.
    StepShapeSolidModel, StepShapeSolidModelState, "StepShapeSolidModel", StepGeomGeometricRepresentationItem, {
    }
}

step_entity! {
    /// Solid replica.
    StepShapeSolidReplica, StepShapeSolidReplicaState, "StepShapeSolidReplica", StepShapeSolidModel, {
        parent_solid: Handle<StepShapeSolidModel>,
        transformation: Handle<StepGeomCartesianTransformationOperator3d>,
    }
}
impl StepShapeSolidReplica {
    step_methods! {
        set set_parent_solid = parent_solid: Handle<StepShapeSolidModel>;
        get parent_solid = parent_solid: Handle<StepShapeSolidModel>;
        set set_transformation = transformation: Handle<StepGeomCartesianTransformationOperator3d>;
        get transformation = transformation: Handle<StepGeomCartesianTransformationOperator3d>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_parent_solid: &Handle<StepShapeSolidModel>,
        a_transformation: &Handle<StepGeomCartesianTransformationOperator3d>,
    ) {
        let mut state = self.state.write();
        state.parent_solid = a_parent_solid.clone();
        state.transformation = a_transformation.clone();
        drop(state);
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Sphere.
    StepShapeSphere, StepShapeSphereState, "StepShapeSphere", StepGeomGeometricRepresentationItem, {
        radius: f64,
        centre: Handle<StepGeomPoint>,
    }
}
impl StepShapeSphere {
    step_methods! {
        set_copy set_radius = radius: f64;
        get radius = radius: f64;
        set set_centre = centre: Handle<StepGeomPoint>;
        get centre = centre: Handle<StepGeomPoint>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_radius: f64,
        a_centre: &Handle<StepGeomPoint>,
    ) {
        let mut state = self.state.write();
        state.radius = a_radius;
        state.centre = a_centre.clone();
        drop(state);
        self.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Representation of STEP entity Subedge
    StepShapeSubedge, StepShapeSubedgeState, "StepShapeSubedge", StepShapeEdge, {
        the_parent_edge: Handle<StepShapeEdge>,
    }
}
impl StepShapeSubedge {
    step_methods! {
        get parent_edge = the_parent_edge: Handle<StepShapeEdge>;
        set set_parent_edge = the_parent_edge: Handle<StepShapeEdge>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_edge_edge_start: &Handle<StepShapeVertex>,
        a_edge_edge_end: &Handle<StepShapeVertex>,
        a_parent_edge: &Handle<StepShapeEdge>,
    ) {
        self.parent.init(
            a_representation_item_name,
            a_edge_edge_start,
            a_edge_edge_end,
        );
        self.state.write().the_parent_edge = a_parent_edge.clone();
    }
}

step_entity! {
    /// Representation of STEP entity Subface
    StepShapeSubface, StepShapeSubfaceState, "StepShapeSubface", StepShapeFace, {
        the_parent_face: Handle<StepShapeFace>,
    }
}
impl StepShapeSubface {
    step_methods! {
        get parent_face = the_parent_face: Handle<StepShapeFace>;
        set set_parent_face = the_parent_face: Handle<StepShapeFace>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_face_bounds: &Handle<HArray1<Handle<StepShapeFaceBound>>>,
        a_parent_face: &Handle<StepShapeFace>,
    ) {
        self.parent.init(a_representation_item_name, a_face_bounds);
        self.state.write().the_parent_face = a_parent_face.clone();
    }
}

step_entity! {
    /// Swept area solid.
    StepShapeSweptAreaSolid, StepShapeSweptAreaSolidState, "StepShapeSweptAreaSolid", StepShapeSolidModel, {
        swept_area: Handle<StepGeomCurveBoundedSurface>,
    }
}
impl StepShapeSweptAreaSolid {
    step_methods! {
        set set_swept_area = swept_area: Handle<StepGeomCurveBoundedSurface>;
        get swept_area = swept_area: Handle<StepGeomCurveBoundedSurface>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_swept_area: &Handle<StepGeomCurveBoundedSurface>,
    ) {
        self.state.write().swept_area = a_swept_area.clone();
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Swept face solid.
    StepShapeSweptFaceSolid, StepShapeSweptFaceSolidState, "StepShapeSweptFaceSolid", StepShapeSolidModel, {
        swept_area: Handle<StepShapeFaceSurface>,
    }
}
impl StepShapeSweptFaceSolid {
    /// `init`.
    pub fn init(&self, a_name: &Handle<HAsciiString>, a_swept_area: &Handle<StepShapeFaceSurface>) {
        self.state.write().swept_area = a_swept_area.clone();
        self.parent.parent.parent.init(a_name);
    }
    /// `set_swept_face`.
    pub fn set_swept_face(&self, a_swept_area: &Handle<StepShapeFaceSurface>) {
        self.state.write().swept_area = a_swept_area.clone();
    }
    /// `swept_face`.
    pub fn swept_face(&self) -> Handle<StepShapeFaceSurface> {
        self.state.read().swept_area.clone()
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepShapeToleranceValue, StepShapeToleranceValueState, "StepShapeToleranceValue", StandardTransient, {
        the_lower_bound: Handle<StandardTransient>,
        the_upper_bound: Handle<StandardTransient>,
    }
}
impl StepShapeToleranceValue {
    step_methods! {
        get lower_bound = the_lower_bound: Handle<StandardTransient>;
        set set_lower_bound = the_lower_bound: Handle<StandardTransient>;
        get upper_bound = the_upper_bound: Handle<StandardTransient>;
        set set_upper_bound = the_upper_bound: Handle<StandardTransient>;
    }
    /// `init`.
    pub fn init(
        &self,
        lower_bound: &Handle<StandardTransient>,
        upper_bound: &Handle<StandardTransient>,
    ) {
        let mut state = self.state.write();
        state.the_lower_bound = lower_bound.clone();
        state.the_upper_bound = upper_bound.clone();
    }
}

step_entity! {
    /// Topological representation item.
    StepShapeTopologicalRepresentationItem, StepShapeTopologicalRepresentationItemState, "StepShapeTopologicalRepresentationItem", StepReprRepresentationItem, {
    }
}
